use mf_core::*;

#[derive(Debug, Clone, Default)]
pub struct BundleRegistry {
    bundle: RegistryBundle,
}

impl BundleRegistry {
    pub fn bundle(&self) -> &RegistryBundle {
        &self.bundle
    }
}

#[derive(Debug, Clone, Default)]
pub struct TestRegistryBuilder {
    bundle: RegistryBundle,
}

impl TestRegistryBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_object(mut self, item: QcObject) -> Self {
        self.bundle.objects.push(item);
        self
    }

    pub fn with_regime(mut self, item: RegimePack) -> Self {
        self.bundle.regimes.push(item);
        self
    }

    pub fn with_bridge(mut self, item: BridgeContract) -> Self {
        self.bundle.bridges.push(item);
        self
    }

    pub fn with_equivalence_class(mut self, item: EquivalenceClass) -> Self {
        self.bundle.equivalence_classes.push(item);
        self
    }

    pub fn with_proof_shape(mut self, item: ProofShape) -> Self {
        self.bundle.proof_shapes.push(item);
        self
    }

    pub fn with_atlas_cell(mut self, item: AtlasCell) -> Self {
        self.bundle.atlas_cells.push(item);
        self
    }

    pub fn with_theorem_spec(mut self, item: TheoremSpec) -> Self {
        self.bundle.theorem_specs.push(item);
        self
    }

    pub fn with_obligation(mut self, item: Obligation) -> Self {
        self.bundle.obligations.push(item);
        self
    }

    pub fn with_target_profile(mut self, item: TargetProfile) -> Self {
        self.bundle.target_profiles.push(item);
        self
    }

    pub fn with_route_ledger(mut self, item: RouteLedger) -> Self {
        self.bundle.route_ledgers.push(item);
        self
    }

    pub fn with_certificate(mut self, item: Certificate) -> Self {
        self.bundle.certificates.push(item);
        self
    }

    pub fn with_campaign(mut self, item: Campaign) -> Self {
        self.bundle.campaigns.push(item);
        self
    }

    pub fn with_route_class(mut self, item: RouteClass) -> Self {
        self.bundle.route_classes.push(item);
        self
    }

    pub fn build(self) -> BundleRegistry {
        BundleRegistry {
            bundle: self.bundle,
        }
    }
}

fn get_by_id<T: Clone>(items: &[T], id: &str, id_of: impl Fn(&T) -> &str) -> Option<T> {
    items.iter().find(|item| id_of(item) == id).cloned()
}

