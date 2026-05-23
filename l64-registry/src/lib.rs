use l64_core::{
    AdequacyClause, AliasExpansionPolicy, ArtifactOrigin, AtlasCell, AtlasDeficiency,
    BenchmarkReceipt, BridgeContract, BurdenPack, Campaign, CampaignPortfolio, CapabilityMatrix,
    Certificate, ChallengeReceipt, ClaimPacket, CodebookPack, ComboPack, EquivalenceClass,
    EvidenceContract, FormatTransformReceipt, GlyphPack, MechanizationPackage,
    MechanizationPolicyObject, Obligation, PolicyBinding, ProjectionPolicy, ProofShape, QcObject,
    RegimePack, RegistryBundle, RegistryLookup, ReproducibilityPacket, RoundTripReport, RouteClass,
    RouteLedger, SurfaceDeficiency, SurfacePolicy, TargetProfile, TheoremSpec,
};
use std::collections::HashMap;
use thiserror::Error;

const SEED_DATA: &str = include_str!("../data/seed.v1.json");

#[derive(Debug, Error)]
pub enum RegistryError {
    #[error("invalid registry seed: {0}")]
    InvalidSeed(String),
}

#[derive(Debug, Clone)]
pub struct SeedRegistry {
    bundle: RegistryBundle,
    object_index: HashMap<String, usize>,
    regime_index: HashMap<String, usize>,
    bridge_index: HashMap<String, usize>,
    proof_shape_index: HashMap<String, usize>,
    atlas_cell_index: HashMap<String, usize>,
    mechanization_index: HashMap<String, usize>,
    theorem_index: HashMap<String, usize>,
    obligation_index: HashMap<String, usize>,
    target_profile_index: HashMap<String, usize>,
    route_ledger_index: HashMap<String, usize>,
    certificate_index: HashMap<String, usize>,
    campaign_index: HashMap<String, usize>,
    portfolio_index: HashMap<String, usize>,
    route_class_index: HashMap<String, usize>,
    atlas_deficiency_index: HashMap<String, usize>,
    adequacy_clause_index: HashMap<String, usize>,
    burden_pack_index: HashMap<String, usize>,
    claim_packet_index: HashMap<String, usize>,
    evidence_contract_index: HashMap<String, usize>,
    benchmark_receipt_index: HashMap<String, usize>,
    challenge_receipt_index: HashMap<String, usize>,
    reproducibility_packet_index: HashMap<String, usize>,
    codebook_pack_index: HashMap<String, usize>,
    glyph_pack_index: HashMap<String, usize>,
    combo_pack_index: HashMap<String, usize>,
    projection_policy_index: HashMap<String, usize>,
    alias_policy_index: HashMap<String, usize>,
    surface_policy_index: HashMap<String, usize>,
    capability_index: HashMap<String, usize>,
    roundtrip_index: HashMap<String, usize>,
    receipt_index: HashMap<String, usize>,
    surface_deficiency_index: HashMap<String, usize>,
    policy_object_index: HashMap<String, usize>,
}

