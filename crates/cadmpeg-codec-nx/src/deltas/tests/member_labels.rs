// SPDX-License-Identifier: Apache-2.0

use crate::test_support::test_bytes::encoded_xmt;

fn member(escaped: bool, xmt: u32, label: u32, references: [u16; 6]) -> Vec<u8> {
    let mut bytes = vec![0, 91];
    if escaped {
        bytes.push(255);
    }
    bytes.extend(encoded_xmt(xmt));
    bytes.extend_from_slice(&label.to_be_bytes());
    for (reference, status) in references.into_iter().zip([1, 1, 0, 0, 1, 0]) {
        bytes.extend_from_slice(&reference.to_be_bytes());
        bytes.push(status);
    }
    bytes
}

#[test]
fn member_entity_label_is_full_u32_and_keeps_exact_bits() {
    for escaped in [false, true] {
        for xmt in [20, 65_536] {
            for label in [0, 1, 2, 65_536, u32::MAX] {
                let bytes = member(escaped, xmt, label, [2, 3, 1, 1, 1, 1]);
                let record = crate::deltas::consume_type_91(&bytes, 0).unwrap();
                assert_eq!(record.xmt, xmt);
                assert_eq!(record.end, bytes.len());
                assert_eq!(record.canonical_bytes, bytes);
                assert_eq!(record.family.references(), [2, 3, 1, 1, 1, 1]);
            }
        }
    }
}

#[test]
fn complete_member_owns_interior_compact_loop_and_preserves_following_shell() {
    let mut bytes = member(false, 20, 2, [2, 3, 5, 3842, 1, 1]);
    assert_eq!(&bytes[16..22], &[0, 15, 2, 0, 0, 1]);
    let member_end = bytes.len();
    bytes.extend_from_slice(&[0, 13, 0, 80]);
    bytes.extend_from_slice(&91u32.to_be_bytes());
    for _ in 0..8 {
        bytes.extend_from_slice(&[0, 1, 1]);
    }
    crate::test_support::with_decode_context(|_ctx| {
        let census = crate::deltas::census::walk(&bytes).unwrap();
        assert_eq!(census.records.len(), 2);
        assert_eq!(census.records[0].kind(), 91);
        assert_eq!(census.records[0].end, member_end);
        assert_eq!(census.records[1].kind(), 13);
        assert_eq!(census.records[1].offset, member_end);
        assert_eq!(census.records[1].end, bytes.len());
        assert!(census.tombstones.is_empty());
    });
}

#[test]
fn member_requires_complete_storage_binary_status_and_nonnull_group_and_owner() {
    let bytes = member(false, 20, u32::MAX, [2, 3, 1, 1, 1, 1]);
    for end in 0..bytes.len() {
        assert!(crate::deltas::consume_type_91(&bytes[..end], 0).is_none());
    }
    for at in [10, 13, 16, 19, 22, 25] {
        let mut invalid = bytes.clone();
        invalid[at] = 2;
        assert!(crate::deltas::consume_type_91(&invalid, 0).is_none());
    }
    for slot in 0..6 {
        let mut references = [2, 3, 1, 1, 1, 1];
        references[slot] = 0;
        assert!(crate::deltas::consume_type_91(&member(false, 20, 2, references), 0).is_none());
    }
    for slot in 0..2 {
        let mut references = [2, 3, 1, 1, 1, 1];
        references[slot] = 1;
        assert!(crate::deltas::consume_type_91(&member(false, 20, 2, references), 0).is_none());
    }
}
