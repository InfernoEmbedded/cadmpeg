// SPDX-License-Identifier: Apache-2.0
//! Keywords configuration partition-identity resolution tests.
#![allow(clippy::unwrap_used)]

use crate::records::Configuration;
use crate::test_support::container::make_block;
use crate::test_support::container::outer_header;

fn parsed_configurations(payload: &[u8]) -> Vec<Configuration> {
    let mut source = outer_header();
    source.extend(make_block(0x43, "", payload));
    let scan = crate::test_support::container::scan(&source);
    let arena = cadmpeg_core::decode::DecodeArena::new();
    let (ctx, _) = cadmpeg_core::decode::DecodeContext::from_root_bytes(
        &source,
        &arena,
        &cadmpeg_core::decode::DecodePolicy::service(),
    )
    .unwrap();
    let mut annotations = cadmpeg_ir::annotations::Annotations::default();
    let mut losses = Vec::new();
    let histories = crate::history::histories(&ctx, &scan, &mut annotations, &mut losses).unwrap();
    assert_eq!(histories.len(), 1);
    assert!(losses.is_empty());
    histories.into_iter().next().unwrap().configurations
}

#[test]
fn agreeing_configuration_manager_identity_keeps_the_explicit_index() {
    let configurations = parsed_configurations(
        br#"<Keywords><Configuration Name="Default" Type="ConfigurationManager" id="0" SourceIndex="0"/></Keywords>"#,
    );

    assert_eq!(configurations.len(), 1);
    assert_eq!(configurations[0].source_index, Some(0));
}

#[test]
fn conflicting_configuration_manager_identity_resolves_to_unresolved() {
    let configurations = parsed_configurations(
        br#"<Keywords><Configuration Name="Default" Type="ConfigurationManager" id="2" SourceIndex="5"/></Keywords>"#,
    );

    assert_eq!(configurations.len(), 1);
    assert_eq!(configurations[0].source_index, None);
}

#[test]
fn missing_explicit_index_falls_back_to_the_native_id() {
    let configurations = parsed_configurations(
        br#"<Keywords><Configuration Name="Default" Type="ConfigurationManager" id="3"/></Keywords>"#,
    );

    assert_eq!(configurations.len(), 1);
    assert_eq!(configurations[0].source_index, Some(3));
}

#[test]
fn sparse_reordered_native_ids_resolve_independently_of_element_order() {
    let configurations = parsed_configurations(
        br#"<Keywords><Configuration Name="Second" Type="ConfigurationManager" id="5"/><Configuration Name="First" Type="ConfigurationManager" id="2" SourceIndex="2"/></Keywords>"#,
    );

    assert_eq!(configurations.len(), 2);
    assert_eq!(configurations[0].source_index, Some(5));
    assert_eq!(configurations[0].ordinal, 0);
    assert_eq!(configurations[1].source_index, Some(2));
    assert_eq!(configurations[1].ordinal, 1);
}

#[test]
fn configuration_without_manager_type_keeps_the_explicit_index_alone() {
    let configurations = parsed_configurations(
        br#"<Keywords><Configuration Name="Default" id="2" SourceIndex="5"/><Configuration Name="Legacy" SourceIndex="7"/></Keywords>"#,
    );

    assert_eq!(configurations.len(), 2);
    assert_eq!(configurations[0].source_index, Some(5));
    assert_eq!(configurations[1].source_index, Some(7));
}

#[test]
fn configuration_manager_without_any_index_resolves_to_unresolved() {
    let configurations = parsed_configurations(
        br#"<Keywords><Configuration Name="Default" Type="ConfigurationManager"/></Keywords>"#,
    );

    assert_eq!(configurations.len(), 1);
    assert_eq!(configurations[0].source_index, None);
}