impl SeedRegistry {
    pub fn load() -> Result<Self, RegistryError> {
        let bundle: RegistryBundle = serde_json::from_str(SEED_DATA)
            .map_err(|err| RegistryError::InvalidSeed(err.to_string()))?;
        Ok(Self {
            object_index: build_index(&bundle.objects, |item| item.id.as_str()),
            regime_index: build_index(&bundle.regimes, |item| item.id.as_str()),
            bridge_index: build_index(&bundle.bridges, |item| item.id.as_str()),
            proof_shape_index: build_index(&bundle.proof_shapes, |item| item.id.as_str()),
            atlas_cell_index: build_index(&bundle.atlas_cells, |item| item.id.as_str()),
            mechanization_index: build_index(&bundle.mechanization_packages, |item| {
                item.id.as_str()
            }),
            theorem_index: build_index(&bundle.theorem_specs, |item| item.id.as_str()),
            obligation_index: build_index(&bundle.obligations, |item| item.id.as_str()),
            target_profile_index: build_index(&bundle.target_profiles, |item| item.id.as_str()),
            route_ledger_index: build_index(&bundle.route_ledgers, |item| item.id.as_str()),
            certificate_index: build_index(&bundle.certificates, |item| item.id.as_str()),
            campaign_index: build_index(&bundle.campaigns, |item| item.id.as_str()),
            portfolio_index: build_index(&bundle.campaign_portfolios, |item| item.id.as_str()),
            route_class_index: build_index(&bundle.route_classes, |item| item.id.as_str()),
            atlas_deficiency_index: build_index(&bundle.atlas_deficiencies, |item| {
                item.id.as_str()
            }),
            adequacy_clause_index: build_index(&bundle.adequacy_clauses, |item| item.id.as_str()),
            burden_pack_index: build_index(&bundle.burden_packs, |item| item.id.as_str()),
            claim_packet_index: build_index(&bundle.claim_packets, |item| item.id.as_str()),
            evidence_contract_index: build_index(&bundle.evidence_contracts, |item| {
                item.id.as_str()
            }),
            benchmark_receipt_index: build_index(&bundle.benchmark_receipts, |item| {
                item.id.as_str()
            }),
            challenge_receipt_index: build_index(&bundle.challenge_receipts, |item| {
                item.id.as_str()
            }),
            reproducibility_packet_index: build_index(&bundle.reproducibility_packets, |item| {
                item.id.as_str()
            }),
            codebook_pack_index: build_index(&bundle.codebook_packs, |item| item.id.as_str()),
            glyph_pack_index: build_index(&bundle.glyph_packs, |item| item.id.as_str()),
            combo_pack_index: build_index(&bundle.combo_packs, |item| item.id.as_str()),
            projection_policy_index: build_index(&bundle.projection_policies, |item| {
                item.id.as_str()
            }),
            alias_policy_index: build_index(&bundle.alias_expansion_policies, |item| {
                item.id.as_str()
            }),
            surface_policy_index: build_index(&bundle.surface_policies, |item| item.id.as_str()),
            capability_index: build_index(&bundle.capability_matrices, |item| item.id.as_str()),
            roundtrip_index: build_index(&bundle.roundtrip_reports, |item| item.id.as_str()),
            receipt_index: build_index(&bundle.transform_receipts, |item| item.id.as_str()),
            surface_deficiency_index: build_index(&bundle.surface_deficiencies, |item| {
                item.id.as_str()
            }),
            policy_object_index: build_index(&bundle.policy_objects, |item| item.id.as_str()),
            bundle,
        })
    }

    pub fn bundle(&self) -> &RegistryBundle {
        &self.bundle
    }
}

impl RegistryLookup for SeedRegistry {
    fn get_object(&self, id: &str) -> Option<QcObject> {
        self.object_index
            .get(id)
            .and_then(|index| self.bundle.objects.get(*index))
            .cloned()
    }
    fn get_object_origin(&self, id: &str) -> ArtifactOrigin {
        if self.object_index.contains_key(id) {
            ArtifactOrigin::Seed
        } else {
            ArtifactOrigin::Unknown
        }
    }

    fn get_regime(&self, id: &str) -> Option<RegimePack> {
        self.regime_index
            .get(id)
            .and_then(|index| self.bundle.regimes.get(*index))
            .cloned()
    }

    fn get_bridge(&self, id: &str) -> Option<BridgeContract> {
        self.bridge_index
            .get(id)
            .and_then(|index| self.bundle.bridges.get(*index))
            .cloned()
    }

    fn get_proof_shape(&self, id: &str) -> Option<ProofShape> {
        self.proof_shape_index
            .get(id)
            .and_then(|index| self.bundle.proof_shapes.get(*index))
            .cloned()
    }

    fn get_atlas_cell(&self, id: &str) -> Option<AtlasCell> {
        self.atlas_cell_index
            .get(id)
            .and_then(|index| self.bundle.atlas_cells.get(*index))
            .cloned()
    }

    fn get_mechanization_package(&self, id: &str) -> Option<MechanizationPackage> {
        self.mechanization_index
            .get(id)
            .and_then(|index| self.bundle.mechanization_packages.get(*index))
            .cloned()
    }

    fn get_theorem_spec(&self, id: &str) -> Option<TheoremSpec> {
        self.theorem_index
            .get(id)
            .and_then(|index| self.bundle.theorem_specs.get(*index))
            .cloned()
    }

