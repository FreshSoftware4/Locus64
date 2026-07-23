use std::collections::BTreeMap;

use crate::kernel::EqualityRule;
use crate::{
    ConstraintKind, ContextId, Dimension, DnaError, Graph, LocusWord, NodeId, Obstruction, OpCode,
    Proposal, ROOT_CONTEXT, Route, decode_dna, dna_bytes,
};

const RNA_MAGIC: &[u8] = b"L64R1";
pub const MAX_NATIVE_RNA_BYTES: usize = 1 << 24;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RnaError {
    SourceTooLarge,
    Empty,
    BadHeader,
    InvalidArity { line: u32 },
    InvalidNumber { line: u32 },
    UnknownInstruction { line: u32 },
    SlotOrder { line: u32, slot: u64 },
    ContextOrder { line: u32, context: u32 },
    UnknownSlot { line: u32, slot: u64 },
    UnrepresentableGraph,
    Graph(Obstruction),
    Dna(DnaError),
}

impl From<Obstruction> for RnaError {
    fn from(value: Obstruction) -> Self {
        Self::Graph(value)
    }
}

impl From<DnaError> for RnaError {
    fn from(value: DnaError) -> Self {
        Self::Dna(value)
    }
}

include!("rna/compile.rs");
include!("rna/sequence.rs");
include!("rna/parse.rs");
include!("rna/render.rs");
