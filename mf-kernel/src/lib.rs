use mf_core::{
    Budget, Canonicalize, CheckProofShape, ComposeBridge, EvidenceMaturity, GateVerdict, Judgment,
    ObligationEvaluationMode, ObligationStatus, Promote, ProofShape, ProofShapeKind, QcObject,
    RegistryLookup, ValidateJudgment,
};
use thiserror::Error;

#[derive(Debug, Error, PartialEq, Eq)]
pub enum KernelError {
    #[error("{0}")]
    Message(String),
}

#[derive(Debug, Default)]
pub struct ConstitutionKernel;

impl ConstitutionKernel {
    pub fn validate_object(
        &self,
        object: &QcObject,
        registry: &dyn RegistryLookup,
    ) -> Result<(), KernelError> {
        if object.identity.cid.trim().is_empty() {
            return Err(KernelError::Message("QC-0 identity cid is empty".into()));
        }
        if object.structural.head.trim().is_empty() {
            return Err(KernelError::Message("QC-0 structural head is empty".into()));
        }
        if object.constraint.regime.trim().is_empty() {
            return Err(KernelError::Message(
                "QC-0 constraint regime is empty".into(),
            ));
        }
        if registry.get_regime(&object.constraint.regime).is_none() {
            return Err(KernelError::Message(format!(
                "unknown regime `{}`",
                object.constraint.regime
            )));
        }
        if object.constraint.equivalence.trim().is_empty() {
            return Err(KernelError::Message("QC-0 equivalence is empty".into()));
        }
        if object.evidence.gate_verdict == GateVerdict::Fail {
            return Err(KernelError::Message(
                "evidence gate is failing and object cannot validate".into(),
            ));
        }
        if object.alias.qa_binding.trim().is_empty() {
            return Err(KernelError::Message("QA-0 binding is empty".into()));
        }
        Ok(())
    }

    pub fn validate_regime(&self, regime: &mf_core::RegimePack) -> Result<(), KernelError> {
        let required = [
            regime.ctx_law.as_str(),
            regime.cut_law.as_str(),
            regime.thr_law.as_str(),
            regime.brc_law.as_str(),
            regime.slk_law.as_str(),
            regime.tol_law.as_str(),
            regime.knt_law.as_str(),
            regime.eq_law.as_str(),
            regime.adm_law.as_str(),
        ];
        if required.iter().any(|value| value.trim().is_empty()) {
            return Err(KernelError::Message(format!(
                "regime `{}` is missing at least one law slot",
                regime.id
            )));
        }
        Ok(())
    }

    pub fn validate_bridge(
        &self,
        bridge: &mf_core::BridgeContract,
        registry: &dyn RegistryLookup,
    ) -> Result<(), KernelError> {
        if registry.get_regime(&bridge.src).is_none() || registry.get_regime(&bridge.tgt).is_none()
        {
            return Err(KernelError::Message(format!(
                "bridge `{}` references an unknown regime",
                bridge.id
            )));
        }
        if bridge.id_pres.trim().is_empty() || bridge.eq_pres.trim().is_empty() {
            return Err(KernelError::Message(format!(
                "bridge `{}` is missing preservation clauses",
                bridge.id
            )));
        }
        if bridge.reversibility == mf_core::ReversibilityClass::Invalid {
            return Err(KernelError::Message(format!(
                "bridge `{}` is constitutionally invalid",
                bridge.id
            )));
        }
        Ok(())
    }

