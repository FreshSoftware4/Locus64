use l64_native::{
    ConstraintKind, DecodeError, Dimension, Graph, LocusWord, Obstruction, OpCode, Proposal,
    ROOT_CONTEXT, Route, canonical_bytes, decode_canonical,
};

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x434f4e5354524149)).composed(LocusWord(index))
}

fn quantity_fixture() -> (Graph, u32, u32, u32, u32, u32) {
    let mut graph = Graph::new();
    let real = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let length = graph
        .declare_quantity_type(route(2), real, Dimension::LENGTH)
        .unwrap();
    let time = graph
        .declare_quantity_type(route(3), real, Dimension::TIME)
        .unwrap();
    let speed = graph
        .declare_quantity_type(
            route(4),
            real,
            Dimension::LENGTH.divide(Dimension::TIME).unwrap(),
        )
        .unwrap();
    let area = graph
        .declare_quantity_type(
            route(5),
            real,
            Dimension::LENGTH.multiply(Dimension::LENGTH).unwrap(),
        )
        .unwrap();
    (graph, real, length, time, speed, area)
}

#[test]
fn dimension_vector_composes_without_schema_objects() {
    let velocity = Dimension::LENGTH.divide(Dimension::TIME).unwrap();
    let acceleration = velocity.divide(Dimension::TIME).unwrap();
    assert_eq!(velocity.exponents(), [1, 0, -1, 0, 0, 0, 0]);
    assert_eq!(acceleration.exponents(), [1, 0, -2, 0, 0, 0, 0]);
    assert_eq!(Dimension::LENGTH.square_root(), None);
    assert_eq!(
        Dimension::LENGTH
            .multiply(Dimension::LENGTH)
            .unwrap()
            .square_root(),
        Some(Dimension::LENGTH)
    );
}

#[test]
fn incompatible_addition_rejects_without_mutation() {
    let (mut graph, _, length, time, _, _) = quantity_fixture();
    let left = graph.insert_value(route(10), ROOT_CONTEXT, length).unwrap();
    let right = graph.insert_value(route(11), ROOT_CONTEXT, time).unwrap();
    let before = canonical_bytes(&graph);

    let result = graph.transact(Proposal::add(route(12), ROOT_CONTEXT, left, right, length));

    assert_eq!(
        result,
        Err(Obstruction::DimensionMismatch {
            left: Dimension::LENGTH,
            right: Dimension::TIME,
        })
    );
    assert_eq!(canonical_bytes(&graph), before);
    assert!(graph.resolve(&route(12)).is_none());
}

#[test]
fn multiplication_and_division_validate_derived_dimensions() {
    let (mut graph, _, length, time, speed, area) = quantity_fixture();
    let left = graph.insert_value(route(20), ROOT_CONTEXT, length).unwrap();
    let right = graph.insert_value(route(21), ROOT_CONTEXT, length).unwrap();
    let duration = graph.insert_value(route(22), ROOT_CONTEXT, time).unwrap();

    let product = graph
        .transact(Proposal::multiply(
            route(23),
            ROOT_CONTEXT,
            left,
            right,
            area,
        ))
        .unwrap();
    let quotient = graph
        .transact(Proposal::divide(
            route(24),
            ROOT_CONTEXT,
            left,
            duration,
            speed,
        ))
        .unwrap();

    assert_eq!(
        graph.node(product.evidence).unwrap().opcode(),
        OpCode::KernelWitness
    );
    assert_eq!(
        graph.node(quotient.evidence).unwrap().opcode(),
        OpCode::KernelWitness
    );
    let bytes = canonical_bytes(&graph);
    let decoded = decode_canonical(&bytes).unwrap();
    assert_eq!(canonical_bytes(&decoded), bytes);
}

#[test]
fn decoder_rechecks_operation_law_instead_of_trusting_witness_shape() {
    let (mut graph, _, length, time, speed, area) = quantity_fixture();
    let left = graph.insert_value(route(30), ROOT_CONTEXT, length).unwrap();
    let duration = graph.insert_value(route(31), ROOT_CONTEXT, time).unwrap();
    let committed = graph
        .transact(Proposal::divide(
            route(32),
            ROOT_CONTEXT,
            left,
            duration,
            speed,
        ))
        .unwrap();
    let mut bytes = canonical_bytes(&graph);
    let node_record = 22 + committed.node as usize * 24;
    bytes[node_record + 6..node_record + 10].copy_from_slice(&area.to_le_bytes());
    let judgment = committed.node + 1;
    let conclusion_port = graph.node(judgment).unwrap().port_range().end - 1;
    let port_section = 22 + graph.node_count() * 24;
    let conclusion_record = port_section + conclusion_port * 8;
    bytes[conclusion_record..conclusion_record + 4].copy_from_slice(&area.to_le_bytes());

    assert_eq!(
        decode_canonical(&bytes).unwrap_err(),
        DecodeError::InvalidAuthority
    );
}

