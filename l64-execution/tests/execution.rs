use l64_certification::CertificationVerdict;
use l64_execution::{
    ExecutionError, ExecutionSource, execute_canonical_graph, execute_dna, execute_rna,
};
use l64_native::{
    Dimension, Graph, LocusWord, Proposal, ROOT_CONTEXT, Route, dna_bytes, rna_to_dna,
};

const TRIANGLE: &[u8] = b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n";

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x4558454355544531)).composed(LocusWord(index))
}

#[test]
fn rna_and_dna_run_through_the_same_structural_carrier() {
    let dna = rna_to_dna(TRIANGLE).unwrap();
    let from_rna = execute_rna(TRIANGLE).unwrap();
    let from_dna = execute_dna(&dna).unwrap();
    assert_eq!(from_rna.source, ExecutionSource::Rna);
    assert_eq!(from_dna.source, ExecutionSource::Dna);
    assert_eq!(from_rna.symbol, from_dna.symbol);
    assert_eq!(from_rna.projection, from_dna.projection);
    assert_eq!(from_rna.certification, from_dna.certification);
    assert_eq!(
        from_rna.certification.verdict,
        CertificationVerdict::Certified
    );
    assert!(from_rna.canonical_roundtrip);
    assert!(from_rna.projection_verified);
}

#[test]
fn execution_reports_open_structure_without_creating_a_campaign() {
    let mut graph = Graph::new();
    let scalar = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let quantity = graph
        .declare_quantity_type(route(2), scalar, Dimension::new([0; 7]))
        .unwrap();
    let value = graph
        .insert_value(route(3), ROOT_CONTEXT, quantity)
        .unwrap();
    graph
        .transact(Proposal::square_root(
            route(4),
            ROOT_CONTEXT,
            value,
            quantity,
        ))
        .unwrap();
    let execution = execute_canonical_graph(&graph, ExecutionSource::BundleMember).unwrap();
    assert_eq!(execution.certification.verdict, CertificationVerdict::Open);
    assert!(execution.projection.report.open > 0);
    assert!(execution.render_text().contains("authority_mutation=none"));
}

#[test]
fn noncanonical_or_trailing_dna_is_rejected() {
    let mut dna =
        dna_bytes(&l64_native::decode_dna(&rna_to_dna(TRIANGLE).unwrap()).unwrap()).unwrap();
    dna.push(0);
    assert!(matches!(execute_dna(&dna), Err(ExecutionError::Dna(_))));
}
