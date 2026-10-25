// SPDX-License-Identifier: Apache-2.0
//! Source identity and ambiguity controls for configuration joins.

use super::ConfigurationBindings;

const FEATURES: &str = r#"<swSolidWorks xmlns="http://www.solidworks.com/sw2003/schema"><swModel id="2" swConfigurationName="Default" swConfigurationId="0"/><swModel id="9" swConfigurationName="HI" swConfigurationId="3"/><swConfiguration id="1" swName="Default" swID="0" swModelRef="2" swMostRecentConfiguration="NO" swConfigurationNeedsUpdate="NO"/><swConfiguration id="8" swName="HI" swID="3" swModelRef="9" swMostRecentConfiguration="YES" swConfigurationNeedsUpdate="NO"/></swSolidWorks>"#;
const KEYWORDS: &str =
    r#"<Keywords><Configuration Name="Default"/><Configuration Name="HI"/></Keywords>"#;

fn bindings(features: &str, keywords: &str) -> ConfigurationBindings {
    let mut bindings = ConfigurationBindings::default();
    bindings.record_features(&roxmltree::Document::parse(features).expect("features XML"));
    bindings.record_keywords(&roxmltree::Document::parse(keywords).expect("keywords XML"));
    bindings.finish();
    bindings
}

#[test]
fn complete_join_binds_inactive_and_sparse_active_partition_indices() {
    let bindings = bindings(FEATURES, KEYWORDS);
    assert_eq!(bindings.index_for("Default"), Some(Some(0)));
    assert_eq!(bindings.index_for("HI"), Some(Some(3)));
    assert_eq!(bindings.active().unique(), Some((3, Some("HI".into()))));
}

#[test]
fn generic_row_ids_and_consistent_object_reference_renumbering_do_not_select_partitions() {
    for features in [
        FEATURES.replace("id=\"1\" swName", "id=\"99\" swName"),
        FEATURES
            .replace("id=\"2\"", "id=\"500\"")
            .replace("swModelRef=\"2\"", "swModelRef=\"500\""),
    ] {
        let bindings = bindings(&features, KEYWORDS);
        assert_eq!(bindings.index_for("Default"), Some(Some(0)));
        assert_eq!(bindings.index_for("HI"), Some(Some(3)));
    }
}

#[test]
fn mismatched_or_missing_redundant_identity_leaves_the_name_unresolved() {
    for features in [
        FEATURES.replace("swName=\"HI\"", "swName=\"Other\""),
        FEATURES.replace(
            "swConfigurationName=\"HI\"",
            "swConfigurationName=\"Other\"",
        ),
        FEATURES.replace("swConfigurationId=\"3\"", "swConfigurationId=\"2\""),
        FEATURES.replace("swModelRef=\"9\"", "swModelRef=\"missing\""),
        FEATURES.replace("swID=\"3\"", "swID=\"\""),
        FEATURES.replace("swID=\"3\"", "swID=\"+3\""),
    ] {
        let bindings = bindings(&features, KEYWORDS);
        assert_eq!(bindings.index_for("HI"), Some(None), "{features}");
        assert!(bindings.active().unique().is_none());
    }
}

#[test]
fn duplicate_name_row_model_or_decimal_identity_blocks_selection() {
    let repeated_row = r#"<swConfiguration swName="HI" swID="8"/>"#;
    let repeated_model = r#"<swModel id="9" swConfigurationName="HI" swConfigurationId="3"/>"#;
    for (features, keywords) in [
        (
            FEATURES.to_string(),
            KEYWORDS.replace("</Keywords>", "<Configuration Name=\"HI\"/></Keywords>"),
        ),
        (
            FEATURES.replace("</swSolidWorks>", &format!("{repeated_row}</swSolidWorks>")),
            KEYWORDS.to_string(),
        ),
        (
            FEATURES.replace(
                "</swSolidWorks>",
                &format!("{repeated_model}</swSolidWorks>"),
            ),
            KEYWORDS.to_string(),
        ),
        (
            FEATURES.replace("swID=\"0\"", "swID=\"03\""),
            KEYWORDS.to_string(),
        ),
    ] {
        let bindings = bindings(&features, &keywords);
        assert_eq!(bindings.index_for("HI"), Some(None));
        assert!(bindings.active().unique().is_none());
    }
}

