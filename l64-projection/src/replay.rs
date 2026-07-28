use crate::{ContextAnalysis, ProjectionError, ProjectionSource, SourceRef, source_ref};
use l64_native::{ContextId, Graph, OpCode, SymbolicSeal};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayStep {
    pub event: u32,
    pub operation: OpCode,
    pub subject: SourceRef,
    pub before: SymbolicSeal,
    pub after: SymbolicSeal,
    pub parent: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayView {
    pub source: ProjectionSource,
    pub steps: Vec<ReplayStep>,
}

impl ReplayView {
    pub fn derive(graph: &Graph, context: ContextId) -> Result<Self, ProjectionError> {
        let analysis = ContextAnalysis::derive(graph, context)?;
        Self::derive_analyzed(graph, &analysis)
    }

    pub(crate) fn derive_analyzed(
        graph: &Graph,
        analysis: &ContextAnalysis,
    ) -> Result<Self, ProjectionError> {
        let visible = analysis.nodes();
        let mut steps = Vec::new();
        for (event, item) in graph.journal().iter().enumerate() {
            if visible.binary_search(&item.subject()).is_err() {
                continue;
            }
            steps.push(ReplayStep {
                event: event as u32,
                operation: item.operation(),
                subject: source_ref(graph, item.subject())?,
                before: item.before(),
                after: item.after(),
                parent: item.parent(),
            });
        }
        Ok(Self {
            source: analysis.source.clone(),
            steps,
        })
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
