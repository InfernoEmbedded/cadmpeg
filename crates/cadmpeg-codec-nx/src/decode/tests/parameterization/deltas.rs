use crate::deltas::record_kind::RecordKind;
// SPDX-License-Identifier: Apache-2.0

use crate::framing::node_kind::NodeKind;
use crate::test_support::test_deltas::circle_topology_partition_stream;
use crate::test_support::test_deltas::cone_topology_partition_stream;
use crate::test_support::test_deltas::cylinder_topology_partition_stream;
use crate::test_support::test_deltas::deltas_blend_surface_partition_stream;
use crate::test_support::test_deltas::deltas_circle_partition_stream;
use crate::test_support::test_deltas::deltas_cone_partition_stream;
use crate::test_support::test_deltas::deltas_cylinder_partition_stream;
use crate::test_support::test_deltas::deltas_edge_partition_stream;
use crate::test_support::test_deltas::deltas_ellipse_partition_stream;
use crate::test_support::test_deltas::deltas_face_vertex_partition_stream;
use crate::test_support::test_deltas::deltas_fin_partition_stream;
use crate::test_support::test_deltas::deltas_line_partition_stream;
use crate::test_support::test_deltas::deltas_loop_partition_stream;
use crate::test_support::test_deltas::deltas_offset_surface_partition_stream;
use crate::test_support::test_deltas::deltas_plane_partition_stream;
use crate::test_support::test_deltas::deltas_point_partition_stream;
use crate::test_support::test_deltas::deltas_shell_partition_stream;
use crate::test_support::test_deltas::deltas_sphere_partition_stream;
use crate::test_support::test_deltas::deltas_surface_curve_partition_stream;
use crate::test_support::test_deltas::deltas_torus_partition_stream;
use crate::test_support::test_deltas::deltas_trimmed_curve_partition_stream;
use crate::test_support::test_deltas::ellipse_topology_partition_stream;
use crate::test_support::test_deltas::sphere_topology_partition_stream;
use crate::test_support::test_deltas::status_framed_deltas_stream;
use crate::test_support::test_deltas::torus_topology_partition_stream;
use crate::test_support::test_deltas::trimmed_topology_partition_stream;
use crate::test_support::test_deltas::variable_status_framed_deltas_stream;
use crate::test_support::test_deltas::DELTAS_PREAMBLE;
use crate::test_support::test_prt::prt_with_partition;
use crate::test_support::test_prt::prt_with_streams;
use crate::test_support::test_streams::blend_surface_topology_partition_stream;
use crate::test_support::test_streams::charted_intersection_curve_topology_partition_stream;
use crate::test_support::test_streams::ext11_charted_intersection_curve_stream;
use crate::test_support::test_streams::offset_surface_topology_partition_stream;
use crate::test_support::test_streams::prt_with_ext11_intersection;
use crate::test_support::test_streams::surface_curve_topology_partition_stream;
use crate::test_support::test_streams::topology_partition_stream;
use crate::NxCodec;
use cadmpeg_ir::codec::Codec;
use cadmpeg_ir::codec::DecodeOptions;
use std::io::Cursor;

// A physical pairing is provenance, not an owned directed pmark edge. Exercise
// each carrier against its independently decoded saved-image baseline.
fn assert_saved_partition_model(partition: &[u8], deltas: &[u8], expected_raw_kind: RecordKind) {
    let baseline = NxCodec
        .decode(
            &mut Cursor::new(prt_with_partition(partition)),
            &DecodeOptions::default(),
        )
        .unwrap();
    let actual = NxCodec
        .decode(
            &mut Cursor::new(prt_with_streams(&[partition, deltas])),
            &DecodeOptions::default(),
        )
        .unwrap();
    assert_eq!(
        serde_json::to_value(&actual.ir().model).unwrap(),
        serde_json::to_value(&baseline.ir().model).unwrap()
    );
    let census = crate::deltas::census::walk(deltas).unwrap();
    assert_eq!(
        census
            .records
            .iter()
            .filter(|record| record.kind() == u16::from(expected_raw_kind.code()))
            .count(),
        1
    );
    assert!(
        !actual.ir().native.namespace("nx").unwrap().arenas()["parasolid_deltas_records"]
            .is_empty()
    );
    assert!(
        cadmpeg_ir::validate::validate_neutral(actual.ir(), Vec::new())
            .unwrap()
            .is_ok()
    );
}

