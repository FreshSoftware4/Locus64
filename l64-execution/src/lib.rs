#![forbid(unsafe_code)]

use l64_certification::{
    AuthorityCertification, CertificationError, certify_graph_with_root_projection,
};
use l64_native::{
    DnaError, Graph, RnaError, StateSymbol, compile_rna, decode_dna, dna_bytes, rna_bytes,
};
use l64_projection::{ProjectionError, ProjectionSet};
use std::fmt::{self, Write as _};

pub const EXECUTION_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionSource {
    Rna,
    Dna,
    BundleMember,
}

impl ExecutionSource {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Rna => "rna",
            Self::Dna => "dna",
            Self::BundleMember => "bundle_member",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityExecution {
    pub source: ExecutionSource,
    pub canonical_roundtrip: bool,
    pub projection_verified: bool,
    pub symbol: StateSymbol,
    pub node_count: usize,
    pub port_count: usize,
    pub context_count: usize,
    pub journal_len: usize,
    pub projection: ProjectionSet,
    pub certification: AuthorityCertification,
}

impl AuthorityExecution {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "L64 NATIVE EXECUTION v{EXECUTION_VERSION}");
        self.render_into(&mut out, "");
        out
    }

    pub fn render_into(&self, out: &mut String, prefix: &str) {
        let _ = writeln!(out, "{prefix}source={}", self.source.as_str());
        let _ = writeln!(out, "{prefix}mode=structural_authority_evaluation");
        let _ = writeln!(out, "{prefix}authority=exact_canonical_l64d");
        let _ = writeln!(out, "{prefix}authority_mutation=none");
        let _ = writeln!(
            out,
            "{prefix}canonical_roundtrip={}",
            self.canonical_roundtrip
        );
        let _ = writeln!(out, "{prefix}projection_authority=non_authoritative");
        let _ = writeln!(
            out,
            "{prefix}projection_verified={}",
            self.projection_verified
        );
        let _ = writeln!(
            out,
            "{prefix}certification_verdict={}",
            self.certification.verdict
        );
        let _ = writeln!(out, "{prefix}symbol={}", self.symbol);
        let _ = writeln!(out, "{prefix}nodes={}", self.node_count);
        let _ = writeln!(out, "{prefix}ports={}", self.port_count);
        let _ = writeln!(out, "{prefix}contexts={}", self.context_count);
        let _ = writeln!(out, "{prefix}journal={}", self.journal_len);
        let _ = writeln!(out, "{prefix}root.closed={}", self.projection.report.closed);
        let _ = writeln!(out, "{prefix}root.open={}", self.projection.report.open);
        let _ = writeln!(
            out,
            "{prefix}root.invalid={}",
            self.projection.report.invalid
        );
        let _ = writeln!(
            out,
            "{prefix}root.research_candidates={}/{}",
            self.projection.research.candidates.len(),
            self.projection.research.total_candidates
        );
        for context in &self.certification.contexts {
            let _ = writeln!(
                out,
                "{prefix}context.{}.verdict={}",
                context.context, context.verdict
            );
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionError {
    Rna(RnaError),
    Dna(DnaError),
    NonCanonicalDna,
    RoundtripMismatch,
    Projection(ProjectionError),
    Certification(CertificationError),
}

impl ExecutionError {
    pub const fn rna_error(&self) -> Option<RnaError> {
        match self {
            Self::Rna(error) => Some(*error),
            _ => None,
        }
    }
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rna(error) => write!(f, "RNA execution failed: {error}"),
            Self::Dna(error) => write!(f, "DNA execution failed: {error}"),
            Self::NonCanonicalDna => f.write_str("execution requires canonical L64D authority"),
            Self::RoundtripMismatch => f.write_str("RNA/DNA exact roundtrip mismatch"),
            Self::Projection(error) => write!(f, "execution projection failed: {error}"),
            Self::Certification(error) => write!(f, "execution certification failed: {error}"),
        }
    }
}

impl std::error::Error for ExecutionError {}

pub fn execute_rna(source: &[u8]) -> Result<AuthorityExecution, ExecutionError> {
    let graph = compile_rna(source).map_err(ExecutionError::Rna)?;
    let canonical_rna = rna_bytes(&graph).map_err(ExecutionError::Rna)?;
    let canonical_dna = dna_bytes(&graph).map_err(ExecutionError::Dna)?;
    let rebuilt_graph = compile_rna(&canonical_rna).map_err(ExecutionError::Rna)?;
    let rebuilt_dna = dna_bytes(&rebuilt_graph).map_err(ExecutionError::Dna)?;
    if rebuilt_dna != canonical_dna {
        return Err(ExecutionError::RoundtripMismatch);
    }
    execute_graph(&graph, ExecutionSource::Rna, true)
}

pub fn execute_dna(bytes: &[u8]) -> Result<AuthorityExecution, ExecutionError> {
    let graph = decode_dna(bytes).map_err(ExecutionError::Dna)?;
    let canonical_dna = dna_bytes(&graph).map_err(ExecutionError::Dna)?;
    if canonical_dna != bytes {
        return Err(ExecutionError::NonCanonicalDna);
    }
    let canonical_rna = rna_bytes(&graph).map_err(ExecutionError::Rna)?;
    let rebuilt_graph = compile_rna(&canonical_rna).map_err(ExecutionError::Rna)?;
    let rebuilt_dna = dna_bytes(&rebuilt_graph).map_err(ExecutionError::Dna)?;
    if rebuilt_dna != bytes {
        return Err(ExecutionError::RoundtripMismatch);
    }
    execute_graph(&graph, ExecutionSource::Dna, true)
}

pub fn execute_canonical_graph(
    graph: &Graph,
    source: ExecutionSource,
) -> Result<AuthorityExecution, ExecutionError> {
    let dna = dna_bytes(graph).map_err(ExecutionError::Dna)?;
    let decoded = decode_dna(&dna).map_err(ExecutionError::Dna)?;
    let rebuilt = dna_bytes(&decoded).map_err(ExecutionError::Dna)?;
    if rebuilt != dna {
        return Err(ExecutionError::RoundtripMismatch);
    }
    execute_graph(graph, source, true)
}

fn execute_graph(
    graph: &Graph,
    source: ExecutionSource,
    canonical_roundtrip: bool,
) -> Result<AuthorityExecution, ExecutionError> {
    // Certification owns the shared fresh root derivation. Retained or externally supplied
    // projections still cross `ProjectionSet::verify` before reuse.
    let (certification, projection) =
        certify_graph_with_root_projection(graph, 16).map_err(ExecutionError::Certification)?;
    Ok(AuthorityExecution {
        source,
        canonical_roundtrip,
        projection_verified: true,
        symbol: graph.state_symbol(),
        node_count: graph.node_count(),
        port_count: graph.port_count(),
        context_count: graph.context_count(),
        journal_len: graph.journal_len(),
        projection,
        certification,
    })
}
