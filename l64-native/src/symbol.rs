use core::fmt;

use crate::{ContextDelta, Graph, Node, NodeId, Port, Route, canonical_bytes};
use l64_symbolic::{Composer, SymbolicIdentity, SymbolicSeal};

pub const STATE_SYMBOL_DOMAIN: &str = "l64.native.state.v2";
const NODE_DOMAIN: &str = "l64.native.node.v1";
const PORT_DOMAIN: &str = "l64.native.port.v1";
const CONTEXT_DOMAIN: &str = "l64.native.context.v1";
const ROUTE_DOMAIN: &str = "l64.native.route.v1";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StateSymbol {
    pub root: SymbolicSeal,
    pub nodes: SymbolicSeal,
    pub ports: SymbolicSeal,
    pub contexts: SymbolicSeal,
    pub routes: SymbolicSeal,
}

impl StateSymbol {
    pub const ZERO: Self = Self {
        root: SymbolicSeal::ZERO,
        nodes: SymbolicSeal::ZERO,
        ports: SymbolicSeal::ZERO,
        contexts: SymbolicSeal::ZERO,
        routes: SymbolicSeal::ZERO,
    };

    pub fn changed_sections(self, other: Self) -> [bool; 4] {
        [
            self.nodes != other.nodes,
            self.ports != other.ports,
            self.contexts != other.contexts,
            self.routes != other.routes,
        ]
    }
}

impl fmt::Display for StateSymbol {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} ∣ nodes={} ports={} contexts={} routes={}",
            self.root.pretty(STATE_SYMBOL_DOMAIN),
            self.nodes,
            self.ports,
            self.contexts,
            self.routes
        )
    }
}

pub fn state_identity(graph: &Graph) -> SymbolicIdentity {
    SymbolicIdentity::new(STATE_SYMBOL_DOMAIN, canonical_bytes(graph))
}

pub(crate) fn state_symbol(graph: &Graph) -> StateSymbol {
    let nodes = ordered_section("nodes", graph.nodes_raw().iter().map(node_seal));
    let ports = ordered_section("ports", graph.ports_raw().iter().map(port_seal));
    let contexts = ordered_section("contexts", graph.contexts_raw().iter().map(context_seal));
    let routes = ordered_section(
        "routes",
        graph
            .routes_raw()
            .iter()
            .map(|(route, node)| route_seal(route, *node)),
    );
    let mut root = Composer::new(STATE_SYMBOL_DOMAIN);
    root.u16("codec", crate::codec::CODEC_VERSION)
        .u64("node-count", graph.nodes_raw().len() as u64)
        .u64("port-count", graph.ports_raw().len() as u64)
        .u64("context-count", graph.contexts_raw().len() as u64)
        .u64("route-count", graph.routes_raw().len() as u64)
        .ordered("sections", &[nodes, ports, contexts, routes]);
    StateSymbol {
        root: root.finish(),
        nodes,
        ports,
        contexts,
        routes,
    }
}

fn ordered_section(label: &str, parts: impl Iterator<Item = SymbolicSeal>) -> SymbolicSeal {
    let parts = parts.collect::<Vec<_>>();
    let mut composer = Composer::new("l64.native.section.v1");
    composer.ordered(label, &parts);
    composer.finish()
}

fn node_seal(node: &Node) -> SymbolicSeal {
    let range = node.port_range();
    let mut composer = Composer::new(NODE_DOMAIN);
    composer
        .u16("opcode", node.opcode() as u16)
        .u32("context", node.context())
        .u32("type", node.ty().unwrap_or(NodeId::MAX))
        .u64("payload", node.payload())
        .u32("first-port", range.start as u32)
        .u16("port-count", (range.end - range.start) as u16);
    composer.finish()
}

fn port_seal(port: &Port) -> SymbolicSeal {
    let mut composer = Composer::new(PORT_DOMAIN);
    composer
        .u32("target", port.target())
        .u8("role", port.role() as u8)
        .u8("flags", port.flags())
        .u16("ordinal", port.ordinal());
    composer.finish()
}

fn context_seal(context: &ContextDelta) -> SymbolicSeal {
    let mut composer = Composer::new(CONTEXT_DOMAIN);
    composer
        .u32("parent", context.parent())
        .u32("binding", context.binding().unwrap_or(NodeId::MAX));
    composer.finish()
}

fn route_seal(route: &Route, node: NodeId) -> SymbolicSeal {
    let mut tail = Vec::with_capacity(route.tail().len() * 8);
    for word in route.tail() {
        tail.extend_from_slice(&word.0.to_le_bytes());
    }
    let mut composer = Composer::new(ROUTE_DOMAIN);
    composer
        .u64("domain", route.domain().0)
        .field("tail", &tail)
        .u32("node", node);
    composer.finish()
}