    pub fn validate_theorem_spec(
        &self,
        theorem: &mf_core::TheoremSpec,
        registry: &dyn RegistryLookup,
    ) -> Result<(), KernelError> {
        if theorem.statement.trim().is_empty() {
            return Err(KernelError::Message(format!(
                "theorem `{}` is missing a statement",
                theorem.id
            )));
        }
        if theorem.hosts.is_empty() {
            return Err(KernelError::Message(format!(
                "theorem `{}` has no host regimes",
                theorem.id
            )));
        }
        if theorem
            .hosts
            .iter()
            .any(|host| registry.get_regime(host).is_none())
        {
            return Err(KernelError::Message(format!(
                "theorem `{}` references an unknown host regime",
                theorem.id
            )));
        }
        if theorem.obligations.is_empty() {
            return Err(KernelError::Message(format!(
                "theorem `{}` has no obligation inventory",
                theorem.id
            )));
        }
        if theorem
            .proof_shapes
            .iter()
            .any(|shape| registry.get_proof_shape(shape).is_none())
        {
            return Err(KernelError::Message(format!(
                "theorem `{}` references an unknown proof shape",
                theorem.id
            )));
        }
        Ok(())
    }

    pub fn validate_target_profile(
        &self,
        target: &mf_core::TargetProfile,
    ) -> Result<(), KernelError> {
        if target.host_cluster.is_empty() {
            return Err(KernelError::Message(format!(
                "target profile `{}` has an empty host cluster",
                target.id
            )));
        }
        if target.target_equivalence.trim().is_empty() {
            return Err(KernelError::Message(format!(
                "target profile `{}` is missing target equivalence",
                target.id
            )));
        }
        Ok(())
    }

