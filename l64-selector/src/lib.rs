use l64_core::{
    AtlasCell, Budget, CertificationCandidate, CertificationReport, CertificationVerdict,
    ComposeBridge, ObligationStatus, RegistryLookup, RequiredProofShapeFamily, RouteSelection,
    SelectRoute, WinnerState,
};
use l64_kernel::ConstitutionKernel;

#[derive(Debug, Clone)]
pub struct AtlasSelector<R> {
    registry: R,
}

impl<R> AtlasSelector<R> {
    pub fn new(registry: R) -> Self {
        Self { registry }
    }

    pub fn registry(&self) -> &R {
        &self.registry
    }
}

impl<R> SelectRoute for AtlasSelector<R>
where
    R: l64_core::RegistryLookup,
{
    fn select_route(
        &self,
        src: &str,
        tgt: &str,
        proof_target: Option<&str>,
        budget: Option<&Budget>,
    ) -> Result<RouteSelection, String> {
        let mut candidates = self
            .registry
            .atlas_cells()
            .into_iter()
            .filter(|cell| cell.source_regime == src && cell.target_regime == tgt)
            .filter(|cell| {
                proof_target
                    .map(|target| cell.proof_target == target)
                    .unwrap_or(true)
            })
            .filter(|cell| {
                budget
                    .map(|budget| {
                        cell.loss_profile.items.len() <= budget.max_loss
                            && (budget.allow_lossy_supported || cell.loss_profile.items.is_empty())
                    })
                    .unwrap_or(true)
            })
            .collect::<Vec<_>>();

        candidates.sort_by(|left, right| left.id.cmp(&right.id));

        let winner = candidates.iter().find(|cell| {
            cell.winner_state == WinnerState::SeedWinner
                && cell.candidate_paths.contains(&cell.normalized_winner)
        });

        let reasons = if let Some(winner) = winner {
            vec![
                format!("selected atlas cell `{}`", winner.id),
                format!("winner path `{}`", winner.normalized_winner.join(" -> ")),
            ]
        } else if candidates.is_empty() {
            vec!["no atlas cell matched the request".into()]
        } else {
            vec!["candidate cells exist but none declare a seed winner".into()]
        };

        Ok(RouteSelection {
            candidates: candidates.clone(),
            winner: winner.cloned(),
            reasons,
            route_explanation: None,
        })
    }
}

#[derive(Debug)]
pub struct CampaignCertifier<R> {
    registry: R,
    kernel: ConstitutionKernel,
}

impl<R> CampaignCertifier<R> {
    pub fn new(registry: R) -> Self {
        Self {
            registry,
            kernel: ConstitutionKernel,
        }
    }
}

