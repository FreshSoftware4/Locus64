use l64_native::{
    ClosureState, ConstraintKind, Dimension, Graph, LocusWord, OpCode, Proposal, ROOT_CONTEXT,
    Route, canonical_bytes, decode_canonical,
};
use l64_projection::{BurdenState, ProjectionError, ProjectionSet};

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x50524f4a45435431)).composed(LocusWord(index))
}

struct Fixture {
    graph: Graph,
    subject: u32,
    sqrt: u32,
    downstream: u32,
    independent: u32,
}

fn fixture() -> Fixture {
    let mut graph = Graph::new();
    let scalar = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let quantity = graph
        .declare_quantity_type(route(2), scalar, Dimension::new([0, 0, 0, 0, 0, 0, 0]))
        .unwrap();
    let subject = graph
        .insert_value(route(3), ROOT_CONTEXT, quantity)
        .unwrap();
    let peer = graph
        .insert_value(route(4), ROOT_CONTEXT, quantity)
        .unwrap();
    let sqrt = graph
        .transact(Proposal::square_root(
            route(5),
            ROOT_CONTEXT,
            subject,
            quantity,
        ))
        .unwrap()
        .node;
    let downstream = graph
        .transact(Proposal::multiply(
            route(6),
            ROOT_CONTEXT,
            sqrt,
            peer,
            quantity,
        ))
        .unwrap()
        .node;
    let left = graph
        .insert_value(route(7), ROOT_CONTEXT, quantity)
        .unwrap();
    let right = graph
        .insert_value(route(8), ROOT_CONTEXT, quantity)
        .unwrap();
    let independent = graph
        .transact(Proposal::add(route(9), ROOT_CONTEXT, left, right, quantity))
        .unwrap()
        .node;
    graph
        .prove_reflexive_equality(route(10), ROOT_CONTEXT, independent)
        .unwrap();
    Fixture {
        graph,
        subject,
        sqrt,
        downstream,
        independent,
    }
}

#[test]
fn all_upper_views_are_derived_without_mutating_native_authority() {
    let fixture = fixture();
    let before = canonical_bytes(&fixture.graph);
    let commitment = fixture.graph.state_commitment();
    let counts = (
        fixture.graph.node_count(),
        fixture.graph.port_count(),
        fixture.graph.context_count(),
        fixture.graph.journal_len(),
    );

    let projection = ProjectionSet::derive(&fixture.graph, ROOT_CONTEXT, 16).unwrap();
    projection.verify(&fixture.graph).unwrap();

    assert_eq!(canonical_bytes(&fixture.graph), before);
    assert_eq!(fixture.graph.state_commitment(), commitment);
    assert_eq!(
        (
            fixture.graph.node_count(),
            fixture.graph.port_count(),
            fixture.graph.context_count(),
            fixture.graph.journal_len(),
        ),
        counts
    );
    assert!(
        projection
            .atlas
            .candidates
            .iter()
            .any(|item| item.source.node == fixture.sqrt && item.closure == ClosureState::Open)
    );
    assert!(
        projection
            .certification
            .burdens
            .iter()
            .any(|item| item.subject.node == fixture.sqrt && item.state == BurdenState::Open)
    );
    assert!(!projection.replay.steps.is_empty());
    assert!(projection.report.open > 0);
    assert!(
        projection
            .research
            .candidates
            .iter()
            .any(|item| item.source.node == fixture.sqrt)
    );
}

#[test]
fn regeneration_and_rendering_are_deterministic() {
    let fixture = fixture();
    let first = ProjectionSet::derive(&fixture.graph, ROOT_CONTEXT, 8).unwrap();
    let second = ProjectionSet::derive(&fixture.graph, ROOT_CONTEXT, 8).unwrap();
    assert_eq!(first, second);
    assert_eq!(first.render_text(), second.render_text());
}

