use l64_native::{
    ConstraintKind, Dimension, Obstruction, RnaError, RnaSpan, compile_rna, decode_dna, dna_bytes,
    dna_to_rna, normalize_rna, rna_diagnostic, rna_to_dna,
};

const CANONICAL_COMPOSE: &[u8] = b"L64R1 0x4c36344e41544956\na 1 0x41\na 2 0x42\na 3 0x43\nf 4 1 2\nf 5 2 3\nf 6 1 3\nv 7 4\nv 8 5\nc 9 7 8 6\n";

#[test]
fn native_rna_compiles_and_reaches_exact_dna_fixed_point() {
    let graph = compile_rna(CANONICAL_COMPOSE).unwrap();
    assert_eq!(graph.node_count(), 11);
    assert_eq!(
        graph.journal_len(),
        0,
        "canonical source compilation is bulk construction"
    );

    let exact_dna = dna_bytes(&graph).unwrap();
    let decoded = decode_dna(&exact_dna).unwrap();
    assert_eq!(decoded.state_symbol(), graph.state_symbol());

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
        Err(RnaError::Graph {
            span: RnaSpan::new(8, 1, 1),
            error: Obstruction::MatrixShapeMismatch {
                left_cols: 3,
                right_rows: 4,
            },
        })
    );
}

#[test]
fn native_rna_rejects_unknown_and_non_topological_slots() {
    let unknown = b"L64R1 0x1\na 1 0x41\nf 2 1 9\n";
    assert_eq!(
        compile_rna(unknown).unwrap_err(),
        RnaError::UnknownSlot {
            span: RnaSpan::new(3, 7, 1),
            slot: 9,
        }
    );

    let unordered = b"L64R1 0x1\na 2 0x41\na 1 0x42\n";
    assert_eq!(
        compile_rna(unordered).unwrap_err(),
        RnaError::SlotOrder {
            span: RnaSpan::new(3, 3, 1),
            slot: 1,
        }
    );
}

#[test]
fn native_rna_rejects_record_like_or_named_instructions() {
    let source = b"L64R1 0x1\nclaim 1 2\n";
    assert_eq!(
        compile_rna(source).unwrap_err(),
        RnaError::UnknownInstruction {
            span: RnaSpan::new(2, 1, 5),
        }
    );
}

const CONSTRAINT_RNA: &[u8] = b"L64R1 0x434f4e5354524149\na 1 0x52\nq 2 1 1 0 0 0 0 0 0\nq 3 1 2 0 0 0 0 0 0\nv 4 3\nr 5 4 2\nk 6 4 1 1\nh 1 0 6\nr 7 4 2 1\n";

#[test]
fn constraints_contexts_and_obligations_reach_exact_rna_dna_fixed_point() {
    let graph = compile_rna(CONSTRAINT_RNA).unwrap();
    assert_eq!(graph.context_count(), 2);
    let dna = rna_to_dna(CONSTRAINT_RNA).unwrap();
    let sequenced = dna_to_rna(&dna).unwrap();
    assert_eq!(sequenced, CONSTRAINT_RNA);
    assert_eq!(rna_to_dna(&sequenced).unwrap(), dna);
    assert!(!sequenced.windows(2).any(|window| window == b"{:"));
}

#[test]
fn rna_preserves_unknown_guard_and_refuses_refuted_guard() {
    let refuted = b"L64R1 0x434f4e5354524149\na 1 0x52\nq 2 1 1 0 0 0 0 0 0\nq 3 1 2 0 0 0 0 0 0\nv 4 3\nk 5 4 1 0\nh 1 0 5\nr 6 4 2 1\n";
    assert_eq!(
        rna_to_dna(refuted),
        Err(RnaError::Graph {
            span: RnaSpan::new(8, 1, 1),
            error: Obstruction::GuardViolated {
                subject: 3,
                kind: ConstraintKind::NonNegative,
            },
        })
    );
}

#[test]
fn rna_rejects_dimensionally_invalid_addition() {
    let source = b"L64R1 0x434f4e5354524149\na 1 0x52\nq 2 1 1 0 0 0 0 0 0\nq 3 1 0 0 1 0 0 0 0\nv 4 2\nv 5 3\n+ 6 4 5 2\n";
    assert_eq!(
        rna_to_dna(source),
        Err(RnaError::Graph {
            span: RnaSpan::new(7, 1, 1),
            error: Obstruction::DimensionMismatch {
                left: Dimension::LENGTH,
                right: Dimension::TIME,
            },
        })
    );
}

#[test]
fn rna_rejects_nonsequential_context_ids() {
    let source = b"L64R1 0x1\na 1 0x52\nq 2 1 0 0 0 0 0 0 0\nv 3 2\nk 4 3 1 1\nh 2 0 4\n";
    assert_eq!(
        compile_rna(source).unwrap_err(),
        RnaError::ContextOrder {
            span: RnaSpan::new(6, 3, 1),
            context: 2,
        }
    );
}

#[test]
fn rna_diagnostic_renders_exact_token_span() {
    let source = b"L64R1 0x1\na nope 0x41\n";
    let error = compile_rna(source).unwrap_err();
    assert_eq!(
        error,
        RnaError::InvalidNumber {
            span: RnaSpan::new(2, 3, 4),
        }
    );
    let diagnostic = format!("{}", rna_diagnostic(source, error));
    assert!(diagnostic.contains("invalid number at RNA 2:3"));
    assert!(diagnostic.contains("2 | a nope 0x41"));
    assert!(diagnostic.contains("|   ^^^^"));
}
