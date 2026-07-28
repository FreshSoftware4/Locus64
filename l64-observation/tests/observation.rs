use l64_certification::CertificationVerdict;
use l64_native::{
    ConstraintKind, Dimension, Graph, LocusWord, Proposal, ROOT_CONTEXT, Route, dna_bytes,
    rna_to_dna,
};
use l64_observation::{ObservationError, observe_bundle, observe_dna};
use l64_transport::bundle_bytes;

const CERTIFIED_RNA: &[u8] = b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n";

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x4f42534552564531)).composed(LocusWord(index))
}

fn open_graph() -> (Graph, u32) {
    let mut graph = Graph::new();
    let scalar = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let quantity = graph
        .declare_quantity_type(route(2), scalar, Dimension::new([0, 0, 0, 0, 0, 0, 0]))
        .unwrap();
    let subject = graph
        .insert_value(route(3), ROOT_CONTEXT, quantity)
        .unwrap();
    graph
        .transact(Proposal::square_root(
            route(4),
            ROOT_CONTEXT,
            subject,
            quantity,
        ))
        .unwrap();
    (graph, subject)
}

#[test]
fn canonical_authority_observation_binds_verified_native_surfaces() {
    let dna = rna_to_dna(CERTIFIED_RNA).unwrap();
    let observation = observe_dna(&dna).unwrap();
    assert_eq!(
        observation.certification.verdict,
        CertificationVerdict::Certified
    );
    assert_eq!(observation.contexts.len(), 1);
    let root = &observation.contexts[0];
    assert!(root.certification.projection_verified);
    assert_eq!(
        root.replay.steps.len(),
        observation.certification.journal_len
    );
    assert_eq!(root.report.invalid, 0);
    let rendered = observation.render_text();
    assert!(rendered.contains("authority=exact_canonical_l64d"));
    assert!(rendered.contains("projection_authority=non_authoritative"));
    assert!(rendered.contains("replay_projection_verified=true"));
    assert!(rendered.contains("report_projection_verified=true"));
}

#[test]
fn open_burden_is_observed_without_becoming_a_campaign() {
    let (graph, _) = open_graph();
    let observation = observe_dna(&dna_bytes(&graph).unwrap()).unwrap();
    assert_eq!(
        observation.certification.verdict,
        CertificationVerdict::Open
    );
    assert_eq!(observation.contexts[0].certification.burdens.open, 1);
    assert!(observation.contexts[0].report.open >= 1);
}

#[test]
fn invalid_child_context_remains_visible_in_report_and_certification() {
    let (mut graph, subject) = open_graph();
    let cause = graph
        .declare_constraint(
            route(5),
            ROOT_CONTEXT,
            subject,
            ConstraintKind::NonNegative,
            false,
        )
        .unwrap();
    let child = graph.extend_context(ROOT_CONTEXT, cause).unwrap();
    let observation = observe_dna(&dna_bytes(&graph).unwrap()).unwrap();
    assert_eq!(
        observation.certification.verdict,
        CertificationVerdict::Invalid
    );
    let child = &observation.contexts[child as usize];
    assert_eq!(child.certification.verdict, CertificationVerdict::Invalid);
    assert!(child.report.invalid > 0);
    assert!(!child.report.invalid_routes.is_empty());
}

#[test]
fn bundle_members_are_observed_independently() {
    let certified = rna_to_dna(CERTIFIED_RNA).unwrap();
    let (open, _) = open_graph();
    let open = dna_bytes(&open).unwrap();
    let bundle = bundle_bytes(&[&certified, &open]).unwrap();
    let observation = observe_bundle(&bundle).unwrap();
    assert_eq!(observation.members.len(), 2);
    assert_eq!(
        observation.members[0].certification.verdict,
        CertificationVerdict::Certified
    );
    assert_eq!(
        observation.members[1].certification.verdict,
        CertificationVerdict::Open
    );
    let rendered = observation.render_text();
    assert!(rendered.contains("composite_authority=none"));
    assert!(rendered.contains("composite_verdict=none"));
    assert!(rendered.contains("composite_observation=none"));
    assert!(!rendered.contains("bundle_verdict="));
}

#[test]
fn trailing_dna_bytes_are_rejected() {
    let mut dna = rna_to_dna(CERTIFIED_RNA).unwrap();
    dna.push(0);
    assert!(matches!(observe_dna(&dna), Err(ObservationError::Dna(_))));
}
