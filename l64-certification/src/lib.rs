#![forbid(unsafe_code)]

use l64_native::{ContextId, DnaError, Graph, ROOT_CONTEXT, StateSymbol, decode_dna, dna_bytes};
use l64_projection::{BurdenState, CertificationView, ProjectionError, ProjectionSet};
use std::fmt::{self, Write as _};

pub const CERTIFICATION_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CertificationVerdict {
    Certified,
    Open,
    Incomplete,
    Invalid,
}

impl fmt::Display for CertificationVerdict {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(match self {
            Self::Certified => "CERTIFIED",
            Self::Open => "OPEN",
            Self::Incomplete => "INCOMPLETE",
            Self::Invalid => "INVALID",
        })
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct BurdenCounts {
    pub total: usize,
    pub discharged: usize,
    pub open: usize,
    pub invalid: usize,
    pub missing_evidence: usize,
}

impl BurdenCounts {
    fn from_view(view: &CertificationView) -> Self {
        let mut counts = Self::default();
        for burden in &view.burdens {
            counts.total += 1;
            match burden.state {
                BurdenState::Discharged => counts.discharged += 1,
                BurdenState::Open => counts.open += 1,
                BurdenState::Invalid => counts.invalid += 1,
                BurdenState::MissingEvidence => counts.missing_evidence += 1,
            }
        }
        counts
    }

    pub fn verdict(self) -> CertificationVerdict {
        if self.invalid != 0 {
            CertificationVerdict::Invalid
        } else if self.missing_evidence != 0 {
            CertificationVerdict::Incomplete
        } else if self.open != 0 {
            CertificationVerdict::Open
        } else {
            CertificationVerdict::Certified
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextCertification {
    pub context: ContextId,
    pub projection_verified: bool,
    pub burdens: BurdenCounts,
    pub verdict: CertificationVerdict,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityCertification {
    pub symbol: StateSymbol,
    pub node_count: usize,
    pub context_count: usize,
    pub journal_len: usize,
    pub canonical_dna: bool,
    pub contexts: Vec<ContextCertification>,
    pub verdict: CertificationVerdict,
}

impl AuthorityCertification {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "L64 NATIVE CERTIFICATION v{CERTIFICATION_VERSION}");
        self.render_into(&mut out, "");
        out
    }

    pub fn render_into(&self, out: &mut String, prefix: &str) {
        let _ = writeln!(out, "{prefix}scope=native_structural_closure");
        let _ = writeln!(out, "{prefix}authority=exact_canonical_l64d");
        let _ = writeln!(out, "{prefix}canonical_dna={}", self.canonical_dna);
        let _ = writeln!(out, "{prefix}symbol={}", self.symbol);
        let _ = writeln!(out, "{prefix}symbol_authority=non_authoritative");
        let _ = writeln!(out, "{prefix}nodes={}", self.node_count);
        let _ = writeln!(out, "{prefix}contexts={}", self.context_count);
        let _ = writeln!(out, "{prefix}journal={}", self.journal_len);
        let _ = writeln!(out, "{prefix}verdict={}", self.verdict);
        for context in &self.contexts {
            let context_prefix = format!("{prefix}context.{}.", context.context);
            let _ = writeln!(
                out,
                "{context_prefix}projection_verified={}",
                context.projection_verified
            );
            let _ = writeln!(out, "{context_prefix}verdict={}", context.verdict);
            let _ = writeln!(out, "{context_prefix}burdens={}", context.burdens.total);
            let _ = writeln!(
                out,
                "{context_prefix}discharged={}",
                context.burdens.discharged
            );
            let _ = writeln!(out, "{context_prefix}open={}", context.burdens.open);
            let _ = writeln!(out, "{context_prefix}invalid={}", context.burdens.invalid);
            let _ = writeln!(
                out,
                "{context_prefix}missing_evidence={}",
                context.burdens.missing_evidence
            );
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CertificationError {
    Dna(DnaError),
    NonCanonicalDna,
    Projection {
        context: ContextId,
        error: ProjectionError,
    },
}

impl fmt::Display for CertificationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dna(error) => write!(f, "DNA certification failed: {error}"),
            Self::NonCanonicalDna => f.write_str("certification requires canonical L64D authority"),
            Self::Projection { context, error } => write!(
                f,
                "certification projection failed in context {context}: {error}"
            ),
        }
    }
}

impl std::error::Error for CertificationError {}

pub fn certify_dna(bytes: &[u8]) -> Result<AuthorityCertification, CertificationError> {
    let graph = decode_dna(bytes).map_err(CertificationError::Dna)?;
    let canonical = dna_bytes(&graph).map_err(CertificationError::Dna)?;
    if canonical != bytes {
        return Err(CertificationError::NonCanonicalDna);
    }
    certify_graph(&graph)
}

pub fn certify_graph(graph: &Graph) -> Result<AuthorityCertification, CertificationError> {
    certify_graph_from_fresh_root(graph, None)
}

pub fn certify_graph_with_root_projection(
    graph: &Graph,
    research_limit: usize,
) -> Result<(AuthorityCertification, ProjectionSet), CertificationError> {
    // The root projection is derived inside the certification boundary. No retained or external
    // projection can enter this path without a complete fresh derivation from exact authority.
    let projection =
        ProjectionSet::derive(graph, ROOT_CONTEXT, research_limit).map_err(|error| {
            CertificationError::Projection {
                context: ROOT_CONTEXT,
                error,
            }
        })?;
    let certification = certify_graph_from_fresh_root(graph, Some(&projection.certification))?;
    Ok((certification, projection))
}

fn certify_graph_from_fresh_root(
    graph: &Graph,
    root: Option<&CertificationView>,
) -> Result<AuthorityCertification, CertificationError> {
    let mut contexts = Vec::with_capacity(graph.context_count());
    for context in 0..graph.context_count() as ContextId {
        // Fresh in-process derivation is graph-bound by construction. Retained or external
        // certification projections must still cross `CertificationView::verify` before reuse.
        let derived;
        let view = if context == ROOT_CONTEXT {
            if let Some(root) = root {
                root
            } else {
                derived = CertificationView::derive(graph, context)
                    .map_err(|error| CertificationError::Projection { context, error })?;
                &derived
            }
        } else {
            derived = CertificationView::derive(graph, context)
                .map_err(|error| CertificationError::Projection { context, error })?;
            &derived
        };
        let burdens = BurdenCounts::from_view(view);
        contexts.push(ContextCertification {
            context,
            projection_verified: true,
            verdict: burdens.verdict(),
            burdens,
        });
    }
    let verdict = authority_verdict(&contexts);
    Ok(AuthorityCertification {
        symbol: graph.state_symbol(),
        node_count: graph.node_count(),
        context_count: graph.context_count(),
        journal_len: graph.journal_len(),
        canonical_dna: true,
        contexts,
        verdict,
    })
}

fn authority_verdict(contexts: &[ContextCertification]) -> CertificationVerdict {
    if contexts
        .iter()
        .any(|context| context.verdict == CertificationVerdict::Invalid)
    {
        CertificationVerdict::Invalid
    } else if contexts
        .iter()
        .any(|context| context.verdict == CertificationVerdict::Incomplete)
    {
        CertificationVerdict::Incomplete
    } else if contexts
        .iter()
        .any(|context| context.verdict == CertificationVerdict::Open)
    {
        CertificationVerdict::Open
    } else {
        CertificationVerdict::Certified
    }
}
