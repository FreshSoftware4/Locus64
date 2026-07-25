use l64_native::{
    ConstraintKind, DecodeError, Dimension, Graph, LocusWord, OpCode, ROOT_CONTEXT, Route,
    canonical_bytes, decode_canonical, dna_to_rna, rna_to_dna,
};

fn route(slot: u64) -> Route {
    Route::root(LocusWord(0x4551)).composed(LocusWord(slot))
}

#[test]
fn primitive_equality_is_a_judgment_with_checked_witness() {
    let mut graph = Graph::new();
    let left = graph.declare_atom_type(route(1), LocusWord(0x41)).unwrap();
    let right = graph.declare_atom_type(route(2), LocusWord(0x41)).unwrap();

    let structural = graph
        .prove_structural_equality(route(3), ROOT_CONTEXT, left, right)
        .unwrap();
    assert_eq!(
        graph.node(structural.node).unwrap().opcode(),
        OpCode::TypeEquality
    );
    assert_eq!(
        graph.node(structural.evidence).unwrap().opcode(),
        OpCode::EqualityWitness
    );
    assert_eq!(
        graph.node(structural.evidence).unwrap().ty(),
        Some(structural.node)
    );

    let reflexive = graph
        .prove_reflexive_equality(route(4), ROOT_CONTEXT, left)
        .unwrap();
    assert_eq!(
        graph.node(reflexive.node).unwrap().opcode(),
        OpCode::TypeEquality
    );

    let bytes = canonical_bytes(&graph);
    let decoded = decode_canonical(&bytes).unwrap();
    assert_eq!(canonical_bytes(&decoded), bytes);
}

#[test]
fn symmetry_transitivity_and_route_canonicalization_follow_proof_paths() {
    let mut graph = Graph::new();
    let left = graph.declare_atom_type(route(10), LocusWord(0x41)).unwrap();
    let right = graph.declare_atom_type(route(20), LocusWord(0x41)).unwrap();
    let forward = graph
        .prove_structural_equality(route(30), ROOT_CONTEXT, left, right)
        .unwrap();
    let backward = graph
        .prove_symmetric_equality(route(40), ROOT_CONTEXT, forward.node)
        .unwrap();
    graph
        .prove_transitive_equality(route(50), ROOT_CONTEXT, forward.node, backward.node)
        .unwrap();

    assert_eq!(
        graph.canonical_representative(ROOT_CONTEXT, right).unwrap(),
        left
    );
    assert_eq!(
        graph
            .canonical_representative_route(ROOT_CONTEXT, right)
            .unwrap(),
        route(10)
    );
}

#[test]
fn congruence_lifts_checked_argument_equalities() {
    let mut graph = Graph::new();
    let carrier_left = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let carrier_right = graph.declare_atom_type(route(2), LocusWord(0x52)).unwrap();
    let carrier_eq = graph
        .prove_structural_equality(route(3), ROOT_CONTEXT, carrier_left, carrier_right)
        .unwrap();

    let dimension = Dimension::new([1, 0, 0, 0, 0, 0, 0]);
    let quantity_left = graph
        .declare_quantity_type(route(4), carrier_left, dimension)
        .unwrap();
    let quantity_right = graph
        .declare_quantity_type(route(5), carrier_right, dimension)
        .unwrap();
    let quantity_eq = graph
        .prove_congruent_equality(
            route(6),
            ROOT_CONTEXT,
            quantity_left,
            quantity_right,
            &[carrier_eq.node],
        )
        .unwrap();

    let function_left = graph
        .declare_function_type(route(7), quantity_left, quantity_left)
        .unwrap();
    let function_right = graph
        .declare_function_type(route(8), quantity_right, quantity_right)
        .unwrap();
    graph
        .prove_congruent_equality(
            route(9),
            ROOT_CONTEXT,
            function_left,
            function_right,
            &[quantity_eq.node, quantity_eq.node],
        )
        .unwrap();

    assert_eq!(
        graph
            .canonical_representative(ROOT_CONTEXT, function_right)
            .unwrap(),
        function_left
    );
}

