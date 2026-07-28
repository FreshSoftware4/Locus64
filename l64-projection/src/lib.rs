#![forbid(unsafe_code)]

mod analysis;
mod atlas;
mod certification;
mod order;
mod replay;
mod report;
mod research;
mod set;
mod source;

pub use atlas::{AtlasCandidate, AtlasCandidateKind, AtlasView};
pub use certification::{BurdenState, CertificationBurden, CertificationView};
pub use replay::{ReplayStep, ReplayView};
pub use report::{OpcodeCount, ReportView};
pub use research::{ResearchCandidate, ResearchView};
pub use set::ProjectionSet;
pub use source::{EvidenceRef, PortRef, ProjectionError, ProjectionSource, SourceRef};

pub const PROJECTION_VERSION: u16 = 1;

pub(crate) use analysis::ContextAnalysis;
pub(crate) use order::closure_rank;
pub(crate) use source::{evidence_ref, source_ref};
