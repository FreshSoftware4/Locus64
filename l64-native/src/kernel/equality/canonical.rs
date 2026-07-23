impl Graph {
    pub fn equality_path(
        &self,
        context: ContextId,
        left: NodeId,
        right: NodeId,
    ) -> Result<Vec<NodeId>, Obstruction> {
        self.ensure_context(context)?;
        let left_node = self.ensure_node(left)?;
        let right_node = self.ensure_node(right)?;
        if !self.equality_endpoint_allowed(left_node)
            || !self.equality_endpoint_allowed(right_node)
            || !self.context_visible(left_node.context(), context)
            || !self.context_visible(right_node.context(), context)
        {
            return Err(Obstruction::NoEqualityPath { left, right });
        }
        self.validate_equality_authority()?;
        if left == right {
            return Ok(Vec::new());
        }

        let mut queue = std::collections::VecDeque::from([left]);
        let mut parent = std::collections::BTreeMap::<NodeId, (NodeId, NodeId)>::new();
        parent.insert(left, (left, u32::MAX));
        while let Some(current) = queue.pop_front() {
            for judgment in self.routes_raw().values() {
                let equality = self
                    .node(*judgment)
                    .ok_or(Obstruction::UnknownNode { node: *judgment })?;
                if equality.opcode() != OpCode::TypeEquality
                    || !self.context_visible(equality.context(), context)
                {
                    continue;
                }
                let (edge_left, edge_right) = self.equality_parts(*judgment)?;
                let next = if edge_left == current {
                    Some(edge_right)
                } else if edge_right == current {
                    Some(edge_left)
                } else {
                    None
                };
                let Some(next) = next else { continue };
                if parent.contains_key(&next) {
                    continue;
                }
                parent.insert(next, (current, *judgment));
                if next == right {
                    let mut path = Vec::new();
                    let mut cursor = right;
                    while cursor != left {
                        let (previous, proof) = parent[&cursor];
                        path.push(proof);
                        cursor = previous;
                    }
                    path.reverse();
                    return Ok(path);
                }
                queue.push_back(next);
            }
        }
        Err(Obstruction::NoEqualityPath { left, right })
    }

    pub fn canonical_representative(
        &self,
        context: ContextId,
        node: NodeId,
    ) -> Result<NodeId, Obstruction> {
        self.ensure_context(context)?;
        let subject = self.ensure_node(node)?;
        if !self.equality_endpoint_allowed(subject) || !self.context_visible(subject.context(), context)
        {
            return Err(Obstruction::UncanonicalizableNode { node });
        }
        self.validate_equality_authority()?;

        let mut members = std::collections::BTreeSet::from([node]);
        loop {
            let before = members.len();
            for candidate in 0..self.node_count() as NodeId {
                let equality = self
                    .node(candidate)
                    .ok_or(Obstruction::UnknownNode { node: candidate })?;
                if equality.opcode() != OpCode::TypeEquality
                    || !self.context_visible(equality.context(), context)
                {
                    continue;
                }
                let (left, right) = self.equality_parts(candidate)?;
                if members.contains(&left) || members.contains(&right) {
                    members.insert(left);
                    members.insert(right);
                }
            }
            if members.len() == before {
                break;
            }
        }

        self.routes_raw()
            .iter()
            .find_map(|(_, candidate)| members.contains(candidate).then_some(*candidate))
            .ok_or(Obstruction::UncanonicalizableNode { node })
    }

    pub fn canonical_representative_route(
        &self,
        context: ContextId,
        node: NodeId,
    ) -> Result<Route, Obstruction> {
        let representative = self.canonical_representative(context, node)?;
        self.route_for_node(representative)
            .cloned()
            .ok_or(Obstruction::UncanonicalizableNode {
                node: representative,
            })
    }
}
