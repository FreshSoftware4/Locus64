#![forbid(unsafe_code)]

mod stream;

pub use stream::{
    BundleDecoder, BundleEncoder, BundleStreamError, BundleWriteSummary, OwnedBundleMember,
};

use l64_certification::AuthorityCertification;
use l64_execution::{AuthorityExecution, ExecutionError, ExecutionSource, execute_canonical_graph};
use l64_native::{DnaError, Graph, StateSymbol, decode_dna, dna_bytes};
use std::fmt::{self, Write as _};

const MAGIC: &[u8; 4] = b"L64B";
pub const BUNDLE_VERSION: u16 = 1;
const FLAGS: u16 = 0;
const HEADER_BYTES: usize = 16;
const MEMBER_HEADER_BYTES: usize = 4;
pub const MAX_BUNDLE_MEMBERS: usize = 4096;
pub const MAX_NATIVE_BUNDLE_BYTES: usize = 1 << 30;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BundleError {
    Empty,
    TooManyMembers,
    TooLarge,
    Truncated,
    BadMagic,
    UnsupportedVersion { version: u16 },
    UnsupportedFlags { flags: u16 },
    ZeroLengthMember { index: usize },
    TrailingBytes,
    NonCanonicalDna { index: usize },
    Dna { index: usize, error: DnaError },
    Execution { index: usize, error: ExecutionError },
}

impl fmt::Display for BundleError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Empty => f.write_str("L64B transport must contain at least one member"),
            Self::TooManyMembers => f.write_str("L64B transport contains too many members"),
            Self::TooLarge => f.write_str("L64B transport exceeds the native size limit"),
            Self::Truncated => f.write_str("L64B transport is truncated"),
            Self::BadMagic => f.write_str("input is not L64B transport"),
            Self::UnsupportedVersion { version } => write!(f, "unsupported L64B version {version}"),
            Self::UnsupportedFlags { flags } => write!(f, "unsupported L64B flags {flags}"),
            Self::ZeroLengthMember { index } => write!(f, "L64B member {index} has zero length"),
            Self::TrailingBytes => f.write_str("L64B transport has trailing bytes"),
            Self::NonCanonicalDna { index } => {
                write!(f, "L64B member {index} is not canonical L64D authority")
            }
            Self::Dna { index, error } => write!(f, "invalid L64B member {index}: {error}"),
            Self::Execution { index, error } => {
                write!(f, "L64B member {index} execution failed: {error}")
            }
        }
    }
}

impl std::error::Error for BundleError {}

#[derive(Debug)]
pub struct BundleMember<'a> {
    index: usize,
    dna: &'a [u8],
    graph: Graph,
}

impl<'a> BundleMember<'a> {
    pub fn index(&self) -> usize {
        self.index
    }
    pub fn dna(&self) -> &'a [u8] {
        self.dna
    }
    pub fn graph(&self) -> &Graph {
        &self.graph
    }
    pub fn symbol(&self) -> StateSymbol {
        self.graph.state_symbol()
    }
}

#[derive(Debug)]
pub struct NativeBundle<'a> {
    bytes: &'a [u8],
    members: Vec<BundleMember<'a>>,
}

