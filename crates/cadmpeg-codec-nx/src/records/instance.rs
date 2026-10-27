// SPDX-License-Identifier: Apache-2.0

use super::{DataType, EmbeddedSchema};
use crate::framing::read_and_advance;
use crate::framing::xmt_reference::NonNullXmt;
use cadmpeg_core::decode::View;
use cadmpeg_core::CodecError;
use cadmpeg_ir::scalar::FiniteReal;
use cadmpeg_ir::units::FiniteVector;
use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", content = "value", rename_all = "snake_case")]
pub(crate) enum FieldValue {
    Byte(u8),
    Char(u8),
    Logical(bool),
    Short(u16),
    Word(u16),
    Integer(u32),
    Pointer { target: NonZeroU32, status: bool },
    Real(FiniteReal),
    Interval(FiniteVector<2>),
    Vector(FiniteVector<3>),
    Box(FiniteVector<6>),
    Hvec(FiniteVector<11>),
}

impl FieldValue {
    fn data_type(&self) -> DataType {
        match self {
            Self::Byte(_) => DataType::Byte,
            Self::Char(_) => DataType::Char,
            Self::Logical(_) => DataType::Logical,
            Self::Short(_) => DataType::Short,
            Self::Word(_) => DataType::Word,
            Self::Integer(_) => DataType::Integer,
            Self::Pointer { .. } => DataType::Pointer,
            Self::Real(_) => DataType::Real,
            Self::Interval(_) => DataType::Interval,
            Self::Vector(_) => DataType::Vector,
            Self::Box(_) => DataType::Box,
            Self::Hvec(_) => DataType::Hvec,
        }
    }

    fn read(kind: DataType, stream: &[u8], view: &mut View<'_>) -> Option<Self> {
        Some(match kind {
            DataType::Byte => Self::Byte(view.u8()?),
            DataType::Char => Self::Char(view.u8()?),
            DataType::Logical => Self::Logical(match view.u8()? {
                0 => false,
                1 => true,
                _ => return None,
            }),
            DataType::Short => Self::Short(view.u16_be()?),
            DataType::Word => Self::Word(view.u16_be()?),
            DataType::Integer => Self::Integer(view.u32_be()?),
            DataType::Pointer => {
                let mut at = view.position();
                let target = NonZeroU32::new(read_and_advance(stream, &mut at)?)?;
                view.seek(at)?;
                let status = match view.u8()? {
                    0 => false,
                    1 => true,
                    _ => return None,
                };
                Self::Pointer { target, status }
            }
            DataType::Real => Self::Real(FiniteReal::new(view.f64_be()?)?),
            DataType::Interval => Self::Interval(vector(view)?),
            DataType::Vector => Self::Vector(vector(view)?),
            DataType::Box => Self::Box(vector(view)?),
            DataType::Hvec => Self::Hvec(vector(view)?),
            DataType::Unestablished(_) => return None,
        })
    }
}

fn vector<const N: usize>(view: &mut View<'_>) -> Option<FiniteVector<N>> {
    let mut values = [0.0; N];
    for value in &mut values {
        *value = view.f64_be()?;
    }
    FiniteVector::new(values)
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(try_from = "NodeWire")]
pub(crate) struct SchemaNode {
    definition: EmbeddedSchema,
    schema_byte_len: usize,
    xmt: NonNullXmt,
    variable_length: Option<u32>,
    values: Vec<Vec<FieldValue>>,
}

#[derive(Deserialize)]
struct NodeWire {
    definition: EmbeddedSchema,
    schema_byte_len: usize,
    xmt: NonNullXmt,
    variable_length: Option<u32>,
    values: Vec<Vec<FieldValue>>,
}

impl TryFrom<NodeWire> for SchemaNode {
    type Error = &'static str;
    fn try_from(wire: NodeWire) -> Result<Self, Self::Error> {
        if wire.schema_byte_len < 3 || wire.definition.plan.fields.is_empty() {
            return Err("definition: requires bounded nonempty schema metadata");
        }
        let variable = wire
            .definition
            .plan
            .fields
            .iter()
            .any(|field| field.n_elements == 1);
        if variable != wire.variable_length.is_some()
            || wire.values.len() != wire.definition.plan.fields.len()
        {
            return Err("values: effective field cardinality differs from definition");
        }
        for (field, values) in wire.definition.plan.fields.iter().zip(&wire.values) {
            let count = if !field.transmit {
                0
            } else if field.n_elements == 1 {
                wire.variable_length.ok_or("variable_length: missing")?
            } else {
                field.n_elements.max(1)
            };
            if usize::try_from(count).ok() != Some(values.len())
                || values.iter().any(|v| v.data_type() != field.data_type)
            {
                return Err("values: field type or element count differs from definition");
            }
        }
        Ok(Self {
            definition: wire.definition,
            schema_byte_len: wire.schema_byte_len,
            xmt: wire.xmt,
            variable_length: wire.variable_length,
            values: wire.values,
        })
    }
}

