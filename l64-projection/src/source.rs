use crate::PROJECTION_VERSION;
use l64_native::{ClosureState, ContextId, Graph, NodeId, Obstruction, OpCode, PortRole, Route};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ProjectionError {
    UnknownContext(ContextId),
    UnknownNode(NodeId),
    MissingRoute(NodeId),
    Native(Obstruction),
    VersionMismatch { expected: u16, actual: u16 },
    SourceMismatch,
    ProjectionMismatch,
}

impl From<Obstruction> for ProjectionError {
    fn from(value: Obstruction) -> Self {
        Self::Native(value)
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProjectionSource {
    pub commitment: [u8; 32],
    pub context: ContextId,
    pub node_count: usize,
    pub context_count: usize,
    pub journal_len: usize,
    pub version: u16,
}

impl ProjectionSource {
    pub fn capture(graph: &Graph, context: ContextId) -> Result<Self, ProjectionError> {
        graph
            .context(context)
            .ok_or(ProjectionError::UnknownContext(context))?;
        Ok(Self {
            commitment: graph.state_commitment(),
            context,
            node_count: graph.node_count(),
            context_count: graph.context_count(),
            journal_len: graph.journal_len(),
            version: PROJECTION_VERSION,
        })
    }

    pub fn verify(&self, graph: &Graph) -> Result<(), ProjectionError> {
        if self.version != PROJECTION_VERSION {
            return Err(ProjectionError::VersionMismatch {
                expected: PROJECTION_VERSION,
                actual: self.version,
            });
        }
        graph
            .context(self.context)
            .ok_or(ProjectionError::UnknownContext(self.context))?;
        if self.commitment != graph.state_commitment()
            || self.node_count != graph.node_count()
            || self.context_count != graph.context_count()
            || self.journal_len != graph.journal_len()
        {
            return Err(ProjectionError::SourceMismatch);
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct SourceRef {
    pub node: NodeId,
    pub route: Route,
    pub declared_context: ContextId,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PortRef {
    pub role: PortRole,
    pub ordinal: u16,
    pub target: SourceRef,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvidenceRef {
    pub judgment: Option<SourceRef>,
    pub evidence: SourceRef,
    pub opcode: OpCode,
    pub closure: ClosureState,
}

pub(crate) fn source_ref(graph: &Graph, node: NodeId) -> Result<SourceRef, ProjectionError> {
    let record = graph.node(node).ok_or(ProjectionError::UnknownNode(node))?;
    let route = graph
        .route_for_node(node)
        .cloned()
        .ok_or(ProjectionError::MissingRoute(node))?;
    Ok(SourceRef {
        node,
        route,
        declared_context: record.context(),
    })
}

pub(crate) fn evidence_ref(
    graph: &Graph,
    context: ContextId,
    subject: NodeId,
) -> Result<Option<EvidenceRef>, ProjectionError> {
    let subject_node = graph
        .node(subject)
        .ok_or(ProjectionError::UnknownNode(subject))?;
    if subject_node.opcode() == OpCode::TypeEquality {
        let Some(evidence) = graph.equality_witness_for(subject) else {
            return Ok(None);
        };
        let evidence_node = graph
            .node(evidence)
            .ok_or(ProjectionError::UnknownNode(evidence))?;
        return Ok(Some(EvidenceRef {
            judgment: Some(source_ref(graph, subject)?),
            evidence: source_ref(graph, evidence)?,
            opcode: evidence_node.opcode(),
            closure: graph.closure_state(context, evidence)?,
        }));
    }

    let mut judgments = graph
        .direct_dependents(subject)?
        .iter()
        .copied()
        .filter(|candidate| {
            let Some(node) = graph.node(*candidate) else {
                return false;
            };
            if node.opcode() != OpCode::TypeJudgment {
                return false;
            }
            graph
                .ports(*candidate)
                .and_then(|ports| ports.first())
                .map(|port| port.role() == PortRole::Subject && port.target() == subject)
                .unwrap_or(false)
        })
        .collect::<Vec<_>>();
    judgments.sort_by_key(|node| graph.route_for_node(*node).cloned());
    let Some(judgment) = judgments.first().copied() else {
        return Ok(None);
    };
    let mut evidence = graph
        .direct_dependents(judgment)?
        .iter()
        .copied()
        .filter(|candidate| {
            let Some(node) = graph.node(*candidate) else {
                return false;
            };
            node.ty() == Some(judgment)
                && matches!(node.opcode(), OpCode::KernelWitness | OpCode::Obligation)
        })
        .collect::<Vec<_>>();
    evidence.sort_by_key(|node| graph.route_for_node(*node).cloned());
    let Some(evidence) = evidence.first().copied() else {
        return Ok(None);
    };
    let evidence_node = graph
        .node(evidence)
        .ok_or(ProjectionError::UnknownNode(evidence))?;
    Ok(Some(EvidenceRef {
        judgment: Some(source_ref(graph, judgment)?),
        evidence: source_ref(graph, evidence)?,
        opcode: evidence_node.opcode(),
        closure: graph.closure_state(context, evidence)?,
    }))
}