#[test]
fn every_projection_reference_resolves_to_native_authority() {
    let fixture = fixture();
    let projection = ProjectionSet::derive(&fixture.graph, ROOT_CONTEXT, 32).unwrap();
    for candidate in &projection.atlas.candidates {
        assert_eq!(
            fixture.graph.resolve(&candidate.source.route),
            Some(candidate.source.node)
        );
        for port in &candidate.ports {
            assert_eq!(
                fixture.graph.resolve(&port.target.route),
                Some(port.target.node)
            );
        }
        for dependent in &candidate.direct_dependents {
            assert_eq!(
                fixture.graph.resolve(&dependent.route),
                Some(dependent.node)
            );
        }
        if let Some(evidence) = &candidate.evidence {
            assert_eq!(
                fixture.graph.resolve(&evidence.evidence.route),
                Some(evidence.evidence.node)
            );
            if let Some(judgment) = &evidence.judgment {
                assert_eq!(fixture.graph.resolve(&judgment.route), Some(judgment.node));
            }
        }
    }
}

#[test]
fn stale_and_forged_projections_are_rejected() {
    let mut fixture = fixture();
    let projection = ProjectionSet::derive(&fixture.graph, ROOT_CONTEXT, 8).unwrap();

    let mut forged = projection.clone();
    forged.report.closed = forged.report.closed.saturating_add(1);
    assert_eq!(
        forged.verify(&fixture.graph),
        Err(ProjectionError::ProjectionMismatch)
    );

    fixture
        .graph
        .declare_atom_type(route(100), LocusWord(0x99))
        .unwrap();
    assert_eq!(
        projection.verify(&fixture.graph),
        Err(ProjectionError::SourceMismatch)
    );
}

#[test]
fn context_specific_views_preserve_open_closed_and_invalid_divergence() {
    let mut positive = fixture();
    let cause = positive
        .graph
        .declare_constraint(
            route(20),
            ROOT_CONTEXT,
            positive.subject,
            ConstraintKind::NonNegative,
            true,
        )
        .unwrap();
    let child = positive.graph.extend_context(ROOT_CONTEXT, cause).unwrap();
    let root = ProjectionSet::derive(&positive.graph, ROOT_CONTEXT, 16).unwrap();
    let refined = ProjectionSet::derive(&positive.graph, child, 16).unwrap();
    let root_sqrt = root
        .certification
        .burdens
        .iter()
        .find(|item| item.subject.node == positive.sqrt)
        .unwrap();
    let child_sqrt = refined
        .certification
        .burdens
        .iter()
        .find(|item| item.subject.node == positive.sqrt)
        .unwrap();
    assert_eq!(root_sqrt.state, BurdenState::Open);
    assert_eq!(child_sqrt.state, BurdenState::Discharged);
    assert_eq!(child_sqrt.evidence_opcode, Some(OpCode::Obligation));
    assert_eq!(child_sqrt.closure, ClosureState::Closed);
    assert_eq!(root.source.context, ROOT_CONTEXT);
    assert_eq!(refined.source.context, child);

    let mut negative = fixture();
    let cause = negative
        .graph
        .declare_constraint(
            route(30),
            ROOT_CONTEXT,
            negative.subject,
            ConstraintKind::NonNegative,
            false,
        )
        .unwrap();
    let child = negative.graph.extend_context(ROOT_CONTEXT, cause).unwrap();
    let invalid = ProjectionSet::derive(&negative.graph, child, 16).unwrap();
    assert!(
        invalid.certification.burdens.iter().any(|item| {
            item.subject.node == negative.sqrt && item.state == BurdenState::Invalid
        })
    );
    assert!(invalid.certification.burdens.iter().any(|item| {
        item.subject.node == negative.downstream && item.state == BurdenState::Invalid
    }));
    assert!(!invalid.certification.burdens.iter().any(|item| {
        item.subject.node == negative.independent && item.state == BurdenState::Invalid
    }));
}

#[test]
fn replay_projection_is_bound_to_the_actual_runtime_journal() {
    let fixture = fixture();
    let projection = ProjectionSet::derive(&fixture.graph, ROOT_CONTEXT, 8).unwrap();
    let decoded = decode_canonical(&canonical_bytes(&fixture.graph)).unwrap();
    assert_eq!(decoded.state_commitment(), fixture.graph.state_commitment());
    assert_eq!(decoded.journal_len(), 0);
    assert_eq!(
        projection.verify(&decoded),
        Err(ProjectionError::SourceMismatch)
    );
}
