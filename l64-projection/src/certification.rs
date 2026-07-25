use crate::{AtlasView, ProjectionError, ProjectionSource, SourceRef};
use l64_native::{ClosureState, ContextId, Graph, OpCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum BurdenState {
    Discharged,
    Open,
    Invalid,
    MissingEvidence,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificationBurden {
    pub subject: SourceRef,
    pub closure: ClosureState,
    pub state: BurdenState,
    pub judgment: Option<SourceRef>,
    pub evidence: Option<SourceRef>,
    pub evidence_opcode: Option<OpCode>,
    pub dependencies: Vec<SourceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificationView {
    pub source: ProjectionSource,
    pub burdens: Vec<CertificationBurden>,
}

impl CertificationView {
    pub fn derive(graph: &Graph, context: ContextId) -> Result<Self, ProjectionError> {
        let source = ProjectionSource::capture(graph, context)?;
        let atlas = AtlasView::derive(graph, context)?;
        let mut burdens = Vec::with_capacity(atlas.candidates.len());
        for candidate in atlas.candidates {
            let closure = candidate.closure;
            let (judgment, evidence, evidence_opcode) = candidate
                .evidence
                .as_ref()
                .map(|item| {
                    (
                        item.judgment.clone(),
                        Some(item.evidence.clone()),
                        Some(item.opcode),
                    )
                })
                .unwrap_or((None, None, None));
            let state = match (closure, evidence_opcode) {
                (ClosureState::Invalid, _) => BurdenState::Invalid,
                (ClosureState::Open, _) => BurdenState::Open,
                (ClosureState::Closed, Some(_)) => BurdenState::Discharged,
                (ClosureState::Closed, None) => BurdenState::MissingEvidence,
            };
            let mut dependencies = candidate
                .ports
                .into_iter()
                .map(|port| port.target)
                .collect::<Vec<_>>();
            if let Some(output_type) = candidate.output_type {
                dependencies.push(output_type);
            }
            dependencies.sort();
            dependencies.dedup();
            burdens.push(CertificationBurden {
                subject: candidate.source,
                closure,
                state,
                judgment,
                evidence,
                evidence_opcode,
                dependencies,
            });
        }
        burdens.sort_by(|left, right| {
            left.state
                .cmp(&right.state)
                .then_with(|| left.subject.route.cmp(&right.subject.route))
        });
        Ok(Self { source, burdens })
    }

    pub fn verify(&self, graph: &Graph) -> Result<(), ProjectionError> {
        self.source.verify(graph)?;
        let rebuilt = Self::derive(graph, self.source.context)?;
        if &rebuilt == self {
            Ok(())
        } else {
            Err(ProjectionError::ProjectionMismatch)
        }
    }
}