    fn get_obligation(&self, id: &str) -> Option<Obligation> {
        self.obligation_index
            .get(id)
            .and_then(|index| self.bundle.obligations.get(*index))
            .cloned()
    }

    fn get_target_profile(&self, id: &str) -> Option<TargetProfile> {
        self.target_profile_index
            .get(id)
            .and_then(|index| self.bundle.target_profiles.get(*index))
            .cloned()
    }

    fn get_route_ledger(&self, id: &str) -> Option<RouteLedger> {
        self.route_ledger_index
            .get(id)
            .and_then(|index| self.bundle.route_ledgers.get(*index))
            .cloned()
    }

    fn get_certificate(&self, id: &str) -> Option<Certificate> {
        self.certificate_index
            .get(id)
            .and_then(|index| self.bundle.certificates.get(*index))
            .cloned()
    }

    fn get_campaign(&self, id: &str) -> Option<Campaign> {
        self.campaign_index
            .get(id)
            .and_then(|index| self.bundle.campaigns.get(*index))
            .cloned()
    }

    fn get_campaign_portfolio(&self, id: &str) -> Option<CampaignPortfolio> {
        self.portfolio_index
            .get(id)
            .and_then(|index| self.bundle.campaign_portfolios.get(*index))
            .cloned()
    }

    fn get_route_class(&self, id: &str) -> Option<RouteClass> {
        self.route_class_index
            .get(id)
            .and_then(|index| self.bundle.route_classes.get(*index))
            .cloned()
    }

    fn get_atlas_deficiency(&self, id: &str) -> Option<AtlasDeficiency> {
        self.atlas_deficiency_index
            .get(id)
            .and_then(|index| self.bundle.atlas_deficiencies.get(*index))
            .cloned()
    }
    fn atlas_deficiencies(&self) -> Vec<AtlasDeficiency> {
        self.bundle.atlas_deficiencies.clone()
    }

    fn get_adequacy_clause(&self, id: &str) -> Option<AdequacyClause> {
        self.adequacy_clause_index
            .get(id)
            .and_then(|index| self.bundle.adequacy_clauses.get(*index))
            .cloned()
    }
    fn adequacy_clauses(&self) -> Vec<AdequacyClause> {
        self.bundle.adequacy_clauses.clone()
    }
    fn get_burden_pack(&self, id: &str) -> Option<BurdenPack> {
        self.burden_pack_index
            .get(id)
            .and_then(|index| self.bundle.burden_packs.get(*index))
            .cloned()
    }
    fn burden_packs(&self) -> Vec<BurdenPack> {
        self.bundle.burden_packs.clone()
    }
    fn get_claim_packet(&self, id: &str) -> Option<ClaimPacket> {
        self.claim_packet_index
            .get(id)
            .and_then(|index| self.bundle.claim_packets.get(*index))
            .cloned()
    }
    fn claim_packets(&self) -> Vec<ClaimPacket> {
        self.bundle.claim_packets.clone()
    }
    fn get_evidence_contract(&self, id: &str) -> Option<EvidenceContract> {
        self.evidence_contract_index
            .get(id)
            .and_then(|index| self.bundle.evidence_contracts.get(*index))
            .cloned()
    }
    fn evidence_contracts(&self) -> Vec<EvidenceContract> {
        self.bundle.evidence_contracts.clone()
    }
    fn get_benchmark_receipt(&self, id: &str) -> Option<BenchmarkReceipt> {
        self.benchmark_receipt_index
            .get(id)
            .and_then(|index| self.bundle.benchmark_receipts.get(*index))
            .cloned()
    }
    fn benchmark_receipts(&self) -> Vec<BenchmarkReceipt> {
        self.bundle.benchmark_receipts.clone()
    }
    fn get_challenge_receipt(&self, id: &str) -> Option<ChallengeReceipt> {
        self.challenge_receipt_index
            .get(id)
            .and_then(|index| self.bundle.challenge_receipts.get(*index))
            .cloned()
    }
    fn challenge_receipts(&self) -> Vec<ChallengeReceipt> {
        self.bundle.challenge_receipts.clone()
    }
    fn get_reproducibility_packet(&self, id: &str) -> Option<ReproducibilityPacket> {
        self.reproducibility_packet_index
            .get(id)
            .and_then(|index| self.bundle.reproducibility_packets.get(*index))
            .cloned()
    }
    fn reproducibility_packets(&self) -> Vec<ReproducibilityPacket> {
        self.bundle.reproducibility_packets.clone()
    }