impl<'a> NativeBundle<'a> {
    pub fn bytes(&self) -> &'a [u8] {
        self.bytes
    }
    pub fn members(&self) -> &[BundleMember<'a>] {
        &self.members
    }
    pub fn len(&self) -> usize {
        self.members.len()
    }
    pub fn is_empty(&self) -> bool {
        self.members.is_empty()
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MemberExecution {
    pub index: usize,
    pub execution: AuthorityExecution,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleExecution {
    pub members: Vec<MemberExecution>,
}

impl BundleExecution {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "L64 NATIVE BUNDLE EXECUTION v{BUNDLE_VERSION}");
        let _ = writeln!(out, "members={}", self.members.len());
        let _ = writeln!(out, "composite_authority=none");
        let _ = writeln!(out, "composite_execution=none");
        let _ = writeln!(out, "ordering=transport_sequence");
        for member in &self.members {
            member
                .execution
                .render_into(&mut out, &format!("member.{}.", member.index));
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleCertification {
    pub members: Vec<AuthorityCertification>,
}

impl BundleCertification {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "L64 NATIVE BUNDLE CERTIFICATION v1");
        let _ = writeln!(out, "transport=canonical_l64b");
        let _ = writeln!(out, "transport_verified=true");
        let _ = writeln!(out, "members={}", self.members.len());
        let _ = writeln!(out, "composite_authority=none");
        let _ = writeln!(out, "composite_certificate=none");
        for (index, member) in self.members.iter().enumerate() {
            member.render_into(&mut out, &format!("member.{index}."));
        }
        out
    }
}

pub fn bundle_bytes(members: &[&[u8]]) -> Result<Vec<u8>, BundleError> {
    validate_member_count(members.len())?;
    let mut payload_len = 0usize;
    for (index, member) in members.iter().enumerate() {
        validate_dna(index, member)?;
        if member.len() > u32::MAX as usize {
            return Err(BundleError::TooLarge);
        }
        payload_len = payload_len
            .checked_add(MEMBER_HEADER_BYTES)
            .and_then(|value| value.checked_add(member.len()))
            .ok_or(BundleError::TooLarge)?;
    }
    let total_len = HEADER_BYTES
        .checked_add(payload_len)
        .ok_or(BundleError::TooLarge)?;
    if total_len > MAX_NATIVE_BUNDLE_BYTES || payload_len > u32::MAX as usize {
        return Err(BundleError::TooLarge);
    }
    let mut out = Vec::with_capacity(total_len);
    out.extend_from_slice(MAGIC);
    out.extend_from_slice(&BUNDLE_VERSION.to_le_bytes());
    out.extend_from_slice(&FLAGS.to_le_bytes());
    out.extend_from_slice(&(members.len() as u32).to_le_bytes());
    out.extend_from_slice(&(payload_len as u32).to_le_bytes());
    for member in members {
        out.extend_from_slice(&(member.len() as u32).to_le_bytes());
        out.extend_from_slice(member);
    }
    Ok(out)
}

fn parse_header(bytes: &[u8]) -> Result<(usize, usize), BundleError> {
    if bytes.len() < HEADER_BYTES {
        return Err(BundleError::Truncated);
    }
    if &bytes[..4] != MAGIC {
        return Err(BundleError::BadMagic);
    }
    let version = u16::from_le_bytes(bytes[4..6].try_into().expect("fixed version field"));
    if version != BUNDLE_VERSION {
        return Err(BundleError::UnsupportedVersion { version });
    }
    let flags = u16::from_le_bytes(bytes[6..8].try_into().expect("fixed flags field"));
    if flags != FLAGS {
        return Err(BundleError::UnsupportedFlags { flags });
    }
    let count = u32::from_le_bytes(bytes[8..12].try_into().expect("fixed count field")) as usize;
    validate_member_count(count)?;
    let payload_len =
        u32::from_le_bytes(bytes[12..16].try_into().expect("fixed length field")) as usize;
    let expected_len = HEADER_BYTES
        .checked_add(payload_len)
        .ok_or(BundleError::TooLarge)?;
    if expected_len > MAX_NATIVE_BUNDLE_BYTES {
        return Err(BundleError::TooLarge);
    }
    Ok((count, payload_len))
}

pub fn decode_bundle(bytes: &[u8]) -> Result<NativeBundle<'_>, BundleError> {
    let (count, payload_len) = parse_header(bytes)?;
    let expected_len = HEADER_BYTES + payload_len;
    if bytes.len() < expected_len {
        return Err(BundleError::Truncated);
    }
    if bytes.len() > expected_len {
        return Err(BundleError::TrailingBytes);
    }
    let mut cursor = HEADER_BYTES;
    let mut members = Vec::with_capacity(count);
    for index in 0..count {
        let length_end = cursor
            .checked_add(MEMBER_HEADER_BYTES)
            .ok_or(BundleError::TooLarge)?;
        if length_end > bytes.len() {
            return Err(BundleError::Truncated);
        }
        let member_len = u32::from_le_bytes(
            bytes[cursor..length_end]
                .try_into()
                .expect("fixed member length field"),
        ) as usize;
        if member_len == 0 {
            return Err(BundleError::ZeroLengthMember { index });
        }
        cursor = length_end;
        let member_end = cursor
            .checked_add(member_len)
            .ok_or(BundleError::TooLarge)?;
        if member_end > bytes.len() {
            return Err(BundleError::Truncated);
        }
        let dna = &bytes[cursor..member_end];
        let graph = validate_dna(index, dna)?;
        members.push(BundleMember { index, dna, graph });
        cursor = member_end;
    }
    if cursor != bytes.len() {
        return Err(BundleError::TrailingBytes);
    }
    Ok(NativeBundle { bytes, members })
}

pub fn execute_bundle(bytes: &[u8]) -> Result<BundleExecution, BundleError> {
    let bundle = decode_bundle(bytes)?;
    let mut members = Vec::with_capacity(bundle.len());
    for member in bundle.members() {
        let execution = execute_canonical_graph(member.graph(), ExecutionSource::BundleMember)
            .map_err(|error| BundleError::Execution {
                index: member.index(),
                error,
            })?;
        members.push(MemberExecution {
            index: member.index(),
            execution,
        });
    }
    Ok(BundleExecution { members })
}

pub fn certify_bundle(bytes: &[u8]) -> Result<BundleCertification, BundleError> {
    let execution = execute_bundle(bytes)?;
    Ok(BundleCertification {
        members: execution
            .members
            .into_iter()
            .map(|member| member.execution.certification)
            .collect(),
    })
}

fn validate_member_count(count: usize) -> Result<(), BundleError> {
    if count == 0 {
        Err(BundleError::Empty)
    } else if count > MAX_BUNDLE_MEMBERS || count > u32::MAX as usize {
        Err(BundleError::TooManyMembers)
    } else {
        Ok(())
    }
}

fn validate_dna(index: usize, dna: &[u8]) -> Result<Graph, BundleError> {
    let graph = decode_dna(dna).map_err(|error| BundleError::Dna { index, error })?;
    let canonical = dna_bytes(&graph).map_err(|error| BundleError::Dna { index, error })?;
    if canonical != dna {
        return Err(BundleError::NonCanonicalDna { index });
    }
    Ok(graph)
}
