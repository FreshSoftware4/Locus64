use l64_native::{
    ClosureState, ConstraintKind, Dimension, Graph, LocusWord, Obstruction, Proposal, ROOT_CONTEXT,
    Route, canonical_bytes, decode_canonical,
};

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x434c4f5355524539)).composed(LocusWord(index))
}

struct GuardedFixture {
    graph: Graph,
    subject: u32,
    sqrt: u32,
    sqrt_evidence: u32,
    downstream: u32,
    independent: u32,
}

fn guarded_fixture() -> GuardedFixture {
    let mut graph = Graph::new();
    let scalar = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let dimensionless = Dimension::new([0, 0, 0, 0, 0, 0, 0]);
    let quantity = graph
        .declare_quantity_type(route(2), scalar, dimensionless)
        .unwrap();
    let subject = graph
        .insert_value(route(3), ROOT_CONTEXT, quantity)
        .unwrap();
    let peer = graph
        .insert_value(route(4), ROOT_CONTEXT, quantity)
        .unwrap();
    let sqrt_result = graph
        .transact(Proposal::square_root(
            route(5),
            ROOT_CONTEXT,
            subject,
            quantity,
        ))
        .unwrap();
    let downstream = graph
        .transact(Proposal::multiply(
            route(6),
            ROOT_CONTEXT,
            sqrt_result.node,
            peer,
            quantity,
        ))
        .unwrap()
        .node;

    let independent_left = graph
        .insert_value(route(7), ROOT_CONTEXT, quantity)
        .unwrap();
    let independent_right = graph
        .insert_value(route(8), ROOT_CONTEXT, quantity)
        .unwrap();
    let independent = graph
        .transact(Proposal::add(
            route(9),
            ROOT_CONTEXT,
            independent_left,
            independent_right,
            quantity,
        ))
        .unwrap()
        .node;

    GuardedFixture {
        graph,
        subject,
        sqrt: sqrt_result.node,
        sqrt_evidence: sqrt_result.evidence,
        downstream,
        independent,
    }
}

#[test]
fn reverse_incidence_is_exact_and_rebuilt_from_canonical_graph() {
    let fixture = guarded_fixture();
    let affected = fixture.graph.affected_nodes(fixture.subject).unwrap();
    assert!(affected.contains(&fixture.sqrt));
    assert!(affected.contains(&fixture.sqrt_evidence));
    assert!(affected.contains(&fixture.downstream));
    assert!(!affected.contains(&fixture.independent));

    let bytes = canonical_bytes(&fixture.graph);
    let decoded = decode_canonical(&bytes).unwrap();
    assert_eq!(decoded.affected_nodes(fixture.subject).unwrap(), affected);
    assert_eq!(canonical_bytes(&decoded), bytes);
}

