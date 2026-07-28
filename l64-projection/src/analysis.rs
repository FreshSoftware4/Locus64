use crate::{ProjectionError, ProjectionSource};
use l64_native::{ClosureState, ContextId, Graph, NodeId};

pub(crate) struct ContextAnalysis {
    pub source: ProjectionSource,
    nodes: Vec<NodeId>,
    closures: Vec<Option<ClosureState>>,
}

impl ContextAnalysis {
    pub fn derive(graph: &Graph, context: ContextId) -> Result<Self, ProjectionError> {
        let source = ProjectionSource::capture(graph, context)?;
        let states = graph.context_closure_states(context)?;
        let mut nodes = Vec::with_capacity(states.len());
        let mut closures = vec![None; graph.node_count()];
        for (node, state) in states {
            nodes.push(node);
            closures[node as usize] = Some(state);
        }
        Ok(Self {
            source,
            nodes,
            closures,
        })
    }

    pub fn nodes(&self) -> &[NodeId] {
        &self.nodes
    }

    pub fn closure(&self, node: NodeId) -> Result<ClosureState, ProjectionError> {
        self.closures
            .get(node as usize)
            .and_then(|state| *state)
            .ok_or(ProjectionError::UnknownNode(node))
    }
}