#[test]
fn child_context_equality_does_not_escape_to_parent() {
    let mut graph = Graph::new();
    let carrier = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let quantity = graph
        .declare_quantity_type(route(2), carrier, Dimension::new([0, 0, 0, 0, 0, 0, 0]))
        .unwrap();
    let value = graph
        .insert_value(route(3), ROOT_CONTEXT, quantity)
        .unwrap();
    let guard = graph
        .declare_constraint(
            route(4),
            ROOT_CONTEXT,
            value,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    let child = graph.extend_context(ROOT_CONTEXT, guard).unwrap();

    let left = graph.declare_atom_type(route(5), LocusWord(0x41)).unwrap();
    let right = graph.declare_atom_type(route(6), LocusWord(0x41)).unwrap();
    graph
        .prove_structural_equality(route(7), child, left, right)
        .unwrap();

    assert_eq!(
        graph.canonical_representative(ROOT_CONTEXT, right).unwrap(),
        right
    );
    assert_eq!(graph.canonical_representative(child, right).unwrap(), left);
}

#[test]
fn forged_equality_rule_is_rejected_during_decode() {
    let mut graph = Graph::new();
    let left = graph.declare_atom_type(route(1), LocusWord(0x41)).unwrap();
    let right = graph.declare_atom_type(route(2), LocusWord(0x41)).unwrap();
    let proof = graph
        .prove_structural_equality(route(3), ROOT_CONTEXT, left, right)
        .unwrap();
    let mut bytes = canonical_bytes(&graph);

    const HEADER_BYTES: usize = 22;
    const NODE_BYTES: usize = 24;
    const PAYLOAD_OFFSET: usize = 10;
    let offset = HEADER_BYTES + proof.evidence as usize * NODE_BYTES + PAYLOAD_OFFSET;
    bytes[offset..offset + 8].copy_from_slice(&1_u64.to_le_bytes());

    assert!(matches!(
        decode_canonical(&bytes),
        Err(DecodeError::InvalidAuthority)
    ));
}

#[test]
fn equality_path_exposes_deterministic_checked_provenance() {
    let mut graph = Graph::new();
    let first = graph.declare_atom_type(route(10), LocusWord(0x41)).unwrap();
    let second = graph.declare_atom_type(route(20), LocusWord(0x41)).unwrap();
    let third = graph.declare_atom_type(route(30), LocusWord(0x41)).unwrap();
    let first_proof = graph
        .prove_structural_equality(route(40), ROOT_CONTEXT, first, second)
        .unwrap();
    let second_proof = graph
        .prove_structural_equality(route(50), ROOT_CONTEXT, second, third)
        .unwrap();

    assert_eq!(
        graph.equality_path(ROOT_CONTEXT, first, third).unwrap(),
        vec![first_proof.node, second_proof.node]
    );
}

#[test]
fn invalid_merges_are_rejected_before_state_mutation() {
    let mut graph = Graph::new();
    let left = graph.declare_atom_type(route(1), LocusWord(0x41)).unwrap();
    let right = graph.declare_atom_type(route(2), LocusWord(0x42)).unwrap();
    let before = (
        graph.node_count(),
        graph.port_count(),
        graph.journal_len(),
        graph.state_symbol(),
    );

    assert!(matches!(
        graph.prove_structural_equality(route(3), ROOT_CONTEXT, left, right),
        Err(l64_native::Obstruction::InvalidEqualityRule)
    ));
    assert_eq!(
        (
            graph.node_count(),
            graph.port_count(),
            graph.journal_len(),
            graph.state_symbol(),
        ),
        before
    );
}

#[test]
fn forged_congruence_premise_path_is_rejected_during_decode() {
    let mut graph = Graph::new();
    let carrier_left = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let carrier_right = graph.declare_atom_type(route(2), LocusWord(0x52)).unwrap();
    let carrier_eq = graph
        .prove_structural_equality(route(3), ROOT_CONTEXT, carrier_left, carrier_right)
        .unwrap();
    let dimension = Dimension::new([1, 0, 0, 0, 0, 0, 0]);
    let quantity_left = graph
        .declare_quantity_type(route(4), carrier_left, dimension)
        .unwrap();
    let quantity_right = graph
        .declare_quantity_type(route(5), carrier_right, dimension)
        .unwrap();
    let proof = graph
        .prove_congruent_equality(
            route(6),
            ROOT_CONTEXT,
            quantity_left,
            quantity_right,
            &[carrier_eq.node],
        )
        .unwrap();
    let evidence = graph.node(proof.evidence).unwrap();
    let premise_port = evidence.port_range().start;
    let mut bytes = canonical_bytes(&graph);

    const HEADER_BYTES: usize = 22;
    const NODE_BYTES: usize = 24;
    const PORT_BYTES: usize = 8;
    let port_base = HEADER_BYTES + graph.node_count() * NODE_BYTES;
    let target_offset = port_base + premise_port * PORT_BYTES;
    bytes[target_offset..target_offset + 4].copy_from_slice(&carrier_left.to_le_bytes());

    assert!(matches!(
        decode_canonical(&bytes),
        Err(DecodeError::InvalidAuthority)
    ));
}

#[test]
fn equality_proofs_reach_exact_rna_dna_fixed_point() {
    let source = b"L64R1 0x4551\na 1 0x41\na 2 0x41\ne 3 2 1 2 0\ne 4 3 2 1 0 3\ne 5 4 1 1 0 3 4\n";
    let dna = rna_to_dna(source).unwrap();
    let canonical_rna = dna_to_rna(&dna).unwrap();
    let rebuilt = rna_to_dna(&canonical_rna).unwrap();
    assert_eq!(rebuilt, dna);
    assert!(canonical_rna.windows(2).any(|window| window == b"e "));
}
