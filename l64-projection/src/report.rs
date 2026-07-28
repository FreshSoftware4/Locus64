use crate::{ContextAnalysis, ProjectionError, ProjectionSource, source_ref};
use l64_native::{ClosureState, ContextId, Graph, OpCode, Route};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct OpcodeCount {
    pub opcode: OpCode,
    pub count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ReportView {
    pub source: ProjectionSource,
    pub closed: usize,
    pub open: usize,
    pub invalid: usize,
    pub opcode_counts: Vec<OpcodeCount>,
    pub obligation_routes: Vec<Route>,
    pub invalid_routes: Vec<Route>,
}

impl ReportView {
    pub fn derive(graph: &Graph, context: ContextId) -> Result<Self, ProjectionError> {
        let analysis = ContextAnalysis::derive(graph, context)?;
        Self::derive_analyzed(graph, &analysis)
    }

    pub(crate) fn derive_analyzed(
        graph: &Graph,
        analysis: &ContextAnalysis,
    ) -> Result<Self, ProjectionError> {
        let mut closed = 0;
        let mut open = 0;
        let mut invalid = 0;
        let mut counts = BTreeMap::<u16, (OpCode, usize)>::new();
        let mut obligation_routes = Vec::new();
        let mut invalid_routes = Vec::new();
        for &node in analysis.nodes() {
            let record = graph.node(node).ok_or(ProjectionError::UnknownNode(node))?;
            match analysis.closure(node)? {
                ClosureState::Closed => closed += 1,
                ClosureState::Open => open += 1,
                ClosureState::Invalid => {
                    invalid += 1;
                    invalid_routes.push(source_ref(graph, node)?.route);
                }
            }
            let entry = counts
                .entry(record.opcode() as u16)
                .or_insert((record.opcode(), 0));
            entry.1 += 1;
            if record.opcode() == OpCode::Obligation {
                obligation_routes.push(source_ref(graph, node)?.route);
            }
        }
        obligation_routes.sort();
        invalid_routes.sort();
        let opcode_counts = counts
            .into_values()
            .map(|(opcode, count)| OpcodeCount { opcode, count })
            .collect();
        Ok(Self {
            source: analysis.source.clone(),
            closed,
            open,
            invalid,
            opcode_counts,
            obligation_routes,
            invalid_routes,
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
