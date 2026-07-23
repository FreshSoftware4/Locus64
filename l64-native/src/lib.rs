#![forbid(unsafe_code)]

mod codec;
mod context;
mod dna;
mod graph;
mod journal;
mod kernel;
mod route;

pub use codec::{DecodeError, canonical_bytes, decode_canonical};
pub use context::ContextDelta;
pub use dna::{DnaError, MAX_NATIVE_DNA_PAYLOAD_BYTES, decode_dna, dna_bytes};
pub use graph::{ContextId, EventId, Graph, Node, NodeId, ROOT_CONTEXT};
pub use journal::JournalEvent;
pub use kernel::{CommitResult, Obstruction, OpCode, Port, PortRole, Proposal};
pub use route::{LocusWord, Route};
