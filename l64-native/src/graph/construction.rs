impl Graph {
    pub(crate) fn from_decoded_parts(
        nodes: Vec<Node>,
        ports: Vec<Port>,
        contexts: Vec<ContextDelta>,
        routes: BTreeMap<Route, NodeId>,
    ) -> Self {
        let mut graph = Self {
            nodes,
            ports,
            contexts,
            routes,
            journal: Vec::new(),
            symbol: StateSymbol::ZERO,
            derived: DerivedIndex::default(),
            track_mutations: true,
        };
        graph.rebuild_derived_index();
        graph.symbol = crate::symbol::state_symbol(&graph);
        graph
    }

    pub fn new() -> Self {
        let mut graph = Self {
            nodes: Vec::new(),
            ports: Vec::new(),
            contexts: vec![ContextDelta::root()],
            routes: BTreeMap::new(),
            journal: Vec::new(),
            symbol: StateSymbol::ZERO,
            derived: DerivedIndex::empty(1),
            track_mutations: true,
        };
        graph.symbol = crate::symbol::state_symbol(&graph);
        graph
    }

    pub(crate) fn new_bulk() -> Self {
        Self {
            nodes: Vec::new(),
            ports: Vec::new(),
            contexts: vec![ContextDelta::root()],
            routes: BTreeMap::new(),
            journal: Vec::new(),
            symbol: StateSymbol::ZERO,
            derived: DerivedIndex::empty(1),
            track_mutations: false,
        }
    }

    pub(crate) fn finish_bulk(mut self) -> Self {
        debug_assert!(!self.track_mutations);
        self.symbol = crate::symbol::state_symbol(&self);
        self.track_mutations = true;
        self
    }

    fn record_mutation(&mut self, operation: OpCode, subject: NodeId) -> SymbolicSeal {
        if !self.track_mutations {
            return SymbolicSeal::ZERO;
        }
        let before = self.symbol.root;
        let next = crate::symbol::state_symbol(self);
        let after = next.root;
        self.push_event(operation, subject, before, after);
        self.symbol = next;
        after
    }

    pub fn node(&self, id: NodeId) -> Option<&Node> {
        self.nodes.get(id as usize)
    }

    pub fn ports(&self, node: NodeId) -> Option<&[Port]> {
        let node = self.node(node)?;
        self.ports.get(node.port_range())
    }

    pub fn resolve(&self, route: &Route) -> Option<NodeId> {
        self.routes.get(route).copied()
    }

    pub fn node_count(&self) -> usize {
        self.nodes.len()
    }

    pub fn port_count(&self) -> usize {
        self.ports.len()
    }

    pub fn context_count(&self) -> usize {
        self.contexts.len()
    }

    pub fn context(&self, id: ContextId) -> Option<&ContextDelta> {
        self.contexts.get(id as usize)
    }

    pub fn journal_len(&self) -> usize {
        self.journal.len()
    }

    pub fn state_symbol(&self) -> StateSymbol {
        self.symbol
    }

    pub fn exact_state_identity(&self) -> crate::SymbolicIdentity {
        crate::state_identity(self)
    }

    pub fn journal(&self) -> &[JournalEvent] {
        &self.journal
    }

    pub fn extend_context(
        &mut self,
        parent: ContextId,
        binding: NodeId,
    ) -> Result<ContextId, Obstruction> {
        self.ensure_context(parent)?;
        self.ensure_node(binding)?;
        self.validate_context_binding(parent, binding)?;
        let id = self.contexts.len() as ContextId;
        self.contexts.push(ContextDelta { parent, binding });
        self.derived.by_context.push(Vec::new());
        self.record_mutation(OpCode::ExtendContext, binding);
        Ok(id)
    }

    pub fn declare_atom_type(
        &mut self,
        route: Route,
        atom_code: LocusWord,
    ) -> Result<NodeId, Obstruction> {
        self.insert_committed_node(
            route,
            ROOT_CONTEXT,
            META_TYPE,
            OpCode::TypeAtom,
            atom_code.0,
            &[],
        )
    }

    pub fn declare_matrix_type(
        &mut self,
        route: Route,
        element: NodeId,
        rows: u32,
        cols: u32,
    ) -> Result<NodeId, Obstruction> {
        self.ensure_type(element)?;
        let payload = u64::from(rows) << 32 | u64::from(cols);
        let port = Port::new(element, PortRole::Parameter, 0);
        self.insert_committed_node(
            route,
            ROOT_CONTEXT,
            META_TYPE,
            OpCode::TypeMatrix,
            payload,
            &[port],
        )
    }

    pub fn declare_function_type(
        &mut self,
        route: Route,
        domain: NodeId,
        codomain: NodeId,
    ) -> Result<NodeId, Obstruction> {
        self.ensure_type(domain)?;
        self.ensure_type(codomain)?;
        let ports = [
            Port::new(domain, PortRole::Domain, 0),
            Port::new(codomain, PortRole::Codomain, 1),
        ];
        self.insert_committed_node(
            route,
            ROOT_CONTEXT,
            META_TYPE,
            OpCode::TypeFunction,
            0,
            &ports,
        )
    }

    pub fn declare_quantity_type(
        &mut self,
        route: Route,
        carrier: NodeId,
        dimension: Dimension,
    ) -> Result<NodeId, Obstruction> {
        self.ensure_type(carrier)?;
        let port = Port::new(carrier, PortRole::Parameter, 0);
        self.insert_committed_node(
            route,
            ROOT_CONTEXT,
            META_TYPE,
            OpCode::TypeQuantity,
            dimension.bits(),
            &[port],
        )
    }

    pub fn declare_constraint(
        &mut self,
        route: Route,
        context: ContextId,
        subject: NodeId,
        kind: ConstraintKind,
        holds: bool,
    ) -> Result<NodeId, Obstruction> {
        self.ensure_context(context)?;
        self.ensure_node(subject)?;
        let port = Port::new(subject, PortRole::Subject, 0);
        self.insert_committed_node(
            route,
            context,
            META_TYPE,
            OpCode::Constraint,
            kind.payload(holds),
            &[port],
        )
    }

    pub fn insert_value(
        &mut self,
        route: Route,
        context: ContextId,
        ty: NodeId,
    ) -> Result<NodeId, Obstruction> {
        self.ensure_context(context)?;
        let type_node = self.ensure_type(ty)?;
        if matches!(type_node.opcode(), OpCode::TypeJudgment | OpCode::TypeEquality) {
            return Err(Obstruction::EvidenceOnlyType { node: ty });
        }
        self.insert_committed_node(route, context, ty, OpCode::Value, 0, &[])
    }

    pub(crate) fn insert_admitted_operation(
        &mut self,
        routes: [Route; 3],
        context: ContextId,
        ty: NodeId,
        opcode: OpCode,
        inputs: &[NodeId],
        evidence_plan: EvidencePlan,
    ) -> crate::CommitResult {
        let [route, judgment_route, evidence_route] = routes;
        let operation = self.nodes.len() as NodeId;
        let operation_first_port = self.ports.len() as u32;
        self.ports.extend(
            inputs
                .iter()
                .enumerate()
                .map(|(ordinal, target)| Port::new(*target, PortRole::Argument, ordinal as u16)),
        );
        self.nodes.push(Node {
            payload: 0,
            context,
            ty,
            first_port: operation_first_port,
            opcode: opcode as u16,
            port_count: inputs.len() as u16,
        });

        let judgment = self.nodes.len() as NodeId;
        let judgment_first_port = self.ports.len() as u32;
        self.ports.push(Port::new(operation, PortRole::Subject, 0));
        self.ports.extend(
            inputs
                .iter()
                .enumerate()
                .map(|(ordinal, target)| Port::new(*target, PortRole::Premise, ordinal as u16 + 1)),
        );
        self.ports
            .push(Port::new(ty, PortRole::Conclusion, inputs.len() as u16 + 1));
        self.nodes.push(Node {
            payload: opcode as u64,
            context,
            ty: META_TYPE,
            first_port: judgment_first_port,
            opcode: OpCode::TypeJudgment as u16,
            port_count: inputs.len() as u16 + 2,
        });

        let evidence = self.nodes.len() as NodeId;
        let evidence_first_port = self.ports.len() as u32;
        let (evidence_opcode, evidence_payload, evidence_port_count) = match evidence_plan {
            EvidencePlan::Witness => (OpCode::KernelWitness, 0, 0),
            EvidencePlan::Obligation { kind, subject } => {
                self.ports.push(Port::new(subject, PortRole::Premise, 0));
                (OpCode::Obligation, kind.payload(true), 1)
            }
        };
        self.nodes.push(Node {
            payload: evidence_payload,
            context,
            ty: judgment,
            first_port: evidence_first_port,
            opcode: evidence_opcode as u16,
            port_count: evidence_port_count,
        });

        self.routes.insert(route.clone(), operation);
        self.routes.insert(judgment_route.clone(), judgment);
        self.routes.insert(evidence_route.clone(), evidence);
        self.register_derived_node(operation);
        self.register_derived_node(judgment);
        self.register_derived_node(evidence);
        self.register_derived_route(&route, operation);
        self.register_derived_route(&judgment_route, judgment);
        self.register_derived_route(&evidence_route, evidence);

        let after = self.record_mutation(opcode, operation);
        crate::CommitResult {
            node: operation,
            evidence,
            event: self.journal.len().saturating_sub(1) as EventId,
            symbol: after,
        }
    }

    pub(crate) fn insert_admitted_equality(
        &mut self,
        routes: [Route; 2],
        context: ContextId,
        left: NodeId,
        right: NodeId,
        rule: EqualityRule,
        premises: &[NodeId],
    ) -> crate::CommitResult {
        let [route, evidence_route] = routes;

        let judgment = self.nodes.len() as NodeId;
        let judgment_first_port = self.ports.len() as u32;
        self.ports.push(Port::new(left, PortRole::Left, 0));
        self.ports.push(Port::new(right, PortRole::Right, 1));
        self.nodes.push(Node {
            payload: 0,
            context,
            ty: META_TYPE,
            first_port: judgment_first_port,
            opcode: OpCode::TypeEquality as u16,
            port_count: 2,
        });

        let evidence = self.nodes.len() as NodeId;
        let evidence_first_port = self.ports.len() as u32;
        self.ports.extend(
            premises
                .iter()
                .enumerate()
                .map(|(ordinal, target)| Port::new(*target, PortRole::Premise, ordinal as u16)),
        );
        self.nodes.push(Node {
            payload: rule as u64,
            context,
            ty: judgment,
            first_port: evidence_first_port,
            opcode: OpCode::EqualityWitness as u16,
            port_count: premises.len() as u16,
        });

        self.routes.insert(route.clone(), judgment);
        self.routes.insert(evidence_route.clone(), evidence);
        self.register_derived_node(judgment);
        self.register_derived_node(evidence);
        self.register_derived_route(&route, judgment);
        self.register_derived_route(&evidence_route, evidence);
        let after = self.record_mutation(OpCode::EqualityWitness, judgment);
        crate::CommitResult {
            node: judgment,
            evidence,
            event: self.journal.len().saturating_sub(1) as EventId,
            symbol: after,
        }
    }
}