#[test]
fn decode_reports_full_raw_records_without_certifying_deletion_shapes() {
    let stream = status_framed_deltas_stream();
    assert_eq!(
        crate::deltas::census::walk(&stream)
            .unwrap()
            .bytes_decoded(),
        stream.len() - DELTAS_PREAMBLE.len()
    );
    let mut cur = Cursor::new(prt_with_partition(&stream));
    let result = NxCodec.decode(&mut cur, &DecodeOptions::default()).unwrap();
    let attributes = &result
        .ir()
        .source
        .as_ref()
        .expect("source metadata")
        .attributes;

    assert_eq!(
        attributes.get("deltas.0.full.FACE").map(String::as_str),
        Some("1")
    );
    assert_eq!(
        attributes
            .get("deltas.0.tombstone.EDGE")
            .map(String::as_str),
        None
    );
    assert_eq!(
        attributes.get("deltas.0.grammar").map(String::as_str),
        Some("typed_status_framed_records")
    );
}

#[test]
fn decode_accepts_exact_loop_and_rejects_incomplete_fin_deltas() {
    let ambiguous = variable_status_framed_deltas_stream();
    let ambiguous_census =
        crate::test_support::with_decode_context(|_ctx| crate::deltas::census::walk(&ambiguous))
            .unwrap();
    assert!(ambiguous_census
        .records
        .iter()
        .all(|record| record.kind() != 15));
    let mut stream = deltas_loop_partition_stream();
    stream.extend_from_slice(&ambiguous[DELTAS_PREAMBLE.len()..]);
    let mut cur = Cursor::new(prt_with_partition(&stream));
    let result = NxCodec.decode(&mut cur, &DecodeOptions::default()).unwrap();
    let attributes = &result
        .ir()
        .source
        .as_ref()
        .expect("source metadata")
        .attributes;

    assert!(!attributes.contains_key("deltas.0.full.FIN"));
    assert_eq!(
        attributes.get("deltas.0.full.LOOP").map(String::as_str),
        Some("1")
    );
}

#[test]
fn decode_retains_deltas_only_point_without_neutral_promotion() {
    let mut cur = Cursor::new(prt_with_partition(&deltas_point_partition_stream()));
    let result = NxCodec.decode(&mut cur, &DecodeOptions::default()).unwrap();
    assert!(result.ir().model.points.is_empty());
    let arenas = result.ir().native.namespace("nx").unwrap().arenas();
    assert_eq!(arenas["parasolid_deltas_records"].len(), 1);
}

#[test]
fn decode_preserves_saved_point_against_same_xmt_history() {
    let partition = topology_partition_stream();
    let mut deltas = deltas_point_partition_stream();
    let record = deltas
        .windows(2)
        .rposition(|window| window == 29u16.to_be_bytes())
        .expect("deltas POINT");
    deltas[record + 2..record + 4].copy_from_slice(&11u16.to_be_bytes());
    assert_saved_partition_model(&partition, &deltas, RecordKind::Point);
}