#[test]
fn contradictory_context_is_rejected_before_context_mutation() {
    let (mut graph, _, length, _, _, _) = quantity_fixture();
    let value = graph.insert_value(route(40), ROOT_CONTEXT, length).unwrap();
    let nonnegative = graph
        .declare_constraint(
            route(41),
            ROOT_CONTEXT,
            value,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    let positive_context = graph.extend_context(ROOT_CONTEXT, nonnegative).unwrap();
    let negative = graph
        .declare_constraint(
            route(42),
            ROOT_CONTEXT,
            value,
            ConstraintKind::NonNegative,
            false,
        )
        .unwrap();
    let before = canonical_bytes(&graph);
    let before_journal = graph.journal_len();

    assert_eq!(
        graph.extend_context(positive_context, negative),
        Err(Obstruction::ContradictoryConstraint {
            subject: value,
            kind: ConstraintKind::NonNegative,
        })
    );
    assert_eq!(canonical_bytes(&graph), before);
    assert_eq!(graph.journal_len(), before_journal);
}

#[test]
fn decoder_rejects_a_serialized_contradictory_context() {
    let (mut graph, _, length, _, _, _) = quantity_fixture();
    let value = graph.insert_value(route(50), ROOT_CONTEXT, length).unwrap();
    let first = graph
        .declare_constraint(
            route(51),
            ROOT_CONTEXT,
            value,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    let first_context = graph.extend_context(ROOT_CONTEXT, first).unwrap();
    let second = graph
        .declare_constraint(
            route(52),
            first_context,
            value,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    graph.extend_context(first_context, second).unwrap();

    let valid = canonical_bytes(&graph);
    let decoded = decode_canonical(&valid).unwrap();
    assert_eq!(canonical_bytes(&decoded), valid);

    let mut contradictory = valid;
    let second_record = 22 + second as usize * 24;
    contradictory[second_record + 10..second_record + 18].copy_from_slice(&1_u64.to_le_bytes());
    assert_eq!(
        decode_canonical(&contradictory).unwrap_err(),
        DecodeError::InvalidAuthority
    );
}

fn square_root_fixture() -> (Graph, u32, u32, u32) {
    let mut graph = Graph::new();
    let real = graph.declare_atom_type(route(60), LocusWord(0x52)).unwrap();
    let length = graph
        .declare_quantity_type(route(61), real, Dimension::LENGTH)
        .unwrap();
    let area = graph
        .declare_quantity_type(
            route(62),
            real,
            Dimension::LENGTH.multiply(Dimension::LENGTH).unwrap(),
        )
        .unwrap();
    let value = graph.insert_value(route(63), ROOT_CONTEXT, area).unwrap();
    (graph, value, length, area)
}

#[test]
fn unresolved_square_root_guard_is_preserved_as_obligation() {
    let (mut graph, value, length, _) = square_root_fixture();
    let committed = graph
        .transact(Proposal::square_root(
            route(64),
            ROOT_CONTEXT,
            value,
            length,
        ))
        .unwrap();

    let evidence = graph.node(committed.evidence).unwrap();
    assert_eq!(evidence.opcode(), OpCode::Obligation);
    assert_eq!(graph.ports(committed.evidence).unwrap()[0].target(), value);
    let bytes = canonical_bytes(&graph);
    let decoded = decode_canonical(&bytes).unwrap();
    assert_eq!(canonical_bytes(&decoded), bytes);
}

#[test]
fn discharged_square_root_guard_produces_kernel_witness() {
    let (mut graph, value, length, _) = square_root_fixture();
    let condition = graph
        .declare_constraint(
            route(65),
            ROOT_CONTEXT,
            value,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    let context = graph.extend_context(ROOT_CONTEXT, condition).unwrap();
    let committed = graph
        .transact(Proposal::square_root(route(66), context, value, length))
        .unwrap();

    assert_eq!(
        graph.node(committed.evidence).unwrap().opcode(),
        OpCode::KernelWitness
    );
}

#[test]
fn refuted_square_root_guard_rejects_without_mutation() {
    let (mut graph, value, length, _) = square_root_fixture();
    let condition = graph
        .declare_constraint(
            route(67),
            ROOT_CONTEXT,
            value,
            ConstraintKind::NonNegative,
            false,
        )
        .unwrap();
    let context = graph.extend_context(ROOT_CONTEXT, condition).unwrap();
    let before = canonical_bytes(&graph);
    let before_journal = graph.journal_len();

    assert_eq!(
        graph.transact(Proposal::square_root(route(68), context, value, length)),
        Err(Obstruction::GuardViolated {
            subject: value,
            kind: ConstraintKind::NonNegative,
        })
    );
    assert_eq!(canonical_bytes(&graph), before);
    assert_eq!(graph.journal_len(), before_journal);
}

#[test]
fn decoder_refuses_a_forged_witness_for_an_unresolved_guard() {
    let (mut graph, value, length, _) = square_root_fixture();
    let committed = graph
        .transact(Proposal::square_root(
            route(69),
            ROOT_CONTEXT,
            value,
            length,
        ))
        .unwrap();
    let mut bytes = canonical_bytes(&graph);
    let node_count = graph.node_count();
    let port_count = graph.port_count();
    let evidence_record = 22 + committed.evidence as usize * 24;
    bytes[evidence_record..evidence_record + 2]
        .copy_from_slice(&(OpCode::KernelWitness as u16).to_le_bytes());
    bytes[evidence_record + 22..evidence_record + 24].copy_from_slice(&0_u16.to_le_bytes());
    bytes[10..14].copy_from_slice(&((port_count - 1) as u32).to_le_bytes());
    let last_port = 22 + node_count * 24 + (port_count - 1) * 8;
    bytes.drain(last_port..last_port + 8);

    assert_eq!(
        decode_canonical(&bytes).unwrap_err(),
        DecodeError::InvalidAuthority
    );
}