impl RegistryLookup for BundleRegistry {
    fn get_object(&self, id: &str) -> Option<QcObject> {
        get_by_id(&self.bundle.objects, id, |item| item.id.as_str())
    }
    fn get_regime(&self, id: &str) -> Option<RegimePack> {
        get_by_id(&self.bundle.regimes, id, |item| item.id.as_str())
    }
    fn get_bridge(&self, id: &str) -> Option<BridgeContract> {
        get_by_id(&self.bundle.bridges, id, |item| item.id.as_str())
    }
    fn get_proof_shape(&self, id: &str) -> Option<ProofShape> {
        get_by_id(&self.bundle.proof_shapes, id, |item| item.id.as_str())
    }
    fn get_atlas_cell(&self, id: &str) -> Option<AtlasCell> {
        get_by_id(&self.bundle.atlas_cells, id, |item| item.id.as_str())
    }
    fn get_mechanization_package(&self, id: &str) -> Option<MechanizationPackage> {
        get_by_id(&self.bundle.mechanization_packages, id, |item| {
            item.id.as_str()
        })
    }
    fn get_theorem_spec(&self, id: &str) -> Option<TheoremSpec> {
        get_by_id(&self.bundle.theorem_specs, id, |item| item.id.as_str())
    }
    fn get_obligation(&self, id: &str) -> Option<Obligation> {
        get_by_id(&self.bundle.obligations, id, |item| item.id.as_str())
    }
    fn get_target_profile(&self, id: &str) -> Option<TargetProfile> {
        get_by_id(&self.bundle.target_profiles, id, |item| item.id.as_str())
    }
    fn get_route_ledger(&self, id: &str) -> Option<RouteLedger> {
        get_by_id(&self.bundle.route_ledgers, id, |item| item.id.as_str())
    }
    fn get_certificate(&self, id: &str) -> Option<Certificate> {
        get_by_id(&self.bundle.certificates, id, |item| item.id.as_str())
    }
    fn get_campaign(&self, id: &str) -> Option<Campaign> {
        get_by_id(&self.bundle.campaigns, id, |item| item.id.as_str())
    }
    fn get_campaign_portfolio(&self, id: &str) -> Option<CampaignPortfolio> {
        get_by_id(&self.bundle.campaign_portfolios, id, |item| {
            item.id.as_str()
        })
    }
    fn get_route_class(&self, id: &str) -> Option<RouteClass> {
        get_by_id(&self.bundle.route_classes, id, |item| item.id.as_str())
    }
    fn get_atlas_deficiency(&self, id: &str) -> Option<AtlasDeficiency> {
        get_by_id(&self.bundle.atlas_deficiencies, id, |item| item.id.as_str())
    }
    fn atlas_deficiencies(&self) -> Vec<AtlasDeficiency> {
        self.bundle.atlas_deficiencies.clone()
    }
    fn get_adequacy_clause(&self, id: &str) -> Option<AdequacyClause> {
        get_by_id(&self.bundle.adequacy_clauses, id, |item| item.id.as_str())
    }
    fn adequacy_clauses(&self) -> Vec<AdequacyClause> {
        self.bundle.adequacy_clauses.clone()
    }
    fn get_burden_pack(&self, id: &str) -> Option<BurdenPack> {
        get_by_id(&self.bundle.burden_packs, id, |item| item.id.as_str())
    }
    fn burden_packs(&self) -> Vec<BurdenPack> {
        self.bundle.burden_packs.clone()
    }
    fn get_claim_packet(&self, id: &str) -> Option<ClaimPacket> {
        get_by_id(&self.bundle.claim_packets, id, |item| item.id.as_str())
    }
    fn claim_packets(&self) -> Vec<ClaimPacket> {
        self.bundle.claim_packets.clone()
    }
    fn get_evidence_contract(&self, id: &str) -> Option<EvidenceContract> {
        get_by_id(&self.bundle.evidence_contracts, id, |item| item.id.as_str())
    }
    fn evidence_contracts(&self) -> Vec<EvidenceContract> {
        self.bundle.evidence_contracts.clone()
    }
    fn get_benchmark_receipt(&self, id: &str) -> Option<BenchmarkReceipt> {
        get_by_id(&self.bundle.benchmark_receipts, id, |item| item.id.as_str())
    }
    fn benchmark_receipts(&self) -> Vec<BenchmarkReceipt> {
        self.bundle.benchmark_receipts.clone()
    }
    fn get_challenge_receipt(&self, id: &str) -> Option<ChallengeReceipt> {
        get_by_id(&self.bundle.challenge_receipts, id, |item| item.id.as_str())
    }
    fn challenge_receipts(&self) -> Vec<ChallengeReceipt> {
        self.bundle.challenge_receipts.clone()
    }
    fn get_reproducibility_packet(&self, id: &str) -> Option<ReproducibilityPacket> {
        get_by_id(&self.bundle.reproducibility_packets, id, |item| {
            item.id.as_str()
        })
    }
    fn reproducibility_packets(&self) -> Vec<ReproducibilityPacket> {
        self.bundle.reproducibility_packets.clone()
    }
    fn get_codebook_pack(&self, id: &str) -> Option<CodebookPack> {
        get_by_id(&self.bundle.codebook_packs, id, |item| item.id.as_str())
    }
    fn get_glyph_pack(&self, id: &str) -> Option<GlyphPack> {
        get_by_id(&self.bundle.glyph_packs, id, |item| item.id.as_str())
    }
    fn get_combo_pack(&self, id: &str) -> Option<ComboPack> {
        get_by_id(&self.bundle.combo_packs, id, |item| item.id.as_str())
    }
    fn get_projection_policy(&self, id: &str) -> Option<ProjectionPolicy> {
        get_by_id(&self.bundle.projection_policies, id, |item| {
            item.id.as_str()
        })
    }
    fn get_alias_expansion_policy(&self, id: &str) -> Option<AliasExpansionPolicy> {
        get_by_id(&self.bundle.alias_expansion_policies, id, |item| {
            item.id.as_str()
        })
    }
    fn get_surface_policy(&self, id: &str) -> Option<SurfacePolicy> {
        get_by_id(&self.bundle.surface_policies, id, |item| item.id.as_str())
    }
    fn get_capability_matrix(&self, id: &str) -> Option<CapabilityMatrix> {
        get_by_id(&self.bundle.capability_matrices, id, |item| {
            item.id.as_str()
        })
    }
    fn get_roundtrip_report(&self, id: &str) -> Option<RoundTripReport> {
        get_by_id(&self.bundle.roundtrip_reports, id, |item| item.id.as_str())
    }
    fn get_transform_receipt(&self, id: &str) -> Option<FormatTransformReceipt> {
        get_by_id(&self.bundle.transform_receipts, id, |item| item.id.as_str())
    }
    fn get_surface_deficiency(&self, id: &str) -> Option<SurfaceDeficiency> {
        get_by_id(&self.bundle.surface_deficiencies, id, |item| {
            item.id.as_str()
        })
    }
    fn get_policy_object(&self, id: &str) -> Option<MechanizationPolicyObject> {
        get_by_id(&self.bundle.policy_objects, id, |item| item.id.as_str())
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