#[test]
fn positive_refinement_discharges_only_the_dependent_obligation_chain() {
    let mut fixture = guarded_fixture();
    assert_eq!(
        fixture
            .graph
            .closure_state(ROOT_CONTEXT, fixture.sqrt)
            .unwrap(),
        ClosureState::Open
    );
    assert_eq!(
        fixture.graph.context_closure(ROOT_CONTEXT).unwrap(),
        ClosureState::Open
    );

    let cause = fixture
        .graph
        .declare_constraint(
            route(20),
            ROOT_CONTEXT,
            fixture.subject,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    let child = fixture.graph.extend_context(ROOT_CONTEXT, cause).unwrap();
    let transitions = fixture
        .graph
        .closure_transitions(ROOT_CONTEXT, child)
        .unwrap();

    assert!(transitions.iter().any(|transition| {
        transition.node() == fixture.sqrt
            && transition.before() == ClosureState::Open
            && transition.after() == ClosureState::Closed
            && transition.cause() == cause
    }));
    assert!(transitions.iter().any(|transition| {
        transition.node() == fixture.sqrt_evidence
            && transition.before() == ClosureState::Open
            && transition.after() == ClosureState::Closed
    }));
    assert!(
        !transitions
            .iter()
            .any(|transition| transition.node() == fixture.independent)
    );
    assert_eq!(
        fixture.graph.context_closure(child).unwrap(),
        ClosureState::Closed
    );
    assert_eq!(
        fixture.graph.global_closure().unwrap(),
        ClosureState::Open,
        "the original open context remains valid and independently visible"
    );
}

#[test]
fn negative_refinement_invalidates_exactly_the_reverse_reachable_subgraph() {
    let mut fixture = guarded_fixture();
    let independent_before = fixture
        .graph
        .closure_state(ROOT_CONTEXT, fixture.independent)
        .unwrap();
    let cause = fixture
        .graph
        .declare_constraint(
            route(30),
            ROOT_CONTEXT,
            fixture.subject,
            ConstraintKind::NonNegative,
            false,
        )
        .unwrap();
    let child = fixture.graph.extend_context(ROOT_CONTEXT, cause).unwrap();
    let transitions = fixture
        .graph
        .closure_transitions(ROOT_CONTEXT, child)
        .unwrap();

    for dependent in [fixture.sqrt, fixture.sqrt_evidence, fixture.downstream] {
        assert!(transitions.iter().any(|transition| {
            transition.node() == dependent && transition.after() == ClosureState::Invalid
        }));
    }
    assert!(
        !transitions
            .iter()
            .any(|transition| transition.node() == fixture.independent)
    );
    assert_eq!(
        fixture
            .graph
            .closure_state(child, fixture.independent)
            .unwrap(),
        independent_before
    );
    assert_eq!(
        fixture.graph.context_closure(child).unwrap(),
        ClosureState::Invalid
    );
    assert_eq!(
        fixture.graph.global_closure().unwrap(),
        ClosureState::Invalid
    );
}

#[test]
fn equality_provenance_is_invalidated_when_its_endpoint_closure_fails() {
    let mut fixture = guarded_fixture();
    let equality = fixture
        .graph
        .prove_reflexive_equality(route(40), ROOT_CONTEXT, fixture.sqrt)
        .unwrap();
    let cause = fixture
        .graph
        .declare_constraint(
            route(41),
            ROOT_CONTEXT,
            fixture.subject,
            ConstraintKind::NonNegative,
            false,
        )
        .unwrap();
    let child = fixture.graph.extend_context(ROOT_CONTEXT, cause).unwrap();
    let transitions = fixture
        .graph
        .closure_transitions(ROOT_CONTEXT, child)
        .unwrap();

    assert!(transitions.iter().any(|transition| {
        transition.node() == equality.node && transition.after() == ClosureState::Invalid
    }));
    assert!(transitions.iter().any(|transition| {
        transition.node() == equality.evidence && transition.after() == ClosureState::Invalid
    }));
}

#[test]
fn closure_transition_requires_one_direct_constraint_refinement() {
    let mut fixture = guarded_fixture();
    let first_binding = fixture
        .graph
        .declare_constraint(
            route(50),
            ROOT_CONTEXT,
            fixture.subject,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    let child = fixture
        .graph
        .extend_context(ROOT_CONTEXT, first_binding)
        .unwrap();
    let second_subject = fixture
        .graph
        .insert_value(
            route(51),
            child,
            fixture.graph.node(fixture.subject).unwrap().ty().unwrap(),
        )
        .unwrap();
    let second_binding = fixture
        .graph
        .declare_constraint(
            route(52),
            child,
            second_subject,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    let grandchild = fixture.graph.extend_context(child, second_binding).unwrap();

    assert_eq!(
        fixture.graph.closure_transitions(ROOT_CONTEXT, grandchild),
        Err(Obstruction::ContextNotDirectRefinement {
            parent: ROOT_CONTEXT,
            child: grandchild,
        })
    );

    let value_binding = fixture
        .graph
        .insert_value(
            route(53),
            ROOT_CONTEXT,
            fixture.graph.node(fixture.subject).unwrap().ty().unwrap(),
        )
        .unwrap();
    let non_constraint_child = fixture
        .graph
        .extend_context(ROOT_CONTEXT, value_binding)
        .unwrap();
    assert_eq!(
        fixture
            .graph
            .closure_transitions(ROOT_CONTEXT, non_constraint_child),
        Err(Obstruction::RefinementBindingNotConstraint {
            context: non_constraint_child,
            binding: value_binding,
        })
    );
}

#[test]
fn closure_queries_do_not_mutate_authority_or_canonical_bytes() {
    let mut fixture = guarded_fixture();
    let cause = fixture
        .graph
        .declare_constraint(
            route(60),
            ROOT_CONTEXT,
            fixture.subject,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    let child = fixture.graph.extend_context(ROOT_CONTEXT, cause).unwrap();
    let before = canonical_bytes(&fixture.graph);
    let commitment = fixture.graph.state_commitment();
    let journal = fixture.graph.journal_len();

    let _ = fixture.graph.direct_dependents(fixture.subject).unwrap();
    let _ = fixture.graph.affected_nodes(fixture.subject).unwrap();
    let _ = fixture
        .graph
        .closure_transitions(ROOT_CONTEXT, child)
        .unwrap();
    let _ = fixture.graph.context_closure(child).unwrap();
    let _ = fixture.graph.global_closure().unwrap();

    assert_eq!(canonical_bytes(&fixture.graph), before);
    assert_eq!(fixture.graph.state_commitment(), commitment);
    assert_eq!(fixture.graph.journal_len(), journal);
}
