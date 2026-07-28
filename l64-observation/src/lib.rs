#![forbid(unsafe_code)]

use l64_certification::{
    AuthorityCertification, CertificationError, ContextCertification, certify_graph,
};
use l64_native::{ContextId, DnaError, Graph, OpCode, Route, decode_dna, dna_bytes};
use l64_projection::{ProjectionError, ReplayView, ReportView};
use l64_transport::{BundleError, decode_bundle};
use std::fmt::{self, Write as _};

pub const OBSERVATION_VERSION: u16 = 1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObservationSurface {
    Replay,
    Report,
}

impl fmt::Display for ObservationSurface {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Replay => "replay",
            Self::Report => "report",
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextObservation {
    pub certification: ContextCertification,
    pub replay: ReplayView,
    pub report: ReportView,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AuthorityObservation {
    pub certification: AuthorityCertification,
    pub contexts: Vec<ContextObservation>,
}

impl AuthorityObservation {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "L64 NATIVE OBSERVATION v{OBSERVATION_VERSION}");
        self.render_into(&mut out, "");
        out
    }

    pub fn render_into(&self, out: &mut String, prefix: &str) {
        render_authority(out, self, prefix);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BundleObservation {
    pub members: Vec<AuthorityObservation>,
}

impl BundleObservation {
    pub fn render_text(&self) -> String {
        let mut out = String::new();
        let _ = writeln!(out, "L64 NATIVE BUNDLE OBSERVATION v{OBSERVATION_VERSION}");
        let _ = writeln!(out, "transport=canonical_l64b");
        let _ = writeln!(out, "transport_verified=true");
        let _ = writeln!(out, "members={}", self.members.len());
        let _ = writeln!(out, "composite_authority=none");
        let _ = writeln!(out, "composite_verdict=none");
        let _ = writeln!(out, "composite_observation=none");
        for (index, member) in self.members.iter().enumerate() {
            member.render_into(&mut out, &format!("member.{index}."));
        }
        out
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ObservationError {
    Dna(DnaError),
    NonCanonicalDna,
    Certification(CertificationError),
    Projection {
        context: ContextId,
        surface: ObservationSurface,
        error: ProjectionError,
    },
    Bundle(BundleError),
}

impl fmt::Display for ObservationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Dna(error) => write!(f, "DNA observation failed: {error}"),
            Self::NonCanonicalDna => f.write_str("observation requires canonical L64D authority"),
            Self::Certification(error) => write!(f, "observation certification failed: {error}"),
            Self::Projection {
                context,
                surface,
                error,
            } => write!(
                f,
                "{surface} projection failed in context {context}: {error}"
            ),
            Self::Bundle(error) => write!(f, "bundle observation failed: {error}"),
        }
    }
}

impl std::error::Error for ObservationError {}

pub fn observe_dna(bytes: &[u8]) -> Result<AuthorityObservation, ObservationError> {
    let graph = decode_dna(bytes).map_err(ObservationError::Dna)?;
    let canonical = dna_bytes(&graph).map_err(ObservationError::Dna)?;
    if canonical != bytes {
        return Err(ObservationError::NonCanonicalDna);
    }
    observe_canonical_graph(&graph)
}

pub fn observe_bundle(bytes: &[u8]) -> Result<BundleObservation, ObservationError> {
    let bundle = decode_bundle(bytes).map_err(ObservationError::Bundle)?;
    let mut members = Vec::with_capacity(bundle.len());
    for member in bundle.members() {
        members.push(observe_canonical_graph(member.graph())?);
    }
    Ok(BundleObservation { members })
}

pub fn observe_canonical_graph(graph: &Graph) -> Result<AuthorityObservation, ObservationError> {
    let certification = certify_graph(graph).map_err(ObservationError::Certification)?;
    let mut contexts = Vec::with_capacity(graph.context_count());
    for context in 0..graph.context_count() as ContextId {
        let replay =
            ReplayView::derive(graph, context).map_err(|error| ObservationError::Projection {
                context,
                surface: ObservationSurface::Replay,
                error,
            })?;
        replay
            .verify(graph)
            .map_err(|error| ObservationError::Projection {
                context,
                surface: ObservationSurface::Replay,
                error,
            })?;

        let report =
            ReportView::derive(graph, context).map_err(|error| ObservationError::Projection {
                context,
                surface: ObservationSurface::Report,
                error,
            })?;
        report
            .verify(graph)
            .map_err(|error| ObservationError::Projection {
                context,
                surface: ObservationSurface::Report,
                error,
            })?;

        let context_certification = certification.contexts[context as usize].clone();
        contexts.push(ContextObservation {
            certification: context_certification,
            replay,
            report,
        });
    }
    Ok(AuthorityObservation {
        certification,
        contexts,
    })
}

fn render_authority(out: &mut String, observation: &AuthorityObservation, prefix: &str) {
    let authority = &observation.certification;
    let _ = writeln!(out, "{prefix}scope=native_certification_replay_observation");
    let _ = writeln!(out, "{prefix}authority=exact_canonical_l64d");
    let _ = writeln!(out, "{prefix}canonical_dna={}", authority.canonical_dna);
    let _ = writeln!(out, "{prefix}projection_authority=non_authoritative");
    let _ = writeln!(out, "{prefix}symbol={}", authority.symbol);
    let _ = writeln!(out, "{prefix}nodes={}", authority.node_count);
    let _ = writeln!(out, "{prefix}contexts={}", authority.context_count);
    let _ = writeln!(out, "{prefix}journal={}", authority.journal_len);
    let _ = writeln!(out, "{prefix}verdict={}", authority.verdict);

    for context in &observation.contexts {
        let id = context.certification.context;
        let context_prefix = format!("{prefix}context.{id}.");
        let _ = writeln!(
            out,
            "{context_prefix}certification_projection_verified={}",
            context.certification.projection_verified
        );
        let _ = writeln!(out, "{context_prefix}replay_projection_verified=true");
        let _ = writeln!(out, "{context_prefix}report_projection_verified=true");
        let _ = writeln!(
            out,
            "{context_prefix}verdict={}",
            context.certification.verdict
        );
        let _ = writeln!(out, "{context_prefix}closed={}", context.report.closed);
        let _ = writeln!(out, "{context_prefix}open={}", context.report.open);
        let _ = writeln!(out, "{context_prefix}invalid={}", context.report.invalid);
        let _ = writeln!(
            out,
            "{context_prefix}obligation_routes={}",
            context.report.obligation_routes.len()
        );
        let _ = writeln!(
            out,
            "{context_prefix}invalid_routes={}",
            context.report.invalid_routes.len()
        );
        for count in &context.report.opcode_counts {
            let _ = writeln!(
                out,
                "{context_prefix}opcode.{}.count={}",
                opcode_number(count.opcode),
                count.count
            );
        }
        let _ = writeln!(
            out,
            "{context_prefix}replay_steps={}",
            context.replay.steps.len()
        );
        for (ordinal, step) in context.replay.steps.iter().enumerate() {
            let step_prefix = format!("{context_prefix}replay.{ordinal}.");
            let _ = writeln!(out, "{step_prefix}event={}", step.event);
            let _ = writeln!(
                out,
                "{step_prefix}operation={}",
                opcode_number(step.operation)
            );
            let _ = writeln!(out, "{step_prefix}subject_node={}", step.subject.node);
            let _ = writeln!(
                out,
                "{step_prefix}subject_context={}",
                step.subject.declared_context
            );
            let _ = writeln!(
                out,
                "{step_prefix}subject_route={}",
                render_route(&step.subject.route)
            );
            let _ = writeln!(out, "{step_prefix}before={}", step.before);
            let _ = writeln!(out, "{step_prefix}after={}", step.after);
            let parent = step
                .parent
                .map(|value| value.to_string())
                .unwrap_or_else(|| "none".to_string());
            let _ = writeln!(out, "{step_prefix}parent={parent}");
        }
    }
}

fn opcode_number(opcode: OpCode) -> u16 {
    opcode as u16
}

fn render_route(route: &Route) -> String {
    let mut out = format!("{:016x}", route.domain().0);
    for word in route.tail() {
        let _ = write!(out, "/{:016x}", word.0);
    }
    out
}