    fn get_codebook_pack(&self, id: &str) -> Option<CodebookPack> {
        self.codebook_pack_index
            .get(id)
            .and_then(|index| self.bundle.codebook_packs.get(*index))
            .cloned()
    }

    fn get_glyph_pack(&self, id: &str) -> Option<GlyphPack> {
        self.glyph_pack_index
            .get(id)
            .and_then(|index| self.bundle.glyph_packs.get(*index))
            .cloned()
    }

    fn get_combo_pack(&self, id: &str) -> Option<ComboPack> {
        self.combo_pack_index
            .get(id)
            .and_then(|index| self.bundle.combo_packs.get(*index))
            .cloned()
    }

    fn get_projection_policy(&self, id: &str) -> Option<ProjectionPolicy> {
        self.projection_policy_index
            .get(id)
            .and_then(|index| self.bundle.projection_policies.get(*index))
            .cloned()
    }

    fn get_alias_expansion_policy(&self, id: &str) -> Option<AliasExpansionPolicy> {
        self.alias_policy_index
            .get(id)
            .and_then(|index| self.bundle.alias_expansion_policies.get(*index))
            .cloned()
    }

    fn get_surface_policy(&self, id: &str) -> Option<SurfacePolicy> {
        self.surface_policy_index
            .get(id)
            .and_then(|index| self.bundle.surface_policies.get(*index))
            .cloned()
    }

    fn get_capability_matrix(&self, id: &str) -> Option<CapabilityMatrix> {
        self.capability_index
            .get(id)
            .and_then(|index| self.bundle.capability_matrices.get(*index))
            .cloned()
    }

    fn get_roundtrip_report(&self, id: &str) -> Option<RoundTripReport> {
        self.roundtrip_index
            .get(id)
            .and_then(|index| self.bundle.roundtrip_reports.get(*index))
            .cloned()
    }

    fn get_transform_receipt(&self, id: &str) -> Option<FormatTransformReceipt> {
        self.receipt_index
            .get(id)
            .and_then(|index| self.bundle.transform_receipts.get(*index))
            .cloned()
    }

    fn get_surface_deficiency(&self, id: &str) -> Option<SurfaceDeficiency> {
        self.surface_deficiency_index
            .get(id)
            .and_then(|index| self.bundle.surface_deficiencies.get(*index))
            .cloned()
    }

    fn get_policy_object(&self, id: &str) -> Option<MechanizationPolicyObject> {
        self.policy_object_index
            .get(id)
            .and_then(|index| self.bundle.policy_objects.get(*index))
            .cloned()
    }

    fn policy_objects(&self) -> Vec<MechanizationPolicyObject> {
        self.bundle.policy_objects.clone()
    }

    fn policy_bindings(&self) -> Vec<PolicyBinding> {
        self.bundle.policy_bindings.clone()
    }

    fn find_equivalence_class(&self, object_id: &str, regime: &str) -> Option<EquivalenceClass> {
        self.bundle
            .equivalence_classes
            .iter()
            .find(|item| {
                item.regime == regime && item.members.iter().any(|member| member == object_id)
            })
            .cloned()
    }

    fn atlas_cells(&self) -> Vec<AtlasCell> {
        self.bundle.atlas_cells.clone()
    }
}

fn build_index<T>(items: &[T], key: impl Fn(&T) -> &str) -> HashMap<String, usize> {
    items
        .iter()
        .enumerate()
        .map(|(index, item)| (key(item).to_string(), index))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn seed_registry_loads() {
        let registry = SeedRegistry::load().expect("seed registry should load");
        assert!(registry.get_regime("R_SET").is_some());
        assert!(registry.get_bridge("B_TYPE_TO_SET").is_some());
        assert!(registry.get_atlas_cell("A_TYPE_TO_SET").is_some());
        assert!(registry.get_theorem_spec("THS_CHAIN_RULE").is_some());
        assert!(registry.get_campaign("CPG_CHAIN_RULE").is_some());
    }
}
