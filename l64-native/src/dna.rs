use crate::{DecodeError, Graph, canonical_bytes, decode_canonical};

const DNA_MAGIC: &[u8; 4] = b"L64D";
const DNA_VERSION: u16 = 1;
const DNA_FLAGS: u16 = 0;
const DNA_COMMITMENT_DOMAIN: &[u8] = b"l64-native-dna-v1\0";
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
    CommitmentMismatch,
    Canonical(DecodeError),
}

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
    out.extend_from_slice(&dna_commitment(&payload));
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

    let stored_commitment: [u8; 32] = bytes[12..44]
        .try_into()
        .expect("fixed DNA commitment field");
    let payload = &bytes[DNA_HEADER_BYTES..];
    let computed_commitment = dna_commitment(payload);
    if stored_commitment != computed_commitment {
        return Err(DnaError::CommitmentMismatch);
    }

    Ok(decode_canonical(payload)?)
}

fn dna_commitment(payload: &[u8]) -> [u8; 32] {
    let mut hasher = blake3::Hasher::new();
    hasher.update(DNA_COMMITMENT_DOMAIN);
    hasher.update(payload);
    *hasher.finalize().as_bytes()
}