impl<R> CampaignCertifier<R>
where
    R: RegistryLookup,
{
    pub fn certify_campaign(&self, campaign_id: &str) -> Result<CertificationReport, String> {
        let campaign = self
            .registry
            .get_campaign(campaign_id)
            .ok_or_else(|| format!("unknown campaign `{campaign_id}`"))?;
        let target = self
            .registry
            .get_target_profile(&campaign.target_profile)
            .ok_or_else(|| format!("unknown target profile `{}`", campaign.target_profile))?;
        self.certify_theorem_with_target(&campaign.theorem, &target.id, Some(campaign.id.as_str()))
    }

    pub fn certify_theorem_with_target(
        &self,
        theorem_id: &str,
        target_profile_id: &str,
        campaign_id: Option<&str>,
    ) -> Result<CertificationReport, String> {
        let theorem = self
            .registry
            .get_theorem_spec(theorem_id)
            .ok_or_else(|| format!("unknown theorem `{theorem_id}`"))?;
        let target = self
            .registry
            .get_target_profile(target_profile_id)
            .ok_or_else(|| format!("unknown target profile `{target_profile_id}`"))?;

        self.kernel
            .validate_theorem_spec(&theorem, &self.registry)
            .map_err(|err| err.to_string())?;
        self.kernel
            .validate_target_profile(&target)
            .map_err(|err| err.to_string())?;

        let src = target
            .host_cluster
            .first()
            .ok_or_else(|| format!("target profile `{}` has empty host cluster", target.id))?;
        let tgt = target
            .host_cluster
            .last()
            .ok_or_else(|| format!("target profile `{}` has empty host cluster", target.id))?;

        let atlas_candidates = self
            .registry
            .atlas_cells()
            .into_iter()
            .filter(|cell| cell.source_regime == *src && cell.target_regime == *tgt)
            .filter(|cell| cell.burden_class == target.burden_class)
            .collect::<Vec<_>>();

        let mut candidates = atlas_candidates
            .into_iter()
            .filter_map(|cell| self.score_candidate(&cell, &theorem, &target).ok())
            .collect::<Vec<_>>();
        candidates.sort_by(|left, right| left.score.cmp(&right.score));

        let selected = candidates.first().cloned();
        let route_class = self
            .registry
            .get_route_class(&format!("RTC_{}", theorem.id.trim_start_matches("THS_")));
        let certificate = self
            .registry
            .get_certificate(&format!("CRT_{}", theorem.id.trim_start_matches("THS_")));

        let obligation_statuses = self
            .registry
            .get_campaign(campaign_id.unwrap_or_default())
            .map(|campaign| {
                campaign
                    .obligations
                    .into_iter()
                    .filter_map(|id| self.registry.get_obligation(&id))
                    .map(|obligation| ObligationStatus {
                        obligation_id: obligation.id,
                        kind: obligation.kind,
                        verdict: obligation.status,
                        evaluation_mode: l64_core::ObligationEvaluationMode::StoredReceiptUsed,
                        detail: "stored campaign obligation".into(),
                        receipts: Vec::new(),
                    })
                    .collect::<Vec<_>>()
            })
            .unwrap_or_default();

        let mut reasons = Vec::new();
        let mut diagnostics = Vec::new();
        let verdict = match (&selected, certificate.as_ref()) {
            (None, _) => {
                diagnostics.push("no lawful atlas candidate satisfied the theorem target".into());
                CertificationVerdict::BlockedOpen
            }
            (Some(candidate), Some(certificate))
                if certificate.verdict == CertificationVerdict::Certified
                    || certificate.verdict == CertificationVerdict::Integrated =>
            {
                reasons.push(format!(
                    "certificate `{}` discharges theorem `{}`",
                    certificate.id, theorem.id
                ));
                reasons.push(format!(
                    "selected normalized path `{}`",
                    candidate.path.join(" -> ")
                ));
                certificate.verdict.clone()
            }
            (Some(candidate), Some(certificate)) => {
                reasons.push(format!(
                    "certificate `{}` exists with verdict {:?}",
                    certificate.id, certificate.verdict
                ));
                reasons.push(format!(
                    "selected normalized path `{}`",
                    candidate.path.join(" -> ")
                ));
                certificate.verdict.clone()
            }
            (Some(candidate), None) => {
                reasons.push(format!("selected atlas cell `{}`", candidate.atlas_cell_id));
                reasons.push(format!(
                    "selected normalized path `{}`",
                    candidate.path.join(" -> ")
                ));
                CertificationVerdict::RouteFound
            }
        };

        Ok(CertificationReport {
            theorem_id: theorem.id,
            campaign_id: campaign_id.map(ToString::to_string),
            target_profile_id: target.id,
            verdict,
            selected_atlas_cell: selected.as_ref().map(|item| item.atlas_cell_id.clone()),
            selected_path: selected
                .as_ref()
                .map(|item| item.path.clone())
                .unwrap_or_default(),
            route_class_id: route_class.as_ref().map(|item| item.id.clone()),
            certificate_id: certificate.as_ref().map(|item| item.id.clone()),
            candidates,
            obligations: obligation_statuses,
            reasons,
            diagnostics,
            deficiencies: Vec::new(),
            adequacy_records: Vec::new(),
            checker_receipts: Vec::new(),
            burden_pack_ids: Vec::new(),
            claim_packet_ids: Vec::new(),
            evidence_contract_ids: Vec::new(),
            benchmark_receipt_ids: Vec::new(),
            challenge_receipt_ids: Vec::new(),
            reproducibility_packet_ids: Vec::new(),
            promotion_artifact_ids: Vec::new(),
            reused_artifact_ids: Vec::new(),
            default_selected_artifact_ids: Vec::new(),
            payoff_receipt_ids: Vec::new(),
            policy_resolution: None,
            route_explanation: None,
            execution_envelope: None,
            reconciliation_summary: Vec::new(),
            obligation_plan: None,
            obligation_lanes: Vec::new(),
            obligation_ordering_receipt: None,
            obligation_merge_receipt: None,
            replay_legality_checks: Vec::new(),
            replay_barrier_receipts: Vec::new(),
            replay_merge_receipt: None,
            replay_divergence_records: Vec::new(),
            obligation_cache_shards: Vec::new(),
            reuse_legality_receipts: Vec::new(),
            reuse_decision_receipts: Vec::new(),
            residual_verification_receipts: Vec::new(),
            obligation_write_sets: Vec::new(),
            obligation_collision_reports: Vec::new(),
            obligation_namespace_receipt: None,
        })
    }

    fn score_candidate(
        &self,
        cell: &AtlasCell,
        theorem: &l64_core::TheoremSpec,
        target: &l64_core::TargetProfile,
    ) -> Result<CertificationCandidate, String> {
        let path = cell.normalized_winner.clone();
        let bridges = path
            .iter()
            .map(|id| {
                self.registry
                    .get_bridge(id)
                    .ok_or_else(|| format!("unknown bridge `{id}`"))
            })
            .collect::<Result<Vec<_>, _>>()?;

        if bridges.iter().any(|bridge| {
            !target
                .allowed_bridge_classes
                .contains(&bridge.reversibility)
        }) {
            return Err("bridge class outside target profile".into());
        }

        let budget = Budget {
            max_loss: target.loss_ceiling,
            allow_lossy_supported: target
                .allowed_bridge_classes
                .contains(&l64_core::ReversibilityClass::LossySupported),
            require_proof: true,
        };

        self.kernel.compose_bridge_path(&bridges, Some(&budget))?;

        let proof_family_penalty = proof_family_penalty(
            &target.required_proof_shape_family,
            &cell.proof_shapes_checked,
            &self.registry,
        );

        let route_class_id = self
            .registry
            .get_route_class(&format!("RTC_{}", theorem.id.trim_start_matches("THS_")))
            .and_then(|route_class| {
                if route_class.canonical_path == path {
                    Some(route_class.id)
                } else {
                    None
                }
            });

        Ok(CertificationCandidate {
            atlas_cell_id: cell.id.clone(),
            path: path.clone(),
            loss_count: cell.loss_profile.items.len(),
            proof_shapes: cell.proof_shapes_checked.clone(),
            route_class_id,
            score: vec![
                usize::from(cell.winner_state != WinnerState::SeedWinner),
                cell.loss_profile.items.len(),
                bridges.len(),
                proof_family_penalty,
            ],
            route_score: None,
        })
    }
}

