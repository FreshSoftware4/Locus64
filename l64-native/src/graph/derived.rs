impl DerivedIndex {
    fn empty(context_count: usize) -> Self {
        Self {
            reverse: Vec::new(),
            by_context: vec![Vec::new(); context_count],
            primary_route: Vec::new(),
        }
    }
}

impl Graph {
    fn rebuild_derived_index(&mut self) {
        self.derived = DerivedIndex::empty(self.contexts.len());
        self.derived.reverse.resize_with(self.nodes.len(), Vec::new);
        for node in 0..self.nodes.len() as NodeId {
            self.register_derived_node(node);
        }
        let routes = self
            .routes
            .iter()
            .map(|(route, node)| (route.clone(), *node))
            .collect::<Vec<_>>();
        for (route, node) in routes {
            self.register_derived_route(&route, node);
        }
    }

    fn register_derived_node(&mut self, node: NodeId) {
        let record = self.nodes[node as usize];
        while self.derived.reverse.len() <= node as usize {
            self.derived.reverse.push(Vec::new());
        }
        while self.derived.by_context.len() <= record.context() as usize {
            self.derived.by_context.push(Vec::new());
        }
        self.derived.by_context[record.context() as usize].push(node);

        let mut dependencies = Vec::new();
        if let Some(ty) = record.ty() {
            dependencies.push(ty);
        }
        dependencies.extend(self.ports[record.port_range()].iter().map(Port::target));
        dependencies.sort_unstable();
        dependencies.dedup();
        for dependency in dependencies {
            self.derived.reverse[dependency as usize].push(node);
        }
    }

    fn register_derived_route(&mut self, route: &Route, node: NodeId) {
        while self.derived.primary_route.len() <= node as usize {
            self.derived.primary_route.push(None);
        }
        let slot = &mut self.derived.primary_route[node as usize];
        if slot.as_ref().is_none_or(|current| route < current) {
            *slot = Some(route.clone());
        }
    }

    pub(crate) fn derived_route_for_node(&self, node: NodeId) -> Option<&Route> {
        self.derived
            .primary_route
            .get(node as usize)
            .and_then(Option::as_ref)
    }

    pub fn direct_dependents(&self, node: NodeId) -> Result<&[NodeId], Obstruction> {
        self.ensure_node(node)?;
        Ok(&self.derived.reverse[node as usize])
    }

    pub fn affected_nodes(&self, seed: NodeId) -> Result<Vec<NodeId>, Obstruction> {
        self.ensure_node(seed)?;
        let mut affected = std::collections::BTreeSet::from([seed]);
        let mut queue = std::collections::VecDeque::from([seed]);
        while let Some(current) = queue.pop_front() {
            for dependent in &self.derived.reverse[current as usize] {
                if affected.insert(*dependent) {
                    queue.push_back(*dependent);
                }
            }
        }
        Ok(affected.into_iter().collect())
    }

    pub fn visible_nodes(&self, context: ContextId) -> Result<Vec<NodeId>, Obstruction> {
        self.ensure_context(context)?;
        let mut contexts = Vec::new();
        let mut cursor = context;
        loop {
            contexts.push(cursor);
            if cursor == ROOT_CONTEXT {
                break;
            }
            cursor = self.ensure_context(cursor)?.parent();
        }
        contexts.reverse();
        let mut nodes = Vec::new();
        for context in contexts {
            nodes.extend_from_slice(&self.derived.by_context[context as usize]);
        }
        nodes.sort_unstable();
        Ok(nodes)
    }
}
