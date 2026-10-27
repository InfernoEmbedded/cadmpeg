// SPDX-License-Identifier: Apache-2.0

use crate::deltas::census::walk;
use crate::records::instance::SchemaNode;
use crate::test_support::with_decode_context;

fn reference(bytes: &mut Vec<u8>, target: u16, status: u8) {
    bytes.extend_from_slice(&target.to_be_bytes());
    bytes.push(status);
}

fn shell() -> Vec<u8> {
    let mut bytes = vec![0, 13, 0, 80];
    bytes.extend_from_slice(&91u32.to_be_bytes());
    for _ in 0..8 {
        reference(&mut bytes, 1, 1);
    }
    bytes
}

fn face() -> Vec<u8> {
    let mut bytes = vec![0, 14, 0, 81];
    bytes.extend_from_slice(&92u32.to_be_bytes());
    reference(&mut bytes, 1, 1);
    bytes.extend_from_slice(&0.0f64.to_be_bytes());
    for _ in 0..5 {
        reference(&mut bytes, 1, 1);
    }
    bytes.push(b'+');
    for _ in 0..5 {
        reference(&mut bytes, 1, 1);
    }
    bytes
}

fn body(status: u8, field_count: u8) -> Vec<u8> {
    let mut bytes = vec![0, 7];
    bytes.extend_from_slice(&123u32.to_be_bytes());
    for _ in 0..6 {
        reference(&mut bytes, 1, status);
    }
    for value in [0.125f64, 0.25] {
        bytes.extend_from_slice(&value.to_be_bytes());
    }
    for _ in 0..3 {
        reference(&mut bytes, 1, status);
    }
    bytes.push(0);
    reference(&mut bytes, 2, status);
    bytes.extend_from_slice(&[1, 1]);
    for _ in 0..7 {
        reference(&mut bytes, 1, status);
    }
    let mut tail = if field_count == 36 { face() } else { shell() };
    tail.resize(usize::from(field_count - 23) * 4, 0);
    bytes.extend(tail);
    bytes
}

fn declaration(field_count: u8) -> Vec<u8> {
    let mut bytes = vec![0, 12, field_count];
    bytes.extend(std::iter::repeat_n(b'C', 23));
    for index in 0..field_count - 23 {
        let name = format!("tail_{index}");
        bytes.extend_from_slice(&[b'A', u8::try_from(name.len()).unwrap()]);
        bytes.extend_from_slice(name.as_bytes());
        bytes.extend_from_slice(&[0, 0, 0, 1, 1, b'd']);
    }
    bytes.push(b'Z');
    bytes.extend(body(1, field_count));
    bytes
}

#[test]
fn body_declared_fields_own_embedded_shell_and_face_and_leave_real_successor() {
    with_decode_context(|_ctx| {
        for field_count in [34, 35, 36] {
            let mut bytes = declaration(field_count);
            let start = bytes.len();
            bytes.extend_from_slice(&[0, 12]);
            bytes.extend(body(0, field_count));
            let end = bytes.len();
            bytes.extend(shell());
            let census = walk(&bytes).unwrap();
            assert_eq!(census.schema_nodes.len(), 1);
            assert_eq!(census.body_revisions.len(), 1);
            let revision = &census.body_revisions[0];
            assert_eq!((revision.offset, revision.end), (start, end));
            assert_eq!(revision.node_id, 123);
            assert_eq!(revision.references, [1; 8]);
            assert!(revision.prefix_end < end);
            assert!(census
                .tombstones
                .iter()
                .all(|r| r.offset < start || r.offset >= end));
            assert_eq!(census.records.len(), 1);
            assert_eq!(
                (census.records[0].offset, census.records[0].end),
                (end, bytes.len())
            );
            assert_eq!(census.records[0].xmt, 80);
        }
    });
}

#[test]
fn body_schema_rejects_incomplete_field_invalid_status_and_nonfinite_real() {
    with_decode_context(|_ctx| {
        let (node, _) = SchemaNode::read(&declaration(36), 0).unwrap().unwrap();
        let mut bytes = vec![0, 12];
        bytes.extend(body(1, 36));
        let schema = node.definition();
        assert_eq!(
            schema.read_body_instance(&bytes, 0).unwrap().end,
            bytes.len()
        );
        for length in 0..bytes.len() {
            assert!(schema.read_body_instance(&bytes[..length], 0).is_none());
        }
        let mut extended = bytes.clone();
        extended.splice(8..10, [0xff, 0xfe, 0, 2]);
        let parsed = schema.read_body_instance(&extended, 0).unwrap();
        assert_eq!(parsed.references[0], 65_536);
        assert_eq!(parsed.end, bytes.len() + 2);
        let mut invalid = bytes.clone();
        invalid[10] = 2;
        assert!(schema.read_body_instance(&invalid, 0).is_none());
        let mut invalid = bytes.clone();
        invalid[26..34].copy_from_slice(&f64::NAN.to_be_bytes());
        assert!(schema.read_body_instance(&invalid, 0).is_none());
        let mut truncated = declaration(36);
        let start = truncated.len();
        truncated.extend_from_slice(&bytes[..bytes.len() - 1]);
        assert!(walk(&truncated)
            .unwrap()
            .body_revisions
            .iter()
            .all(|r| r.offset != start));
    });
}