    pub fn validate_route_ledger(
        &self,
        ledger: &mf_core::RouteLedger,
        registry: &dyn RegistryLookup,
    ) -> Result<(), KernelError> {
        if registry.get_theorem_spec(&ledger.theorem).is_none() {
            return Err(KernelError::Message(format!(
                "route ledger `{}` references unknown theorem `{}`",
                ledger.id, ledger.theorem
            )));
        }
        let bridges = ledger
            .normalized_path
            .iter()
            .map(|id| {
                registry.get_bridge(id).ok_or_else(|| {
                    KernelError::Message(format!(
                        "route ledger `{}` references unknown bridge `{id}`",
                        ledger.id
                    ))
                })
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.compose_bridge_path(&bridges, Some(&ledger.budget))
            .map_err(KernelError::Message)
    }

    pub fn validate_certificate(
        &self,
        certificate: &mf_core::Certificate,
        registry: &dyn RegistryLookup,
    ) -> Result<(), KernelError> {
        if registry.get_theorem_spec(&certificate.theorem).is_none() {
            return Err(KernelError::Message(format!(
                "certificate `{}` references unknown theorem",
                certificate.id
            )));
        }
        if registry
            .get_route_ledger(&certificate.route_ledger)
            .is_none()
        {
            return Err(KernelError::Message(format!(
                "certificate `{}` references unknown route ledger",
                certificate.id
            )));
        }
        if certificate.receipts.is_empty() {
            return Err(KernelError::Message(format!(
                "certificate `{}` has no receipts",
                certificate.id
            )));
        }
        Ok(())
    }

    pub fn validate_campaign(
        &self,
        campaign: &mf_core::Campaign,
        registry: &dyn RegistryLookup,
    ) -> Result<(), KernelError> {
        if registry.get_theorem_spec(&campaign.theorem).is_none() {
            return Err(KernelError::Message(format!(
                "campaign `{}` references unknown theorem",
                campaign.id
            )));
        }
        if registry
            .get_target_profile(&campaign.target_profile)
            .is_none()
        {
            return Err(KernelError::Message(format!(
                "campaign `{}` references unknown target profile",
                campaign.id
            )));
        }
        if registry.get_route_ledger(&campaign.route_ledger).is_none() {
            return Err(KernelError::Message(format!(
                "campaign `{}` references unknown route ledger",
                campaign.id
            )));
        }
        for obligation in &campaign.obligations {
            if registry.get_obligation(obligation).is_none() {
                return Err(KernelError::Message(format!(
                    "campaign `{}` references unknown obligation `{obligation}`",
                    campaign.id
                )));
            }
        }
        Ok(())
    }

    pub fn validate_adequacy_clause(
        &self,
        clause: &mf_core::AdequacyClause,
        registry: &dyn RegistryLookup,
    ) -> Result<(), KernelError> {
        if clause.description.trim().is_empty() {
            return Err(KernelError::Message(format!(
                "adequacy clause `{}` is missing a description",
                clause.id
            )));
        }
        if clause.regime_ids.is_empty() {
            return Err(KernelError::Message(format!(
                "adequacy clause `{}` has no regime scope",
                clause.id
            )));
        }
        for regime in &clause.regime_ids {
            if registry.get_regime(regime).is_none() {
                return Err(KernelError::Message(format!(
                    "adequacy clause `{}` references unknown regime `{}`",
                    clause.id, regime
                )));
            }
        }
        for theorem_id in &clause.theorem_ids {
            if registry.get_theorem_spec(theorem_id).is_none() {
                return Err(KernelError::Message(format!(
                    "adequacy clause `{}` references unknown theorem `{}`",
                    clause.id, theorem_id
                )));
            }
        }
        for bridge_id in &clause.bridge_ids {
            if registry.get_bridge(bridge_id).is_none() {
                return Err(KernelError::Message(format!(
                    "adequacy clause `{}` references unknown bridge `{}`",
                    clause.id, bridge_id
                )));
            }
        }
        for burden_pack_id in &clause.burden_pack_ids {
            if registry.get_burden_pack(burden_pack_id).is_none() {
                return Err(KernelError::Message(format!(
                    "adequacy clause `{}` references unknown burden pack `{}`",
                    clause.id, burden_pack_id
                )));
            }
        }
        for claim_packet_id in &clause.claim_packet_ids {
            if registry.get_claim_packet(claim_packet_id).is_none() {
                return Err(KernelError::Message(format!(
                    "adequacy clause `{}` references unknown claim packet `{}`",
                    clause.id, claim_packet_id
                )));
            }
        }
        for contract_id in &clause.evidence_contract_ids {
            if registry.get_evidence_contract(contract_id).is_none() {
                return Err(KernelError::Message(format!(
                    "adequacy clause `{}` references unknown evidence contract `{}`",
                    clause.id, contract_id
                )));
            }
        }
        for receipt_id in &clause.benchmark_receipt_ids {
            if registry.get_benchmark_receipt(receipt_id).is_none() {
                return Err(KernelError::Message(format!(
                    "adequacy clause `{}` references unknown benchmark receipt `{}`",
                    clause.id, receipt_id
                )));
            }
        }
        for receipt_id in &clause.challenge_receipt_ids {
            if registry.get_challenge_receipt(receipt_id).is_none() {
                return Err(KernelError::Message(format!(
                    "adequacy clause `{}` references unknown challenge receipt `{}`",
                    clause.id, receipt_id
                )));
            }
        }
        for packet_id in &clause.reproducibility_packet_ids {
            if registry.get_reproducibility_packet(packet_id).is_none() {
                return Err(KernelError::Message(format!(
                    "adequacy clause `{}` references unknown reproducibility packet `{}`",
                    clause.id, packet_id
                )));
            }
        }
        if matches!(clause.kind, mf_core::AdequacyClauseKind::BridgeSoundness)
            && clause.bridge_ids.is_empty()
        {
            return Err(KernelError::Message(format!(
                "bridge soundness adequacy clause `{}` has no bridge scope",
                clause.id
            )));
        }
        if matches!(
            clause.kind,
            mf_core::AdequacyClauseKind::EvidenceContractInterpretation
                | mf_core::AdequacyClauseKind::BenchmarkInterpretation
                | mf_core::AdequacyClauseKind::StressInterpretation
                | mf_core::AdequacyClauseKind::ChallengeInterpretation
        ) && clause.claim_packet_ids.is_empty()
        {
            return Err(KernelError::Message(format!(
                "evidence adequacy clause `{}` has no claim packet scope",
                clause.id
            )));
        }
        Ok(())
    }

    pub fn validate_obligation_status(&self, status: &ObligationStatus) -> Result<(), KernelError> {
        if status.obligation_id.trim().is_empty() {
            return Err(KernelError::Message(
                "obligation status is missing an obligation id".into(),
            ));
        }
        if status.detail.trim().is_empty() {
            return Err(KernelError::Message(format!(
                "obligation `{}` is missing explanatory detail",
                status.obligation_id
            )));
        }
        let has_residual = status.receipts.iter().any(receipt_has_residual);
        match status.evaluation_mode {
            ObligationEvaluationMode::RecomputedExact => {
                if status.verdict != mf_core::CertificationVerdict::Certified {
                    return Err(KernelError::Message(format!(
                        "obligation `{}` claims exact recomputation without certification",
                        status.obligation_id
                    )));
                }
                if has_residual {
                    return Err(KernelError::Message(format!(
                        "obligation `{}` claims exact recomputation but still carries residual receipts",
                        status.obligation_id
                    )));
                }
            }
            ObligationEvaluationMode::RecomputedPartial => {
                if !has_residual {
                    return Err(KernelError::Message(format!(
                        "obligation `{}` claims partial recomputation without a residual receipt",
                        status.obligation_id
                    )));
                }
            }
            _ => {}
        }
        Ok(())
    }
}

fn receipt_has_residual(receipt: &mf_core::ObligationEvidenceReceipt) -> bool {
    if receipt.subreceipts.is_empty() {
        return !receipt.computed || receipt.verdict != mf_core::CertificationVerdict::Certified;
    }
    receipt.subreceipts.iter().any(receipt_has_residual)
}

impl Canonicalize for ConstitutionKernel {
    fn canonicalize_object(
        &self,
        object: &QcObject,
        registry: &dyn RegistryLookup,
    ) -> Result<QcObject, String> {
        self.validate_object(object, registry)
            .map_err(|err| err.to_string())?;
        if let Some(class) = registry.find_equivalence_class(&object.id, &object.constraint.regime)
        {
            return registry
                .get_object(&class.canonical_id)
                .ok_or_else(|| format!("missing canonical object `{}`", class.canonical_id));
        }
        Ok(object.clone())
    }
}

impl Promote for ConstitutionKernel {
    fn promote(&self, object: &QcObject, registry: &dyn RegistryLookup) -> Result<(), String> {
        self.canonicalize_object(object, registry)?;
        match object.evidence.maturity {
            EvidenceMaturity::Validated | EvidenceMaturity::Certified => {}
            _ => return Err("promotion requires validated or certified maturity".into()),
        }
        if object.evidence.gate_verdict != GateVerdict::Pass {
            return Err("promotion requires a passing evidence gate".into());
        }
        Ok(())
    }
}

impl ValidateJudgment for ConstitutionKernel {
    fn validate_judgment(
        &self,
        judgment: &Judgment,
        registry: &dyn RegistryLookup,
    ) -> Result<(), String> {
        match judgment {
            Judgment::Ctx { object }
            | Judgment::Cut { object }
            | Judgment::Thr { object }
            | Judgment::Brc { object }
            | Judgment::Knt { object }
            | Judgment::Wit { object }
            | Judgment::Prm { object } => {
                let object = registry
                    .get_object(object)
                    .ok_or_else(|| format!("missing object `{object}`"))?;
                self.validate_object(&object, registry)
                    .map_err(|err| err.to_string())
            }
            Judgment::Slk { left, right, .. } => {
                if registry.get_object(left).is_none() || registry.get_object(right).is_none() {
                    return Err("slack judgment references unknown object".into());
                }
                Ok(())
            }
            Judgment::Tol { object, .. } => {
                if registry.get_object(object).is_none() {
                    return Err("toll judgment references unknown object".into());
                }
                Ok(())
            }
            Judgment::Can { object, canonical } => {
                let object = registry
                    .get_object(object)
                    .ok_or_else(|| format!("missing object `{object}`"))?;
                let actual = self.canonicalize_object(&object, registry)?;
                if &actual.id != canonical {
                    return Err(format!(
                        "canonicalization mismatch: expected `{canonical}`, got `{}`",
                        actual.id
                    ));
                }
                Ok(())
            }
            Judgment::Brg { bridge } => self
                .validate_bridge(
                    &registry
                        .get_bridge(bridge)
                        .ok_or_else(|| format!("missing bridge `{bridge}`"))?,
                    registry,
                )
                .map_err(|err| err.to_string()),
            Judgment::Path { bridges } => {
                let path = bridges
                    .iter()
                    .map(|bridge_id| {
                        registry
                            .get_bridge(bridge_id)
                            .ok_or_else(|| format!("missing bridge `{bridge_id}`"))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                self.compose_bridge_path(&path, None)
            }
        }
    }
}

impl CheckProofShape for ConstitutionKernel {
    fn check_proof_shape(
        &self,
        shape: &ProofShape,
        _registry: &dyn RegistryLookup,
    ) -> Result<(), String> {
        let (min_nodes, min_edges) = match shape.kind {
            ProofShapeKind::Triangle => (3, 3),
            ProofShapeKind::Square => (4, 4),
            ProofShapeKind::Diamond => (4, 4),
            ProofShapeKind::Pentagon => (5, 5),
            ProofShapeKind::Hexagon => (6, 6),
        };
        if shape.nodes.len() < min_nodes {
            return Err(format!("proof shape `{}` has too few nodes", shape.id));
        }
        if shape.edges.len() < min_edges {
            return Err(format!("proof shape `{}` has too few edges", shape.id));
        }
        if shape.target_equivalence.trim().is_empty() {
            return Err(format!(
                "proof shape `{}` is missing target equivalence",
                shape.id
            ));
        }
        if shape.equations.is_empty() {
            return Err(format!("proof shape `{}` is missing equations", shape.id));
        }
        Ok(())
    }
}

impl ComposeBridge for ConstitutionKernel {
    fn compose_bridge_path(
        &self,
        bridges: &[mf_core::BridgeContract],
        budget: Option<&Budget>,
    ) -> Result<(), String> {
        if bridges.is_empty() {
            return Err("bridge path is empty".into());
        }
        for window in bridges.windows(2) {
            if window[0].tgt != window[1].src {
                return Err(format!(
                    "bridge chain mismatch: `{}` -> `{}` does not compose into `{}`",
                    window[0].id, window[0].tgt, window[1].src
                ));
            }
        }
        let total_loss = bridges
            .iter()
            .map(|bridge| bridge.loss.len())
            .sum::<usize>();
        if let Some(budget) = budget {
            if total_loss > budget.max_loss {
                return Err(format!(
                    "bridge path exceeds loss budget: {total_loss} > {}",
                    budget.max_loss
                ));
            }
            if !budget.allow_lossy_supported
                && bridges.iter().any(|bridge| {
                    bridge.reversibility == mf_core::ReversibilityClass::LossySupported
                })
            {
                return Err(
                    "bridge path uses lossy-supported transport under a strict budget".into(),
                );
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mf_core::{
        AliasFace, BridgeContract, ConstraintFace, EquivalenceClass, EvidenceFace, IdentityFace,
        ObjectTag, ProofEdge, RegimePack, ReversibilityClass, StructuralFace,
    };
    use mf_testkit::TestRegistryBuilder;

    fn sample_regime() -> RegimePack {
        RegimePack {
            id: "R_SET".into(),
            ctx_law: "ctx".into(),
            cut_law: "cut".into(),
            thr_law: "thr".into(),
            brc_law: "brc".into(),
            slk_law: "slk".into(),
            tol_law: "tol".into(),
            knt_law: "knt".into(),
            eq_law: "eq".into(),
            adm_law: "adm".into(),
            promoted_ops: vec!["canon".into()],
        }
    }

    fn sample_object(id: &str, maturity: EvidenceMaturity) -> QcObject {
        QcObject {
            id: id.into(),
            identity: IdentityFace {
                tag: ObjectTag::Ctx,
                cid: id.into(),
                codebook: "seed".into(),
                remap: "none".into(),
                lineage: "origin".into(),
            },
            structural: StructuralFace {
                head: "carrier".into(),
                args: vec!["x".into()],
                local_sections: vec!["base".into()],
                morphism_hooks: vec!["identity".into()],
            },
            constraint: ConstraintFace {
                regime: "R_SET".into(),
                contracts: vec!["total".into()],
                invariants: vec!["stable".into()],
                equivalence: "eq-set".into(),
                admissibility: "admit".into(),
            },
            evidence: EvidenceFace {
                evidence_class: "Seed".into(),
                traces: vec!["t".into()],
                receipts: vec!["r".into()],
                maturity,
                gate_verdict: GateVerdict::Pass,
            },
            alias: AliasFace {
                aliases: vec!["ctx".into()],
                profile_pack: vec!["STD".into()],
                qm_binding: "reserved".into(),
                qa_binding: "ctx".into(),
                projection_policy: "qa-first".into(),
            },
        }
    }

    #[test]
    fn canonicalization_collapses_equivalence_class() {
        let kernel = ConstitutionKernel;
        let object = sample_object("O1", EvidenceMaturity::Validated);
        let canonical = sample_object("O2", EvidenceMaturity::Certified);
        let registry = TestRegistryBuilder::new()
            .with_object(object.clone())
            .with_object(canonical.clone())
            .with_regime(sample_regime())
            .with_equivalence_class(EquivalenceClass {
                id: "EQ1".into(),
                regime: "R_SET".into(),
                canonical_id: "O2".into(),
                members: vec!["O1".into(), "O2".into()],
            })
            .build();

        let result = kernel.canonicalize_object(&object, &registry).unwrap();
        assert_eq!(result.id, "O2");
    }

    #[test]
    fn promotion_rejects_unvalidated_object() {
        let kernel = ConstitutionKernel;
        let object = sample_object("O1", EvidenceMaturity::Candidate);
        let registry = TestRegistryBuilder::new()
            .with_object(object.clone())
            .with_regime(sample_regime())
            .build();

        let err = kernel.promote(&object, &registry).unwrap_err();
        assert!(err.contains("validated"));
    }

    #[test]
    fn bridge_path_rejects_chain_mismatch() {
        let kernel = ConstitutionKernel;
        let left = BridgeContract {
            id: "B1".into(),
            src: "R_TYP".into(),
            tgt: "R_SET".into(),
            id_pres: "carrier".into(),
            eq_pres: "eq".into(),
            forget: vec![],
            enrich: vec![],
            loss: vec!["proof".into()],
            reversibility: ReversibilityClass::Conservative,
            receipts: vec![],
            rollback: "allowed".into(),
        };
        let right = BridgeContract {
            id: "B2".into(),
            src: "R_PROB".into(),
            tgt: "R_LOG".into(),
            id_pres: "carrier".into(),
            eq_pres: "eq".into(),
            forget: vec![],
            enrich: vec![],
            loss: vec![],
            reversibility: ReversibilityClass::Conservative,
            receipts: vec![],
            rollback: "allowed".into(),
        };

        let err = kernel
            .compose_bridge_path(&[left, right], None)
            .unwrap_err();
        assert!(err.contains("does not compose"));
    }

    #[test]
    fn proof_shape_requires_expected_arity() {
        let kernel = ConstitutionKernel;
        let shape = ProofShape {
            id: "P1".into(),
            kind: ProofShapeKind::Triangle,
            nodes: vec!["a".into(), "b".into()],
            edges: vec![ProofEdge {
                from: "a".into(),
                to: "b".into(),
                label: "f".into(),
            }],
            equations: vec!["f=g".into()],
            target_equivalence: "eq".into(),
            receipts: vec!["r".into()],
            gate: GateVerdict::Pass,
        };

        let registry = TestRegistryBuilder::new().build();
        let err = kernel.check_proof_shape(&shape, &registry).unwrap_err();
        assert!(err.contains("too few nodes"));
    }
}
