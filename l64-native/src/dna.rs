use crate::{DecodeError, Graph, SymbolicSeal, canonical_bytes, decode_canonical};
use l64_symbolic::Composer;
use std::fmt;

const DNA_MAGIC: &[u8; 4] = b"L64D";
const DNA_VERSION: u16 = 2;
const DNA_FLAGS: u16 = 0;
const DNA_SYMBOL_DOMAIN: &str = "l64.native.dna.v2";
const DNA_HEADER_BYTES: usize = 44;
pub const MAX_NATIVE_DNA_PAYLOAD_BYTES: usize = 1 << 28;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DnaError {
    PayloadTooLarge,
    Truncated,
    BadMagic,
    UnsupportedVersion { version: u16 },
    UnsupportedFlags { flags: u16 },
    TrailingBytes,
    SealMismatch,
    Canonical(DecodeError),
}

impl fmt::Display for DnaError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::PayloadTooLarge => f.write_str("L64D payload exceeds the native size limit"),
            Self::Truncated => f.write_str("L64D authority is truncated"),
            Self::BadMagic => f.write_str("input is not L64D authority"),
            Self::UnsupportedVersion { version } => write!(f, "unsupported L64D version {version}"),
            Self::UnsupportedFlags { flags } => write!(f, "unsupported L64D flags {flags}"),
            Self::TrailingBytes => f.write_str("L64D authority has trailing bytes"),
            Self::SealMismatch => {
                f.write_str("L64D symbolic seal does not match its canonical payload")
            }
            Self::Canonical(error) => write!(f, "invalid canonical L64D payload: {error}"),
        }
    }
}

impl std::error::Error for DnaError {}

impl From<DecodeError> for DnaError {
    fn from(value: DecodeError) -> Self {
        Self::Canonical(value)
    }
}

pub fn dna_bytes(graph: &Graph) -> Result<Vec<u8>, DnaError> {
    let payload = canonical_bytes(graph);
    if payload.len() > MAX_NATIVE_DNA_PAYLOAD_BYTES || payload.len() > u32::MAX as usize {
        return Err(DnaError::PayloadTooLarge);
    }

    let mut out = Vec::with_capacity(DNA_HEADER_BYTES + payload.len());
    out.extend_from_slice(DNA_MAGIC);
    out.extend_from_slice(&DNA_VERSION.to_le_bytes());
    out.extend_from_slice(&DNA_FLAGS.to_le_bytes());
    out.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    out.extend_from_slice(&dna_seal(graph, payload.len()).to_bytes());
    out.extend_from_slice(&payload);
    Ok(out)
}

pub fn decode_dna(bytes: &[u8]) -> Result<Graph, DnaError> {
    if bytes.len() < DNA_HEADER_BYTES {
        return Err(DnaError::Truncated);
    }
    if &bytes[..4] != DNA_MAGIC {
        return Err(DnaError::BadMagic);
    }

    let version = u16::from_le_bytes(bytes[4..6].try_into().expect("fixed DNA version field"));
    if version != DNA_VERSION {
        return Err(DnaError::UnsupportedVersion { version });
    }
    let flags = u16::from_le_bytes(bytes[6..8].try_into().expect("fixed DNA flags field"));
    if flags != DNA_FLAGS {
        return Err(DnaError::UnsupportedFlags { flags });
    }
    let payload_len =
        u32::from_le_bytes(bytes[8..12].try_into().expect("fixed DNA length field")) as usize;
    if payload_len > MAX_NATIVE_DNA_PAYLOAD_BYTES {
        return Err(DnaError::PayloadTooLarge);
    }

    let expected_len = DNA_HEADER_BYTES
        .checked_add(payload_len)
        .ok_or(DnaError::PayloadTooLarge)?;
    if bytes.len() < expected_len {
        return Err(DnaError::Truncated);
    }
    if bytes.len() > expected_len {
        return Err(DnaError::TrailingBytes);
    }

    let stored_seal = SymbolicSeal::from_bytes(
        bytes[12..44]
            .try_into()
            .expect("fixed DNA symbolic seal field"),
    );
    let payload = &bytes[DNA_HEADER_BYTES..];
    let graph = decode_canonical(payload)?;
    if stored_seal != dna_seal(&graph, payload.len()) {
        return Err(DnaError::SealMismatch);
    }
    Ok(graph)
}

fn dna_seal(graph: &Graph, payload_len: usize) -> SymbolicSeal {
    let mut composer = Composer::new(DNA_SYMBOL_DOMAIN);
    composer
        .u16("version", DNA_VERSION)
        .u64("payload-length", payload_len as u64)
        .ordered("state", &[graph.state_symbol().root]);
    composer.finish()
}
