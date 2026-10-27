// SPDX-License-Identifier: Apache-2.0

use super::{named_feature_definition, schema_feature_definition};
use crate::feature::operations::{
    FeatureOperation, FeatureRecipe, FeatureRecipeEffect, FeatureRecipeKind, OperationKind,
    OperationName, RecipeResolution,
};
use crate::feature::schema::SchemaClass;
use cadmpeg_ir::document::CadIr;
use cadmpeg_ir::features::{FeatureDefinition, FeatureOperation as IrOperation};

#[test]
fn generic_material_names_do_not_establish_a_sweep_family() {
    let scan = crate::test_support::empty_container_scan();
    let ir = CadIr::empty();
    let carriers = crate::decode::source_carriers::SourceUnitCarriers::default();
    for kind in ["Protrusion", "Cut"] {
        let definition = crate::decode::with_service_context(|ctx| {
            named_feature_definition(ctx, &scan, &ir, &carriers, 824, kind)
        })
        .expect("generic name has no family authority");
        assert!(
            definition.is_none(),
            "generic label projected a sweep: {kind}"
        );
    }
}

#[test]
fn generic_material_schema_without_a_recipe_remains_native() {
    let scan = crate::test_support::empty_container_scan();
    let ir = CadIr::empty();
    let carriers = crate::decode::source_carriers::SourceUnitCarriers::default();
    for (class, kind) in [
        (SchemaClass::Protrusion, "Protrusion"),
        (SchemaClass::Cut, "Cut"),
    ] {
        let definition = crate::decode::with_service_context(|ctx| {
            schema_feature_definition(ctx, &scan, &ir, &carriers, 824, Some(class), kind)
        })
        .expect("schema union has no sweep family");
        assert!(matches!(
            definition,
            FeatureDefinition::Operation(IrOperation::Native { .. })
        ));
    }
}

#[test]
fn admitted_recipe_selects_the_generic_material_family() {
    let ir = CadIr::empty();
    let carriers = crate::decode::source_carriers::SourceUnitCarriers::default();
    for recipe in [
        FeatureRecipe::ProtrudeExtrude,
        FeatureRecipe::CutExtrude,
        FeatureRecipe::ProtrudeRevolve,
        FeatureRecipe::CutRevolve,
    ] {
        let mut scan = crate::test_support::empty_container_scan();
        let (kind, class) = match recipe.effect() {
            FeatureRecipeEffect::Protrude => ("Protrusion", SchemaClass::Protrusion),
            FeatureRecipeEffect::Cut => ("Cut", SchemaClass::Cut),
        };
        scan.features.operations.push(FeatureOperation {
            feature_id: 824,
            kind: OperationKind::Stored(kind.to_owned()),
            name: OperationName::Derived,
            recipe: RecipeResolution::Resolved(recipe),
            display_state_conflict: false,
            depdb: None,
            offset: 1,
            state_offset: 1,
        });
        let named = crate::decode::with_service_context(|ctx| {
            named_feature_definition(ctx, &scan, &ir, &carriers, 824, kind)
        })
        .expect("admitted procedural family")
        .expect("owned family remains projected");
        let schema = crate::decode::with_service_context(|ctx| {
            schema_feature_definition(ctx, &scan, &ir, &carriers, 824, Some(class), kind)
        })
        .expect("admitted schema recipe");
        for definition in [named, schema] {
            match recipe.kind() {
                FeatureRecipeKind::Extrude => assert!(matches!(
                    definition,
                    FeatureDefinition::Operation(IrOperation::Extrude { .. })
                )),
                FeatureRecipeKind::Revolve => assert!(matches!(
                    definition,
                    FeatureDefinition::Operation(IrOperation::Revolve { .. })
                )),
            }
        }
    }
}
