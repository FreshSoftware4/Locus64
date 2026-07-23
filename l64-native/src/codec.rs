use std::collections::BTreeMap;

use crate::kernel::{EVIDENCE_LOCUS, JUDGMENT_LOCUS};
use crate::{ContextDelta, Graph, LocusWord, Node, NodeId, OpCode, Port, PortRole, Route};

const CODEC_VERSION: u16 = 3;
const COMMITMENT_DOMAIN: &[u8] = b"l64-native-state-v3\0";
const MAX_NODES: usize = 1 << 20;
const MAX_PORTS: usize = 1 << 22;
const MAX_CONTEXTS: usize = 1 << 20;
const MAX_ROUTES: usize = 1 << 20;
const MAX_ROUTE_WORDS: usize = 64;
const NO_NODE: NodeId = u32::MAX;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DecodeError {
    Truncated,
    BadMagic,
    UnsupportedVersion { version: u16 },
    StructuralBound,
    UnknownOpcode { opcode: u16 },
    UnknownPortRole { role: u8 },
    NonZeroPortFlags { flags: u8 },
    InvalidNode,
    InvalidPortRange,
    InvalidPortTarget,
    InvalidPortLaw,
    InvalidContext,
    InvalidRoute,
    InvalidAuthority,
    DuplicateRoute,
    TrailingBytes,
    NonCanonical,
}

include!("codec/io.rs");
include!("codec/validation.rs");
include!("codec/reader.rs");