#[test]
fn decode_preserves_partition_edge_topology_over_deltas_history() {
    let partition = topology_partition_stream();
    let deltas = deltas_edge_partition_stream();
    let mut cur = Cursor::new(prt_with_streams(&[&partition, &deltas]));
    let result = NxCodec.decode(&mut cur, &DecodeOptions::default()).unwrap();
    assert_eq!(result.ir().model.edges.len(), 1);
    assert_eq!(
        result.ir().model.edges[0]
            .tolerance
            .map(cadmpeg_ir::scalar::PositiveReal::get),
        Some(0.3)
    );
    assert_eq!(
        result.ir().model.edges[0].curve(),
        Some(&result.ir().model.curves[0].id)
    );
    assert!(
        cadmpeg_ir::validate::validate_neutral(result.ir(), Vec::new())
            .expect("validate the neutral model")
            .is_ok()
    );
}

#[test]
fn decode_preserves_partition_face_and_vertex_topology_over_deltas_history() {
    let partition = topology_partition_stream();
    let deltas = deltas_face_vertex_partition_stream();
    let mut cur = Cursor::new(prt_with_streams(&[&partition, &deltas]));
    let result = NxCodec.decode(&mut cur, &DecodeOptions::default()).unwrap();
    assert_eq!(result.ir().model.faces.len(), 1);
    assert_eq!(
        result.ir().model.faces[0]
            .tolerance
            .map(cadmpeg_ir::scalar::PositiveReal::get),
        Some(0.2)
    );
    assert_eq!(result.ir().model.vertices.len(), 1);
    assert_eq!(
        result.ir().model.vertices[0]
            .tolerance
            .map(cadmpeg_ir::scalar::PositiveReal::get),
        Some(0.1)
    );
    assert!(
        cadmpeg_ir::validate::validate_neutral(result.ir(), Vec::new())
            .expect("validate the neutral model")
            .is_ok()
    );
}

#[test]
fn decode_preserves_partition_loop_topology_over_deltas_history() {
    let partition = topology_partition_stream();
    let deltas = deltas_loop_partition_stream();
    let merged = crate::deltas::merge_full_records(&partition, &deltas).unwrap();
    assert_eq!(
        crate::topology::Graph::parse(&merged)
            .get(NodeKind::Loop, 5)
            .and_then(|node| node.u32_at(4)),
        Some(0)
    );
    let mut cur = Cursor::new(prt_with_streams(&[&partition, &deltas]));
    let result = NxCodec.decode(&mut cur, &DecodeOptions::default()).unwrap();
    assert_eq!(result.ir().model.loops.len(), 1);
    assert_eq!(result.ir().model.coedges.len(), 1);
    assert!(
        cadmpeg_ir::validate::validate_neutral(result.ir(), Vec::new())
            .expect("validate the neutral model")
            .is_ok()
    );
}

#[test]
fn decode_preserves_partition_shell_topology_over_deltas_history() {
    let partition = topology_partition_stream();
    let deltas = deltas_shell_partition_stream();
    let merged = crate::deltas::merge_full_records(&partition, &deltas).unwrap();
    assert_eq!(
        crate::topology::Graph::parse(&merged)
            .get(NodeKind::Shell, 3)
            .and_then(|node| node.u32_at(4)),
        Some(0)
    );
    let mut cur = Cursor::new(prt_with_streams(&[&partition, &deltas]));
    let result = NxCodec.decode(&mut cur, &DecodeOptions::default()).unwrap();
    assert_eq!(result.ir().model.shells.len(), 1);
    assert_eq!(result.ir().model.faces.len(), 1);
    assert!(
        cadmpeg_ir::validate::validate_neutral(result.ir(), Vec::new())
            .expect("validate the neutral model")
            .is_ok()
    );
}

#[test]
fn decode_preserves_partition_fin_topology_over_deltas_history() {
    let partition = topology_partition_stream();
    let deltas = deltas_fin_partition_stream();
    let file = prt_with_streams(&[&partition, &deltas]);
    let mut cur = Cursor::new(file);
    let result = NxCodec.decode(&mut cur, &DecodeOptions::default()).unwrap();

    assert_eq!(result.ir().model.coedges.len(), 1);
    assert_eq!(
        result.ir().model.coedges[0].sense,
        cadmpeg_ir::topology::Sense::Forward
    );
    assert!(
        cadmpeg_ir::validate::validate_neutral(result.ir(), Vec::new())
            .expect("validate the neutral model")
            .is_ok()
    );
}

