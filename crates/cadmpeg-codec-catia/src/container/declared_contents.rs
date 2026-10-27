// SPDX-License-Identifier: Apache-2.0
//! Resolve declaration metadata to its explicitly named content stream.

use super::{contiguous_descriptor_range, Descriptor, InnerDir};
use crate::layout::{container_property_header as header, container_property_record as record};
use cadmpeg_core::decode::{bounded_len, index_from_u32, View};

use std::collections::BTreeMap;

pub(super) struct DirectoryNames {
    directory: usize,
    preceding_ends: BTreeMap<usize, usize>,
}

impl DirectoryNames {
    pub(super) fn new(data: &[u8], outer: &InnerDir) -> Option<Self> {
        if outer.inner != 0 {
            return None;
        }
        let directory = index_from_u32(View::u32_be_at(
            data,
            crate::layout::outer_header::DIRECTORY_OFFSET,
        )?);
        let mut ends = BTreeMap::<usize, usize>::new();
        for descriptor in &outer.descriptors {
            let end = directory
                .checked_add(descriptor.desc_offset)?
                .checked_add(crate::layout::stream_descriptor_header::LEN)?
                .checked_add(
                    descriptor
                        .extents
                        .len()
                        .checked_mul(crate::layout::extent_struct::LEN)?,
                )?;
            ends.entry(descriptor.desc_offset)
                .and_modify(|previous| *previous = (*previous).max(end))
                .or_insert(end);
        }
        let mut previous = directory.checked_add(super::DIR_MAGIC.len())?;
        for end in ends.values_mut() {
            let current = *end;
            *end = previous;
            previous = previous.max(current);
        }
        Some(Self {
            directory,
            preceding_ends: ends,
        })
    }

    fn matches(&self, data: &[u8], descriptor: &Descriptor, name: &str) -> bool {
        if !name.bytes().all(|byte| byte.is_ascii_graphic()) {
            return false;
        }
        let Some(end) = self.directory.checked_add(descriptor.desc_offset) else {
            return false;
        };
        let Some(width) = name.len().checked_mul(2).and_then(|n| n.checked_add(2)) else {
            return false;
        };
        let Some(start) = end.checked_sub(width + 3) else {
            return false;
        };
        let Some(length) = View::u16_le_at(data, start) else {
            return false;
        };
        if usize::from(length) != width || data.get(end - 3..end) != Some(&[0, 0, 0]) {
            return false;
        }
        if !(data.get(start + 2..end - 3).is_some_and(|bytes| {
            bytes
                .chunks_exact(2)
                .zip(name.bytes())
                .all(|(pair, expected)| pair == [expected, 0])
        })) {
            return false;
        }
        self.preceding_ends
            .get(&descriptor.desc_offset)
            .is_some_and(|end| start >= *end)
    }
}

fn content_name(data: &[u8], matches_descriptor: impl Fn(&str) -> bool) -> Option<String> {
    if index_from_u32(View::u32_be_at(data, header::TOTAL_SIZE)?) != data.len() {
        return None;
    }
    let count = View::u32_be_at(data, header::COUNT)?;
    let count = bounded_len(
        u64::from(count),
        record::LEN,
        data.len().checked_sub(header::LEN)?,
    )?;
    if !matches!(count, 2 | 3) {
        return None;
    }
    let mut at = header::LEN;
    let mut selected = None;
    for index in 0..count {
        let size = index_from_u32(View::u32_be_at(data, at + record::SIZE)?);
        let kind = View::u16_be_at(data, at + record::VALUE_TYPE)?;
        let width = index_from_u32(View::u32_be_at(data, at + record::VALUE_BYTES)?);
        if size != record::LEN.checked_add(width)?
            || kind != 2
            || width < 2
            || !width.is_multiple_of(2)
        {
            return None;
        }
        let end = at.checked_add(size)?;
        let value = data.get(at + record::LEN..end)?;
        if value.get(width - 2..) != Some(&[0, 0]) {
            return None;
        }
        if index + 1 < count {
            let expected = if index + 2 == count {
                "CATStorageProperty"
            } else {
                "CATPropertySets"
            };
            let units = std::iter::once(3u16)
                .chain(expected.encode_utf16())
                .chain(std::iter::once(0));
            if !value.iter().copied().eq(units.flat_map(u16::to_be_bytes)) {
                return None;
            }
        } else {
            if width > usize::from(u16::MAX) {
                return None;
            }
            let text = &value[..width - 2];
            if !text
                .chunks_exact(2)
                .all(|pair| pair[0] == 0 && pair[1].is_ascii_graphic())
            {
                return None;
            }
            let name: String = text
                .chunks_exact(2)
                .map(|pair| char::from(pair[1]))
                .collect();
            if name.is_empty() || !matches_descriptor(&name) {
                return None;
            }
            selected = Some(name);
        }
        at = end;
    }
    (at == data.len()).then_some(selected).flatten()
}

pub(super) fn resolve(
    data: &[u8],
    outer: &InnerDir,
    names: &DirectoryNames,
    uuid_name: &str,
) -> Option<String> {
    let mut metadata = outer
        .descriptors
        .iter()
        .filter(|descriptor| names.matches(data, descriptor, uuid_name));
    let descriptor = metadata.next()?;
    if metadata.next().is_some() {
        return None;
    }
    let range = contiguous_descriptor_range(descriptor, outer.inner, data.len())?;
    let target = content_name(data.get(range)?, |name| {
        outer
            .descriptors
            .iter()
            .any(|candidate| names.matches(data, candidate, name))
    })?;
    let mut streams = outer
        .descriptors
        .iter()
        .filter(|descriptor| names.matches(data, descriptor, &target));
    let content = streams.next()?;
    if streams.next().is_some() || content.desc_offset == descriptor.desc_offset {
        return None;
    }
    Some(content.name.clone())
}

#[cfg(test)]
mod tests;
