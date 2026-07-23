use l64_native::{
    DecodeError, Graph, LocusWord, Proposal, ROOT_CONTEXT, Route, canonical_bytes, decode_canonical,
};

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x4c36344e41544956)).composed(LocusWord(index))
}

fn representative_graph() -> Graph {
    let mut graph = Graph::new();
    let scalar = graph.declare_atom_type(route(1), LocusWord(0x52)).unwrap();
    let a = graph.declare_atom_type(route(2), LocusWord(0x41)).unwrap();
    let b = graph.declare_atom_type(route(3), LocusWord(0x42)).unwrap();
    let c = graph.declare_atom_type(route(4), LocusWord(0x43)).unwrap();
    let ab = graph.declare_function_type(route(5), a, b).unwrap();
    let bc = graph.declare_function_type(route(6), b, c).unwrap();
    let ac = graph.declare_function_type(route(7), a, c).unwrap();
    let first = graph.insert_value(route(8), ROOT_CONTEXT, ab).unwrap();
    let second = graph.insert_value(route(9), ROOT_CONTEXT, bc).unwrap();
    graph
        .transact(Proposal::compose(
            route(10),
            ROOT_CONTEXT,
            first,
            second,
            ac,
        ))
        .unwrap();
    let matrix = graph.declare_matrix_type(route(11), scalar, 2, 3).unwrap();
    let value = graph.insert_value(route(12), ROOT_CONTEXT, matrix).unwrap();
    graph.extend_context(ROOT_CONTEXT, value).unwrap();
    graph
}

#[test]
fn canonical_decode_reencode_is_exact_fixed_point() {
    let graph = representative_graph();
    let bytes = canonical_bytes(&graph);
    let decoded = decode_canonical(&bytes).unwrap();

    assert_eq!(canonical_bytes(&decoded), bytes);
    assert_eq!(decoded.state_commitment(), graph.state_commitment());
    assert_eq!(decoded.node_count(), graph.node_count());
    assert_eq!(decoded.port_count(), graph.port_count());
    assert_eq!(decoded.context_count(), graph.context_count());
    assert_eq!(decoded.journal_len(), 0);
    for index in 1..=12 {
        assert_eq!(decoded.resolve(&route(index)), graph.resolve(&route(index)));
    }
}

#[test]
fn decoder_rejects_truncation_and_trailing_bytes() {
    let bytes = canonical_bytes(&representative_graph());
    assert_eq!(
        decode_canonical(&bytes[..bytes.len() - 1]).unwrap_err(),
        DecodeError::Truncated
    );

    let mut trailing = bytes;
    trailing.push(0);
    assert_eq!(
        decode_canonical(&trailing).unwrap_err(),
        DecodeError::TrailingBytes
    );
}

#[test]
fn decoder_rejects_unknown_node_opcode() {
    let mut bytes = canonical_bytes(&representative_graph());
    let first_node_opcode = 22;
    bytes[first_node_opcode..first_node_opcode + 2].copy_from_slice(&u16::MAX.to_le_bytes());
    assert_eq!(
        decode_canonical(&bytes).unwrap_err(),
        DecodeError::UnknownOpcode { opcode: u16::MAX }
    );
}

#[test]
fn decoder_rejects_noncanonical_route_order() {
    let graph = representative_graph();
    let mut bytes = canonical_bytes(&graph);
    let routes_offset =
        22 + graph.node_count() * 24 + graph.port_count() * 8 + graph.context_count() * 8;
    let first_route_len = 8 + 4 + 8 + 4;
    let first = bytes[routes_offset..routes_offset + first_route_len].to_vec();
    let second =
        bytes[routes_offset + first_route_len..routes_offset + 2 * first_route_len].to_vec();
    bytes[routes_offset..routes_offset + first_route_len].copy_from_slice(&second);
    bytes[routes_offset + first_route_len..routes_offset + 2 * first_route_len]
        .copy_from_slice(&first);

    assert_eq!(
        decode_canonical(&bytes).unwrap_err(),
        DecodeError::NonCanonical
    );
}
