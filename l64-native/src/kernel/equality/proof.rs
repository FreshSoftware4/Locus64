impl Graph {
    pub fn prove_reflexive_equality(
        &mut self,
        route: Route,
        context: ContextId,
        subject: NodeId,
    ) -> Result<CommitResult, Obstruction> {
        self.prove_equality(
            route,
            context,
            subject,
            subject,
            EqualityRule::Reflexive,
            &[],
        )
    }

    pub fn prove_structural_equality(
        &mut self,
        route: Route,
        context: ContextId,
        left: NodeId,
        right: NodeId,
    ) -> Result<CommitResult, Obstruction> {
        self.prove_equality(
            route,
            context,
            left,
            right,
            EqualityRule::Structural,
            &[],
        )
    }

    pub fn prove_symmetric_equality(
        &mut self,
        route: Route,
        context: ContextId,
        premise: NodeId,
    ) -> Result<CommitResult, Obstruction> {
        let (left, right) = self.equality_parts(premise)?;
        self.prove_equality(
            route,
            context,
            right,
            left,
            EqualityRule::Symmetry,
            &[premise],
        )
    }

    pub fn prove_transitive_equality(
        &mut self,
        route: Route,
        context: ContextId,
        first: NodeId,
        second: NodeId,
    ) -> Result<CommitResult, Obstruction> {
        let (left, middle) = self.equality_parts(first)?;
        let (second_middle, right) = self.equality_parts(second)?;
        if middle != second_middle {
            return Err(Obstruction::EqualityPremiseMismatch { premise: second });
        }
        self.prove_equality(
            route,
            context,
            left,
            right,
            EqualityRule::Transitive,
            &[first, second],
        )
    }

    pub fn prove_congruent_equality(
        &mut self,
        route: Route,
        context: ContextId,
        left: NodeId,
        right: NodeId,
        premises: &[NodeId],
    ) -> Result<CommitResult, Obstruction> {
        self.prove_equality(
            route,
            context,
            left,
            right,
            EqualityRule::Congruence,
            premises,
        )
    }

    pub(crate) fn prove_equality(
        &mut self,
        route: Route,
        context: ContextId,
        left: NodeId,
        right: NodeId,
        rule: EqualityRule,
        premises: &[NodeId],
    ) -> Result<CommitResult, Obstruction> {
        let evidence_route = route.composed(EVIDENCE_LOCUS);
        if self.resolve(&route).is_some() || self.resolve(&evidence_route).is_some() {
            return Err(Obstruction::RouteOccupied);
        }
        self.validate_equality_rule(context, left, right, rule, premises)?;
        Ok(self.insert_admitted_equality(
            [route, evidence_route],
            context,
            left,
            right,
            rule,
            premises,
        ))
    }

    pub(crate) fn equality_parts(
        &self,
        judgment: NodeId,
    ) -> Result<(NodeId, NodeId), Obstruction> {
        let node = self.ensure_node(judgment)?;
        let ports = self
            .ports(judgment)
            .ok_or(Obstruction::MalformedEquality { node: judgment })?;
        if node.opcode() != OpCode::TypeEquality
            || ports.len() != 2
            || ports[0].role() != PortRole::Left
            || ports[1].role() != PortRole::Right
        {
            return Err(Obstruction::MalformedEquality { node: judgment });
        }
        Ok((ports[0].target(), ports[1].target()))
    }

    pub(crate) fn equality_evidence_parts(
        &self,
        judgment: NodeId,
    ) -> Result<(EqualityRule, Vec<NodeId>), Obstruction> {
        let evidence = self
            .equality_witness_for(judgment)
            .ok_or(Obstruction::MalformedEquality { node: judgment })?;
        let node = self
            .node(evidence)
            .ok_or(Obstruction::MalformedEquality { node: evidence })?;
        let ports = self
            .ports(evidence)
            .ok_or(Obstruction::MalformedEquality { node: evidence })?;
        let rule = EqualityRule::from_raw(node.payload() as u8)
            .filter(|_| node.payload() <= u8::MAX as u64)
            .ok_or(Obstruction::MalformedEquality { node: evidence })?;
        if node.opcode() != OpCode::EqualityWitness
            || node.ty() != Some(judgment)
            || ports.iter().any(|port| port.role() != PortRole::Premise)
        {
            return Err(Obstruction::MalformedEquality { node: evidence });
        }
        Ok((rule, ports.iter().map(Port::target).collect()))
    }
}
