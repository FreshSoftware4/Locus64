use std::{collections::BTreeMap, fmt};

use crate::kernel::{EVIDENCE_LOCUS, EqualityRule, JUDGMENT_LOCUS};
use crate::{ContextDelta, Graph, LocusWord, Node, NodeId, OpCode, Port, PortRole, Route};

pub(crate) const CODEC_VERSION: u16 = 4;
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

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Truncated => f.write_str("canonical payload is truncated"),
            Self::BadMagic => f.write_str("canonical payload magic is invalid"),
            Self::UnsupportedVersion { version } => {
                write!(f, "unsupported canonical codec version {version}")
            }
            Self::StructuralBound => f.write_str("canonical payload exceeds a structural bound"),
            Self::UnknownOpcode { opcode } => write!(f, "unknown opcode {opcode}"),
            Self::UnknownPortRole { role } => write!(f, "unknown port role {role}"),
            Self::NonZeroPortFlags { flags } => {
                write!(f, "unsupported non-zero port flags {flags}")
            }
            Self::InvalidNode => f.write_str("canonical payload contains an invalid node"),
            Self::InvalidPortRange => {
                f.write_str("canonical payload contains an invalid port range")
            }
            Self::InvalidPortTarget => {
                f.write_str("canonical payload contains an invalid port target")
            }
            Self::InvalidPortLaw => f.write_str("canonical payload contains an invalid port law"),
            Self::InvalidContext => f.write_str("canonical payload contains an invalid context"),
            Self::InvalidRoute => f.write_str("canonical payload contains an invalid route"),
            Self::InvalidAuthority => {
                f.write_str("canonical payload contains invalid authority metadata")
            }
            Self::DuplicateRoute => f.write_str("canonical payload contains a duplicate route"),
            Self::TrailingBytes => f.write_str("canonical payload has trailing bytes"),
            Self::NonCanonical => f.write_str("canonical payload is not in canonical form"),
        }
    }
}

impl std::error::Error for DecodeError {}

include!("codec/io.rs");
include!("codec/validation.rs");
include!("codec/reader.rs");