#[test]
fn body_schema_keeps_owned_fixed_fields_and_full_highest_id_domain() {
    with_decode_context(|_ctx| {
        let (node, _) = SchemaNode::read(&declaration(36), 0).unwrap().unwrap();
        let wire = serde_json::to_value(node.definition()).unwrap();
        let mut bytes = vec![0, 12];
        bytes.extend(body(1, 36));
        bytes[4..8].fill(0);
        assert_eq!(
            node.definition()
                .read_body_instance(&bytes, 0)
                .unwrap()
                .node_id,
            0
        );
        for (index, field, value) in [
            (0, "name", serde_json::json!("unassigned")),
            (0, "data_type", serde_json::json!("n")),
            (23, "n_elements", serde_json::json!(1)),
            (23, "n_elements", serde_json::json!(2)),
        ] {
            let mut invalid = wire.clone();
            invalid["fields"][index][field] = value;
            let schema: crate::records::EmbeddedSchema = serde_json::from_value(invalid).unwrap();
            assert!(schema.read_body_instance(&bytes, 0).is_none());
        }
        let mut untransmitted = wire.clone();
        untransmitted["fields"][23]["n_elements"] = serde_json::json!(1);
        untransmitted["fields"][23]["transmit"] = serde_json::json!(false);
        let schema: crate::records::EmbeddedSchema = serde_json::from_value(untransmitted).unwrap();
        assert!(schema.read_body_instance(&bytes, 0).is_none());
        let mut raw_pointer_class = wire;
        raw_pointer_class["fields"][1]["pointer_class"] = serde_json::json!(999);
        let schema: crate::records::EmbeddedSchema =
            serde_json::from_value(raw_pointer_class).unwrap();
        assert_eq!(
            schema.read_body_instance(&bytes, 0).unwrap().references,
            [1; 8]
        );
        let mut unowned = declaration(36);
        unowned.push(0x7f);
        let start = unowned.len();
        unowned.extend(bytes);
        assert!(walk(&unowned)
            .unwrap()
            .body_revisions
            .iter()
            .all(|r| r.offset != start));
    });
}

#[test]
fn complete_body_with_empty_tail_does_not_claim_a_following_opaque_gap() {
    with_decode_context(|_ctx| {
        let mut declaration = vec![0, 12, 9];
        declaration.extend(std::iter::repeat_n(b'D', 23));
        for index in 0..9 {
            let name = if index == 0 {
                "highest_node_id".to_owned()
            } else {
                format!("raw_pointer_{index}")
            };
            declaration.extend_from_slice(&[b'A', u8::try_from(name.len()).unwrap()]);
            declaration.extend_from_slice(name.as_bytes());
            if index == 0 {
                declaration.extend_from_slice(&[0, 0, 0, 1, 1, b'd']);
            } else {
                declaration.extend_from_slice(&[0, 12, 0, 1]);
            }
        }
        declaration.push(b'Z');
        let mut value = vec![0, 7];
        value.extend_from_slice(&123u32.to_be_bytes());
        for _ in 0..8 {
            reference(&mut value, 1, 1);
        }
        declaration.extend_from_slice(&value);
        let start = declaration.len();
        declaration.extend_from_slice(&[0, 12]);
        declaration.extend_from_slice(&value);
        let end = declaration.len();
        declaration.extend_from_slice(&[0x7f; 5]);
        let census = walk(&declaration).unwrap();
        assert_eq!(census.body_revisions.len(), 1);
        let revision = &census.body_revisions[0];
        assert!(revision.schema_complete);
        assert_eq!(
            (revision.offset, revision.prefix_end, revision.end),
            (start, end, end)
        );

        let mut legacy = vec![0, 12];
        legacy.extend(value);
        let prefix_end = legacy.len();
        legacy.extend_from_slice(&[0x7f; 5]);
        let census = walk(&legacy).unwrap();
        assert_eq!(census.body_revisions.len(), 1);
        let revision = &census.body_revisions[0];
        assert!(!revision.schema_complete);
        assert_eq!(revision.prefix_end, prefix_end);
        assert_eq!(revision.end, legacy.len());
    });
}
