impl Graph {
    pub(crate) fn validate_decoded_authority(&self) -> Result<(), Obstruction> {
        self.validate_context_consistency()?;
        let mut attached = vec![false; self.node_count()];
        for (route, node_id) in self.routes_raw() {
            let node = self
                .node(*node_id)
                .ok_or(Obstruction::UnknownNode { node: *node_id })?;
            if !node.opcode().is_executable() {
                continue;
            }
            let operation_ports = self
                .ports(*node_id)
                .ok_or(Obstruction::MalformedEvidence { node: *node_id })?;
            let inputs = operation_ports.iter().map(Port::target).collect::<Vec<_>>();
            let output_type = node
                .ty()
                .ok_or(Obstruction::ExpectedType { node: *node_id })?;
            let plan =
                self.validate_operation(node.context(), node.opcode(), &inputs, output_type)?;

            let judgment = self
                .resolve(&route.composed(JUDGMENT_LOCUS))
                .ok_or(Obstruction::MalformedEvidence { node: *node_id })?;
            let evidence = self
                .resolve(&route.composed(EVIDENCE_LOCUS))
                .ok_or(Obstruction::MalformedEvidence { node: *node_id })?;
            let judgment_node = self
                .node(judgment)
                .ok_or(Obstruction::MalformedEvidence { node: judgment })?;
            let judgment_ports = self
                .ports(judgment)
                .ok_or(Obstruction::MalformedEvidence { node: judgment })?;
            if judgment_node.opcode() != OpCode::TypeJudgment
                || judgment_node.payload() != node.opcode() as u64
                || judgment_ports.len() != inputs.len() + 2
                || judgment_ports[0].target() != *node_id
                || judgment_ports.last().map(Port::target) != Some(output_type)
                || judgment_ports[1..judgment_ports.len() - 1]
                    .iter()
                    .map(Port::target)
                    .ne(inputs.iter().copied())
            {
                return Err(Obstruction::MalformedEvidence { node: judgment });
            }
            let evidence_node = self
                .node(evidence)
                .ok_or(Obstruction::MalformedEvidence { node: evidence })?;
            let evidence_ports = self
                .ports(evidence)
                .ok_or(Obstruction::MalformedEvidence { node: evidence })?;
            let evidence_valid = match plan {
                EvidencePlan::Witness => {
                    evidence_node.opcode() == OpCode::KernelWitness && evidence_ports.is_empty()
                }
                EvidencePlan::Obligation { kind, subject } => {
                    evidence_node.opcode() == OpCode::Obligation
                        && ConstraintKind::from_payload(evidence_node.payload())
                            == Some((kind, true))
                        && evidence_ports.len() == 1
                        && evidence_ports[0].role() == PortRole::Premise
                        && evidence_ports[0].target() == subject
                }
            };
            if !evidence_valid || evidence_node.ty() != Some(judgment) {
                return Err(Obstruction::MalformedEvidence { node: evidence });
            }
            attached[judgment as usize] = true;
            attached[evidence as usize] = true;
        }
        for (index, node) in self.nodes_raw().iter().enumerate() {
            if matches!(
                node.opcode(),
                OpCode::TypeJudgment | OpCode::KernelWitness | OpCode::Obligation
            ) && !attached[index]
            {
                return Err(Obstruction::MalformedEvidence {
                    node: index as NodeId,
                });
            }
        }
        Ok(())
    }
}
