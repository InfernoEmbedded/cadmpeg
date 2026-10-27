// SPDX-License-Identifier: Apache-2.0
//! Closed control fields of a GROUP record.

use crate::records::group::GroupSelector;
use serde::{Deserialize, Serialize};

pub(super) fn selector(value: u8) -> Option<GroupSelector> {
    match value {
        2 => Some(GroupSelector::Face),
        4 => Some(GroupSelector::Edge),
        9 => Some(GroupSelector::Mixed),
        _ => None,
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(try_from = "u8", into = "u8")]
pub(crate) enum GroupReferenceStatus {
    Form0,
    Form1,
}
impl TryFrom<u8> for GroupReferenceStatus {
    type Error = &'static str;
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Form0),
            1 => Ok(Self::Form1),
            _ => Err("linked_reference_status: must be 0 or 1"),
        }
    }
}
impl From<GroupReferenceStatus> for u8 {
    fn from(value: GroupReferenceStatus) -> Self {
        match value {
            GroupReferenceStatus::Form0 => 0,
            GroupReferenceStatus::Form1 => 1,
        }
    }
}

#[cfg(test)]
mod tests {
    fn record(references: [u16; 5], status_framed: bool) -> Vec<u8> {
        let mut bytes = vec![0, 90, 0, 20];
        bytes.extend_from_slice(&123u32.to_be_bytes());
        for value in &references[..4] {
            bytes.extend_from_slice(&value.to_be_bytes());
            if status_framed {
                bytes.push(1);
            }
        }
        bytes.push(9);
        bytes.extend_from_slice(&references[4].to_be_bytes());
        bytes.push(0);
        bytes
    }

    #[test]
    fn group_links_allow_null_except_the_part_owner_and_reject_zero_tokens() {
        for status_framed in [false, true] {
            let read = |bytes: &[u8]| {
                if status_framed {
                    crate::deltas::group_layout_with_statuses(bytes, 0, 0)
                } else {
                    crate::deltas::group_layout_without_leading_statuses(bytes, 0, 0)
                }
            };
            let references = [1, 3, 1, 1, 1];
            let bytes = record(references, status_framed);
            let parsed = read(&bytes).unwrap();
            assert_eq!(parsed.2, references.map(u32::from));
            assert_eq!(parsed.5, bytes.len());
            for slot in 0..5 {
                let mut invalid = references;
                invalid[slot] = 0;
                assert!(read(&record(invalid, status_framed)).is_none());
            }
            let mut invalid = references;
            invalid[1] = 1;
            assert!(read(&record(invalid, status_framed)).is_none());
        }
    }
}
