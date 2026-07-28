use l64_certification::CertificationVerdict;
use l64_change::{ContactChange, compare_bundle, compare_dna};
use l64_native::{
    ConstraintKind, Dimension, Graph, LocusWord, Proposal, ROOT_CONTEXT, Route, dna_bytes,
    rna_to_dna,
};
use l64_transport::bundle_bytes;

const CERTIFIED_RNA: &[u8] = b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n";

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x4348414e47453031)).composed(LocusWord(index))
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
fn identical_dna_is_exactly_unchanged() {
    let dna = rna_to_dna(CERTIFIED_RNA).unwrap();
    let change = compare_dna(&dna, &dna).unwrap();
    assert!(change.exact_equal);
    assert!(!change.sections.any());
    assert_eq!(change.before_verdict, CertificationVerdict::Certified);
    assert_eq!(change.after_verdict, CertificationVerdict::Certified);
    assert!(
        change
            .context_changes
            .iter()
            .all(|context| context.contact == ContactChange::Unchanged)
    );
}

#[test]
fn structural_and_verdict_movement_is_directly_visible() {
    let before = rna_to_dna(CERTIFIED_RNA).unwrap();
    let (open, _) = open_graph();
    let after = dna_bytes(&open).unwrap();
    let change = compare_dna(&before, &after).unwrap();
    assert!(!change.exact_equal);
    assert!(change.sections.any());
    assert_eq!(change.before_verdict, CertificationVerdict::Certified);
    assert_eq!(change.after_verdict, CertificationVerdict::Open);
    assert!(change.nodes.delta() != 0);
    let rendered = change.render_text();
    assert!(rendered.contains("scope=exact_native_authority_change"));
    assert!(rendered.contains("after_verdict=OPEN"));
    assert!(!rendered.contains("prediction_id"));
}

#[test]
fn invalid_child_context_appears_as_context_movement() {
    let (mut before_graph, subject) = open_graph();
    let before = dna_bytes(&before_graph).unwrap();
    let cause = before_graph
        .declare_constraint(
            route(5),
            ROOT_CONTEXT,
            subject,
            ConstraintKind::NonNegative,
            false,
        )
        .unwrap();
    let child = before_graph.extend_context(ROOT_CONTEXT, cause).unwrap();
    let after = dna_bytes(&before_graph).unwrap();
    let change = compare_dna(&before, &after).unwrap();
    assert_eq!(change.after_verdict, CertificationVerdict::Invalid);
    assert_eq!(
        change.context_changes[child as usize].contact,
        ContactChange::Added
    );
    assert_eq!(
        change.context_changes[child as usize].after_verdict,
        Some(CertificationVerdict::Invalid)
    );
}

#[test]
fn bundle_members_remain_independent_change_contacts() {
    let certified = rna_to_dna(CERTIFIED_RNA).unwrap();
    let (open, _) = open_graph();
    let open = dna_bytes(&open).unwrap();
    let before = bundle_bytes(&[&certified, &certified]).unwrap();
    let after = bundle_bytes(&[&certified, &open, &certified]).unwrap();
    let change = compare_bundle(&before, &after).unwrap();
    assert_eq!(change.before_members, 2);
    assert_eq!(change.after_members, 3);
    assert_eq!(change.members[0].contact, ContactChange::Unchanged);
    assert_eq!(change.members[1].contact, ContactChange::Changed);
    assert_eq!(change.members[2].contact, ContactChange::Added);
    let rendered = change.render_text();
    assert!(rendered.contains("composite_authority=none"));
    assert!(rendered.contains("composite_change_verdict=none"));
    assert!(!rendered.contains("bundle_prediction="));
}
