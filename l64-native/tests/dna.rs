use l64_native::{
    DnaError, Graph, LocusWord, Proposal, ROOT_CONTEXT, Route, decode_dna, dna_bytes,
};

fn route(index: u64) -> Route {
    Route::root(LocusWord(0x4c36344e41544956)).composed(LocusWord(index))
}

fn proven_graph() -> Graph {
    let mut graph = Graph::new();
    let a = graph.declare_atom_type(route(1), LocusWord(0x41)).unwrap();
    let b = graph.declare_atom_type(route(2), LocusWord(0x42)).unwrap();
    let c = graph.declare_atom_type(route(3), LocusWord(0x43)).unwrap();
    let ab = graph.declare_function_type(route(4), a, b).unwrap();
    let bc = graph.declare_function_type(route(5), b, c).unwrap();
    let ac = graph.declare_function_type(route(6), a, c).unwrap();
    let first = graph.insert_value(route(7), ROOT_CONTEXT, ab).unwrap();
    let second = graph.insert_value(route(8), ROOT_CONTEXT, bc).unwrap();
    graph
        .transact(Proposal::compose(route(9), ROOT_CONTEXT, first, second, ac))
        .unwrap();
    graph
}

#[test]
fn native_dna_roundtrip_is_exact_fixed_point() {
    let graph = proven_graph();
    let dna = dna_bytes(&graph).unwrap();
    let decoded = decode_dna(&dna).unwrap();

    assert_eq!(dna_bytes(&decoded).unwrap(), dna);
    assert_eq!(decoded.state_commitment(), graph.state_commitment());
    assert_eq!(decoded.journal_len(), 0);
    assert!(!dna.windows(b"claim".len()).any(|window| window == b"claim"));
    assert!(
        !dna.windows(b"theorem".len())
            .any(|window| window == b"theorem")
    );
}

#[test]
fn native_dna_rejects_payload_tampering() {
    let mut dna = dna_bytes(&proven_graph()).unwrap();
    let last = dna.len() - 1;
    dna[last] ^= 1;
    assert_eq!(decode_dna(&dna).unwrap_err(), DnaError::CommitmentMismatch);
}

#[test]
fn native_dna_rejects_unknown_header_state() {
    let original = dna_bytes(&proven_graph()).unwrap();

    let mut version = original.clone();
    version[4..6].copy_from_slice(&2u16.to_le_bytes());
    assert_eq!(
        decode_dna(&version).unwrap_err(),
        DnaError::UnsupportedVersion { version: 2 }
    );

    let mut flags = original;
    flags[6..8].copy_from_slice(&1u16.to_le_bytes());
    assert_eq!(
        decode_dna(&flags).unwrap_err(),
        DnaError::UnsupportedFlags { flags: 1 }
    );
}

#[test]
fn native_dna_rejects_length_mismatch() {
    let dna = dna_bytes(&proven_graph()).unwrap();
    assert_eq!(
        decode_dna(&dna[..dna.len() - 1]).unwrap_err(),
        DnaError::Truncated
    );

    let mut trailing = dna;
    trailing.push(0);
    assert_eq!(decode_dna(&trailing).unwrap_err(), DnaError::TrailingBytes);
}
