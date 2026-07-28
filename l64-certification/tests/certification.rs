use l64_certification::{
    CertificationError, CertificationVerdict, certify_dna, certify_graph,
    certify_graph_with_root_projection,
};
use l64_native::{
    ConstraintKind, Dimension, Graph, LocusWord, Proposal, ROOT_CONTEXT, Route, dna_bytes,
    rna_to_dna,
};

const CERTIFIED_RNA: &[u8] = b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n";

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x4345525449465931)).composed(LocusWord(index))
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
fn canonical_native_authority_certifies_from_direct_burdens() {
    let dna = rna_to_dna(CERTIFIED_RNA).unwrap();
    let certification = certify_dna(&dna).unwrap();
    assert!(certification.canonical_dna);
    assert_eq!(certification.verdict, CertificationVerdict::Certified);
    assert_eq!(certification.contexts.len(), 1);
    assert!(certification.contexts[0].projection_verified);
    assert_eq!(certification.contexts[0].burdens.discharged, 1);
    assert!(
        certification
            .render_text()
            .contains("authority=exact_canonical_l64d")
    );
}

#[test]
fn open_native_burden_remains_open() {
    let (graph, _) = open_graph();
    let certification = certify_dna(&dna_bytes(&graph).unwrap()).unwrap();
    assert_eq!(certification.verdict, CertificationVerdict::Open);
    assert_eq!(certification.contexts[0].burdens.open, 1);
}

#[test]
fn invalid_child_context_invalidates_the_authority_certification() {
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
    let certification = certify_dna(&dna_bytes(&graph).unwrap()).unwrap();
    assert_eq!(certification.context_count, 2);
    assert_eq!(certification.verdict, CertificationVerdict::Invalid);
    assert_eq!(
        certification.contexts[child as usize].verdict,
        CertificationVerdict::Invalid
    );
}

#[test]
fn trailing_dna_bytes_are_rejected() {
    let mut dna = rna_to_dna(CERTIFIED_RNA).unwrap();
    dna.push(0);
    assert!(matches!(certify_dna(&dna), Err(CertificationError::Dna(_))));
}

#[test]
fn shared_fresh_root_projection_matches_direct_certification_exactly() {
    let graph = l64_native::decode_dna(&rna_to_dna(CERTIFIED_RNA).unwrap()).unwrap();
    let direct = certify_graph(&graph).unwrap();
    let (shared, projection) = certify_graph_with_root_projection(&graph, 16).unwrap();
    assert_eq!(shared, direct);
    assert_eq!(projection.certification.source.context, ROOT_CONTEXT);
    assert_eq!(
        shared.contexts[0].burdens.total,
        projection.certification.burdens.len()
    );
}
