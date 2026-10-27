// SPDX-License-Identifier: Apache-2.0

use super::super::walk;
use crate::records::instance::SchemaNode;
use crate::test_support::with_decode_context;

fn reference(bytes: &mut Vec<u8>, target: u16, status: u8) {
    bytes.extend_from_slice(&target.to_be_bytes());
    bytes.push(status);
}

fn region_fields(xmt: u16, frame: bool, status: u8, marker: u8) -> Vec<u8> {
    let mut bytes = xmt.to_be_bytes().to_vec();
    bytes.extend_from_slice(&91_u32.to_be_bytes());
    for target in [1, 3, 1, 1, 8] {
        reference(&mut bytes, target, status);
    }
    if frame {
        reference(&mut bytes, 1, status);
    }
    bytes.push(marker);
    reference(&mut bytes, 1, status);
    bytes
}

fn declaration(frame: bool) -> Vec<u8> {
    let mut bytes = vec![0, 19, if frame { 9 } else { 8 }];
    bytes.extend_from_slice(b"CCCCCC");
    if frame {
        bytes.extend_from_slice(b"I\x05frame\x00\xe6\x00\x01");
    }
    bytes.extend_from_slice(b"CA\x05owner\x00\x0c\x00\x01Z");
    bytes.extend(region_fields(7, frame, 1, b'V'));
    bytes
}

fn shell() -> Vec<u8> {
    let mut bytes = vec![0, 13, 0, 8];
    bytes.extend_from_slice(&92_u32.to_be_bytes());
    for target in [1, 3, 1, 1, 1, 1, 11, 1] {
        reference(&mut bytes, target, 1);
    }
    bytes
}

#[test]
fn declared_region_fields_own_the_marker_tail_and_preserve_the_following_shell() {
    with_decode_context(|_ctx| {
        for frame in [false, true] {
            let mut bytes = declaration(frame);
            let start = bytes.len();
            bytes.extend_from_slice(&[0, 19]);
            bytes.extend(region_fields(11, frame, 1, b'S'));
            let end = bytes.len();
            bytes.extend(shell());
            let census = walk(&bytes).unwrap();
            assert_eq!(census.schema_nodes.len(), 1);
            assert_eq!(census.records.len(), 2);
            assert_eq!(
                (census.records[0].offset, census.records[0].end),
                (start, end)
            );
            assert_eq!(census.records[0].xmt, 11);
            assert_eq!(census.records[0].kind(), 19);
            assert_eq!(census.records[0].family.references(), vec![1, 3, 1, 1]);
            assert_eq!(
                (census.records[1].offset, census.records[1].end),
                (end, bytes.len())
            );
            assert_eq!(census.records[1].xmt, 8);
            assert!(census.reference_marker_packets.is_empty());
            assert!(census
                .tombstones
                .iter()
                .all(|r| r.offset < start || r.offset >= end));
            let mut canonical = vec![0, 19, 0, 11];
            canonical.extend_from_slice(&91_u32.to_be_bytes());
            for target in [1_u16, 3, 1, 1, 8] {
                canonical.extend_from_slice(&target.to_be_bytes());
            }
            if frame {
                canonical.extend_from_slice(&1_u16.to_be_bytes());
            }
            canonical.push(b'S');
            canonical.extend_from_slice(&1_u16.to_be_bytes());
            assert_eq!(census.records[0].canonical_bytes, canonical);
        }
    });
}

#[test]
fn region_schema_requires_each_complete_typed_field_and_keeps_binary_status_zero() {
    with_decode_context(|_ctx| {
        for frame in [false, true] {
            let (first, _) = SchemaNode::read(&declaration(frame), 0).unwrap().unwrap();
            let schema = first.definition();
            let mut bytes = vec![0, 19];
            bytes.extend(region_fields(11, frame, 0, b'V'));
            assert_eq!(
                schema.read_region_instance(&bytes, 0).unwrap().end,
                bytes.len()
            );
            for length in 0..bytes.len() {
                assert!(schema.read_region_instance(&bytes[..length], 0).is_none());
            }
            let mut zero_node = bytes.clone();
            zero_node[4..8].fill(0);
            assert_eq!(
                schema.read_region_instance(&zero_node, 0).unwrap().node_id,
                0
            );
            let mut extended = bytes.clone();
            extended.splice(2..4, [0xff, 0xfe, 0, 2]);
            let parsed = schema.read_region_instance(&extended, 0).unwrap();
            assert_eq!(u32::from(parsed.xmt), 65_536);
            assert_eq!(parsed.end, bytes.len() + 2);
            let mut invalid = bytes.clone();
            invalid[10] = 2;
            assert!(schema.read_region_instance(&invalid, 0).is_none());
            let mut invalid = bytes.clone();
            invalid[8..10].fill(0);
            assert!(schema.read_region_instance(&invalid, 0).is_none());
            let mut invalid = bytes.clone();
            let marker = bytes.len() - 4;
            invalid[marker] = b'X';
            assert!(schema.read_region_instance(&invalid, 0).is_none());
        }
    });
}

#[test]
fn declared_region_does_not_resynchronize_inside_unowned_bytes() {
    with_decode_context(|_ctx| {
        for frame in [false, true] {
            let mut bytes = declaration(frame);
            bytes.push(0x7f);
            bytes.extend_from_slice(&[0, 19]);
            bytes.extend(region_fields(11, frame, 1, b'S'));
            let census = walk(&bytes).unwrap();
            assert!(census.records.iter().all(|record| record.kind() != 19));
        }
    });
}

#[test]
fn declared_region_pointer_roles_require_their_source_classes() {
    with_decode_context(|_ctx| {
        for frame in [false, true] {
            let (first, _) = SchemaNode::read(&declaration(frame), 0).unwrap().unwrap();
            let mut bytes = vec![0, 19];
            bytes.extend(region_fields(11, frame, 1, b'S'));
            let wire = serde_json::to_value(first.definition()).unwrap();
            let mut checked = 0;
            for index in 0..wire["fields"].as_array().unwrap().len() {
                if wire["fields"][index]["data_type"] != "p" {
                    continue;
                }
                checked += 1;
                let mut wrong = wire.clone();
                wrong["fields"][index]["pointer_class"] = serde_json::json!(999);
                let schema: crate::records::EmbeddedSchema = serde_json::from_value(wrong).unwrap();
                assert!(schema.read_region_instance(&bytes, 0).is_none());
            }
            assert_eq!(checked, 6 + usize::from(frame));
        }
    });
}
