use crate::{
    ContextAnalysis, ProjectionError, ProjectionSource, SourceRef, closure_rank, evidence_ref,
    source_ref,
};
use l64_native::{ClosureState, ContextId, Graph, OpCode, Route};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResearchCandidate {
    pub source: SourceRef,
    pub opcode: OpCode,
    pub closure: ClosureState,
    pub direct_dependents: usize,
    pub affected_nodes: usize,
    pub canonical_route: Option<Route>,
    pub evidence: Option<SourceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ResearchView {
    pub source: ProjectionSource,
    pub total_candidates: usize,
    pub limit: usize,
    pub candidates: Vec<ResearchCandidate>,
}

impl ResearchView {
    pub fn derive(
        graph: &Graph,
        context: ContextId,
        limit: usize,
    ) -> Result<Self, ProjectionError> {
        let analysis = ContextAnalysis::derive(graph, context)?;
        Self::derive_analyzed(graph, &analysis, limit)
    }

    pub(crate) fn derive_analyzed(
        graph: &Graph,
        analysis: &ContextAnalysis,
        limit: usize,
    ) -> Result<Self, ProjectionError> {
        let context = analysis.source.context;
        let mut candidates = Vec::new();
        for &node in analysis.nodes() {
            let record = graph.node(node).ok_or(ProjectionError::UnknownNode(node))?;
            let closure = analysis.closure(node)?;
            let direct_dependents = graph.direct_dependents(node)?.len();
            let candidate_opcode = matches!(
                record.opcode(),
                OpCode::Compose
                    | OpCode::MatMul
                    | OpCode::Add
                    | OpCode::Multiply
                    | OpCode::Divide
                    | OpCode::Sqrt
                    | OpCode::TypeEquality
            );
            if closure == ClosureState::Closed && !(candidate_opcode && direct_dependents > 0) {
                continue;
            }
            let affected_nodes = graph.affected_nodes(node)?.len();
            candidates.push(ResearchCandidate {
                source: source_ref(graph, node)?,
                opcode: record.opcode(),
                closure,
                direct_dependents,
                affected_nodes,
                canonical_route: graph.canonical_representative_route(context, node).ok(),
                evidence: evidence_ref(graph, node, |id| analysis.closure(id))?
                    .map(|item| item.evidence),
            });
        }
        candidates.sort_by(|left, right| {
            closure_rank(left.closure)
                .cmp(&closure_rank(right.closure))
                .then_with(|| right.affected_nodes.cmp(&left.affected_nodes))
                .then_with(|| right.direct_dependents.cmp(&left.direct_dependents))
                .then_with(|| left.source.route.cmp(&right.source.route))
        });
        let total_candidates = candidates.len();
        if limit < candidates.len() {
            candidates.truncate(limit);
        }
        Ok(Self {
            source: analysis.source.clone(),
            total_candidates,
            limit,
            candidates,
        })
    }

    pub fn verify(&self, graph: &Graph) -> Result<(), ProjectionError> {
        self.source.verify(graph)?;
        let rebuilt = Self::derive(graph, self.source.context, self.limit)?;
        if &rebuilt == self {
            Ok(())
        } else {
            Err(ProjectionError::ProjectionMismatch)
        }
    }
}
