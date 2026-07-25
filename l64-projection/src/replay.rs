use crate::{ProjectionError, ProjectionSource, SourceRef, source_ref};
use l64_native::{ContextId, Graph, OpCode};
use std::collections::BTreeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayStep {
    pub event: u32,
    pub operation: OpCode,
    pub subject: SourceRef,
    pub before: [u8; 32],
    pub after: [u8; 32],
    pub parent: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReplayView {
    pub source: ProjectionSource,
    pub steps: Vec<ReplayStep>,
}

impl ReplayView {
    pub fn derive(graph: &Graph, context: ContextId) -> Result<Self, ProjectionError> {
        let source = ProjectionSource::capture(graph, context)?;
        let visible = graph
            .visible_nodes(context)?
            .into_iter()
            .collect::<BTreeSet<_>>();
        let mut steps = Vec::new();
        for (event, item) in graph.journal().iter().enumerate() {
            if !visible.contains(&item.subject()) {
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
        Ok(Self { source, steps })
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