#[test]
fn source_index_must_be_valid_and_agree_with_the_complete_manifest() {
    for index in ["0", "", "unknown"] {
        let keywords = KEYWORDS.replace(
            "Name=\"HI\"",
            &format!("Name=\"HI\" SourceIndex=\"{index}\""),
        );
        assert_eq!(bindings(FEATURES, &keywords).index_for("HI"), Some(None));
    }
    let keywords = KEYWORDS.replace("Name=\"HI\"", "Name=\"HI\" SourceIndex=\"3\"");
    assert_eq!(bindings(FEATURES, &keywords).index_for("HI"), Some(Some(3)));
}

#[test]
fn needs_update_and_non_unique_activity_abstain() {
    let features = FEATURES.replace(
        "swConfigurationNeedsUpdate=\"NO\"",
        "swConfigurationNeedsUpdate=\"YES\"",
    );
    let result = bindings(&features, KEYWORDS);
    assert_eq!(result.index_for("Default"), Some(None));
    assert_eq!(result.index_for("HI"), Some(None));
    assert!(result.active().unique().is_none());
    let features = FEATURES.replace(
        "swMostRecentConfiguration=\"NO\"",
        "swMostRecentConfiguration=\"YES\"",
    );
    assert!(bindings(&features, KEYWORDS).active().unique().is_none());
}

#[test]
fn object_references_do_not_cross_manifest_document_boundaries() {
    let mut bindings = ConfigurationBindings::default();
    let first = r#"<swSolidWorks><swConfiguration swName="HI" swID="3" swModelRef="9" swMostRecentConfiguration="YES" swConfigurationNeedsUpdate="NO"/></swSolidWorks>"#;
    let second = r#"<swSolidWorks><swModel id="9" swConfigurationName="HI" swConfigurationId="3"/></swSolidWorks>"#;
    for text in [first, second] {
        bindings.record_features(&roxmltree::Document::parse(text).expect("XML"));
    }
    bindings.record_keywords(&roxmltree::Document::parse(KEYWORDS).expect("XML"));
    bindings.finish();
    assert_eq!(bindings.index_for("HI"), Some(None));
    assert!(bindings.active().unique().is_none());
}

#[test]
fn duplicate_manifest_carriers_do_not_collapse_into_a_unique_row() {
    let mut bindings = bindings(FEATURES, KEYWORDS);
    bindings.record_features(&roxmltree::Document::parse(FEATURES).expect("XML"));
    bindings.finish();
    assert_eq!(bindings.index_for("HI"), Some(None));
    assert!(bindings.active().unique().is_none());
}

#[test]
fn absent_manifest_has_no_binding_claim_and_empty_keyword_names_are_unresolved() {
    let absent = bindings("<swSolidWorks/>", KEYWORDS);
    assert_eq!(absent.index_for("HI"), None);
    let keywords = KEYWORDS.replace("Name=\"HI\"", "Name=\"\"");
    let bindings = bindings(FEATURES, &keywords);
    assert_eq!(bindings.index_for(""), Some(None));
    assert!(bindings.active().unique().is_none());
}

#[test]
fn valid_explicit_index_survives_compatible_partial_manifest_metadata() {
    let features = r#"<swSolidWorks><swModel id="m" swConfigurationName="Default"/><swConfiguration swID="0" swModelRef="m" swMostRecentConfiguration="YES"/></swSolidWorks>"#;
    let keywords = r#"<Keywords><Configuration Name="Default" SourceIndex="0"/></Keywords>"#;
    let result = bindings(features, keywords);
    assert_eq!(result.index_for("Default"), Some(Some(0)));
    assert_eq!(result.active().unique(), Some((0, Some("Default".into()))));
    let without_index = keywords.replace(" SourceIndex=\"0\"", "");
    let result = bindings(features, &without_index);
    assert_eq!(result.index_for("Default"), Some(None));
    assert!(result.active().unique().is_none());
}

#[test]
fn contradictory_partial_active_identity_and_duplicate_direct_indices_abstain() {
    let features = r#"<swSolidWorks><swModel id="m" swConfigurationName="Other"/><swConfiguration swID="0" swModelRef="m" swMostRecentConfiguration="YES"/></swSolidWorks>"#;
    let keywords = r#"<Keywords><Configuration Name="Default" SourceIndex="0"/></Keywords>"#;
    let result = bindings(features, keywords);
    assert_eq!(result.index_for("Default"), Some(Some(0)));
    assert!(result.active().unique().is_none());
    let keywords = keywords.replace(
        "</Keywords>",
        "<Configuration Name=\"Other\" SourceIndex=\"00\"/></Keywords>",
    );
    let result = bindings(features, &keywords);
    assert_eq!(result.index_for("Default"), Some(None));
    assert_eq!(result.index_for("Other"), Some(None));
    assert!(result.active().unique().is_none());
}