fn proof_family_penalty<R: RegistryLookup>(
    family: &RequiredProofShapeFamily,
    proof_shapes: &[String],
    registry: &R,
) -> usize {
    match family {
        RequiredProofShapeFamily::Minimal => usize::from(proof_shapes.is_empty()),
        RequiredProofShapeFamily::MixedBattery => usize::from(proof_shapes.is_empty()),
        RequiredProofShapeFamily::Triangle => usize::from(!proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Triangle)
                .unwrap_or(false)
        })),
        RequiredProofShapeFamily::Square => usize::from(!proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Square)
                .unwrap_or(false)
        })),
        RequiredProofShapeFamily::Diamond => usize::from(!proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Diamond)
                .unwrap_or(false)
        })),
        RequiredProofShapeFamily::Pentagon => usize::from(!proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Pentagon)
                .unwrap_or(false)
        })),
        RequiredProofShapeFamily::Hexagon => usize::from(!proof_shapes.iter().any(|id| {
            registry
                .get_proof_shape(id)
                .map(|shape| shape.kind == l64_core::ProofShapeKind::Hexagon)
                .unwrap_or(false)
        })),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use l64_core::{
        BurdenClass, Campaign, CampaignClass, Certificate, CertificationVerdict, FailureSignature,
        LossProfile, Obligation, ObligationKind, RecipeMaturity, RegistryLookup, RouteClass,
        RouteLedger, TargetProfile, TheoremSpec, WinnerState,
    };

    #[derive(Clone)]
    struct TestRegistry {
        cells: Vec<l64_core::AtlasCell>,
        regimes: Vec<l64_core::RegimePack>,
        proof_shapes: Vec<l64_core::ProofShape>,
        bridges: Vec<l64_core::BridgeContract>,
        theorem_specs: Vec<TheoremSpec>,
        obligations: Vec<Obligation>,
        target_profiles: Vec<TargetProfile>,
        route_ledgers: Vec<RouteLedger>,
        certificates: Vec<Certificate>,
        campaigns: Vec<Campaign>,
        route_classes: Vec<RouteClass>,
    }

    impl RegistryLookup for TestRegistry {
        fn get_object(&self, _id: &str) -> Option<l64_core::QcObject> {
            None
        }
        fn get_regime(&self, id: &str) -> Option<l64_core::RegimePack> {
            self.regimes.iter().find(|item| item.id == id).cloned()
        }
        fn get_bridge(&self, id: &str) -> Option<l64_core::BridgeContract> {
            self.bridges.iter().find(|item| item.id == id).cloned()
        }
        fn get_proof_shape(&self, id: &str) -> Option<l64_core::ProofShape> {
            self.proof_shapes.iter().find(|item| item.id == id).cloned()
        }
        fn get_atlas_cell(&self, _id: &str) -> Option<l64_core::AtlasCell> {
            None
        }
        fn get_mechanization_package(&self, _id: &str) -> Option<l64_core::MechanizationPackage> {
            None
        }
        fn get_theorem_spec(&self, id: &str) -> Option<l64_core::TheoremSpec> {
            self.theorem_specs
                .iter()
                .find(|item| item.id == id)
                .cloned()
        }
        fn get_obligation(&self, id: &str) -> Option<l64_core::Obligation> {
            self.obligations.iter().find(|item| item.id == id).cloned()
        }
        fn get_target_profile(&self, id: &str) -> Option<l64_core::TargetProfile> {
            self.target_profiles
                .iter()
                .find(|item| item.id == id)
                .cloned()
        }
        fn get_route_ledger(&self, id: &str) -> Option<l64_core::RouteLedger> {
            self.route_ledgers
                .iter()
                .find(|item| item.id == id)
                .cloned()
        }
        fn get_certificate(&self, id: &str) -> Option<l64_core::Certificate> {
            self.certificates.iter().find(|item| item.id == id).cloned()
        }
        fn get_campaign(&self, id: &str) -> Option<l64_core::Campaign> {
            self.campaigns.iter().find(|item| item.id == id).cloned()
        }
        fn get_campaign_portfolio(&self, _id: &str) -> Option<l64_core::CampaignPortfolio> {
            None
        }
        fn get_route_class(&self, id: &str) -> Option<l64_core::RouteClass> {
            self.route_classes
                .iter()
                .find(|item| item.id == id)
                .cloned()
        }
        fn get_atlas_deficiency(&self, _id: &str) -> Option<l64_core::AtlasDeficiency> {
            None
        }
        fn atlas_deficiencies(&self) -> Vec<l64_core::AtlasDeficiency> {
            Vec::new()
        }
        fn get_adequacy_clause(&self, _id: &str) -> Option<l64_core::AdequacyClause> {
            None
        }
        fn adequacy_clauses(&self) -> Vec<l64_core::AdequacyClause> {
            Vec::new()
        }
        fn get_codebook_pack(&self, _id: &str) -> Option<l64_core::CodebookPack> {
            None
        }
        fn get_glyph_pack(&self, _id: &str) -> Option<l64_core::GlyphPack> {
            None
        }
        fn get_combo_pack(&self, _id: &str) -> Option<l64_core::ComboPack> {
            None
        }
        fn get_projection_policy(&self, _id: &str) -> Option<l64_core::ProjectionPolicy> {
            None
        }
        fn get_alias_expansion_policy(&self, _id: &str) -> Option<l64_core::AliasExpansionPolicy> {
            None
        }
        fn get_surface_policy(&self, _id: &str) -> Option<l64_core::SurfacePolicy> {
            None
        }
        fn get_capability_matrix(&self, _id: &str) -> Option<l64_core::CapabilityMatrix> {
            None
        }
        fn get_roundtrip_report(&self, _id: &str) -> Option<l64_core::RoundTripReport> {
            None
        }
        fn get_transform_receipt(&self, _id: &str) -> Option<l64_core::FormatTransformReceipt> {
            None
        }
        fn get_surface_deficiency(&self, _id: &str) -> Option<l64_core::SurfaceDeficiency> {
            None
        }
        fn get_policy_object(&self, _id: &str) -> Option<l64_core::MechanizationPolicyObject> {
            None
        }
        fn policy_objects(&self) -> Vec<l64_core::MechanizationPolicyObject> {
            Vec::new()
        }
        fn policy_bindings(&self) -> Vec<l64_core::PolicyBinding> {
            Vec::new()
        }
        fn find_equivalence_class(
            &self,
            _object_id: &str,
            _regime: &str,
        ) -> Option<l64_core::EquivalenceClass> {
            None
        }
        fn atlas_cells(&self) -> Vec<l64_core::AtlasCell> {
            self.cells.clone()
        }
    }

    #[test]
    fn selector_prefers_seed_winner() {
        let registry = TestRegistry {
            cells: vec![l64_core::AtlasCell {
                id: "A1".into(),
                source_regime: "R_TYP".into(),
                target_regime: "R_SET".into(),
                burden_class: BurdenClass::ExtensionalCarrierReasoning,
                proof_target: "carrier".into(),
                candidate_paths: vec![vec!["B1".into()]],
                normalized_winner: vec!["B1".into()],
                winner_state: WinnerState::SeedWinner,
                loss_profile: LossProfile { items: vec![] },
                proof_shapes_checked: vec![],
                recipe_maturity: RecipeMaturity::Seeded,
                failure_signatures: vec![FailureSignature {
                    code: "OK".into(),
                    message: "ok".into(),
                }],
                side_conditions: vec![],
                surface_transition: None,
            }],
            regimes: vec![],
            proof_shapes: vec![],
            bridges: vec![],
            theorem_specs: vec![],
            obligations: vec![],
            target_profiles: vec![],
            route_ledgers: vec![],
            certificates: vec![],
            campaigns: vec![],
            route_classes: vec![],
        };
        let selector = AtlasSelector::new(registry);
        let result = selector
            .select_route("R_TYP", "R_SET", Some("carrier"), None)
            .unwrap();
        assert_eq!(result.winner.unwrap().id, "A1");
    }

    #[test]
    fn certifier_returns_benchmarked_campaign_report() {
        let registry = TestRegistry {
            cells: vec![l64_core::AtlasCell {
                id: "A_TOP_TO_CALC".into(),
                source_regime: "R_TOP".into(),
                target_regime: "R_CALC".into(),
                burden_class: BurdenClass::DerivativeLocalWitnessExtraction,
                proof_target: "derivative witness extraction".into(),
                candidate_paths: vec![vec!["B_TOP_TO_CALC".into()]],
                normalized_winner: vec!["B_TOP_TO_CALC".into()],
                winner_state: WinnerState::SeedWinner,
                loss_profile: LossProfile { items: vec![] },
                proof_shapes_checked: vec!["PS_SQUARE".into()],
                recipe_maturity: RecipeMaturity::Seeded,
                failure_signatures: vec![],
                side_conditions: vec![],
                surface_transition: None,
            }],
            regimes: vec![
                l64_core::RegimePack {
                    id: "R_TOP".into(),
                    ctx_law: "ctx".into(),
                    cut_law: "cut".into(),
                    thr_law: "thr".into(),
                    brc_law: "brc".into(),
                    slk_law: "slk".into(),
                    tol_law: "tol".into(),
                    knt_law: "knt".into(),
                    eq_law: "eq".into(),
                    adm_law: "adm".into(),
                    promoted_ops: vec![],
                },
                l64_core::RegimePack {
                    id: "R_CALC".into(),
                    ctx_law: "ctx".into(),
                    cut_law: "cut".into(),
                    thr_law: "thr".into(),
                    brc_law: "brc".into(),
                    slk_law: "slk".into(),
                    tol_law: "tol".into(),
                    knt_law: "knt".into(),
                    eq_law: "eq".into(),
                    adm_law: "adm".into(),
                    promoted_ops: vec![],
                },
            ],
            proof_shapes: vec![l64_core::ProofShape {
                id: "PS_SQUARE".into(),
                kind: l64_core::ProofShapeKind::Square,
                nodes: vec!["a".into(), "b".into(), "c".into(), "d".into()],
                edges: vec![
                    l64_core::ProofEdge {
                        from: "a".into(),
                        to: "b".into(),
                        label: "f".into(),
                    },
                    l64_core::ProofEdge {
                        from: "b".into(),
                        to: "d".into(),
                        label: "g".into(),
                    },
                    l64_core::ProofEdge {
                        from: "a".into(),
                        to: "c".into(),
                        label: "h".into(),
                    },
                    l64_core::ProofEdge {
                        from: "c".into(),
                        to: "d".into(),
                        label: "i".into(),
                    },
                ],
                equations: vec!["g∘f=i∘h".into()],
                target_equivalence: "eq".into(),
                receipts: vec!["r".into()],
                gate: l64_core::GateVerdict::Pass,
            }],
            bridges: vec![l64_core::BridgeContract {
                id: "B_TOP_TO_CALC".into(),
                src: "R_TOP".into(),
                tgt: "R_CALC".into(),
                id_pres: "pres".into(),
                eq_pres: "eq".into(),
                forget: vec![],
                enrich: vec!["der".into()],
                loss: vec![],
                reversibility: l64_core::ReversibilityClass::Enriching,
                receipts: vec!["Ref_1".into()],
                rollback: "allowed".into(),
            }],
            theorem_specs: vec![TheoremSpec {
                id: "THS_CHAIN_RULE".into(),
                statement: "chain".into(),
                hosts: vec!["R_TOP".into(), "R_CALC".into()],
                bridges: vec!["B_TOP_TO_CALC".into()],
                operators: vec!["OPR.Chain1".into()],
                target_equivalence: "eq".into(),
                obligations: vec![ObligationKind::OblEq],
                primary_zone: l64_core::ProofMechanismZone::PmzStructural,
                verdict: CertificationVerdict::Benchmarked,
                proof_shapes: vec!["PS_SQUARE".into()],
            }],
            obligations: vec![Obligation {
                id: "OBL_CHAIN_EQ".into(),
                kind: ObligationKind::OblEq,
                description: "eq".into(),
                status: CertificationVerdict::Benchmarked,
            }],
            target_profiles: vec![TargetProfile {
                id: "TGT_CHAIN_RULE".into(),
                burden_class: BurdenClass::DerivativeLocalWitnessExtraction,
                host_cluster: vec!["R_TOP".into(), "R_CALC".into()],
                target_equivalence: "eq".into(),
                allowed_bridge_classes: vec![l64_core::ReversibilityClass::Enriching],
                loss_ceiling: 1,
                rollback_ceiling: 1,
                required_receipt_class: "RC".into(),
                required_proof_shape_family: l64_core::RequiredProofShapeFamily::Square,
                promotion_goal: l64_core::PromotionGoal::PromoteOperator,
                primary_zone: l64_core::ProofMechanismZone::PmzStructural,
                surface_requirement: None,
                preferred_surface_target: None,
                optimizer_policy: None,
                policy_binding_ids: vec![],
            }],
            route_ledgers: vec![RouteLedger {
                id: "TRL_CHAIN_RULE".into(),
                theorem: "THS_CHAIN_RULE".into(),
                paths: vec![vec!["B_TOP_TO_CALC".into()]],
                budget: l64_core::Budget {
                    max_loss: 1,
                    allow_lossy_supported: false,
                    require_proof: true,
                },
                losses: vec![],
                receipts: vec!["Ref_1".into()],
                normalized_path: vec!["B_TOP_TO_CALC".into()],
            }],
            certificates: vec![Certificate {
                id: "CRT_CHAIN_RULE".into(),
                theorem: "THS_CHAIN_RULE".into(),
                route_ledger: "TRL_CHAIN_RULE".into(),
                proof_shapes: vec!["PS_SQUARE".into()],
                receipts: vec!["RC".into()],
                verdict: CertificationVerdict::Benchmarked,
            }],
            campaigns: vec![Campaign {
                id: "CPG_CHAIN_RULE".into(),
                theorem: "THS_CHAIN_RULE".into(),
                target_profile: "TGT_CHAIN_RULE".into(),
                route_ledger: "TRL_CHAIN_RULE".into(),
                obligations: vec!["OBL_CHAIN_EQ".into()],
                certificates: vec!["CRT_CHAIN_RULE".into()],
                dependencies: vec![],
                campaign_class: CampaignClass::COperator,
                verdict: CertificationVerdict::Benchmarked,
                payoff: vec!["OPR.Chain1".into()],
            }],
            route_classes: vec![RouteClass {
                id: "RTC_CHAIN_RULE".into(),
                theorem: "THS_CHAIN_RULE".into(),
                target_profile: "TGT_CHAIN_RULE".into(),
                equivalent_paths: vec![vec!["B_TOP_TO_CALC".into()]],
                canonical_path: vec!["B_TOP_TO_CALC".into()],
                verdict: CertificationVerdict::Benchmarked,
            }],
        };
        let certifier = CampaignCertifier::new(registry);
        let report = certifier.certify_campaign("CPG_CHAIN_RULE").unwrap();
        assert_eq!(report.verdict, CertificationVerdict::Benchmarked);
        assert_eq!(report.selected_atlas_cell.as_deref(), Some("A_TOP_TO_CALC"));
    }
}