impl SchemaNode {
    pub(crate) fn kind(&self) -> u16 {
        self.definition.plan.kind
    }

    pub(crate) fn definition(&self) -> &EmbeddedSchema {
        &self.definition
    }

    pub(crate) fn read(stream: &[u8], offset: usize) -> Result<Option<(Self, usize)>, CodecError> {
        let Some((definition, schema_end)) = super::read(stream, offset)? else {
            return Ok(None);
        };
        let mut view = View::over_retained(stream);
        let Some((variable_length, xmt)) = (|| {
            view.seek(schema_end)?;
            let variable_length = if definition
                .plan
                .fields
                .iter()
                .any(|field| field.n_elements == 1)
            {
                Some(view.u32_be()?)
            } else {
                None
            };
            let mut at = view.position();
            let xmt = NonNullXmt::try_from(read_and_advance(stream, &mut at)?).ok()?;
            view.seek(at)?;
            Some((variable_length, xmt))
        })() else {
            return Ok(None);
        };
        let mut values = Vec::new();
        for field in &definition.plan.fields {
            let count = if !field.transmit {
                0
            } else if field.n_elements == 1 {
                variable_length.unwrap_or(0)
            } else {
                field.n_elements.max(1)
            };
            // Each element consumes at least one byte, even before its type is read.
            if usize::try_from(count)
                .ok()
                .is_none_or(|n| n > stream.len() - view.position())
            {
                return Ok(None);
            }
            let mut elements = Vec::new();
            for _ in 0..count {
                let Some(value) = FieldValue::read(field.data_type, stream, &mut view) else {
                    return Ok(None);
                };
                elements.push(value);
            }
            values.push(elements);
        }
        Ok(Some((
            Self {
                definition,
                schema_byte_len: schema_end - offset,
                xmt,
                variable_length,
                values,
            },
            view.position(),
        )))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub(crate) struct RegionInstance {
    pub(crate) xmt: NonNullXmt,
    pub(crate) node_id: u32,
    pub(crate) references: [u32; 4],
    pub(crate) canonical_bytes: Vec<u8>,
    pub(crate) end: usize,
}

impl EmbeddedSchema {
    pub(crate) fn read_region_instance(
        &self,
        stream: &[u8],
        offset: usize,
    ) -> Option<RegionInstance> {
        if self.plan.kind != 19 || View::u16_be_at(stream, offset)? != 19 {
            return None;
        }
        let transmitted = self.plan.fields.iter().filter(|field| field.transmit);
        let base = [
            ("node_id", DataType::Integer, 0),
            ("attributes_features", DataType::Pointer, 1019),
            ("body", DataType::Pointer, 12),
            ("next", DataType::Pointer, 19),
            ("previous", DataType::Pointer, 19),
            ("shell", DataType::Pointer, 13),
        ];
        let terminal = [
            ("type", DataType::Char, 0),
            ("owner", DataType::Pointer, 12),
        ];
        let has_frame = if self.matches_fields(19, base.into_iter().chain(terminal)) {
            false
        } else if self.matches_fields(
            19,
            base.into_iter()
                .chain([("frame", DataType::Pointer, 230)])
                .chain(terminal),
        ) {
            true
        } else {
            return None;
        };
        let mut at = offset.checked_add(2)?;
        let xmt = NonNullXmt::try_from(read_and_advance(stream, &mut at)?).ok()?;
        let mut canonical_bytes = stream.get(offset..at)?.to_vec();
        let mut view = View::over_retained(stream);
        view.seek(at)?;
        let mut node_id = None;
        let mut references = [0; 4];
        let mut pointer_index = 0;
        for field in transmitted {
            if field.n_elements != 0 {
                return None;
            }
            let start = view.position();
            let value = FieldValue::read(field.data_type, stream, &mut view)?;
            match (field.name.as_str(), value) {
                ("node_id", FieldValue::Integer(value)) => node_id = Some(value),
                ("type", FieldValue::Char(b'S' | b'V')) => {}
                (name, FieldValue::Pointer { target, .. })
                    if name != "node_id" && name != "type" =>
                {
                    if name == "frame" && (!has_frame || field.pointer_class != 230) {
                        return None;
                    }
                    if name == "owner" && field.pointer_class != 12 {
                        return None;
                    }
                    if pointer_index < references.len() {
                        references[pointer_index] = target.get();
                    }
                    pointer_index += 1;
                }
                _ => return None,
            }
            let end = view.position() - usize::from(field.data_type == DataType::Pointer);
            canonical_bytes.extend_from_slice(stream.get(start..end)?);
        }
        Some(RegionInstance {
            xmt,
            node_id: node_id?,
            references,
            canonical_bytes,
            end: view.position(),
        })
    }
}

#[cfg(test)]
mod tests;
