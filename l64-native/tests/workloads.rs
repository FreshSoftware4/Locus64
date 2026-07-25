use l64_native::{Graph, LocusWord, Obstruction, OpCode, Proposal, ROOT_CONTEXT, Route};

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x4c36344e41544956)).composed(LocusWord(index))
}

#[test]
fn valid_typed_function_composition_commits() {
    let mut graph = Graph::new();
    let a = graph.declare_atom_type(route(1), LocusWord(0x41)).unwrap();
    let b = graph.declare_atom_type(route(2), LocusWord(0x42)).unwrap();
    let c = graph.declare_atom_type(route(3), LocusWord(0x43)).unwrap();
    let ab = graph.declare_function_type(route(4), a, b).unwrap();
    let bc = graph.declare_function_type(route(5), b, c).unwrap();
    let ac = graph.declare_function_type(route(6), a, c).unwrap();
    let first = graph.insert_value(route(7), ROOT_CONTEXT, ab).unwrap();
    let second = graph.insert_value(route(8), ROOT_CONTEXT, bc).unwrap();
    let before = graph.state_symbol();
    let before_nodes = graph.node_count();

    let committed = graph
        .transact(Proposal::compose(route(9), ROOT_CONTEXT, first, second, ac))
        .unwrap();

    assert_eq!(graph.node_count(), before_nodes + 3);
    assert_ne!(graph.state_symbol(), before);
    assert_eq!(graph.node(committed.node).unwrap().ty(), Some(ac));
    assert_eq!(graph.ports(committed.node).unwrap().len(), 2);
    let evidence = graph.node(committed.evidence).unwrap();
    assert_eq!(evidence.opcode(), OpCode::KernelWitness);
    let judgment = evidence.ty().unwrap();
    assert_eq!(graph.node(judgment).unwrap().opcode(), OpCode::TypeJudgment);
    assert_eq!(graph.ports(judgment).unwrap().len(), 4);
    assert_eq!(graph.journal().last().unwrap().after(), committed.symbol);
}

#[test]
fn invalid_matrix_shape_rejects_without_mutation() {
    let mut graph = Graph::new();
    let scalar = graph.declare_atom_type(route(11), LocusWord(0x52)).unwrap();
    let graph
        .declare_matrix_type(route(12), scalar, 2, 3)
        .unwrap();
    let graph
        .declare_matrix_type(route(13), scalar, 4, 2)
        .unwrap();
    let graph
        .declare_matrix_type(route(14), scalar, 2, 2)
        .unwrap();
    let graph
        .insert_value(route(15), ROOT_CONTEXT, left_ty)
        .unwrap();
    let graph
        .insert_value(route(16), ROOT_CONTEXT, right_ty)
        .unwrap();

    let before_commitment = graph.state_symbol();
    let before_nodes = graph.node_count();
    let before_ports = graph.port_count();
    let before_contexts = graph.context_count();
    let before_journal = graph.journal_len();

    let result = graph.transact(Proposal::matrix_multiply(
        route(17),
        ROOT_CONTEXT,
        left,
        right,
        output_ty,
    ));

    assert_eq!(
        result,
        Err(Obstruction::MatrixShapeMismatch {
            left_cols: 3,
            right_rows: 4,
        })
    );
    assert_eq!(graph.state_symbol(), before_commitment);
    assert_eq!(graph.node_count(), before_nodes);
    assert_eq!(graph.port_count(), before_ports);
    assert_eq!(graph.context_count(), before_contexts);
    assert_eq!(graph.journal_len(), before_journal);
    assert!(graph.resolve(&route(17)).is_none());
}

#[test]
fn kernel_judgment_cannot_be_forged_through_value_insertion() {
    let mut graph = Graph::new();
    let a = graph.declare_atom_type(route(31), LocusWord(0x41)).unwrap();
    let b = graph.declare_atom_type(route(32), LocusWord(0x42)).unwrap();
    let c = graph.declare_atom_type(route(33), LocusWord(0x43)).unwrap();
    let ab = graph.declare_function_type(route(34), a, b).unwrap();
    let bc = graph.declare_function_type(route(35), b, c).unwrap();
    let ac = graph.declare_function_type(route(36), a, c).unwrap();
    let first = graph.insert_value(route(37), ROOT_CONTEXT, ab).unwrap();
    let second = graph.insert_value(route(38), ROOT_CONTEXT, bc).unwrap();
    let committed = graph
        .transact(Proposal::compose(
            route(39),
            ROOT_CONTEXT,
            first,
            second,
            ac,
        ))
        .unwrap();
    let judgment = graph.node(committed.evidence).unwrap().ty().unwrap();
    let before = graph.state_symbol();
    let before_nodes = graph.node_count();

    assert_eq!(
        graph.insert_value(route(40), ROOT_CONTEXT, judgment),
        Err(Obstruction::EvidenceOnlyType { node: judgment })
    );
    assert_eq!(graph.state_symbol(), before);
    assert_eq!(graph.node_count(), before_nodes);
    assert!(graph.resolve(&route(40)).is_none());
}