#[test]
fn decode_preserves_saved_line_against_unselected_history() {
    assert_saved_partition_model(
        &topology_partition_stream(),
        &deltas_line_partition_stream(),
        RecordKind::Line,
    );
}

#[test]
fn decode_preserves_saved_plane_against_unselected_history() {
    assert_saved_partition_model(
        &topology_partition_stream(),
        &deltas_plane_partition_stream(),
        RecordKind::Plane,
    );
}

#[test]
fn decode_preserves_saved_offset_surface_against_unselected_history() {
    assert_saved_partition_model(
        &offset_surface_topology_partition_stream(),
        &deltas_offset_surface_partition_stream(),
        RecordKind::OffsetSurf,
    );
}

#[test]
fn decode_preserves_saved_blend_surface_against_unselected_history() {
    assert_saved_partition_model(
        &blend_surface_topology_partition_stream(),
        &deltas_blend_surface_partition_stream(),
        RecordKind::BlendSurf,
    );
}

#[test]
fn decode_preserves_saved_trimmed_curve_against_unselected_history() {
    assert_saved_partition_model(
        &trimmed_topology_partition_stream(),
        &deltas_trimmed_curve_partition_stream(),
        RecordKind::TrimmedCurve,
    );
}

#[test]
fn decode_preserves_saved_surface_curve_against_unselected_history() {
    assert_saved_partition_model(
        &surface_curve_topology_partition_stream(),
        &deltas_surface_curve_partition_stream(),
        RecordKind::SpCurve,
    );
}

#[test]
fn decode_preserves_saved_circle_against_unselected_history() {
    assert_saved_partition_model(
        &circle_topology_partition_stream(),
        &deltas_circle_partition_stream(),
        RecordKind::Circle,
    );
}

#[test]
fn decode_preserves_saved_ellipse_against_unselected_history() {
    assert_saved_partition_model(
        &ellipse_topology_partition_stream(),
        &deltas_ellipse_partition_stream(),
        RecordKind::Ellipse,
    );
}

#[test]
fn decode_preserves_saved_cylinder_against_unselected_history() {
    assert_saved_partition_model(
        &cylinder_topology_partition_stream(),
        &deltas_cylinder_partition_stream(),
        RecordKind::Cylinder,
    );
}

#[test]
fn decode_preserves_saved_cone_against_unselected_history() {
    assert_saved_partition_model(
        &cone_topology_partition_stream(),
        &deltas_cone_partition_stream(),
        RecordKind::Cone,
    );
}

#[test]
fn decode_preserves_saved_sphere_against_unselected_history() {
    assert_saved_partition_model(
        &sphere_topology_partition_stream(),
        &deltas_sphere_partition_stream(),
        RecordKind::Sphere,
    );
}

#[test]
fn decode_preserves_saved_torus_against_unselected_history() {
    assert_saved_partition_model(
        &torus_topology_partition_stream(),
        &deltas_torus_partition_stream(),
        RecordKind::Torus,
    );
}

#[test]
fn decode_preserves_saved_chart_without_an_owned_ext11_history_edge() {
    let stream = ext11_charted_intersection_curve_stream();
    let partition = charted_intersection_curve_topology_partition_stream();
    let actual = NxCodec
        .decode(
            &mut Cursor::new(prt_with_ext11_intersection(&partition, &stream)),
            &DecodeOptions::default(),
        )
        .unwrap();
    let baseline = NxCodec
        .decode(
            &mut Cursor::new(prt_with_partition(&partition)),
            &DecodeOptions::default(),
        )
        .unwrap();
    assert_eq!(
        serde_json::to_value(&actual.ir().model).unwrap(),
        serde_json::to_value(&baseline.ir().model).unwrap()
    );
}
