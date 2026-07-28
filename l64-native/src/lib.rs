#![forbid(unsafe_code)]

mod closure;
mod codec;
mod context;
mod dimension;
mod dna;
mod graph;
mod journal;
mod kernel;
mod rna;
mod route;
mod symbol;

pub use closure::{ClosureState, ClosureTransition};
pub use codec::{DecodeError, canonical_bytes, decode_canonical};
pub use context::ContextDelta;
pub use dimension::Dimension;
pub use dna::{DnaError, MAX_NATIVE_DNA_PAYLOAD_BYTES, decode_dna, dna_bytes};
pub use graph::{ContextId, EventId, Graph, Node, NodeId, ROOT_CONTEXT};
pub use journal::JournalEvent;
pub use kernel::{CommitResult, ConstraintKind, Obstruction, OpCode, Port, PortRole, Proposal};
pub use l64_symbolic::{SymbolicIdentity, SymbolicSeal};
pub use rna::{
    MAX_NATIVE_RNA_BYTES, RnaError, RnaSpan, compile_rna, dna_to_rna, normalize_rna, rna_bytes,
    rna_diagnostic, rna_to_dna,
};
pub use route::{LocusWord, Route};
pub use symbol::{STATE_SYMBOL_DOMAIN, StateSymbol, state_identity};
