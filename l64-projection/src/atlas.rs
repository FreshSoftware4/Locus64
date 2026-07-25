use crate::{
    EvidenceRef, PortRef, ProjectionError, ProjectionSource, SourceRef, closure_rank, evidence_ref,
    source_ref,
};
use l64_native::{ClosureState, ContextId, Graph, OpCode, Route};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum AtlasCandidateKind {
    Operation,
    Equality,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtlasCandidate {
    pub kind: AtlasCandidateKind,
    pub source: SourceRef,
    pub opcode: OpCode,
    pub closure: ClosureState,
    pub output_type: Option<SourceRef>,
    pub ports: Vec<PortRef>,
    pub direct_dependents: Vec<SourceRef>,
    pub canonical_route: Option<Route>,
    pub evidence: Option<EvidenceRef>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AtlasView {
    pub source: ProjectionSource,
    pub candidates: Vec<AtlasCandidate>,
}

impl AtlasView {
    pub fn derive(graph: &Graph, context: ContextId) -> Result<Self, ProjectionError> {
        let source = ProjectionSource::capture(graph, context)?;
        let mut candidates = Vec::new();
        for node in graph.visible_nodes(context)? {
            let record = graph.node(node).ok_or(ProjectionError::UnknownNode(node))?;
            let kind = match record.opcode() {
                OpCode::Compose
                | OpCode::MatMul
                | OpCode::Add
                | OpCode::Multiply
                | OpCode::Divide
                | OpCode::Sqrt => AtlasCandidateKind::Operation,
                OpCode::TypeEquality => AtlasCandidateKind::Equality,
                _ => continue,
            };
            let source = source_ref(graph, node)?;
            let output_type = record.ty().map(|id| source_ref(graph, id)).transpose()?;
            let ports = graph
                .ports(node)
                .ok_or(ProjectionError::UnknownNode(node))?
                .iter()
                .map(|port| {
                    Ok(PortRef {
                        role: port.role(),
                        ordinal: port.ordinal(),
                        target: source_ref(graph, port.target())?,
                    })
                })
                .collect::<Result<Vec<_>, ProjectionError>>()?;
            let mut direct_dependents = graph
                .direct_dependents(node)?
                .iter()
                .copied()
                .map(|id| source_ref(graph, id))
                .collect::<Result<Vec<_>, ProjectionError>>()?;
            direct_dependents.sort();
            let canonical_route = graph.canonical_representative_route(context, node).ok();
            let evidence = evidence_ref(graph, context, node)?;
            candidates.push(AtlasCandidate {
                kind,
                source,
                opcode: record.opcode(),
                closure: graph.closure_state(context, node)?,
                output_type,
                ports,
                direct_dependents,
                canonical_route,
                evidence,
            });
        }
        candidates.sort_by(|left, right| {
            closure_rank(left.closure)
                .cmp(&closure_rank(right.closure))
                .then_with(|| left.kind.cmp(&right.kind))
                .then_with(|| left.source.route.cmp(&right.source.route))
        });
        Ok(Self { source, candidates })
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
