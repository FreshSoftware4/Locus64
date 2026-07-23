use l64_native::{Obstruction, RnaError, compile_rna, dna_to_rna, normalize_rna, rna_to_dna};

const CANONICAL_COMPOSE: &[u8] = b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n";

#[test]
fn native_rna_compiles_and_reaches_exact_dna_fixed_point() {
    let graph = compile_rna(CANONICAL_COMPOSE).unwrap();
    assert_eq!(graph.node_count(), 11);

    let dna = rna_to_dna(CANONICAL_COMPOSE).unwrap();
    let sequenced = dna_to_rna(&dna).unwrap();
    assert_eq!(sequenced, CANONICAL_COMPOSE);
    assert_eq!(rna_to_dna(&sequenced).unwrap(), dna);
}

#[test]
fn native_rna_normalization_removes_surface_variation() {
    let source = b"\n L64R1   0X4C36344E41544956\n a  1  0X41\n a 2 0x42\n a 3 0x43\n f 4 1 2\n f 5 2 3\n f 6 1 3\n v 7 4\n v 8 5\n c 9 7 8 6\n\n";
    assert_eq!(normalize_rna(source).unwrap(), CANONICAL_COMPOSE);
}

#[test]
fn invalid_matrix_source_refuses_before_dna_exists() {
    let source = b"L64R1 0x4c36344e41544956\na 1 0x52\nm 2 1 2 3\nm 3 1 4 2\nm 4 1 2 2\nv 5 2\nv 6 3\nx 7 5 6 4\n";
    assert_eq!(
        rna_to_dna(source),
        Err(RnaError::Graph(Obstruction::MatrixShapeMismatch {
            left_cols: 3,
            right_rows: 4,
        }))
    );
}

#[test]
fn native_rna_rejects_unknown_and_non_topological_slots() {
    let unknown = b"L64R1 0x1\na 1 0x41\nf 2 1 9\n";
    assert_eq!(
        compile_rna(unknown).unwrap_err(),
        RnaError::UnknownSlot { line: 3, slot: 9 }
    );

    let unordered = b"L64R1 0x1\na 2 0x41\na 1 0x42\n";
    assert_eq!(
        compile_rna(unordered).unwrap_err(),
        RnaError::SlotOrder { line: 3, slot: 1 }
    );
}

#[test]
fn native_rna_rejects_record_like_or_named_instructions() {
    let source = b"L64R1 0x1\nclaim 1 2\n";
    assert_eq!(
        compile_rna(source).unwrap_err(),
        RnaError::UnknownInstruction { line: 2 }
    );
}
