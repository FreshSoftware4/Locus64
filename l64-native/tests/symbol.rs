use l64_native::{Graph, LocusWord, ROOT_CONTEXT, Route, canonical_bytes, decode_canonical};

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x53594d424f4c4943)).composed(LocusWord(index))
}

#[test]
fn exact_state_identity_is_canonical_byte_identity() {
    let mut left = Graph::new();
    left.declare_atom_type(route(1), LocusWord(0x41)).unwrap();

    let mut right = Graph::new();
    right.declare_atom_type(route(1), LocusWord(0x42)).unwrap();

    let left_identity = left.exact_state_identity();
    let right_identity = right.exact_state_identity();
    assert!(left_identity.verify_exact("l64.native.state.v2", &canonical_bytes(&left)));
    assert_ne!(left_identity, right_identity);
    assert_ne!(left_identity.seal(), right_identity.seal());
}

#[test]
fn state_symbol_localizes_structural_change_classes() {
    let mut graph = Graph::new();
    let empty = graph.state_symbol();

    let atom = graph.declare_atom_type(route(1), LocusWord(0x41)).unwrap();
    let after_atom = graph.state_symbol();
    assert_eq!(
        empty.changed_sections(after_atom),
        [true, false, false, true]
    );

    graph.declare_function_type(route(2), atom, atom).unwrap();
    let after_function = graph.state_symbol();
    assert_eq!(
        after_atom.changed_sections(after_function),
        [true, true, false, true]
    );

    graph.extend_context(ROOT_CONTEXT, atom).unwrap();
    let after_context = graph.state_symbol();
    assert_eq!(
        after_function.changed_sections(after_context),
        [false, false, true, false]
    );
}

#[test]
fn symbolic_and_exact_identity_survive_canonical_fixed_point() {
    let mut graph = Graph::new();
    let atom = graph.declare_atom_type(route(1), LocusWord(0x41)).unwrap();
    graph.declare_function_type(route(2), atom, atom).unwrap();
    graph.extend_context(ROOT_CONTEXT, atom).unwrap();

    let bytes = canonical_bytes(&graph);
    let decoded = decode_canonical(&bytes).unwrap();
    assert_eq!(decoded.state_symbol(), graph.state_symbol());
    assert_eq!(decoded.exact_state_identity(), graph.exact_state_identity());
    assert!(
        decoded
            .exact_state_identity()
            .verify_exact("l64.native.state.v2", &bytes)
    );
}
