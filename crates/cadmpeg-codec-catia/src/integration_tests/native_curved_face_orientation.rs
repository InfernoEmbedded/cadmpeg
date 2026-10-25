// SPDX-License-Identifier: Apache-2.0

use cadmpeg_ir::geometry::{SolvedSurfaceGeometry, SurfaceGeometry};
use cadmpeg_ir::topology::Sense;

use super::{assert_valid, decode};

#[test]
fn native_cylinder_faces_follow_outer_control_in_the_preserved_chart() {
    let positive = decode(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/float_packed_native_cylinder_positive.catpart"
        ))
        .to_vec(),
    );
    let negative = decode(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/float_packed_native_cylinder_negative.catpart"
        ))
        .to_vec(),
    );
    for (result, expected) in [(&positive, Sense::Forward), (&negative, Sense::Reversed)] {
        assert_valid(result);
        assert_eq!(result.ir().model.faces.len(), 1);
        assert_eq!(result.ir().model.faces[0].sense, expected);
        assert!(matches!(
            result.ir().model.surfaces[0].geometry,
            SurfaceGeometry::Solved(SolvedSurfaceGeometry::Cylinder(_))
        ));
    }
    assert_eq!(positive.ir().model.surfaces, negative.ir().model.surfaces);
}

#[test]
fn native_nurbs_faces_follow_outer_control_without_transposing_the_carrier() {
    let positive = decode(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/float_packed_native_nurbs_positive.catpart"
        ))
        .to_vec(),
    );
    let negative = decode(
        include_bytes!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/tests/fixtures/float_packed_native_nurbs_negative.catpart"
        ))
        .to_vec(),
    );
    for (result, expected) in [(&positive, Sense::Forward), (&negative, Sense::Reversed)] {
        assert_valid(result);
        assert_eq!(result.ir().model.faces.len(), 1);
        assert_eq!(result.ir().model.faces[0].sense, expected);
        let SurfaceGeometry::Solved(SolvedSurfaceGeometry::Nurbs(surface)) =
            &result.ir().model.surfaces[0].geometry
        else {
            panic!("direct native NURBS carrier");
        };
        assert!(!surface.normal_reversed());
        assert_eq!(surface.u_degree(), 2);
        assert_eq!(surface.v_degree(), 2);
    }
    assert_eq!(positive.ir().model.surfaces, negative.ir().model.surfaces);
}
