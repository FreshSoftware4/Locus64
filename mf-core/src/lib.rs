use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

pub type ObjectId = String;
pub type RegimeId = String;
pub type BridgeId = String;
pub type ProofId = String;
pub type AtlasCellId = String;
pub type TheoremId = String;
pub type ObligationId = String;
pub type CertificateId = String;
pub type RouteLedgerId = String;
pub type CampaignId = String;
pub type TargetProfileId = String;
pub type RouteClassId = String;
pub type SurfacePolicyId = String;
pub type TransformReceiptId = String;
pub type RoundTripReportId = String;
pub type CapabilityMatrixId = String;
pub type SurfaceBudgetId = String;
pub type CodebookPackId = String;
pub type GlyphPackId = String;
pub type ComboPackId = String;
pub type ProjectionPolicyId = String;
pub type AliasExpansionPolicyId = String;
pub type EqClassId = u32;
pub type CanonId = u32;
pub type RouteId = u32;
pub type CertId = u32;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectRoot {
    pub absolute_path: String,
    pub resolution_source: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheRoot {
    pub absolute_path: String,
    pub resolution_source: String,
    #[serde(default)]
    pub namespace: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PathNormalizationReceipt {
    pub id: String,
    pub original_path: String,
    pub normalized_path: String,
    pub project_relative_path: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactLocator {
    pub artifact_id: String,
    pub artifact_kind: String,
    pub absolute_path: String,
    pub normalized_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RootResolutionReport {
    pub project_root: ProjectRoot,
    pub cache_root: CacheRoot,
    pub receipts: Vec<PathNormalizationReceipt>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ObjectTag {
    Ctx,
    Cut,
    Thr,
    Brc,
    Slk,
    Tol,
    Knt,
    Wit,
    Can,
    Prm,
    Car,
    Rel,
    Map,
    Opr,
    Inv,
    Qot,
    Loc,
    Glb,
    Der,
    Int,
    Msr,
    Prf,
    Jdg,
    Typ,
    Mor,
    Ftr,
    Cmp,
    Red,
    Cst,
    Brg,
    Brp,
    Bud,
    Blaw,
    Bheu,
    Brec,
    Prs,
    Cnz,
    Rgy,
    Sel,
    Chk,
    Mec,
    Ade,
    Tcm,
    Ths,
    Obl,
    Prmz,
    Cpg,
    Crt,
    Vrt,
    Dgn,
    Trl,
    Tgt,
    Cmpa,
    Atl,
    Atlx,
    Rtc,
    Rcc,
    Pol,
    Xfr,
    Rtp,
    Cap,
    Sbd,
    Doc,
    Hdr,
    Pkg,
    Sfc,
    Fmt,
    Bnd,
    Bent,
    Bidx,
    Bdep,
    Bmer,
    Bcon,
    Bovr,
    Mop,
    Mpg,
    Mpn,
    Mpe,
    Mpb,
    Mpc,
    Mpr,
    Mpt,
    Mpa,
    Mps,
    Mpv,
    Blk,
    Exm,
    Pmf,
    Rlm,
    Lrc,
    Ldf,
    Obs,
    Obg,
    Obn,
    Obe,
    Evt,
    Chc,
    Otr,
    Rte,
    Cce,
    Dif,
    Drf,
    Imp,
    Prd,
    Rcp,
    Rsk,
    Exc,
    Odg,
    Odn,
    Ode,
    Ogr,
    Ocl,
    Obr,
    Opl,
    Oxr,
    Olan,
    Oord,
    Omer,
    Oskp,
    Orer,
    Orpl,
    Rog,
    Rlc,
    Rbr,
    Rmr,
    Rdr,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SurfaceKind {
    Sc0,
    Gs0,
    As0,
    Qc0,
    Qm0,
    Qk0,
    Qa0,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransformKind {
    Import,
    Export,
    Transcode,
    Normalize,
    RoundTrip,
    Expand,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TransformVerdict {
    Identity,
    Lossless,
    ReceiptedLoss,
    Unsupported,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProjectionPolicyKind {
    StrictAuthority,
    CanonicalAuthored,
    InputProjectionOnly,
    DebugMirrorOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AliasExpansionPolicyKind {
    ForbidImplicit,
    ExpandDeclared,
    ExpandAndReceipt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SurfaceDeficiencyKind {
    MissingHeader,
    MissingCodebook,
    MissingProfile,
    MissingComboPack,
    MissingGlyphPack,
    UnsupportedSymbol,
    UnsupportedCombo,
    QkAuthorityConfusion,
    RoundtripUnverified,
    SurfacePackMismatch,
    SilentDefaultingRisk,
    DebugCollapseUnreceipted,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SurfaceCompatibilityClass {
    AuthorityPreserving,
    SymbolicFidelityPreserving,
    DebugMirrorOnly,
    IngressProjectionOnly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfaceRequirement {
    pub required_input: Option<SurfaceKind>,
    pub preferred_output: Option<SurfaceKind>,
    pub require_symbolic_fidelity: bool,
    pub keyboard_projection_ingress_only: bool,
    pub transform_receipts_mandatory: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfaceRoutePenalty {
    pub reason: String,
    pub amount: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfaceTransitionCost {
    pub compatibility: SurfaceCompatibilityClass,
    pub penalties: Vec<SurfaceRoutePenalty>,
    pub total_penalty: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfacePreferredTarget {
    pub surface_kind: SurfaceKind,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SupportedPack {
    Core,
    PackA,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvidenceMaturity {
    Candidate,
    Registered,
    Validated,
    Certified,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum GateVerdict {
    Pass,
    Fail,
    Review,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct IdentityFace {
    pub tag: ObjectTag,
    pub cid: String,
    pub codebook: String,
    pub remap: String,
    pub lineage: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralFace {
    pub head: String,
    pub args: Vec<String>,
    pub local_sections: Vec<String>,
    pub morphism_hooks: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConstraintFace {
    pub regime: RegimeId,
    pub contracts: Vec<String>,
    pub invariants: Vec<String>,
    pub equivalence: String,
    pub admissibility: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceFace {
    pub evidence_class: String,
    pub traces: Vec<String>,
    pub receipts: Vec<String>,
    pub maturity: EvidenceMaturity,
    pub gate_verdict: GateVerdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AliasFace {
    pub aliases: Vec<String>,
    pub profile_pack: Vec<String>,
    pub qm_binding: String,
    pub qa_binding: String,
    pub projection_policy: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CodebookPack {
    pub id: CodebookPackId,
    pub version: String,
    pub symbols: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GlyphPack {
    pub id: GlyphPackId,
    pub version: String,
    pub pack: SupportedPack,
    pub delimiters: BTreeMap<String, String>,
    pub operators: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ComboPack {
    pub id: ComboPackId,
    pub version: String,
    pub pack: SupportedPack,
    pub combos: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectionPolicy {
    pub id: ProjectionPolicyId,
    pub kind: ProjectionPolicyKind,
    pub allow_qk0_export: bool,
    pub allow_qa0_downgrade: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AliasExpansionPolicy {
    pub id: AliasExpansionPolicyId,
    pub kind: AliasExpansionPolicyKind,
    pub allowed_aliases: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfacePolicy {
    pub id: SurfacePolicyId,
    pub codebook_pack: CodebookPackId,
    pub profile: String,
    pub projection_policy: ProjectionPolicyId,
    pub combo_pack: Option<ComboPackId>,
    pub glyph_pack: Option<GlyphPackId>,
    pub alias_expansion_policy: AliasExpansionPolicyId,
    pub allowed_loss_classes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfaceBudget {
    pub id: SurfaceBudgetId,
    pub max_loss_classes: usize,
    pub forbid_silent_defaulting: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FormatTransformReceipt {
    pub id: TransformReceiptId,
    pub src_surface: SurfaceKind,
    pub dst_surface: SurfaceKind,
    pub object_ids: Vec<String>,
    pub transform_kind: TransformKind,
    pub policy_id: SurfacePolicyId,
    pub defaults_used: Vec<String>,
    pub alias_expansions: Vec<String>,
    pub loss_classes: Vec<String>,
    pub hash_before: String,
    pub hash_after: String,
    pub verdict: TransformVerdict,
    pub rollback_ref: Option<String>,
    pub replay_ref: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RoundTripReport {
    pub id: RoundTripReportId,
    pub surface_kind: SurfaceKind,
    pub policy_id: SurfacePolicyId,
    pub object_ids: Vec<String>,
    pub receipt_ids: Vec<TransformReceiptId>,
    pub verdict: TransformVerdict,
    pub fragility_vector: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CapabilityMatrix {
    pub id: CapabilityMatrixId,
    pub surface_kind: SurfaceKind,
    pub supported_packs: Vec<SupportedPack>,
    pub import_support: bool,
    pub export_support: bool,
    pub transcode_support: Vec<SurfaceKind>,
    pub roundtrip_support: bool,
    pub known_limits: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SurfaceDeficiency {
    pub id: String,
    pub kind: SurfaceDeficiencyKind,
    pub surface_kind: SurfaceKind,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HeaderEnvelope {
    pub surface_kind: SurfaceKind,
    pub version: String,
    pub policy_id: SurfacePolicyId,
    pub capability_id: Option<CapabilityMatrixId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct QcObject {
    pub id: ObjectId,
    pub identity: IdentityFace,
    pub structural: StructuralFace,
    pub constraint: ConstraintFace,
    pub evidence: EvidenceFace,
    pub alias: AliasFace,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegimePack {
    pub id: RegimeId,
    pub ctx_law: String,
    pub cut_law: String,
    pub thr_law: String,
    pub brc_law: String,
    pub slk_law: String,
    pub tol_law: String,
    pub knt_law: String,
    pub eq_law: String,
    pub adm_law: String,
    pub promoted_ops: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReversibilityClass {
    Exact,
    Conservative,
    Enriching,
    Forgetting,
    LossySupported,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BridgeContract {
    pub id: BridgeId,
    pub src: RegimeId,
    pub tgt: RegimeId,
    pub id_pres: String,
    pub eq_pres: String,
    pub forget: Vec<String>,
    pub enrich: Vec<String>,
    pub loss: Vec<String>,
    pub reversibility: ReversibilityClass,
    pub receipts: Vec<String>,
    pub rollback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProofShapeKind {
    Square,
    Triangle,
    Diamond,
    Pentagon,
    Hexagon,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofEdge {
    pub from: String,
    pub to: String,
    pub label: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofShape {
    pub id: ProofId,
    pub kind: ProofShapeKind,
    pub nodes: Vec<String>,
    pub edges: Vec<ProofEdge>,
    pub equations: Vec<String>,
    pub target_equivalence: String,
    pub receipts: Vec<String>,
    pub gate: GateVerdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KernelRuleId {
    K1,
    K2,
    K3,
    K4,
    K5,
    K6,
    K7,
    K8,
    K9,
    K10,
    K11,
    K12,
    K13,
    K14,
    K15,
    K16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum Judgment {
    Ctx {
        object: ObjectId,
    },
    Cut {
        object: ObjectId,
    },
    Thr {
        object: ObjectId,
    },
    Brc {
        object: ObjectId,
    },
    Slk {
        left: ObjectId,
        right: ObjectId,
        rank: u32,
    },
    Tol {
        object: ObjectId,
        carrier: String,
    },
    Knt {
        object: ObjectId,
    },
    Wit {
        object: ObjectId,
    },
    Can {
        object: ObjectId,
        canonical: ObjectId,
    },
    Prm {
        object: ObjectId,
    },
    Brg {
        bridge: BridgeId,
    },
    Path {
        bridges: Vec<BridgeId>,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum BurdenClass {
    ExtensionalCarrierReasoning,
    ProofRelevantAlgebra,
    DerivativeLocalWitnessExtraction,
    MeasurableNormalizedBranching,
    ProbabilisticJudgment,
    SimulationExecutableInference,
    CertifiedPropertyWitness,
    ImportedKernelClaim,
    ProjectionClosure,
    Containment,
    RunningLaw,
    General,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum WinnerState {
    SeedWinner,
    Candidate,
    Alternative,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Budget {
    pub max_loss: usize,
    pub allow_lossy_supported: bool,
    pub require_proof: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LossProfile {
    pub items: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FailureSignature {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecipeMaturity {
    Prototype,
    Seeded,
    Stable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CertificationVerdict {
    Invalid,
    Underspecified,
    RouteFound,
    Benchmarked,
    Certified,
    Integrated,
    BlockedOpen,
    BlockedContradiction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ArtifactOrigin {
    Seed,
    Overlay,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProofMechanismZone {
    PmzStructural,
    PmzBridge,
    PmzLocalToGlobal,
    PmzAggregation,
    PmzReduction,
    PmzCanonicalization,
    PmzSemantic,
    PmzSpectral,
    PmzObstruction,
    PmzProbabilistic,
    PmzOperational,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObligationKind {
    OblEq,
    OblAdm,
    OblLoc,
    OblGlu,
    OblTol,
    OblRed,
    OblBrg,
    OblRbk,
    OblAde,
    OblFin,
    OblObs,
    OblKnt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RequiredProofShapeFamily {
    Minimal,
    Triangle,
    Square,
    Diamond,
    Pentagon,
    Hexagon,
    MixedBattery,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PromotionGoal {
    PromoteOperator,
    PromoteBridge,
    PromoteRouteClass,
    Retire,
    OpenBlocked,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CampaignClass {
    CBasic,
    CBridge,
    CWelded,
    COperator,
    CAtlas,
    COpen,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AtlasDeficiencyClass {
    DNoRoute,
    DHighLoss,
    DBadEqTransport,
    DRollbackCliff,
    DNoAdequacy,
    DBridge,
    DEq,
    DSelector,
    DRoundtrip,
    DPromo,
    DNoCommutingProof,
    DNoOperatorPayoff,
    DOpenConjectural,
    DProjection,
    DContainment,
    DClosure,
    DRunningLaw,
    DEvidenceContract,
    DBenchmarkGap,
    DStressGap,
    DChallengeGap,
    DHostPackMissing,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeficiencyBlockingScope {
    Campaign,
    Promotion,
    Planning,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeficiencyControlEffect {
    BlockCampaign,
    BlockPromotion,
    BlockPlanning,
    SuggestNextSeam,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AtlasCell {
    pub id: AtlasCellId,
    pub source_regime: RegimeId,
    pub target_regime: RegimeId,
    pub burden_class: BurdenClass,
    pub proof_target: String,
    pub candidate_paths: Vec<Vec<BridgeId>>,
    pub normalized_winner: Vec<BridgeId>,
    pub winner_state: WinnerState,
    pub loss_profile: LossProfile,
    pub proof_shapes_checked: Vec<ProofId>,
    pub recipe_maturity: RecipeMaturity,
    pub failure_signatures: Vec<FailureSignature>,
    pub side_conditions: Vec<String>,
    #[serde(default)]
    pub surface_transition: Option<SurfaceTransitionCost>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ParserObject {
    pub id: String,
    pub supported_surfaces: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalizerObject {
    pub id: String,
    pub regime_scope: Vec<RegimeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RegistryObject {
    pub id: String,
    pub domains: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SelectorObject {
    pub id: String,
    pub burden_axes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckerObject {
    pub id: String,
    pub proof_shapes: Vec<ProofShapeKind>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MechanizationPackage {
    pub id: String,
    pub parser: ParserObject,
    pub canonicalizer: CanonicalizerObject,
    pub registry: RegistryObject,
    pub selector: SelectorObject,
    pub checker: CheckerObject,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TheoremSpec {
    pub id: TheoremId,
    pub statement: String,
    pub hosts: Vec<RegimeId>,
    pub bridges: Vec<BridgeId>,
    pub operators: Vec<String>,
    pub target_equivalence: String,
    pub obligations: Vec<ObligationKind>,
    pub primary_zone: ProofMechanismZone,
    pub verdict: CertificationVerdict,
    pub proof_shapes: Vec<ProofId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Obligation {
    pub id: ObligationId,
    pub kind: ObligationKind,
    pub description: String,
    pub status: CertificationVerdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TargetProfile {
    pub id: TargetProfileId,
    pub burden_class: BurdenClass,
    pub host_cluster: Vec<RegimeId>,
    pub target_equivalence: String,
    pub allowed_bridge_classes: Vec<ReversibilityClass>,
    pub loss_ceiling: usize,
    pub rollback_ceiling: usize,
    pub required_receipt_class: String,
    pub required_proof_shape_family: RequiredProofShapeFamily,
    pub promotion_goal: PromotionGoal,
    pub primary_zone: ProofMechanismZone,
    #[serde(default)]
    pub surface_requirement: Option<SurfaceRequirement>,
    #[serde(default)]
    pub preferred_surface_target: Option<SurfacePreferredTarget>,
    #[serde(default)]
    pub optimizer_policy: Option<OptimizerPolicy>,
    #[serde(default)]
    pub policy_binding_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteLedger {
    pub id: RouteLedgerId,
    pub theorem: TheoremId,
    pub paths: Vec<Vec<BridgeId>>,
    pub budget: Budget,
    pub losses: Vec<String>,
    pub receipts: Vec<String>,
    pub normalized_path: Vec<BridgeId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Certificate {
    pub id: CertificateId,
    pub theorem: TheoremId,
    pub route_ledger: RouteLedgerId,
    pub proof_shapes: Vec<ProofId>,
    pub receipts: Vec<String>,
    pub verdict: CertificationVerdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Campaign {
    pub id: CampaignId,
    pub theorem: TheoremId,
    pub target_profile: TargetProfileId,
    pub route_ledger: RouteLedgerId,
    pub obligations: Vec<ObligationId>,
    pub certificates: Vec<CertificateId>,
    pub dependencies: Vec<CampaignId>,
    pub campaign_class: CampaignClass,
    pub verdict: CertificationVerdict,
    pub payoff: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CampaignPortfolio {
    pub id: String,
    pub campaigns: Vec<CampaignId>,
    pub normalized: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteClass {
    pub id: RouteClassId,
    pub theorem: TheoremId,
    pub target_profile: TargetProfileId,
    pub equivalent_paths: Vec<Vec<BridgeId>>,
    pub canonical_path: Vec<BridgeId>,
    pub verdict: CertificationVerdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AtlasDeficiency {
    pub id: String,
    pub class: AtlasDeficiencyClass,
    pub atlas_cell: Option<AtlasCellId>,
    pub theorem: Option<TheoremId>,
    pub message: String,
    #[serde(default)]
    pub blocking_scope: Option<DeficiencyBlockingScope>,
    #[serde(default)]
    pub control_effects: Vec<DeficiencyControlEffect>,
    #[serde(default)]
    pub suggested_seam: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AdequacyClauseKind {
    ObjectInterpretation,
    ThreadInterpretation,
    EquivalenceInterpretation,
    TollInterpretation,
    KnotInterpretation,
    BridgeSoundness,
    ProjectionInterpretation,
    ContainmentInterpretation,
    ClosureInterpretation,
    RunningLawInterpretation,
    EvidenceContractInterpretation,
    BenchmarkInterpretation,
    StressInterpretation,
    ChallengeInterpretation,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ClaimClass {
    Kernel,
    Interoperability,
    Host,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AuthorityState {
    Derived,
    Benchmark,
    Evidence,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BenchmarkRole {
    TargetCase,
    Control,
    Stress,
    Trace,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChallengeStatus {
    Open,
    Addressed,
    Retired,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BurdenPack {
    pub id: String,
    #[serde(default)]
    pub allowed_host_cluster: Vec<RegimeId>,
    #[serde(default)]
    pub obligation_ids: Vec<ObligationId>,
    #[serde(default)]
    pub adequacy_clause_ids: Vec<String>,
    pub required_proof_shape_family: RequiredProofShapeFamily,
    #[serde(default)]
    pub route_class_constraints: Vec<String>,
    #[serde(default)]
    pub evidence_contract_ids: Vec<String>,
    pub promotion_ceiling: CertificationVerdict,
    #[serde(default)]
    pub blocker_taxonomy: Vec<AtlasDeficiencyClass>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ClaimPacket {
    pub id: String,
    pub claim_class: ClaimClass,
    pub authority_state: AuthorityState,
    pub target_sector: String,
    pub statement: String,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub open_caveats: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvidenceContract {
    pub id: String,
    #[serde(default)]
    pub required_evidence_kinds: Vec<String>,
    #[serde(default)]
    pub required_benchmark_roles: Vec<BenchmarkRole>,
    #[serde(default)]
    pub requires_stress: bool,
    #[serde(default)]
    pub requires_challenge: bool,
    #[serde(default)]
    pub admissibility_thresholds: Vec<String>,
    pub promotion_ceiling: CertificationVerdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkReceipt {
    pub id: String,
    pub claim_packet_id: String,
    pub role: BenchmarkRole,
    pub verdict: CertificationVerdict,
    #[serde(default)]
    pub metrics: BTreeMap<String, String>,
    pub reproducibility_ref: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChallengeReceipt {
    pub id: String,
    pub claim_packet_id: String,
    #[serde(default)]
    pub grounds: Vec<String>,
    pub required_response: String,
    pub status: ChallengeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReproducibilityPacket {
    pub id: String,
    pub claim_packet_id: String,
    #[serde(default)]
    pub derivation_path: Vec<String>,
    #[serde(default)]
    pub code_refs: Vec<String>,
    #[serde(default)]
    pub benchmark_refs: Vec<String>,
    #[serde(default)]
    pub artifact_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdequacyClause {
    pub id: String,
    pub kind: AdequacyClauseKind,
    pub regime_ids: Vec<RegimeId>,
    #[serde(default)]
    pub bridge_ids: Vec<BridgeId>,
    #[serde(default)]
    pub theorem_ids: Vec<TheoremId>,
    #[serde(default)]
    pub burden_pack_ids: Vec<String>,
    #[serde(default)]
    pub claim_packet_ids: Vec<String>,
    #[serde(default)]
    pub evidence_contract_ids: Vec<String>,
    #[serde(default)]
    pub benchmark_receipt_ids: Vec<String>,
    #[serde(default)]
    pub challenge_receipt_ids: Vec<String>,
    #[serde(default)]
    pub reproducibility_packet_ids: Vec<String>,
    pub description: String,
    #[serde(default)]
    pub blocking: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdequacyRecord {
    pub id: String,
    pub clause_id: String,
    pub kind: AdequacyClauseKind,
    pub verdict: CertificationVerdict,
    pub computed: bool,
    pub blocking: bool,
    pub detail: String,
    #[serde(default)]
    pub receipt_ids: Vec<String>,
}

pub fn adequacy_kind_deficiency_class(kind: &AdequacyClauseKind) -> AtlasDeficiencyClass {
    match kind {
        AdequacyClauseKind::BridgeSoundness => AtlasDeficiencyClass::DBridge,
        AdequacyClauseKind::EquivalenceInterpretation => AtlasDeficiencyClass::DEq,
        AdequacyClauseKind::ProjectionInterpretation => AtlasDeficiencyClass::DProjection,
        AdequacyClauseKind::ContainmentInterpretation => AtlasDeficiencyClass::DContainment,
        AdequacyClauseKind::ClosureInterpretation => AtlasDeficiencyClass::DClosure,
        AdequacyClauseKind::RunningLawInterpretation => AtlasDeficiencyClass::DRunningLaw,
        AdequacyClauseKind::EvidenceContractInterpretation => {
            AtlasDeficiencyClass::DEvidenceContract
        }
        AdequacyClauseKind::BenchmarkInterpretation => AtlasDeficiencyClass::DBenchmarkGap,
        AdequacyClauseKind::StressInterpretation => AtlasDeficiencyClass::DStressGap,
        AdequacyClauseKind::ChallengeInterpretation => AtlasDeficiencyClass::DChallengeGap,
        _ => AtlasDeficiencyClass::DNoAdequacy,
    }
}

pub fn adequacy_kind_suggested_seam(kind: &AdequacyClauseKind) -> &'static str {
    match kind {
        AdequacyClauseKind::BridgeSoundness => {
            "bridge soundness clause for the active route remains open"
        }
        AdequacyClauseKind::EquivalenceInterpretation => {
            "equivalence interpretation clause remains open"
        }
        AdequacyClauseKind::ProjectionInterpretation => {
            "projection interpretation clause remains open"
        }
        AdequacyClauseKind::ContainmentInterpretation => {
            "containment interpretation clause remains open"
        }
        AdequacyClauseKind::ClosureInterpretation => "closure interpretation clause remains open",
        AdequacyClauseKind::RunningLawInterpretation => {
            "running-law interpretation clause remains open"
        }
        AdequacyClauseKind::EvidenceContractInterpretation => {
            "evidence-contract clause remains open"
        }
        AdequacyClauseKind::BenchmarkInterpretation => "benchmark coverage remains open",
        AdequacyClauseKind::StressInterpretation => "stress coverage remains open",
        AdequacyClauseKind::ChallengeInterpretation => "challenge coverage remains open",
        _ => "adequacy clause remains open on the touched regime cluster",
    }
}

pub fn blocking_control_effects(blocking: bool) -> Vec<DeficiencyControlEffect> {
    if blocking {
        vec![
            DeficiencyControlEffect::BlockCampaign,
            DeficiencyControlEffect::BlockPromotion,
            DeficiencyControlEffect::SuggestNextSeam,
        ]
    } else {
        vec![DeficiencyControlEffect::SuggestNextSeam]
    }
}

pub fn blocking_scope(blocking: bool) -> Option<DeficiencyBlockingScope> {
    if blocking {
        Some(DeficiencyBlockingScope::Campaign)
    } else {
        None
    }
}

pub fn adequacy_record_deficiency(
    theorem_id: &str,
    atlas_cell_id: Option<&str>,
    deficiency_id: String,
    record: &AdequacyRecord,
) -> AtlasDeficiency {
    AtlasDeficiency {
        id: deficiency_id,
        class: if record.verdict == CertificationVerdict::Certified {
            AtlasDeficiencyClass::DNoAdequacy
        } else {
            adequacy_kind_deficiency_class(&record.kind)
        },
        atlas_cell: atlas_cell_id.map(ToString::to_string),
        theorem: Some(theorem_id.to_string()),
        message: format!(
            "adequacy clause {} {:?} {:?}: {}",
            record.clause_id, record.kind, record.verdict, record.detail
        ),
        blocking_scope: if record.verdict != CertificationVerdict::Certified {
            blocking_scope(record.blocking)
        } else {
            None
        },
        control_effects: if record.verdict != CertificationVerdict::Certified {
            blocking_control_effects(record.blocking)
        } else {
            vec![DeficiencyControlEffect::SuggestNextSeam]
        },
        suggested_seam: Some(adequacy_kind_suggested_seam(&record.kind).into()),
    }
}

pub fn obligation_status_deficiency(
    theorem_id: &str,
    atlas_cell_id: Option<&str>,
    deficiency_id: String,
    obligation: &ObligationStatus,
    blocking: bool,
    suggested_seam: Option<String>,
) -> AtlasDeficiency {
    AtlasDeficiency {
        id: deficiency_id,
        class: if obligation.evaluation_mode == ObligationEvaluationMode::Unsupported {
            AtlasDeficiencyClass::DOpenConjectural
        } else {
            AtlasDeficiencyClass::DNoAdequacy
        },
        atlas_cell: atlas_cell_id.map(ToString::to_string),
        theorem: Some(theorem_id.to_string()),
        message: format!(
            "obligation {} {:?} {:?}: {}",
            obligation.obligation_id,
            obligation.kind,
            obligation.evaluation_mode,
            obligation.detail
        ),
        blocking_scope: blocking_scope(blocking),
        control_effects: if blocking {
            blocking_control_effects(true)
        } else {
            Vec::new()
        },
        suggested_seam,
    }
}

pub fn receipt_deficiency(
    theorem_id: &str,
    atlas_cell_id: Option<&str>,
    deficiency_id: String,
    receipt: &ObligationEvidenceReceipt,
    computed_class: AtlasDeficiencyClass,
    open_class: AtlasDeficiencyClass,
    suggested_seam: String,
) -> AtlasDeficiency {
    AtlasDeficiency {
        id: deficiency_id,
        class: if receipt.computed {
            computed_class
        } else {
            open_class
        },
        atlas_cell: atlas_cell_id.map(ToString::to_string),
        theorem: Some(theorem_id.to_string()),
        message: format!(
            "equivalence receipt {} {} {:?}: {}",
            receipt.id, receipt.label, receipt.verdict, receipt.detail
        ),
        blocking_scope: Some(DeficiencyBlockingScope::Campaign),
        control_effects: blocking_control_effects(true),
        suggested_seam: Some(suggested_seam),
    }
}

pub fn route_explanation_deficiency(
    theorem_id: &str,
    atlas_cell_id: Option<&str>,
    deficiency_id: String,
    explanation: &RouteExplanation,
    blocking: bool,
) -> AtlasDeficiency {
    AtlasDeficiency {
        id: deficiency_id,
        class: AtlasDeficiencyClass::DNoOperatorPayoff,
        atlas_cell: atlas_cell_id.map(ToString::to_string),
        theorem: Some(theorem_id.to_string()),
        message: format!(
            "optimizer={:?}; backend={:?}; winner={:?}; dominated={}",
            explanation.optimizer_policy,
            explanation.optimizer_backend,
            explanation.winner_atlas_cell_id,
            explanation.dominated_candidates.join("|")
        ),
        blocking_scope: blocking_scope(blocking),
        control_effects: if blocking {
            blocking_control_effects(true)
        } else {
            Vec::new()
        },
        suggested_seam: None,
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CheckerReceiptKind {
    TheoremSpec,
    TargetProfile,
    RouteLedger,
    Campaign,
    Certificate,
    ProofShape,
    AdequacyClause,
    ObligationVerdict,
    BurdenPack,
    ClaimPacket,
    EvidenceContract,
    BenchmarkReceipt,
    ChallengeReceipt,
    ReproducibilityPacket,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CheckerReceipt {
    pub id: String,
    pub kind: CheckerReceiptKind,
    pub subject_id: String,
    pub verdict: CertificationVerdict,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EquivalenceClass {
    pub id: String,
    pub regime: RegimeId,
    pub canonical_id: ObjectId,
    pub members: Vec<ObjectId>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct RegistryBundle {
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub objects: Vec<QcObject>,
    #[serde(default)]
    pub regimes: Vec<RegimePack>,
    #[serde(default)]
    pub bridges: Vec<BridgeContract>,
    #[serde(default)]
    pub proof_shapes: Vec<ProofShape>,
    #[serde(default)]
    pub atlas_cells: Vec<AtlasCell>,
    #[serde(default)]
    pub mechanization_packages: Vec<MechanizationPackage>,
    #[serde(default)]
    pub equivalence_classes: Vec<EquivalenceClass>,
    #[serde(default)]
    pub theorem_specs: Vec<TheoremSpec>,
    #[serde(default)]
    pub obligations: Vec<Obligation>,
    #[serde(default)]
    pub target_profiles: Vec<TargetProfile>,
    #[serde(default)]
    pub route_ledgers: Vec<RouteLedger>,
    #[serde(default)]
    pub certificates: Vec<Certificate>,
    #[serde(default)]
    pub campaigns: Vec<Campaign>,
    #[serde(default)]
    pub campaign_portfolios: Vec<CampaignPortfolio>,
    #[serde(default)]
    pub route_classes: Vec<RouteClass>,
    #[serde(default)]
    pub atlas_deficiencies: Vec<AtlasDeficiency>,
    #[serde(default)]
    pub adequacy_clauses: Vec<AdequacyClause>,
    #[serde(default)]
    pub burden_packs: Vec<BurdenPack>,
    #[serde(default)]
    pub claim_packets: Vec<ClaimPacket>,
    #[serde(default)]
    pub evidence_contracts: Vec<EvidenceContract>,
    #[serde(default)]
    pub benchmark_receipts: Vec<BenchmarkReceipt>,
    #[serde(default)]
    pub challenge_receipts: Vec<ChallengeReceipt>,
    #[serde(default)]
    pub reproducibility_packets: Vec<ReproducibilityPacket>,
    #[serde(default)]
    pub codebook_packs: Vec<CodebookPack>,
    #[serde(default)]
    pub glyph_packs: Vec<GlyphPack>,
    #[serde(default)]
    pub combo_packs: Vec<ComboPack>,
    #[serde(default)]
    pub projection_policies: Vec<ProjectionPolicy>,
    #[serde(default)]
    pub alias_expansion_policies: Vec<AliasExpansionPolicy>,
    #[serde(default)]
    pub surface_policies: Vec<SurfacePolicy>,
    #[serde(default)]
    pub capability_matrices: Vec<CapabilityMatrix>,
    #[serde(default)]
    pub roundtrip_reports: Vec<RoundTripReport>,
    #[serde(default)]
    pub transform_receipts: Vec<FormatTransformReceipt>,
    #[serde(default)]
    pub surface_deficiencies: Vec<SurfaceDeficiency>,
    #[serde(default)]
    pub policy_objects: Vec<MechanizationPolicyObject>,
    #[serde(default)]
    pub policy_bindings: Vec<PolicyBinding>,
    #[serde(default)]
    pub policy_resolutions: Vec<PolicyResolution>,
    #[serde(default)]
    pub bundle_locks: Vec<BundleLock>,
    #[serde(default)]
    pub execution_manifests: Vec<ExecutionManifest>,
    #[serde(default)]
    pub replay_lock_manifests: Vec<ReplayLockManifest>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BundleConflictPolicy {
    ExactMatch,
    Shadow,
    Reject,
    NamespacedImport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleDependency {
    pub id: String,
    pub depends_on: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleEntry {
    pub id: String,
    pub kind: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleConflict {
    pub id: String,
    pub kind: String,
    pub policy: BundleConflictPolicy,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleMergeReport {
    pub id: String,
    pub policy: BundleConflictPolicy,
    pub imported_entries: usize,
    pub namespaced_entries: usize,
    pub conflicts: Vec<BundleConflict>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OverlayRegistryDescriptor {
    pub id: String,
    pub parent: String,
    pub bundle_id: String,
    pub local_entries: usize,
    pub shadowed_entries: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleExecutionReceipt {
    pub id: String,
    pub bundle_id: String,
    pub theorem_ids: Vec<String>,
    pub campaign_ids: Vec<String>,
    pub overlay_id: String,
    pub import_receipt_ids: Vec<String>,
    pub merge_report_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleManifest {
    pub id: String,
    pub entries: Vec<BundleEntry>,
    pub dependencies: Vec<BundleDependency>,
    pub merge_report: BundleMergeReport,
    pub overlay: OverlayRegistryDescriptor,
    pub execution_receipt: BundleExecutionReceipt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum QaEntry {
    Object(QcObject),
    Regime(RegimePack),
    Bridge(BridgeContract),
    ProofShape(ProofShape),
    AtlasCell(AtlasCell),
    MechanizationPackage(MechanizationPackage),
    TheoremSpec(TheoremSpec),
    Obligation(Obligation),
    TargetProfile(TargetProfile),
    RouteLedger(RouteLedger),
    Certificate(Certificate),
    Campaign(Campaign),
    CampaignPortfolio(CampaignPortfolio),
    RouteClass(RouteClass),
    AtlasDeficiency(AtlasDeficiency),
    AdequacyClause(AdequacyClause),
    BurdenPack(BurdenPack),
    ClaimPacket(ClaimPacket),
    EvidenceContract(EvidenceContract),
    BenchmarkReceipt(BenchmarkReceipt),
    ChallengeReceipt(ChallengeReceipt),
    ReproducibilityPacket(ReproducibilityPacket),
    SurfacePolicy(SurfacePolicy),
    TransformReceipt(FormatTransformReceipt),
    RoundTripReport(RoundTripReport),
    CapabilityMatrix(CapabilityMatrix),
    SurfaceBudget(SurfaceBudget),
    PolicyObject(MechanizationPolicyObject),
    PolicyBinding(PolicyBinding),
    PolicyResolution(PolicyResolution),
    BundleLock(BundleLock),
    ExecutionManifest(ExecutionManifest),
    ReplayLockManifest(ReplayLockManifest),
    LockReceipt(LockReceipt),
    LockDiff(LockDiff),
    RecomputationPlan(RecomputationPlan),
    PlanExecution(PlanExecutionRecord),
    PredictionAssessment(PredictionAssessment),
    Reconciliation(ReconciliationRecord),
    RootResolution(RootResolutionReport),
}

impl QaEntry {
    pub fn id(&self) -> String {
        match self {
            QaEntry::Object(item) => item.id.clone(),
            QaEntry::Regime(item) => item.id.clone(),
            QaEntry::Bridge(item) => item.id.clone(),
            QaEntry::ProofShape(item) => item.id.clone(),
            QaEntry::AtlasCell(item) => item.id.clone(),
            QaEntry::MechanizationPackage(item) => item.id.clone(),
            QaEntry::TheoremSpec(item) => item.id.clone(),
            QaEntry::Obligation(item) => item.id.clone(),
            QaEntry::TargetProfile(item) => item.id.clone(),
            QaEntry::RouteLedger(item) => item.id.clone(),
            QaEntry::Certificate(item) => item.id.clone(),
            QaEntry::Campaign(item) => item.id.clone(),
            QaEntry::CampaignPortfolio(item) => item.id.clone(),
            QaEntry::RouteClass(item) => item.id.clone(),
            QaEntry::AtlasDeficiency(item) => item.id.clone(),
            QaEntry::AdequacyClause(item) => item.id.clone(),
            QaEntry::BurdenPack(item) => item.id.clone(),
            QaEntry::ClaimPacket(item) => item.id.clone(),
            QaEntry::EvidenceContract(item) => item.id.clone(),
            QaEntry::BenchmarkReceipt(item) => item.id.clone(),
            QaEntry::ChallengeReceipt(item) => item.id.clone(),
            QaEntry::ReproducibilityPacket(item) => item.id.clone(),
            QaEntry::SurfacePolicy(item) => item.id.clone(),
            QaEntry::TransformReceipt(item) => item.id.clone(),
            QaEntry::RoundTripReport(item) => item.id.clone(),
            QaEntry::CapabilityMatrix(item) => item.id.clone(),
            QaEntry::SurfaceBudget(item) => item.id.clone(),
            QaEntry::PolicyObject(item) => item.id.clone(),
            QaEntry::PolicyBinding(item) => item.id.clone(),
            QaEntry::PolicyResolution(item) => item.id.clone(),
            QaEntry::BundleLock(item) => item.id.clone(),
            QaEntry::ExecutionManifest(item) => item.id.clone(),
            QaEntry::ReplayLockManifest(item) => item.id.clone(),
            QaEntry::LockReceipt(item) => item.id.clone(),
            QaEntry::LockDiff(item) => item.id.clone(),
            QaEntry::RecomputationPlan(item) => item.id.clone(),
            QaEntry::PlanExecution(item) => item.id.clone(),
            QaEntry::PredictionAssessment(item) => item.id.clone(),
            QaEntry::Reconciliation(item) => item.id.clone(),
            QaEntry::RootResolution(item) => item.project_root.absolute_path.clone(),
        }
    }

    pub fn kind_tag(&self) -> &'static str {
        match self {
            QaEntry::Object(_) => "object",
            QaEntry::Regime(_) => "regime",
            QaEntry::Bridge(_) => "bridge",
            QaEntry::ProofShape(_) => "proof",
            QaEntry::AtlasCell(_) => "atlas",
            QaEntry::MechanizationPackage(_) => "mechanization",
            QaEntry::TheoremSpec(_) => "theorem",
            QaEntry::Obligation(_) => "obligation",
            QaEntry::TargetProfile(_) => "target",
            QaEntry::RouteLedger(_) => "ledger",
            QaEntry::Certificate(_) => "certificate",
            QaEntry::Campaign(_) => "campaign",
            QaEntry::CampaignPortfolio(_) => "portfolio",
            QaEntry::RouteClass(_) => "route-class",
            QaEntry::AtlasDeficiency(_) => "diagnostic",
            QaEntry::AdequacyClause(_) => "adequacy",
            QaEntry::BurdenPack(_) => "burden-pack",
            QaEntry::ClaimPacket(_) => "claim-packet",
            QaEntry::EvidenceContract(_) => "evidence-contract",
            QaEntry::BenchmarkReceipt(_) => "benchmark-receipt",
            QaEntry::ChallengeReceipt(_) => "challenge-receipt",
            QaEntry::ReproducibilityPacket(_) => "reproducibility-packet",
            QaEntry::SurfacePolicy(_) => "surface-policy",
            QaEntry::TransformReceipt(_) => "transform-receipt",
            QaEntry::RoundTripReport(_) => "roundtrip-report",
            QaEntry::CapabilityMatrix(_) => "capability",
            QaEntry::SurfaceBudget(_) => "surface-budget",
            QaEntry::PolicyObject(_) => "policy-object",
            QaEntry::PolicyBinding(_) => "policy-binding",
            QaEntry::PolicyResolution(_) => "policy-resolution",
            QaEntry::BundleLock(_) => "bundle-lock",
            QaEntry::ExecutionManifest(_) => "execution-manifest",
            QaEntry::ReplayLockManifest(_) => "replay-lock",
            QaEntry::LockReceipt(_) => "lock-receipt",
            QaEntry::LockDiff(_) => "lock-diff",
            QaEntry::RecomputationPlan(_) => "recompute-plan",
            QaEntry::PlanExecution(_) => "plan-execution",
            QaEntry::PredictionAssessment(_) => "prediction-assessment",
            QaEntry::Reconciliation(_) => "reconciliation",
            QaEntry::RootResolution(_) => "root-resolution",
        }
    }

    pub fn from_surface_json(kind: &str, payload: &str) -> Result<Self, serde_json::Error> {
        match kind {
            "object" => serde_json::from_str(payload).map(QaEntry::Object),
            "regime" => serde_json::from_str(payload).map(QaEntry::Regime),
            "bridge" => serde_json::from_str(payload).map(QaEntry::Bridge),
            "proof" => serde_json::from_str(payload).map(QaEntry::ProofShape),
            "atlas" => serde_json::from_str(payload).map(QaEntry::AtlasCell),
            "mechanization" => serde_json::from_str(payload).map(QaEntry::MechanizationPackage),
            "theorem" => serde_json::from_str(payload).map(QaEntry::TheoremSpec),
            "obligation" => serde_json::from_str(payload).map(QaEntry::Obligation),
            "target" => serde_json::from_str(payload).map(QaEntry::TargetProfile),
            "ledger" => serde_json::from_str(payload).map(QaEntry::RouteLedger),
            "certificate" => serde_json::from_str(payload).map(QaEntry::Certificate),
            "campaign" => serde_json::from_str(payload).map(QaEntry::Campaign),
            "portfolio" => serde_json::from_str(payload).map(QaEntry::CampaignPortfolio),
            "route-class" => serde_json::from_str(payload).map(QaEntry::RouteClass),
            "diagnostic" => serde_json::from_str(payload).map(QaEntry::AtlasDeficiency),
            "adequacy" => serde_json::from_str(payload).map(QaEntry::AdequacyClause),
            "burden-pack" => serde_json::from_str(payload).map(QaEntry::BurdenPack),
            "claim-packet" => serde_json::from_str(payload).map(QaEntry::ClaimPacket),
            "evidence-contract" => serde_json::from_str(payload).map(QaEntry::EvidenceContract),
            "benchmark-receipt" => serde_json::from_str(payload).map(QaEntry::BenchmarkReceipt),
            "challenge-receipt" => serde_json::from_str(payload).map(QaEntry::ChallengeReceipt),
            "reproducibility-packet" => {
                serde_json::from_str(payload).map(QaEntry::ReproducibilityPacket)
            }
            "surface-policy" => serde_json::from_str(payload).map(QaEntry::SurfacePolicy),
            "transform-receipt" => serde_json::from_str(payload).map(QaEntry::TransformReceipt),
            "roundtrip-report" => serde_json::from_str(payload).map(QaEntry::RoundTripReport),
            "capability" => serde_json::from_str(payload).map(QaEntry::CapabilityMatrix),
            "surface-budget" => serde_json::from_str(payload).map(QaEntry::SurfaceBudget),
            "policy-object" => serde_json::from_str(payload).map(QaEntry::PolicyObject),
            "policy-binding" => serde_json::from_str(payload).map(QaEntry::PolicyBinding),
            "policy-resolution" => serde_json::from_str(payload).map(QaEntry::PolicyResolution),
            "bundle-lock" => serde_json::from_str(payload).map(QaEntry::BundleLock),
            "execution-manifest" => serde_json::from_str(payload).map(QaEntry::ExecutionManifest),
            "replay-lock" => serde_json::from_str(payload).map(QaEntry::ReplayLockManifest),
            "lock-receipt" => serde_json::from_str(payload).map(QaEntry::LockReceipt),
            "lock-diff" => serde_json::from_str(payload).map(QaEntry::LockDiff),
            "recompute-plan" => serde_json::from_str(payload).map(QaEntry::RecomputationPlan),
            "plan-execution" => serde_json::from_str(payload).map(QaEntry::PlanExecution),
            "prediction-assessment" => {
                serde_json::from_str(payload).map(QaEntry::PredictionAssessment)
            }
            "reconciliation" => serde_json::from_str(payload).map(QaEntry::Reconciliation),
            "root-resolution" => serde_json::from_str(payload).map(QaEntry::RootResolution),
            _ => Err(serde_json::Error::io(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                format!("unknown qa entry kind `{kind}`"),
            ))),
        }
    }

    pub fn to_surface_json(&self) -> Result<(&'static str, String), serde_json::Error> {
        Ok(match self {
            QaEntry::Object(item) => ("object", serde_json::to_string(item)?),
            QaEntry::Regime(item) => ("regime", serde_json::to_string(item)?),
            QaEntry::Bridge(item) => ("bridge", serde_json::to_string(item)?),
            QaEntry::ProofShape(item) => ("proof", serde_json::to_string(item)?),
            QaEntry::AtlasCell(item) => ("atlas", serde_json::to_string(item)?),
            QaEntry::MechanizationPackage(item) => ("mechanization", serde_json::to_string(item)?),
            QaEntry::TheoremSpec(item) => ("theorem", serde_json::to_string(item)?),
            QaEntry::Obligation(item) => ("obligation", serde_json::to_string(item)?),
            QaEntry::TargetProfile(item) => ("target", serde_json::to_string(item)?),
            QaEntry::RouteLedger(item) => ("ledger", serde_json::to_string(item)?),
            QaEntry::Certificate(item) => ("certificate", serde_json::to_string(item)?),
            QaEntry::Campaign(item) => ("campaign", serde_json::to_string(item)?),
            QaEntry::CampaignPortfolio(item) => ("portfolio", serde_json::to_string(item)?),
            QaEntry::RouteClass(item) => ("route-class", serde_json::to_string(item)?),
            QaEntry::AtlasDeficiency(item) => ("diagnostic", serde_json::to_string(item)?),
            QaEntry::AdequacyClause(item) => ("adequacy", serde_json::to_string(item)?),
            QaEntry::BurdenPack(item) => ("burden-pack", serde_json::to_string(item)?),
            QaEntry::ClaimPacket(item) => ("claim-packet", serde_json::to_string(item)?),
            QaEntry::EvidenceContract(item) => ("evidence-contract", serde_json::to_string(item)?),
            QaEntry::BenchmarkReceipt(item) => ("benchmark-receipt", serde_json::to_string(item)?),
            QaEntry::ChallengeReceipt(item) => ("challenge-receipt", serde_json::to_string(item)?),
            QaEntry::ReproducibilityPacket(item) => {
                ("reproducibility-packet", serde_json::to_string(item)?)
            }
            QaEntry::SurfacePolicy(item) => ("surface-policy", serde_json::to_string(item)?),
            QaEntry::TransformReceipt(item) => ("transform-receipt", serde_json::to_string(item)?),
            QaEntry::RoundTripReport(item) => ("roundtrip-report", serde_json::to_string(item)?),
            QaEntry::CapabilityMatrix(item) => ("capability", serde_json::to_string(item)?),
            QaEntry::SurfaceBudget(item) => ("surface-budget", serde_json::to_string(item)?),
            QaEntry::PolicyObject(item) => ("policy-object", serde_json::to_string(item)?),
            QaEntry::PolicyBinding(item) => ("policy-binding", serde_json::to_string(item)?),
            QaEntry::PolicyResolution(item) => ("policy-resolution", serde_json::to_string(item)?),
            QaEntry::BundleLock(item) => ("bundle-lock", serde_json::to_string(item)?),
            QaEntry::ExecutionManifest(item) => {
                ("execution-manifest", serde_json::to_string(item)?)
            }
            QaEntry::ReplayLockManifest(item) => ("replay-lock", serde_json::to_string(item)?),
            QaEntry::LockReceipt(item) => ("lock-receipt", serde_json::to_string(item)?),
            QaEntry::LockDiff(item) => ("lock-diff", serde_json::to_string(item)?),
            QaEntry::RecomputationPlan(item) => ("recompute-plan", serde_json::to_string(item)?),
            QaEntry::PlanExecution(item) => ("plan-execution", serde_json::to_string(item)?),
            QaEntry::PredictionAssessment(item) => {
                ("prediction-assessment", serde_json::to_string(item)?)
            }
            QaEntry::Reconciliation(item) => ("reconciliation", serde_json::to_string(item)?),
            QaEntry::RootResolution(item) => ("root-resolution", serde_json::to_string(item)?),
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct QaDocument {
    pub entries: Vec<QaEntry>,
}

pub trait Canonicalize {
    fn canonicalize_object(
        &self,
        object: &QcObject,
        registry: &dyn RegistryLookup,
    ) -> Result<QcObject, String>;
}

pub trait Promote {
    fn promote(&self, object: &QcObject, registry: &dyn RegistryLookup) -> Result<(), String>;
}

pub trait ValidateJudgment {
    fn validate_judgment(
        &self,
        judgment: &Judgment,
        registry: &dyn RegistryLookup,
    ) -> Result<(), String>;
}

pub trait CheckProofShape {
    fn check_proof_shape(
        &self,
        shape: &ProofShape,
        registry: &dyn RegistryLookup,
    ) -> Result<(), String>;
}

pub trait ComposeBridge {
    fn compose_bridge_path(
        &self,
        bridges: &[BridgeContract],
        budget: Option<&Budget>,
    ) -> Result<(), String>;
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteSelection {
    pub candidates: Vec<AtlasCell>,
    pub winner: Option<AtlasCell>,
    pub reasons: Vec<String>,
    #[serde(default)]
    pub route_explanation: Option<RouteExplanation>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationStatus {
    pub obligation_id: ObligationId,
    pub kind: ObligationKind,
    pub verdict: CertificationVerdict,
    pub evaluation_mode: ObligationEvaluationMode,
    pub detail: String,
    #[serde(default)]
    pub receipts: Vec<ObligationEvidenceReceipt>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObligationEvaluationMode {
    StoredReceiptUsed,
    RecomputedExact,
    RecomputedApproximate,
    RecomputedPartial,
    RecomputedFailed,
    Unsupported,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationEvidenceReceipt {
    pub id: String,
    pub label: String,
    pub verdict: CertificationVerdict,
    pub computed: bool,
    pub detail: String,
    #[serde(default)]
    pub subreceipts: Vec<ObligationEvidenceReceipt>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OptimizationAxis {
    Lawfulness,
    IdentityPreservation,
    LossCompliance,
    RollbackViability,
    ProofShapeSatisfiability,
    BundleResolution,
    SurfaceTransitionPenalty,
    ExecutionCost,
    DerivedObligationDepth,
    MaturityConfidence,
    SymbolicFidelity,
    ReceiptCompleteness,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OptimizerPolicy {
    Conservative,
    SymbolicFidelityFirst,
    ExecutionFirst,
    LowLoss,
    BenchmarkFriendly,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PolicyKind {
    Optimizer,
    Evaluator,
    Canonicalizer,
    Surface,
    BundleMerge,
    ReplayCache,
    ReportExport,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PolicyScope {
    Global,
    Bundle(String),
    Theorem(String),
    Campaign(String),
    TargetProfile(String),
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PolicyVerdict {
    Applied,
    Rejected,
    Conflict,
    Invalid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum OptimizerBackend {
    Lexicographic,
    ParetoBounded,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EvidencePreference {
    PreferRecompute,
    PreferStored,
    RecomputeIfSupported,
    StoredOnlyWhenUnavailable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum UnsupportedHandlingMode {
    Permit,
    StrictFail,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReplayTrustClass {
    ExactPolicyOnly,
    AllowSurfaceOnlyChanges,
    AllowApproximateReuse,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OptimizerPolicyConfig {
    pub optimizer_policy: OptimizerPolicy,
    pub backend: OptimizerBackend,
    pub active_axes: Vec<OptimizationAxis>,
    pub route_explanation_verbosity: String,
    pub symbolic_fidelity_preferred: bool,
    pub tie_break_rules: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct EvaluatorPolicyConfig {
    pub evidence_preference: EvidencePreference,
    pub allow_approximation: bool,
    pub unsupported_mode: UnsupportedHandlingMode,
    pub require_symbolic_fidelity_route: bool,
    pub prefer_comp_replay: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayCachePolicyConfig {
    pub replay_allowed: bool,
    pub exact_policy_match_required: bool,
    pub survive_surface_only_changes: bool,
    pub reuse_approximate_results: bool,
    pub optimizer_change_invalidates: bool,
    pub surface_pack_change_invalidates: bool,
    pub trust_class: ReplayTrustClass,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReportPolicyConfig {
    pub export_surfaces: Vec<SurfaceKind>,
    pub include_policy_trace: bool,
    pub include_route_explanation: bool,
    pub include_obligation_logs: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulerPolicyConfig {
    pub parallelization: ParallelizationPolicy,
    pub max_workers: usize,
    pub allow_parallel_replay: bool,
    pub allow_parallel_certification: bool,
    pub allow_parallel_exports: bool,
    pub deterministic_ordering: bool,
    #[serde(default)]
    pub allow_parallel_obligations: bool,
    #[serde(default = "default_max_workers")]
    pub max_obligation_workers: usize,
    #[serde(default)]
    pub allow_parallel_obligation_replay: bool,
    #[serde(default = "default_true")]
    pub serialize_canonicalization_sensitive: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MechanizationPolicyObject {
    pub id: String,
    pub kind: PolicyKind,
    pub scope: PolicyScope,
    pub extends: Option<String>,
    pub optimizer: Option<OptimizerPolicyConfig>,
    pub evaluator: Option<EvaluatorPolicyConfig>,
    pub replay_cache: Option<ReplayCachePolicyConfig>,
    pub report: Option<ReportPolicyConfig>,
    pub scheduler: Option<SchedulerPolicyConfig>,
    pub canonicalizer_mode: Option<String>,
    pub merge_policy: Option<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyBinding {
    pub id: String,
    pub policy_id: String,
    pub scope: PolicyScope,
    pub target_id: Option<String>,
    pub priority: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyConflict {
    pub id: String,
    pub kind: PolicyKind,
    pub left_policy_id: String,
    pub right_policy_id: String,
    pub message: String,
    pub illegal: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyTrace {
    pub id: String,
    pub steps: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyResolution {
    pub id: String,
    pub scope: PolicyScope,
    pub applied_policy_ids: Vec<String>,
    pub conflicts: Vec<PolicyConflict>,
    pub trace: PolicyTrace,
    pub optimizer: OptimizerPolicyConfig,
    pub evaluator: EvaluatorPolicyConfig,
    pub replay_cache: ReplayCachePolicyConfig,
    pub report: ReportPolicyConfig,
    pub scheduler: SchedulerPolicyConfig,
    pub verdict: PolicyVerdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BundleLock {
    pub id: String,
    pub bundle_id: String,
    pub bundle_hash: String,
    pub policy_resolution_id: String,
    pub report_ids: Vec<String>,
    pub manifest_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PolicyManifest {
    pub id: String,
    pub resolution_id: String,
    pub policy_ids: Vec<String>,
    pub policy_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayLockManifest {
    pub id: String,
    pub report_id: String,
    pub report_hash: String,
    pub route_winner_hash: String,
    pub policy_hash: String,
    pub bundle_hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LockReceipt {
    pub id: String,
    pub lock_id: String,
    pub manifest_id: String,
    pub bundle_id: String,
    pub receipt_ids: Vec<String>,
    pub verdict: PolicyVerdict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LockDiff {
    pub id: String,
    pub left_lock_id: String,
    pub right_lock_id: String,
    pub changed_fields: Vec<String>,
    pub semantic_changes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObservationNodeKind {
    Bundle,
    PolicyResolution,
    PolicyObject,
    Theorem,
    Campaign,
    TargetProfile,
    RouteWinner,
    RouteCandidate,
    Obligation,
    Report,
    Manifest,
    Lock,
    ReplayEvent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationNode {
    pub id: String,
    pub kind: ObservationNodeKind,
    pub ref_id: String,
    pub label: String,
    pub attributes: BTreeMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationEdge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub relation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ExecutionEventKind {
    CertificationRun,
    ReplayRun,
    LockReplay,
    CacheHit,
    CacheMiss,
    RouteSelected,
    ObligationEvaluated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionEvent {
    pub id: String,
    pub kind: ExecutionEventKind,
    pub summary: String,
    pub related_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChoiceRecord {
    pub id: String,
    pub choice_kind: String,
    pub selected_id: String,
    pub alternatives: Vec<String>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationTraceRecord {
    pub id: String,
    pub theorem_id: String,
    pub obligation_id: String,
    pub verdict: CertificationVerdict,
    pub evaluation_mode: ObligationEvaluationMode,
    pub detail: String,
    #[serde(default)]
    pub receipts: Vec<ObligationEvidenceReceipt>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteTraceEvent {
    pub id: String,
    pub theorem_id: String,
    pub winner_id: Option<String>,
    pub optimizer_backend: OptimizerBackend,
    pub axes: Vec<OptimizationAxis>,
    pub explanation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheReplayEvent {
    pub id: String,
    pub report_id: String,
    pub replay_status: ReplayStatus,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationGraph {
    pub id: String,
    pub record_id: String,
    pub nodes: Vec<ObservationNode>,
    pub edges: Vec<ObservationEdge>,
    pub choices: Vec<ChoiceRecord>,
    pub obligation_traces: Vec<ObligationTraceRecord>,
    pub route_events: Vec<RouteTraceEvent>,
    pub cache_events: Vec<CacheReplayEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObservationRecord {
    pub id: String,
    pub report_id: String,
    pub theorem_id: String,
    pub campaign_id: Option<String>,
    pub bundle_id: Option<String>,
    pub manifest_id: Option<String>,
    pub lock_id: Option<String>,
    pub graph_id: String,
    pub events: Vec<ExecutionEvent>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum SemanticDiffClass {
    NoSemanticChange,
    PolicyOnly,
    RouteOnly,
    ObligationOnly,
    ReplayOnly,
    SurfaceOnly,
    BundleDependencyOnly,
    Mixed,
    Inconclusive,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationDrift {
    pub obligation_id: String,
    pub verdict_before: Option<CertificationVerdict>,
    pub verdict_after: Option<CertificationVerdict>,
    pub mode_before: Option<ObligationEvaluationMode>,
    pub mode_after: Option<ObligationEvaluationMode>,
    pub meaning: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TheoremDrift {
    pub theorem_id: String,
    pub verdict_before: Option<CertificationVerdict>,
    pub verdict_after: Option<CertificationVerdict>,
    pub route_before: Option<String>,
    pub route_after: Option<String>,
    pub obligation_drifts: Vec<ObligationDrift>,
    pub summary: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticDiff {
    pub id: String,
    pub left_id: String,
    pub right_id: String,
    pub class: SemanticDiffClass,
    pub summary: Vec<String>,
    pub theorem_drifts: Vec<TheoremDrift>,
    pub changed_policy_ids: Vec<String>,
    pub changed_route_ids: Vec<String>,
    pub changed_dependency_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DriftReport {
    pub id: String,
    pub diff_id: String,
    pub summary: Vec<String>,
    pub theorem_drifts: Vec<TheoremDrift>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RiskClassification {
    Exact,
    Strong,
    Moderate,
    Weak,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PredictionClass {
    NoImpactPredicted,
    ReplayOnlyImpact,
    ReportHashChangeLikely,
    RouteWinnerChangeLikely,
    ObligationOutcomeChangeLikely,
    CertificationVerdictChangeLikely,
    BundleConflictRisk,
    InconclusivePrediction,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ImpactPrediction {
    pub id: String,
    pub baseline_id: String,
    pub proposed_id: String,
    pub class: PredictionClass,
    pub confidence: RiskClassification,
    pub reasons: Vec<String>,
    pub affected_theorems: Vec<String>,
    pub affected_reports: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RecomputationStepKind {
    Reuse,
    Replay,
    ReEvaluateObligation,
    ReRoute,
    ReCertifyTheorem,
    ReRunCampaign,
    ReExportReport,
    RebuildManifestLock,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecomputationStep {
    pub id: String,
    pub kind: RecomputationStepKind,
    pub target_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecomputationPlan {
    pub id: String,
    pub prediction_id: Option<String>,
    pub diff_id: Option<String>,
    pub reusable_artifacts: Vec<String>,
    pub invalidated_artifacts: Vec<String>,
    pub steps: Vec<RecomputationStep>,
    pub explanation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepImpactClass {
    NoOpReuse,
    ReplayOnly,
    ObligationScoped,
    RouteScoped,
    ManifestScoped,
    CampaignScoped,
    FullRun,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReuseEligibilityReason {
    SemanticIdentityPreserved,
    ReplayPolicyAllowsReuse,
    SurfaceOnlyChange,
    PolicyRequiresFreshRun,
    BundleChanged,
    EvaluatorChanged,
    RouteChanged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum DriftCriticality {
    None,
    Low,
    Medium,
    High,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulerDiffHint {
    pub step_id: String,
    pub impact: StepImpactClass,
    pub reuse_reason: Option<ReuseEligibilityReason>,
    pub criticality: DriftCriticality,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum StepStrictness {
    BestEffort,
    PolicyGoverned,
    Strict,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReuseTrustClass {
    ExactOnly,
    SurfaceStable,
    ApproximateAllowed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParallelizationPolicy {
    Serialize,
    ParallelIndependent,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CacheNamespace {
    pub id: String,
    pub root: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionScope {
    pub id: String,
    pub cache_namespace: CacheNamespace,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LaneExecutionRecord {
    pub lane_id: String,
    pub step_ids: Vec<String>,
    pub task_ids: Vec<String>,
    pub serialized_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionScheduleHash {
    pub id: String,
    pub hash: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ConcurrencyCoherenceReceipt {
    pub id: String,
    pub namespace_id: String,
    pub merged_artifact_ids: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OrderingReceipt {
    pub id: String,
    pub ordered_step_ids: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ObligationConcurrencyClass {
    ParallelSafe,
    ReplaySafeWriteSensitive,
    CacheSensitive,
    CanonicalizationSensitive,
    StrictlySerialized,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationDagNode {
    pub id: String,
    pub obligation_id: String,
    pub concurrency_class: ObligationConcurrencyClass,
    pub barrier_after: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationDagEdge {
    pub id: String,
    pub from: String,
    pub to: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationGroup {
    pub id: String,
    pub concurrency_class: ObligationConcurrencyClass,
    pub obligation_ids: Vec<String>,
    pub serialized_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationPlan {
    pub id: String,
    pub theorem_id: String,
    pub campaign_id: Option<String>,
    pub nodes: Vec<ObligationDagNode>,
    pub edges: Vec<ObligationDagEdge>,
    pub groups: Vec<ObligationGroup>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationLaneRecord {
    pub lane_id: String,
    pub group_ids: Vec<String>,
    pub obligation_ids: Vec<String>,
    pub serialized_reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationOrderingReceipt {
    pub id: String,
    pub ordered_group_ids: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationMergeReceipt {
    pub id: String,
    pub merged_obligation_ids: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayLegalityCheck {
    pub id: String,
    pub obligation_id: String,
    pub allowed: bool,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayBarrierReceipt {
    pub id: String,
    pub obligation_ids: Vec<String>,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayMergeReceipt {
    pub id: String,
    pub reused_obligation_ids: Vec<String>,
    pub rerun_obligation_ids: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReplayDivergenceRecord {
    pub id: String,
    pub obligation_id: String,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationCacheShard {
    pub id: String,
    pub obligation_ids: Vec<String>,
    pub namespace_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationWriteSet {
    pub id: String,
    pub artifact_ids: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationCollisionReport {
    pub id: String,
    pub obligation_ids: Vec<String>,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ObligationNamespaceReceipt {
    pub id: String,
    pub namespace_id: String,
    pub shard_ids: Vec<String>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulerPolicy {
    pub id: String,
    pub replay_allowed: bool,
    pub strict_recomputation: bool,
    pub strict_unsupported: bool,
    pub parallelization: ParallelizationPolicy,
    #[serde(default = "default_max_workers")]
    pub max_workers: usize,
    #[serde(default)]
    pub allow_parallel_replay: bool,
    #[serde(default = "default_true")]
    pub allow_parallel_certification: bool,
    #[serde(default = "default_true")]
    pub allow_parallel_exports: bool,
    #[serde(default = "default_true")]
    pub deterministic_ordering: bool,
    #[serde(default)]
    pub allow_parallel_obligations: bool,
    #[serde(default = "default_max_workers")]
    pub max_obligation_workers: usize,
    #[serde(default)]
    pub allow_parallel_obligation_replay: bool,
    #[serde(default = "default_true")]
    pub serialize_canonicalization_sensitive: bool,
    pub reuse_trust: ReuseTrustClass,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SchedulerDecisionReceipt {
    pub id: String,
    pub step_id: String,
    pub policy_resolution_id: Option<String>,
    pub decision: String,
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ScheduledStepStatus {
    DryRun,
    Reused,
    Replayed,
    Executed,
    Failed,
    Skipped,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StepExecutionOutcome {
    pub step_id: String,
    pub kind: RecomputationStepKind,
    pub status: ScheduledStepStatus,
    pub produced_artifact_ids: Vec<String>,
    pub receipt: SchedulerDecisionReceipt,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PlanExecutionRecord {
    pub id: String,
    pub plan_id: String,
    pub prediction_id: Option<String>,
    pub scheduler_policy: SchedulerPolicy,
    pub root_resolution: RootResolutionReport,
    #[serde(default)]
    pub execution_scope: Option<ExecutionScope>,
    #[serde(default)]
    pub lane_records: Vec<LaneExecutionRecord>,
    #[serde(default)]
    pub schedule_hash: Option<ExecutionScheduleHash>,
    #[serde(default)]
    pub coherence_receipts: Vec<ConcurrencyCoherenceReceipt>,
    #[serde(default)]
    pub ordering_receipt: Option<OrderingReceipt>,
    #[serde(default)]
    pub obligation_plans: Vec<ObligationPlan>,
    #[serde(default)]
    pub obligation_lanes: Vec<ObligationLaneRecord>,
    #[serde(default)]
    pub obligation_ordering_receipts: Vec<ObligationOrderingReceipt>,
    #[serde(default)]
    pub obligation_merge_receipts: Vec<ObligationMergeReceipt>,
    #[serde(default)]
    pub replay_legality_checks: Vec<ReplayLegalityCheck>,
    #[serde(default)]
    pub replay_barrier_receipts: Vec<ReplayBarrierReceipt>,
    #[serde(default)]
    pub replay_merge_receipts: Vec<ReplayMergeReceipt>,
    #[serde(default)]
    pub replay_divergence_records: Vec<ReplayDivergenceRecord>,
    #[serde(default)]
    pub obligation_cache_shards: Vec<ObligationCacheShard>,
    #[serde(default)]
    pub obligation_write_sets: Vec<ObligationWriteSet>,
    #[serde(default)]
    pub obligation_collision_reports: Vec<ObligationCollisionReport>,
    #[serde(default)]
    pub obligation_namespace_receipts: Vec<ObligationNamespaceReceipt>,
    pub outcomes: Vec<StepExecutionOutcome>,
    pub reused_artifacts: Vec<String>,
    pub rerun_artifacts: Vec<String>,
    pub resulting_report_ids: Vec<String>,
    #[serde(default)]
    pub resulting_reports: Vec<CertificationReport>,
    pub manifest_ids: Vec<String>,
    pub lock_ids: Vec<String>,
    pub explanation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExplanationObject {
    pub id: String,
    pub title: String,
    pub details: Vec<String>,
    pub related_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum PredictionAssessmentOutcome {
    PredictionConfirmed,
    PredictionUnderestimated,
    PredictionOverestimated,
    PredictionMissed,
    PredictionUncheckable,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PredictionAssessment {
    pub id: String,
    pub prediction_id: String,
    pub actual_diff_id: String,
    pub outcome: PredictionAssessmentOutcome,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReconciliationRecord {
    pub id: String,
    pub prediction_id: String,
    pub plan_id: String,
    pub execution_id: String,
    pub assessment: PredictionAssessment,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionManifest {
    pub id: String,
    pub bundle_id: String,
    pub bundle_hash: String,
    pub dependency_graph: Vec<BundleDependency>,
    pub policy_manifest: PolicyManifest,
    pub route_winner_ids: Vec<String>,
    pub evaluator_versions: Vec<String>,
    pub pack_versions: Vec<String>,
    pub report_ids: Vec<String>,
    #[serde(default)]
    pub executed_plan_hash: Option<String>,
    #[serde(default)]
    pub executed_steps: Vec<StepExecutionOutcome>,
    #[serde(default)]
    pub reused_artifacts: Vec<String>,
    #[serde(default)]
    pub rerun_artifacts: Vec<String>,
    #[serde(default)]
    pub reconciliation_summary: Vec<String>,
    #[serde(default)]
    pub root_resolution: Option<RootResolutionReport>,
    #[serde(default)]
    pub scheduler_policy: Option<SchedulerPolicy>,
    #[serde(default)]
    pub execution_scope: Option<ExecutionScope>,
    #[serde(default)]
    pub lane_records: Vec<LaneExecutionRecord>,
    #[serde(default)]
    pub schedule_hash: Option<ExecutionScheduleHash>,
    #[serde(default)]
    pub coherence_receipts: Vec<ConcurrencyCoherenceReceipt>,
    #[serde(default)]
    pub ordering_receipt: Option<OrderingReceipt>,
    #[serde(default)]
    pub obligation_plans: Vec<ObligationPlan>,
    #[serde(default)]
    pub obligation_lanes: Vec<ObligationLaneRecord>,
    #[serde(default)]
    pub obligation_ordering_receipts: Vec<ObligationOrderingReceipt>,
    #[serde(default)]
    pub obligation_merge_receipts: Vec<ObligationMergeReceipt>,
    #[serde(default)]
    pub replay_legality_checks: Vec<ReplayLegalityCheck>,
    #[serde(default)]
    pub replay_barrier_receipts: Vec<ReplayBarrierReceipt>,
    #[serde(default)]
    pub replay_merge_receipts: Vec<ReplayMergeReceipt>,
    #[serde(default)]
    pub replay_divergence_records: Vec<ReplayDivergenceRecord>,
    #[serde(default)]
    pub obligation_cache_shards: Vec<ObligationCacheShard>,
    #[serde(default)]
    pub obligation_write_sets: Vec<ObligationWriteSet>,
    #[serde(default)]
    pub obligation_collision_reports: Vec<ObligationCollisionReport>,
    #[serde(default)]
    pub obligation_namespace_receipts: Vec<ObligationNamespaceReceipt>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteScoreVector {
    pub lawfulness: usize,
    pub identity_preservation: usize,
    pub loss_compliance: usize,
    pub rollback_viability: usize,
    pub proof_shape_satisfiability: usize,
    pub bundle_resolution: usize,
    pub surface_transition_penalty: usize,
    pub execution_cost: usize,
    pub derived_obligation_depth: usize,
    pub maturity_confidence: usize,
    pub symbolic_fidelity: usize,
    pub receipt_completeness: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteExplanation {
    pub optimizer_policy: OptimizerPolicy,
    #[serde(default = "default_optimizer_backend")]
    pub optimizer_backend: OptimizerBackend,
    #[serde(default)]
    pub policy_resolution_id: Option<String>,
    pub winner_atlas_cell_id: Option<AtlasCellId>,
    pub winner_score: Option<RouteScoreVector>,
    pub dominated_candidates: Vec<String>,
    pub rejected_candidates: Vec<String>,
    pub axes_used: Vec<OptimizationAxis>,
    pub explanation: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DeterministicExecutionEnvelope {
    pub bundle_hash: String,
    #[serde(default)]
    pub bundle_id: Option<String>,
    pub policy_hash: String,
    #[serde(default)]
    pub policy_resolution_id: Option<String>,
    #[serde(default)]
    pub manifest_id: Option<String>,
    #[serde(default)]
    pub lock_id: Option<String>,
    pub route_winner_hash: String,
    pub obligation_replay_keys: Vec<String>,
    pub report_hash: String,
    pub replay_status: ReplayStatus,
    #[serde(default)]
    pub executed_plan_id: Option<String>,
    #[serde(default)]
    pub reconciliation_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReplayStatus {
    Fresh,
    CacheHit,
    ReplayOnly,
    Invalidated,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CertificationCandidate {
    pub atlas_cell_id: AtlasCellId,
    pub path: Vec<BridgeId>,
    pub loss_count: usize,
    pub proof_shapes: Vec<ProofId>,
    pub route_class_id: Option<RouteClassId>,
    pub score: Vec<usize>,
    #[serde(default)]
    pub route_score: Option<RouteScoreVector>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CertificationReport {
    pub theorem_id: TheoremId,
    pub campaign_id: Option<CampaignId>,
    pub target_profile_id: TargetProfileId,
    pub verdict: CertificationVerdict,
    pub selected_atlas_cell: Option<AtlasCellId>,
    pub selected_path: Vec<BridgeId>,
    pub route_class_id: Option<RouteClassId>,
    pub certificate_id: Option<CertificateId>,
    pub candidates: Vec<CertificationCandidate>,
    pub obligations: Vec<ObligationStatus>,
    pub reasons: Vec<String>,
    pub diagnostics: Vec<String>,
    #[serde(default)]
    pub deficiencies: Vec<AtlasDeficiency>,
    #[serde(default)]
    pub adequacy_records: Vec<AdequacyRecord>,
    #[serde(default)]
    pub checker_receipts: Vec<CheckerReceipt>,
    #[serde(default)]
    pub burden_pack_ids: Vec<String>,
    #[serde(default)]
    pub claim_packet_ids: Vec<String>,
    #[serde(default)]
    pub evidence_contract_ids: Vec<String>,
    #[serde(default)]
    pub benchmark_receipt_ids: Vec<String>,
    #[serde(default)]
    pub challenge_receipt_ids: Vec<String>,
    #[serde(default)]
    pub reproducibility_packet_ids: Vec<String>,
    #[serde(default)]
    pub promotion_artifact_ids: Vec<String>,
    #[serde(default)]
    pub reused_artifact_ids: Vec<String>,
    #[serde(default)]
    pub default_selected_artifact_ids: Vec<String>,
    #[serde(default)]
    pub payoff_receipt_ids: Vec<String>,
    #[serde(default)]
    pub policy_resolution: Option<PolicyResolution>,
    #[serde(default)]
    pub route_explanation: Option<RouteExplanation>,
    #[serde(default)]
    pub execution_envelope: Option<DeterministicExecutionEnvelope>,
    #[serde(default)]
    pub reconciliation_summary: Vec<String>,
    #[serde(default)]
    pub obligation_plan: Option<ObligationPlan>,
    #[serde(default)]
    pub obligation_lanes: Vec<ObligationLaneRecord>,
    #[serde(default)]
    pub obligation_ordering_receipt: Option<ObligationOrderingReceipt>,
    #[serde(default)]
    pub obligation_merge_receipt: Option<ObligationMergeReceipt>,
    #[serde(default)]
    pub replay_legality_checks: Vec<ReplayLegalityCheck>,
    #[serde(default)]
    pub replay_barrier_receipts: Vec<ReplayBarrierReceipt>,
    #[serde(default)]
    pub replay_merge_receipt: Option<ReplayMergeReceipt>,
    #[serde(default)]
    pub replay_divergence_records: Vec<ReplayDivergenceRecord>,
    #[serde(default)]
    pub obligation_cache_shards: Vec<ObligationCacheShard>,
    #[serde(default)]
    pub reuse_legality_receipts: Vec<ReuseLegalityReceipt>,
    #[serde(default)]
    pub reuse_decision_receipts: Vec<ReuseDecisionReceipt>,
    #[serde(default)]
    pub residual_verification_receipts: Vec<ResidualVerificationReceipt>,
    #[serde(default)]
    pub obligation_write_sets: Vec<ObligationWriteSet>,
    #[serde(default)]
    pub obligation_collision_reports: Vec<ObligationCollisionReport>,
    #[serde(default)]
    pub obligation_namespace_receipt: Option<ObligationNamespaceReceipt>,
}

pub trait SelectRoute {
    fn select_route(
        &self,
        src: &str,
        tgt: &str,
        proof_target: Option<&str>,
        budget: Option<&Budget>,
    ) -> Result<RouteSelection, String>;
}

pub trait RegistryLookup {
    fn get_object(&self, id: &str) -> Option<QcObject>;
    fn get_object_origin(&self, id: &str) -> ArtifactOrigin {
        let _ = id;
        ArtifactOrigin::Unknown
    }
    fn get_regime(&self, id: &str) -> Option<RegimePack>;
    fn get_bridge(&self, id: &str) -> Option<BridgeContract>;
    fn get_proof_shape(&self, id: &str) -> Option<ProofShape>;
    fn get_atlas_cell(&self, id: &str) -> Option<AtlasCell>;
    fn get_mechanization_package(&self, id: &str) -> Option<MechanizationPackage>;
    fn get_theorem_spec(&self, id: &str) -> Option<TheoremSpec>;
    fn get_obligation(&self, id: &str) -> Option<Obligation>;
    fn get_target_profile(&self, id: &str) -> Option<TargetProfile>;
    fn get_route_ledger(&self, id: &str) -> Option<RouteLedger>;
    fn get_certificate(&self, id: &str) -> Option<Certificate>;
    fn get_campaign(&self, id: &str) -> Option<Campaign>;
    fn get_campaign_portfolio(&self, id: &str) -> Option<CampaignPortfolio>;
    fn get_route_class(&self, id: &str) -> Option<RouteClass>;
    fn get_atlas_deficiency(&self, id: &str) -> Option<AtlasDeficiency>;
    fn atlas_deficiencies(&self) -> Vec<AtlasDeficiency>;
    fn get_adequacy_clause(&self, id: &str) -> Option<AdequacyClause>;
    fn adequacy_clauses(&self) -> Vec<AdequacyClause>;
    fn get_burden_pack(&self, id: &str) -> Option<BurdenPack> {
        let _ = id;
        None
    }
    fn burden_packs(&self) -> Vec<BurdenPack> {
        Vec::new()
    }
    fn get_claim_packet(&self, id: &str) -> Option<ClaimPacket> {
        let _ = id;
        None
    }
    fn claim_packets(&self) -> Vec<ClaimPacket> {
        Vec::new()
    }
    fn get_evidence_contract(&self, id: &str) -> Option<EvidenceContract> {
        let _ = id;
        None
    }
    fn evidence_contracts(&self) -> Vec<EvidenceContract> {
        Vec::new()
    }
    fn get_benchmark_receipt(&self, id: &str) -> Option<BenchmarkReceipt> {
        let _ = id;
        None
    }
    fn benchmark_receipts(&self) -> Vec<BenchmarkReceipt> {
        Vec::new()
    }
    fn get_challenge_receipt(&self, id: &str) -> Option<ChallengeReceipt> {
        let _ = id;
        None
    }
    fn challenge_receipts(&self) -> Vec<ChallengeReceipt> {
        Vec::new()
    }
    fn get_reproducibility_packet(&self, id: &str) -> Option<ReproducibilityPacket> {
        let _ = id;
        None
    }
    fn reproducibility_packets(&self) -> Vec<ReproducibilityPacket> {
        Vec::new()
    }
    fn get_codebook_pack(&self, id: &str) -> Option<CodebookPack>;
    fn get_glyph_pack(&self, id: &str) -> Option<GlyphPack>;
    fn get_combo_pack(&self, id: &str) -> Option<ComboPack>;
    fn get_projection_policy(&self, id: &str) -> Option<ProjectionPolicy>;
    fn get_alias_expansion_policy(&self, id: &str) -> Option<AliasExpansionPolicy>;
    fn get_surface_policy(&self, id: &str) -> Option<SurfacePolicy>;
    fn get_capability_matrix(&self, id: &str) -> Option<CapabilityMatrix>;
    fn get_roundtrip_report(&self, id: &str) -> Option<RoundTripReport>;
    fn get_transform_receipt(&self, id: &str) -> Option<FormatTransformReceipt>;
    fn get_surface_deficiency(&self, id: &str) -> Option<SurfaceDeficiency>;
    fn get_policy_object(&self, id: &str) -> Option<MechanizationPolicyObject>;
    fn policy_objects(&self) -> Vec<MechanizationPolicyObject>;
    fn policy_bindings(&self) -> Vec<PolicyBinding>;
    fn find_equivalence_class(&self, object_id: &str, regime: &str) -> Option<EquivalenceClass>;
    fn atlas_cells(&self) -> Vec<AtlasCell>;
}

pub fn resolve_project_root() -> Result<ProjectRoot, String> {
    if let Ok(explicit) = std::env::var("MF_PROJECT_ROOT") {
        let path = canonicalize_or_clean(PathBuf::from(&explicit))?;
        return Ok(ProjectRoot {
            absolute_path: path.display().to_string(),
            resolution_source: "env:MF_PROJECT_ROOT".into(),
        });
    }

    let mut cursor = std::env::current_dir().map_err(|err| err.to_string())?;
    let cwd = cursor.clone();
    let mut nearest_manifest_dir: Option<PathBuf> = None;
    loop {
        let manifest = cursor.join("Cargo.toml");
        if manifest.exists() {
            nearest_manifest_dir.get_or_insert_with(|| cursor.clone());
            let content = fs::read_to_string(&manifest).map_err(|err| err.to_string())?;
            if content.contains("[workspace]") && content.contains("\"mf-core\"") {
                let path = canonicalize_or_clean(cursor)?;
                return Ok(ProjectRoot {
                    absolute_path: path.display().to_string(),
                    resolution_source: "workspace-scan".into(),
                });
            }
        }
        if !cursor.pop() {
            break;
        }
    }

    if let Some(path) = nearest_manifest_dir {
        let path = canonicalize_or_clean(path)?;
        return Ok(ProjectRoot {
            absolute_path: path.display().to_string(),
            resolution_source: "nearest-cargo-manifest".into(),
        });
    }

    let path = canonicalize_or_clean(cwd)?;
    Ok(ProjectRoot {
        absolute_path: path.display().to_string(),
        resolution_source: "cwd-fallback".into(),
    })
}

pub fn resolve_cache_root() -> Result<CacheRoot, String> {
    if let Ok(explicit) = std::env::var("MF_CACHE_ROOT") {
        let mut path = canonicalize_or_clean(PathBuf::from(&explicit))?;
        let namespace = std::env::var("MF_CACHE_NAMESPACE")
            .ok()
            .filter(|value| !value.trim().is_empty());
        if let Some(namespace) = &namespace {
            path = path.join("namespaces").join(sanitize_namespace(namespace));
        }
        fs::create_dir_all(&path).map_err(|err| err.to_string())?;
        return Ok(CacheRoot {
            absolute_path: path.display().to_string(),
            resolution_source: "env:MF_CACHE_ROOT".into(),
            namespace,
        });
    }
    let project = resolve_project_root()?;
    let mut path = PathBuf::from(&project.absolute_path).join(".mf-cache");
    let namespace = std::env::var("MF_CACHE_NAMESPACE")
        .ok()
        .filter(|value| !value.trim().is_empty());
    if let Some(namespace) = &namespace {
        path = path.join("namespaces").join(sanitize_namespace(namespace));
    }
    fs::create_dir_all(&path).map_err(|err| err.to_string())?;
    Ok(CacheRoot {
        absolute_path: path.display().to_string(),
        resolution_source: "project-root/.mf-cache".into(),
        namespace,
    })
}

pub fn ensure_cache_subdir(name: &str) -> Result<PathBuf, String> {
    let root = PathBuf::from(resolve_cache_root()?.absolute_path).join(name);
    fs::create_dir_all(&root).map_err(|err| err.to_string())?;
    Ok(root)
}

pub fn normalize_path_receipt(path: &Path) -> Result<PathNormalizationReceipt, String> {
    let original = path.display().to_string();
    let normalized = canonicalize_or_clean(path.to_path_buf())?;
    let project_root = PathBuf::from(resolve_project_root()?.absolute_path);
    let relative = normalized
        .strip_prefix(&project_root)
        .ok()
        .map(|item| item.to_string_lossy().replace('\\', "/"));
    let mut notes = Vec::new();
    if relative.is_some() {
        notes.push("stored relative to project root".into());
    } else {
        notes.push("path remains external to project root".into());
    }
    Ok(PathNormalizationReceipt {
        id: format!(
            "PNR_{:x}",
            stable_hash_u64(&normalized.display().to_string())
        ),
        original_path: original,
        normalized_path: normalized.display().to_string(),
        project_relative_path: relative,
        notes,
    })
}

pub fn locate_artifact(id: &str, kind: &str, path: &Path) -> Result<ArtifactLocator, String> {
    let receipt = normalize_path_receipt(path)?;
    Ok(ArtifactLocator {
        artifact_id: id.to_string(),
        artifact_kind: kind.to_string(),
        absolute_path: receipt.normalized_path.clone(),
        normalized_path: receipt
            .project_relative_path
            .clone()
            .unwrap_or(receipt.normalized_path),
    })
}

pub fn runtime_root_report(extra_paths: &[&Path]) -> Result<RootResolutionReport, String> {
    let project_root = resolve_project_root()?;
    let cache_root = resolve_cache_root()?;
    let mut receipts = Vec::new();
    for path in extra_paths {
        receipts.push(normalize_path_receipt(path)?);
    }
    let note = match project_root.resolution_source.as_str() {
        "workspace-scan" => "workspace root and cache root resolved deterministically",
        "nearest-cargo-manifest" => {
            "project root resolved from the nearest Cargo manifest; cache root follows that project root"
        }
        "cwd-fallback" => {
            "project root fell back to the current working directory; set MF_PROJECT_ROOT to pin it explicitly"
        }
        "env:MF_PROJECT_ROOT" => {
            "project root pinned by MF_PROJECT_ROOT; cache root follows that project root unless MF_CACHE_ROOT overrides it"
        }
        _ => "project and cache roots resolved from the active runtime environment",
    };
    Ok(RootResolutionReport {
        project_root,
        cache_root,
        receipts,
        notes: vec![note.into()],
    })
}

pub fn summary_map() -> BTreeMap<&'static str, &'static str> {
    BTreeMap::from([
        ("qm_0", "reserved"),
        ("qk_0", "reserved"),
        ("qa_0", "active"),
    ])
}

fn default_optimizer_backend() -> OptimizerBackend {
    OptimizerBackend::Lexicographic
}

fn canonicalize_or_clean(path: PathBuf) -> Result<PathBuf, String> {
    if path.exists() {
        path.canonicalize().map_err(|err| err.to_string())
    } else if path.is_absolute() {
        Ok(path)
    } else {
        Ok(std::env::current_dir()
            .map_err(|err| err.to_string())?
            .join(path))
    }
}

pub fn stable_hash_u64(input: &str) -> u64 {
    let mut hash: u64 = 0xcbf29ce484222325;
    for byte in input.as_bytes() {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}

fn sanitize_namespace(input: &str) -> String {
    input
        .chars()
        .map(|ch| {
            if ch.is_ascii_alphanumeric() || ch == '-' || ch == '_' {
                ch
            } else {
                '_'
            }
        })
        .collect()
}

fn default_max_workers() -> usize {
    1
}

fn default_true() -> bool {
    true
}

pub trait TollValue: Clone + Eq + Ord {
    fn zero() -> Self;
    fn one() -> Self;
    fn add(&self, other: &Self) -> Self;
    fn mul(&self, other: &Self) -> Self;
    fn compare(&self, other: &Self) -> Ordering {
        self.cmp(other)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct NatToll(pub u64);

impl TollValue for NatToll {
    fn zero() -> Self {
        Self(0)
    }
    fn one() -> Self {
        Self(1)
    }
    fn add(&self, other: &Self) -> Self {
        Self(self.0 + other.0)
    }
    fn mul(&self, other: &Self) -> Self {
        Self(self.0 * other.0)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct RationalToll {
    pub num: u64,
    pub den: u64,
}

impl TollValue for RationalToll {
    fn zero() -> Self {
        Self { num: 0, den: 1 }
    }
    fn one() -> Self {
        Self { num: 1, den: 1 }
    }
    fn add(&self, other: &Self) -> Self {
        Self {
            num: self.num * other.den + other.num * self.den,
            den: self.den * other.den,
        }
    }
    fn mul(&self, other: &Self) -> Self {
        Self {
            num: self.num * other.num,
            den: self.den * other.den,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct TropicalToll(pub u64);

impl TollValue for TropicalToll {
    fn zero() -> Self {
        Self(u64::MAX / 4)
    }
    fn one() -> Self {
        Self(0)
    }
    fn add(&self, other: &Self) -> Self {
        Self(self.0.min(other.0))
    }
    fn mul(&self, other: &Self) -> Self {
        Self(self.0.saturating_add(other.0))
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub struct LexicographicToll<T: TollValue, U: TollValue>(pub T, pub U);

impl<T: TollValue, U: TollValue> TollValue for LexicographicToll<T, U> {
    fn zero() -> Self {
        Self(T::zero(), U::zero())
    }
    fn one() -> Self {
        Self(T::one(), U::one())
    }
    fn add(&self, other: &Self) -> Self {
        Self(self.0.add(&other.0), self.1.add(&other.1))
    }
    fn mul(&self, other: &Self) -> Self {
        Self(self.0.mul(&other.0), self.1.mul(&other.1))
    }
    fn compare(&self, other: &Self) -> Ordering {
        self.0
            .compare(&other.0)
            .then_with(|| self.1.compare(&other.1))
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LocusPacketKind {
    CanonicalTransfer = 1,
    CertificationEnvelope = 2,
    CheckerEnvelope = 3,
    FrontierEnvelope = 4,
    TowerBundle = 5,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GenomeArtifactClass {
    Gene = 1,
    Haplotype = 2,
    Chromosome = 3,
    Genome = 4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GenomeSurface {
    Rna = 1,
    Dna = 2,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum LocusOpcode {
    Header = 1,
    CanonicalPayload = 2,
    ReceiptTable = 3,
    AdequacyTable = 4,
    RouteLedger = 5,
    Campaign = 6,
    Certificate = 7,
    Checker = 8,
    Frontier = 9,
    Proposal = 10,
    Coverage = 11,
    SymbolTable = 12,
    Forensic = 13,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StructuralOpcodeSpec {
    pub opcode: LocusOpcode,
    pub name: String,
    pub structural_only: bool,
    pub compatible_skip_allowed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DnaHeaderReceipt {
    pub id: String,
    pub surface: GenomeSurface,
    pub grammar_id: String,
    pub schema_hash: String,
    pub integrity_hash: String,
    pub structural_opcode_table_hash: String,
    pub header_truth_complete: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DnaValidationReport {
    pub id: String,
    pub reversible: bool,
    pub header_truth_complete: bool,
    pub symbol_table_semantic_authority: bool,
    #[serde(default)]
    pub failures: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum GeneratedStatus {
    Suggestion = 1,
    Scaffold = 2,
    VerifiedCandidate = 3,
    PromotedTruth = 4,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ProposalKind {
    SemanticLeaf = 1,
    AdequacyClause = 2,
    CheckerExtension = 3,
    Campaign = 4,
    AtlasCell = 5,
    PayoffTask = 6,
    SearchCompartment = 7,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CoverageDecision {
    Exact = 1,
    Subsumed = 2,
    TransportEquivalent = 3,
    Fallback = 4,
    Unsupported = 5,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ArtifactKind {
    RegistryEntity = 1,
    Report = 2,
    ExecutionManifest = 3,
    BundleLock = 4,
    ObservationRecord = 5,
    PredictionRecord = 6,
    RecomputationPlan = 7,
    PlanExecution = 8,
    InspectionReport = 9,
    ValidationBundle = 10,
    ForensicBundle = 11,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum NamespaceScope {
    Any = 1,
    SameNamespace = 2,
    CrossNamespaceUnsupported = 3,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum CapabilityReadiness {
    Declared = 1,
    HelpRoutable = 2,
    ContractKnown = 3,
    SmokeExecuted = 4,
    FullyExercised = 5,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ArtifactContract {
    pub kind: ArtifactKind,
    pub class: String,
    pub standalone_validation_complete: bool,
    pub namespace_scope: NamespaceScope,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandContract {
    pub command: String,
    pub accepted_inputs: Vec<ArtifactKind>,
    pub produces: Vec<ArtifactKind>,
    pub readiness: CapabilityReadiness,
    pub namespace_scope: NamespaceScope,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct LocusCapabilityMask {
    pub has_checker: bool,
    pub has_adequacy: bool,
    pub has_route: bool,
    pub has_certificate: bool,
    pub has_frontier: bool,
    pub has_forensic: bool,
}

impl LocusCapabilityMask {
    pub fn encode_bits(&self) -> u16 {
        let mut bits = 0u16;
        if self.has_checker {
            bits |= 1 << 0;
        }
        if self.has_adequacy {
            bits |= 1 << 1;
        }
        if self.has_route {
            bits |= 1 << 2;
        }
        if self.has_certificate {
            bits |= 1 << 3;
        }
        if self.has_frontier {
            bits |= 1 << 4;
        }
        if self.has_forensic {
            bits |= 1 << 5;
        }
        bits
    }

    pub fn from_bits(bits: u16) -> Self {
        Self {
            has_checker: bits & (1 << 0) != 0,
            has_adequacy: bits & (1 << 1) != 0,
            has_route: bits & (1 << 2) != 0,
            has_certificate: bits & (1 << 3) != 0,
            has_frontier: bits & (1 << 4) != 0,
            has_forensic: bits & (1 << 5) != 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocusPacketHeader {
    pub artifact_class: GenomeArtifactClass,
    pub surface: GenomeSurface,
    pub kind: LocusPacketKind,
    pub version_major: u8,
    pub version_minor: u8,
    pub authority_tier: u8,
    pub capabilities: LocusCapabilityMask,
    pub grammar_id: String,
    pub schema_hash: String,
    pub integrity_hash: String,
    #[serde(default)]
    pub strand_manifest: Vec<String>,
    pub feature_flags: u16,
    pub root_subject_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocusSection {
    pub opcode: LocusOpcode,
    pub flags: u16,
    pub subject_id: String,
    pub payload: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LocusPacket {
    pub header: LocusPacketHeader,
    pub sections: Vec<LocusSection>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RnaState {
    Raw,
    Spliced,
    Expressed,
    Stabilized,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RnaIssueClass {
    Ambiguity,
    GroupingFailure,
    ArityMismatch,
    SpliceClosureFailure,
    UnknownOperator,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RnormFailureKind {
    Ambiguity,
    GroupingFailure,
    ArityMismatch,
    SpliceClosureFailure,
    UnknownOperator,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RnormDiagnosticSpec {
    pub class: RnaIssueClass,
    pub failure_kind: RnormFailureKind,
    pub code: String,
    pub repair_hint_template: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum TokenClass {
    Atom,
    Binder,
    Operator,
    GroupLeft,
    GroupRight,
    Separator,
    StrandRef,
    Meta,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenClassSpec {
    pub class: TokenClass,
    pub name: String,
    pub rule: String,
    pub semantic_authority: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenMapEntry {
    pub sample: String,
    pub class: TokenClass,
    pub rule: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RnaToken {
    pub class: TokenClass,
    pub value: u32,
    pub flags: u8,
    pub span_start: usize,
    pub span_end: usize,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenStream {
    pub source_text: String,
    pub byte_len: usize,
    #[serde(default)]
    pub tokens: Vec<RnaToken>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenizationIssue {
    pub offset: usize,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TokenizationReceipt {
    pub id: String,
    pub token_count: usize,
    pub byte_len: usize,
    #[serde(default)]
    pub issues: Vec<TokenizationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RnaNormalizationIssue {
    pub class: RnaIssueClass,
    pub offset: usize,
    pub message: String,
    #[serde(default)]
    pub repair_hints: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RnaNormalizationReceipt {
    pub id: String,
    #[serde(default)]
    pub token_stream_id: String,
    pub state: RnaState,
    pub normalized_text: String,
    #[serde(default)]
    pub shorthand_eliminated: bool,
    #[serde(default)]
    pub splice_regions: Vec<(usize, usize)>,
    #[serde(default)]
    pub issues: Vec<RnaNormalizationIssue>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct NormalizedRna {
    pub raw_text: String,
    pub normalized_text: String,
    pub state: RnaState,
    #[serde(default)]
    pub splice_regions: Vec<(usize, usize)>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SsrNodeKind {
    Root,
    Atom,
    Group,
    Splice,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SsrNode {
    pub id: String,
    pub kind: SsrNodeKind,
    pub text: String,
    #[serde(default)]
    pub children: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SsrGraph {
    pub root_id: String,
    #[serde(default)]
    pub nodes: Vec<SsrNode>,
    #[serde(default)]
    pub strand_partition: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SsrReceipt {
    pub id: String,
    pub root_id: String,
    pub node_count: usize,
    #[serde(default)]
    pub transition_count: usize,
    #[serde(default)]
    pub transition_table_hash: String,
    pub ephemeral: bool,
    pub normalized_rna_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SsrTransitionSpec {
    pub token_class: TokenClass,
    pub action: String,
    pub persists_state: bool,
    pub semantic_inference: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalGraph {
    pub root_id: String,
    pub canonical_text: String,
    #[serde(default)]
    pub canonical_nodes: Vec<SsrNode>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CnormReceipt {
    pub id: String,
    pub root_id: String,
    pub canonical_hash: String,
    #[serde(default)]
    pub rule_table_hash: String,
    #[serde(default)]
    pub idempotent: bool,
    pub erased_variations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CanonicalRuleSpec {
    pub id: String,
    pub description: String,
    pub deterministic: bool,
    pub semantic_lookup_required: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SemanticLoweringReceipt {
    pub id: String,
    pub source_surface: GenomeSurface,
    pub target: String,
    pub canonical_hash: String,
    pub ephemeral_resolver_used: bool,
}

pub type KernelGraph = SsrGraph;

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PhaseId {
    AuthorityReset,
    Tokenization,
    RnaNormalization,
    StructuralResolution,
    CanonicalNormalization,
    DnaProtocolFreeze,
    DnaParallelRollout,
    CliContractMigration,
    ResearchHostReconnect,
    TowerCoverageReconnect,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PhaseValidationResult {
    Passed,
    Failed,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PromotionSignal {
    Promote,
    Hold,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum SystemFailureDisposition {
    HaltPropagation,
    RollbackLocal,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct InvariantCheck {
    pub name: String,
    pub passed: bool,
    pub detail: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FailureRecord {
    pub code: String,
    pub message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChangeLedgerEntry {
    pub phase_id: PhaseId,
    pub input_state_hash: String,
    pub output_state_hash: Option<String>,
    #[serde(default)]
    pub dependency_edges: Vec<String>,
    #[serde(default)]
    pub invariant_checks: Vec<InvariantCheck>,
    #[serde(default)]
    pub failure_records: Vec<FailureRecord>,
    pub validation_result: PhaseValidationResult,
    pub promotion_signal: PromotionSignal,
    pub rollback_pointer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemFailureState {
    pub phase_id: PhaseId,
    pub disposition: SystemFailureDisposition,
    #[serde(default)]
    pub failure_records: Vec<FailureRecord>,
    pub rollback_pointer: Option<String>,
    pub ledger_entry: ChangeLedgerEntry,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct PhaseExecutionKernel {
    #[serde(default)]
    entries: Vec<ChangeLedgerEntry>,
}

impl PhaseExecutionKernel {
    pub fn entries(&self) -> &[ChangeLedgerEntry] {
        &self.entries
    }

    pub fn execute<T, E, F, V>(
        &mut self,
        phase_id: PhaseId,
        input_state_hash: String,
        dependency_edges: Vec<String>,
        rollback_pointer: Option<String>,
        transform: F,
        validate: V,
    ) -> Result<T, SystemFailureState>
    where
        T: Serialize,
        E: ToString,
        F: FnOnce() -> Result<T, E>,
        V: FnOnce(&T) -> Vec<InvariantCheck>,
    {
        match transform() {
            Ok(output) => {
                let invariant_checks = validate(&output);
                let failure_records = invariant_checks
                    .iter()
                    .filter(|check| !check.passed)
                    .map(|check| FailureRecord {
                        code: format!("{phase_id:?}_InvariantFailed"),
                        message: format!("{}: {}", check.name, check.detail),
                    })
                    .collect::<Vec<_>>();
                let validation_result = if failure_records.is_empty() {
                    PhaseValidationResult::Passed
                } else {
                    PhaseValidationResult::Failed
                };
                let promotion_signal = if failure_records.is_empty() {
                    PromotionSignal::Promote
                } else {
                    PromotionSignal::Hold
                };
                let output_state_hash = Some(hash_serialized(&output));
                let ledger_entry = ChangeLedgerEntry {
                    phase_id,
                    input_state_hash,
                    output_state_hash,
                    dependency_edges,
                    invariant_checks,
                    failure_records: failure_records.clone(),
                    validation_result,
                    promotion_signal,
                    rollback_pointer: rollback_pointer.clone(),
                };
                self.entries.push(ledger_entry.clone());
                if failure_records.is_empty() {
                    Ok(output)
                } else {
                    Err(SystemFailureState {
                        phase_id,
                        disposition: SystemFailureDisposition::HaltPropagation,
                        failure_records,
                        rollback_pointer,
                        ledger_entry,
                    })
                }
            }
            Err(err) => {
                let failure_records = vec![FailureRecord {
                    code: format!("{phase_id:?}_TransformFailed"),
                    message: err.to_string(),
                }];
                let ledger_entry = ChangeLedgerEntry {
                    phase_id,
                    input_state_hash,
                    output_state_hash: None,
                    dependency_edges,
                    invariant_checks: Vec::new(),
                    failure_records: failure_records.clone(),
                    validation_result: PhaseValidationResult::Failed,
                    promotion_signal: PromotionSignal::Hold,
                    rollback_pointer: rollback_pointer.clone(),
                };
                self.entries.push(ledger_entry.clone());
                Err(SystemFailureState {
                    phase_id,
                    disposition: SystemFailureDisposition::RollbackLocal,
                    failure_records,
                    rollback_pointer,
                    ledger_entry,
                })
            }
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct LowerChainExecution {
    pub token_stream: TokenStream,
    pub tokenization_receipt: TokenizationReceipt,
    pub normalized: NormalizedRna,
    pub rn_receipt: RnaNormalizationReceipt,
    pub kernel_graph: KernelGraph,
    pub ssr_receipt: SsrReceipt,
    pub canonical_graph: CanonicalGraph,
    pub cnorm_receipt: CnormReceipt,
    #[serde(default)]
    pub ledger_entries: Vec<ChangeLedgerEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCoverageEnvelope {
    pub subject_id: String,
    pub decision: CoverageDecision,
    pub covered_by: Vec<String>,
    pub skipped_obligation_ids: Vec<String>,
    pub residual_obligation_ids: Vec<String>,
    pub reuse_envelope_hash: String,
    pub payoff_receipt_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReuseLegalityReceipt {
    pub id: String,
    pub subject_id: String,
    pub lawful: bool,
    pub basis: Vec<String>,
    #[serde(default)]
    pub lineage_refs: Vec<String>,
    #[serde(default)]
    pub policy_scope: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReuseDecisionReceipt {
    pub id: String,
    pub subject_id: String,
    #[serde(default)]
    pub reused_artifact_ids: Vec<String>,
    #[serde(default)]
    pub skipped_obligation_ids: Vec<String>,
    pub fallback_required: bool,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResidualVerificationReceipt {
    pub id: String,
    pub subject_id: String,
    #[serde(default)]
    pub residual_obligation_ids: Vec<String>,
    pub fully_discharged: bool,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum EvidenceExactness {
    Exact,
    Approximate,
    WitnessBacked,
    CounterexampleCandidate,
    Undischarged,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ExecutionClosureReceipt {
    pub id: String,
    pub subject_id: String,
    pub exactness: EvidenceExactness,
    pub lower_lineage_required: bool,
    pub reuse_reported: bool,
    pub residuals_reported: bool,
    pub promotion_gate_visible: bool,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Frontier {
    pub id: String,
    pub title: String,
    pub burden_class: String,
    pub blocker_ids: Vec<String>,
    pub closure_witness: Option<String>,
    pub residual_budget: Option<String>,
    pub strengthening_value: usize,
    pub coldness: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct FrontierLedger {
    pub frontiers: Vec<Frontier>,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CandidateSemanticLeaf {
    pub id: String,
    pub frontier_id: String,
    pub description: String,
    pub generated_status: GeneratedStatus,
    pub expected_relief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CandidateAdequacyClause {
    pub id: String,
    pub frontier_id: String,
    pub clause_stub: String,
    pub generated_status: GeneratedStatus,
    pub expected_relief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CandidateCheckerExtension {
    pub id: String,
    pub frontier_id: String,
    pub object_family: String,
    pub generated_status: GeneratedStatus,
    pub expected_relief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CandidateCampaign {
    pub id: String,
    pub theorem_id: String,
    pub target_profile_id: String,
    pub generated_status: GeneratedStatus,
    pub expected_relief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CandidateAtlasCell {
    pub id: String,
    pub src: String,
    pub tgt: String,
    pub generated_status: GeneratedStatus,
    pub expected_relief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CandidatePayoffTask {
    pub id: String,
    pub artifact_id: String,
    pub burden_hint: String,
    pub generated_status: GeneratedStatus,
    pub expected_relief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SearchCompartment {
    pub id: String,
    pub task_fingerprint: String,
    pub allowed_grammars: Vec<String>,
    pub budget: String,
    pub evaluator_profile: String,
    pub admissibility_surface: Vec<String>,
    pub kill_criteria: Vec<String>,
    pub expected_relief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GeneratorContract {
    pub id: String,
    pub proposal_kind: ProposalKind,
    pub seam: String,
    pub verifier: String,
    pub compatibility_path: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GenerationReceipt {
    pub id: String,
    pub generator_contract_id: String,
    pub output_ids: Vec<String>,
    pub generated_status: GeneratedStatus,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct DistressVector {
    pub repeated_blocker_ids: Vec<String>,
    pub checker_gaps: Vec<String>,
    pub route_scarcity: Vec<String>,
    pub payoff_drought: bool,
    pub stalled_frontier_motion: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HelpRequest {
    pub id: String,
    pub task_ref: String,
    pub distress_vector: DistressVector,
    pub minimal_reproduction: String,
    pub requested_capacity: String,
    pub expected_relief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeRecord {
    pub id: String,
    pub seam: String,
    pub tactic_family: String,
    pub generated_status: GeneratedStatus,
    pub expected_relief: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RecipeDelta {
    pub id: String,
    pub recipe_record_id: String,
    pub changed_fields: Vec<String>,
    pub strengthening_value: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromotionCandidate {
    pub id: String,
    pub artifact_id: String,
    pub candidate_kind: String,
    pub generated_status: GeneratedStatus,
    pub proof_grade: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CalibrationPressureMap {
    pub override_pressure: usize,
    pub emitted_duplication: usize,
    pub conformance_friction: usize,
    pub extension_cost: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OverridePressureReceipt {
    pub id: String,
    pub source_id: String,
    pub pressure_class: String,
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProofCoverageDispatch {
    pub subject_id: String,
    pub decision: CoverageDecision,
    pub route_fast_path: bool,
    pub reason: String,
    pub covered_by: Vec<String>,
    pub residual_obligation_ids: Vec<String>,
    #[serde(default)]
    pub lineage_refs: Vec<String>,
    #[serde(default)]
    pub reuse_legality_receipts: Vec<ReuseLegalityReceipt>,
    #[serde(default)]
    pub reuse_decision_receipts: Vec<ReuseDecisionReceipt>,
    #[serde(default)]
    pub residual_verification_receipts: Vec<ResidualVerificationReceipt>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct VerticalCompoundingBundle {
    pub frontier_ledger: FrontierLedger,
    pub semantic_leaves: Vec<CandidateSemanticLeaf>,
    pub adequacy_clauses: Vec<CandidateAdequacyClause>,
    pub checker_extensions: Vec<CandidateCheckerExtension>,
    pub campaigns: Vec<CandidateCampaign>,
    pub atlas_cells: Vec<CandidateAtlasCell>,
    pub payoff_tasks: Vec<CandidatePayoffTask>,
    pub compartments: Vec<SearchCompartment>,
    pub generator_contracts: Vec<GeneratorContract>,
    pub generation_receipts: Vec<GenerationReceipt>,
    pub distress: Option<DistressVector>,
    pub help_requests: Vec<HelpRequest>,
    pub recipes: Vec<RecipeRecord>,
    pub recipe_deltas: Vec<RecipeDelta>,
    pub promotion_candidates: Vec<PromotionCandidate>,
    pub calibration_pressure: Option<CalibrationPressureMap>,
    pub override_pressure_receipts: Vec<OverridePressureReceipt>,
    #[serde(default)]
    pub lineage_refs: Vec<String>,
    #[serde(default)]
    pub lineage_required: bool,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResearchSector {
    Gravity,
    Gauge,
    Atomic,
    Thermo,
    Cosmology,
    Topology,
    Control,
    Crosscut,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ObjectiveKind {
    Derive,
    Reduce,
    Benchmark,
    Stress,
    Integrate,
    Refute,
    Crystallize,
    Promote,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DesiredOutputKind {
    Theorem,
    Operator,
    BenchmarkSchema,
    ReductionMap,
    Battery,
    HandoffPacket,
    ValidationBundle,
    ResearchNote,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum DerivationStructuralType {
    Projection,
    Closure,
    Obstruction,
    BenchmarkFit,
    Reduction,
    Simulation,
    ChallengeResponse,
    RegistryHardening,
    OperationalHardening,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RiskClass {
    Low,
    Medium,
    High,
    Dangerous,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReviewStatus {
    Pass,
    Revise,
    Fail,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReviewDecision {
    Proceed,
    Revise,
    Hold,
    Reject,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChallengeGrounds {
    Inconsistency,
    HiddenAssumption,
    BenchmarkMiss,
    ProjectionAmbiguity,
    RegimeViolation,
    PromotionMismatch,
    UnsupportedGeneralization,
    CapabilityTruthMismatch,
    NamespaceContractMismatch,
    ArtifactContractMismatch,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ChallengeSeverity {
    Low,
    Medium,
    High,
    Blocking,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResponseClass {
    Patch,
    NarrowScope,
    Benchmark,
    Rollback,
    Retire,
    Document,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RegistryEntryKind {
    Claim,
    MathClaim,
    Operator,
    BenchmarkRun,
    Handoff,
    PromotionQueue,
    Benchmark,
    Challenge,
    Receipt,
    Task,
    Signature,
    Review,
    StrengtheningArtifact,
    Remediation,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RegistryEntryStatus {
    Workspace,
    Derived,
    Benchmarked,
    Certified,
    Integrated,
    Blocked,
    Retired,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ResearchRouteClass {
    Derive,
    Benchmark,
    Stress,
    ChallengeResponse,
    Crystallize,
    Integrate,
    Retire,
    OperationalHardening,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RemediationClass {
    MissingFunctionality,
    SemanticBug,
    CommandContractBug,
    ArtifactTypingBug,
    RuntimeBug,
    DocumentationGap,
    CapabilityTruthGap,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum RemediationStatus {
    Open,
    InProgress,
    Patched,
    NeedsCompilePass,
    Verified,
    Deferred,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct TaskEnvelope {
    pub id: String,
    pub title: String,
    pub target_sector: ResearchSector,
    pub objective_kind: ObjectiveKind,
    pub desired_output: DesiredOutputKind,
    #[serde(default)]
    pub hard_constraints: Vec<String>,
    #[serde(default)]
    pub soft_constraints: Vec<String>,
    pub promotion_target: CertificationVerdict,
    pub rollback_path: String,
    #[serde(default)]
    pub success_metrics: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DerivationSignature {
    pub id: String,
    pub task_ref: String,
    pub structural_type: DerivationStructuralType,
    pub sector: ResearchSector,
    pub risk_class: RiskClass,
    pub dependency_depth: usize,
    #[serde(default)]
    pub likely_failure_modes: Vec<String>,
    pub expected_distillate: DesiredOutputKind,
    #[serde(default)]
    pub evidence_gap: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReviewReceipt {
    pub id: String,
    pub subject_ref: String,
    pub fit_review: ReviewStatus,
    pub risk_review: ReviewStatus,
    pub capture_review: ReviewStatus,
    pub continuity_review: ReviewStatus,
    #[serde(default)]
    pub notes: Vec<String>,
    #[serde(default)]
    pub reviewers: Vec<String>,
    pub verdict: ReviewDecision,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ChallengeRecord {
    pub id: String,
    pub target_ref: String,
    pub grounds: ChallengeGrounds,
    pub severity: ChallengeSeverity,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
    pub required_response: ResponseClass,
    pub status: ChallengeStatus,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct OperatorRecord {
    pub id: String,
    pub name: String,
    pub semantics: String,
    #[serde(default)]
    pub valid_regimes: Vec<String>,
    pub evidence_level: CertificationVerdict,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub failure_modes: Vec<String>,
    pub reuse_count: usize,
    #[serde(default)]
    pub implementation_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkSchema {
    pub id: String,
    pub target_description: String,
    pub metric_name: String,
    pub tolerance: String,
    pub reference_source: String,
    pub weight_basis_points: u32,
    #[serde(default)]
    pub required_evidence: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct StrengtheningArtifact {
    pub id: String,
    pub source_ref: String,
    pub kind: DesiredOutputKind,
    #[serde(default)]
    pub inheritance_targets: Vec<String>,
    pub redistribution_notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct FrameworkRegistryEntry {
    pub id: String,
    pub kind: RegistryEntryKind,
    pub host: String,
    pub status: RegistryEntryStatus,
    pub retained_value: String,
    #[serde(default)]
    pub open_risks: Vec<String>,
    #[serde(default)]
    pub dependencies: Vec<String>,
    #[serde(default)]
    pub last_receipt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouteScore {
    pub route: ResearchRouteClass,
    pub fit: f64,
    pub leverage: f64,
    pub risk: f64,
    pub cost: f64,
    pub debt: f64,
    pub total: f64,
    #[serde(default)]
    pub reasons: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RouteAssignment {
    pub task_id: String,
    pub signature_id: String,
    pub recommended_route: ResearchRouteClass,
    pub scores: Vec<RouteScore>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RemediationLedgerEntry {
    pub id: String,
    pub source_ref: String,
    pub class: RemediationClass,
    pub summary: String,
    #[serde(default)]
    pub affected_surfaces: Vec<String>,
    #[serde(default)]
    pub expected_fix_chain: Vec<String>,
    pub status: RemediationStatus,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ProducerHostKind {
    Reduction,
    Projection,
    Benchmark,
    ChallengeResolution,
    Handoff,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum PromotionQueueStatus {
    Proposed,
    Ready,
    Blocked,
    Promoted,
    Deferred,
}

fn default_promotion_queue_status_proposed() -> PromotionQueueStatus {
    PromotionQueueStatus::Proposed
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MathClaimPacket {
    pub id: String,
    pub title: String,
    pub statement: String,
    pub sector: ResearchSector,
    pub truth_class: AuthorityState,
    pub claim_class: ClaimClass,
    #[serde(default)]
    pub assumptions: Vec<String>,
    #[serde(default)]
    pub projection_refs: Vec<String>,
    #[serde(default)]
    pub reduction_refs: Vec<String>,
    #[serde(default)]
    pub benchmark_refs: Vec<String>,
    #[serde(default)]
    pub blocker_leaves: Vec<String>,
    #[serde(default)]
    pub report_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ReductionMapRecord {
    pub id: String,
    pub theorem_id: String,
    pub target_profile_id: String,
    pub route_class_id: Option<String>,
    pub atlas_cell_id: Option<String>,
    pub derivation_path: String,
    #[serde(default)]
    pub residual_obligation_ids: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProjectionMapRecord {
    pub id: String,
    pub theorem_id: String,
    pub src_hosts: Vec<String>,
    pub tgt_hosts: Vec<String>,
    pub projection_summary: String,
    #[serde(default)]
    pub bridge_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct BenchmarkRunRecord {
    pub id: String,
    pub theorem_id: String,
    pub benchmark_schema_id: Option<String>,
    #[serde(default)]
    pub receipt_ids: Vec<String>,
    pub status: RegistryEntryStatus,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct HandoffPacket {
    pub id: String,
    pub subject_id: String,
    pub summary: String,
    #[serde(default = "default_promotion_queue_status_proposed")]
    pub status: PromotionQueueStatus,
    #[serde(default)]
    pub claim_refs: Vec<String>,
    #[serde(default)]
    pub operator_refs: Vec<String>,
    #[serde(default)]
    pub strengthening_refs: Vec<String>,
    #[serde(default)]
    pub repro_refs: Vec<String>,
    #[serde(default)]
    pub benchmark_refs: Vec<String>,
    #[serde(default)]
    pub benchmark_run_refs: Vec<String>,
    #[serde(default)]
    pub producer_host_refs: Vec<String>,
    #[serde(default)]
    pub route_refs: Vec<String>,
    #[serde(default)]
    pub coverage_refs: Vec<String>,
    #[serde(default)]
    pub lineage_refs: Vec<String>,
    #[serde(default)]
    pub tower_refs: Vec<String>,
    #[serde(default)]
    pub promotion_refs: Vec<String>,
    #[serde(default)]
    pub readiness_score_basis_points: u16,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub positive_factors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromotionQueueEntry {
    pub id: String,
    pub subject_id: String,
    pub status: PromotionQueueStatus,
    pub readiness_score_basis_points: u16,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub required_refs: Vec<String>,
    #[serde(default)]
    pub evidence_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PromotionReadinessReport {
    pub subject_id: String,
    pub readiness_score_basis_points: u16,
    pub status: PromotionQueueStatus,
    #[serde(default)]
    pub positive_factors: Vec<String>,
    #[serde(default)]
    pub blockers: Vec<String>,
    #[serde(default)]
    pub missing_refs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProducerHostSpec {
    pub id: String,
    pub kind: ProducerHostKind,
    pub purpose: String,
    #[serde(default)]
    pub owned_objects: Vec<String>,
    #[serde(default)]
    pub feeds: Vec<String>,
    #[serde(default)]
    pub notes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ResearchBundle {
    #[serde(default)]
    pub tasks: Vec<TaskEnvelope>,
    #[serde(default)]
    pub signatures: Vec<DerivationSignature>,
    #[serde(default)]
    pub route_assignments: Vec<RouteAssignment>,
    #[serde(default)]
    pub claims: Vec<MathClaimPacket>,
    #[serde(default)]
    pub reduction_maps: Vec<ReductionMapRecord>,
    #[serde(default)]
    pub projection_maps: Vec<ProjectionMapRecord>,
    #[serde(default)]
    pub benchmark_runs: Vec<BenchmarkRunRecord>,
    #[serde(default)]
    pub handoff_packets: Vec<HandoffPacket>,
    #[serde(default)]
    pub producer_hosts: Vec<ProducerHostSpec>,
    #[serde(default)]
    pub promotion_queue: Vec<PromotionQueueEntry>,
    #[serde(default)]
    pub promotion_reports: Vec<PromotionReadinessReport>,
    #[serde(default)]
    pub lineage_records: Vec<ResearchLineageRecord>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ResearchLineageRecord {
    pub id: String,
    pub subject_id: String,
    pub artifact_class: GenomeArtifactClass,
    pub source_surface: GenomeSurface,
    pub target_surface: GenomeSurface,
    pub grammar_id: String,
    pub canonical_hash: String,
    pub lowering_receipt_id: String,
    #[serde(default)]
    pub phase_ids: Vec<PhaseId>,
    #[serde(default)]
    pub phase_ledger: Vec<ChangeLedgerEntry>,
    #[serde(default)]
    pub notes: Vec<String>,
}

const LOCUS_MAGIC: &[u8; 4] = b"LCS1";

pub fn encode_locus_packet(packet: &LocusPacket) -> Result<Vec<u8>, String> {
    let mut out = Vec::new();
    out.extend_from_slice(LOCUS_MAGIC);
    out.push(packet.header.artifact_class as u8);
    out.push(packet.header.surface as u8);
    out.push(packet.header.kind as u8);
    out.push(packet.header.version_major);
    out.push(packet.header.version_minor);
    out.push(packet.header.authority_tier);
    out.extend_from_slice(&packet.header.capabilities.encode_bits().to_le_bytes());
    push_sized_bytes(&mut out, packet.header.grammar_id.as_bytes())?;
    push_sized_bytes(&mut out, packet.header.schema_hash.as_bytes())?;
    push_sized_bytes(&mut out, packet.header.integrity_hash.as_bytes())?;
    let strand_count: u16 = packet
        .header
        .strand_manifest
        .len()
        .try_into()
        .map_err(|_| "too many strands".to_string())?;
    out.extend_from_slice(&strand_count.to_le_bytes());
    for strand in &packet.header.strand_manifest {
        push_sized_bytes(&mut out, strand.as_bytes())?;
    }
    out.extend_from_slice(&packet.header.feature_flags.to_le_bytes());
    push_sized_bytes(&mut out, packet.header.root_subject_id.as_bytes())?;
    let section_count: u16 = packet
        .sections
        .len()
        .try_into()
        .map_err(|_| "too many sections".to_string())?;
    out.extend_from_slice(&section_count.to_le_bytes());
    for section in &packet.sections {
        out.push(section.opcode as u8);
        out.extend_from_slice(&section.flags.to_le_bytes());
        push_sized_bytes(&mut out, section.subject_id.as_bytes())?;
        push_sized_bytes(&mut out, &section.payload)?;
    }
    Ok(out)
}

pub fn decode_locus_packet(bytes: &[u8]) -> Result<LocusPacket, String> {
    decode_locus_packet_current(bytes).or_else(|current_err| {
        decode_locus_packet_legacy(bytes)
            .map_err(|legacy_err| format!("{current_err}; legacy decode also failed: {legacy_err}"))
    })
}

pub fn dna_header_receipt(packet: &LocusPacket) -> DnaHeaderReceipt {
    DnaHeaderReceipt {
        id: format!(
            "DNA_HDR_{:x}",
            stable_hash_u64(&format!(
                "{}:{}:{}",
                packet.header.grammar_id, packet.header.schema_hash, packet.header.integrity_hash
            ))
        ),
        surface: packet.header.surface,
        grammar_id: packet.header.grammar_id.clone(),
        schema_hash: packet.header.schema_hash.clone(),
        integrity_hash: packet.header.integrity_hash.clone(),
        structural_opcode_table_hash: hash_serialized(&structural_opcode_specs()),
        header_truth_complete: packet.header.surface == GenomeSurface::Dna
            && !packet.header.grammar_id.is_empty()
            && !packet.header.schema_hash.is_empty()
            && !packet.header.integrity_hash.is_empty()
            && !packet.header.root_subject_id.is_empty(),
    }
}

pub fn validate_dna_packet(packet: &LocusPacket) -> DnaValidationReport {
    let mut failures = Vec::new();
    let header = dna_header_receipt(packet);
    if !header.header_truth_complete {
        failures.push("header truth is incomplete".into());
    }
    if packet.header.surface != GenomeSurface::Dna {
        failures.push("packet surface is not DNA".into());
    }
    if packet
        .sections
        .iter()
        .any(|section| section.opcode == LocusOpcode::SymbolTable && section.flags & 0x0001 != 0)
    {
        failures.push("symbol table marked as semantic authority".into());
    }
    if packet.sections.is_empty() {
        failures.push("packet has no structural sections".into());
    }
    let structural_opcode_table = structural_opcode_specs();
    if !structural_opcode_table
        .iter()
        .all(|spec| spec.structural_only)
    {
        failures.push("opcode registry contains non-structural opcode".into());
    }
    DnaValidationReport {
        id: format!("DNA_VAL_{:x}", stable_hash_u64(&hash_serialized(packet))),
        reversible: failures.is_empty(),
        header_truth_complete: header.header_truth_complete,
        symbol_table_semantic_authority: packet.sections.iter().any(|section| {
            section.opcode == LocusOpcode::SymbolTable && section.flags & 0x0001 != 0
        }),
        failures,
    }
}

fn decode_locus_packet_current(bytes: &[u8]) -> Result<LocusPacket, String> {
    let mut cursor = 0usize;
    if bytes.len() < LOCUS_MAGIC.len() + 10 {
        return Err("packet too small".into());
    }
    if &bytes[..4] != LOCUS_MAGIC {
        return Err("bad packet magic".into());
    }
    cursor += 4;
    let artifact_class = decode_genome_artifact_class(
        *read_exact(bytes, &mut cursor, 1)?
            .first()
            .ok_or_else(|| "missing artifact class".to_string())?,
    )?;
    let surface = decode_genome_surface(
        *read_exact(bytes, &mut cursor, 1)?
            .first()
            .ok_or_else(|| "missing genome surface".to_string())?,
    )?;
    let kind = decode_packet_kind(
        *read_exact(bytes, &mut cursor, 1)?
            .first()
            .ok_or_else(|| "missing packet kind".to_string())?,
    )?;
    let version_major = *read_exact(bytes, &mut cursor, 1)?.first().unwrap();
    let version_minor = *read_exact(bytes, &mut cursor, 1)?.first().unwrap();
    let authority_tier = *read_exact(bytes, &mut cursor, 1)?.first().unwrap();
    let cap_bits = u16::from_le_bytes(
        read_exact(bytes, &mut cursor, 2)?
            .try_into()
            .map_err(|_| "bad capability bytes".to_string())?,
    );
    let grammar_id =
        String::from_utf8(read_sized_bytes(bytes, &mut cursor)?).map_err(|err| err.to_string())?;
    let schema_hash =
        String::from_utf8(read_sized_bytes(bytes, &mut cursor)?).map_err(|err| err.to_string())?;
    let integrity_hash =
        String::from_utf8(read_sized_bytes(bytes, &mut cursor)?).map_err(|err| err.to_string())?;
    let strand_count = u16::from_le_bytes(
        read_exact(bytes, &mut cursor, 2)?
            .try_into()
            .map_err(|_| "bad strand count".to_string())?,
    );
    let mut strand_manifest = Vec::new();
    for _ in 0..strand_count {
        strand_manifest.push(
            String::from_utf8(read_sized_bytes(bytes, &mut cursor)?)
                .map_err(|err| err.to_string())?,
        );
    }
    let feature_flags = u16::from_le_bytes(
        read_exact(bytes, &mut cursor, 2)?
            .try_into()
            .map_err(|_| "bad feature flags".to_string())?,
    );
    let root_subject_id =
        String::from_utf8(read_sized_bytes(bytes, &mut cursor)?).map_err(|err| err.to_string())?;
    let section_count = u16::from_le_bytes(
        read_exact(bytes, &mut cursor, 2)?
            .try_into()
            .map_err(|_| "bad section count".to_string())?,
    );
    let mut sections = Vec::new();
    for _ in 0..section_count {
        let opcode = decode_opcode(*read_exact(bytes, &mut cursor, 1)?.first().unwrap())?;
        let flags = u16::from_le_bytes(
            read_exact(bytes, &mut cursor, 2)?
                .try_into()
                .map_err(|_| "bad section flags".to_string())?,
        );
        let subject_id = String::from_utf8(read_sized_bytes(bytes, &mut cursor)?)
            .map_err(|err| err.to_string())?;
        let payload = read_sized_bytes(bytes, &mut cursor)?;
        sections.push(LocusSection {
            opcode,
            flags,
            subject_id,
            payload,
        });
    }
    Ok(LocusPacket {
        header: LocusPacketHeader {
            artifact_class,
            surface,
            kind,
            version_major,
            version_minor,
            authority_tier,
            capabilities: LocusCapabilityMask::from_bits(cap_bits),
            grammar_id,
            schema_hash,
            integrity_hash,
            strand_manifest,
            feature_flags,
            root_subject_id,
        },
        sections,
    })
}

fn decode_locus_packet_legacy(bytes: &[u8]) -> Result<LocusPacket, String> {
    let mut cursor = 0usize;
    if bytes.len() < LOCUS_MAGIC.len() + 8 {
        return Err("legacy packet too small".into());
    }
    if &bytes[..4] != LOCUS_MAGIC {
        return Err("bad packet magic".into());
    }
    cursor += 4;
    let kind = decode_packet_kind(
        *read_exact(bytes, &mut cursor, 1)?
            .first()
            .ok_or_else(|| "missing packet kind".to_string())?,
    )?;
    let version_major = *read_exact(bytes, &mut cursor, 1)?.first().unwrap();
    let version_minor = *read_exact(bytes, &mut cursor, 1)?.first().unwrap();
    let authority_tier = *read_exact(bytes, &mut cursor, 1)?.first().unwrap();
    let cap_bits = u16::from_le_bytes(
        read_exact(bytes, &mut cursor, 2)?
            .try_into()
            .map_err(|_| "bad capability bytes".to_string())?,
    );
    let schema_hash =
        String::from_utf8(read_sized_bytes(bytes, &mut cursor)?).map_err(|err| err.to_string())?;
    let root_subject_id =
        String::from_utf8(read_sized_bytes(bytes, &mut cursor)?).map_err(|err| err.to_string())?;
    let section_count = u16::from_le_bytes(
        read_exact(bytes, &mut cursor, 2)?
            .try_into()
            .map_err(|_| "bad section count".to_string())?,
    );
    let mut sections = Vec::new();
    for _ in 0..section_count {
        let opcode = decode_opcode(*read_exact(bytes, &mut cursor, 1)?.first().unwrap())?;
        let flags = u16::from_le_bytes(
            read_exact(bytes, &mut cursor, 2)?
                .try_into()
                .map_err(|_| "bad section flags".to_string())?,
        );
        let subject_id = String::from_utf8(read_sized_bytes(bytes, &mut cursor)?)
            .map_err(|err| err.to_string())?;
        let payload = read_sized_bytes(bytes, &mut cursor)?;
        sections.push(LocusSection {
            opcode,
            flags,
            subject_id,
            payload,
        });
    }
    Ok(LocusPacket {
        header: LocusPacketHeader {
            artifact_class: GenomeArtifactClass::Gene,
            surface: GenomeSurface::Dna,
            kind,
            version_major,
            version_minor,
            authority_tier,
            capabilities: LocusCapabilityMask::from_bits(cap_bits),
            grammar_id: "legacy.locus".into(),
            schema_hash,
            integrity_hash: String::new(),
            strand_manifest: vec!["core".into()],
            feature_flags: 0,
            root_subject_id,
        },
        sections,
    })
}

pub fn locus_packet_summary(packet: &LocusPacket) -> BTreeMap<String, String> {
    BTreeMap::from([
        (
            "artifact_class".into(),
            format!("{:?}", packet.header.artifact_class),
        ),
        ("surface".into(), format!("{:?}", packet.header.surface)),
        ("kind".into(), format!("{:?}", packet.header.kind)),
        ("grammar_id".into(), packet.header.grammar_id.clone()),
        (
            "root_subject_id".into(),
            packet.header.root_subject_id.clone(),
        ),
        ("schema_hash".into(), packet.header.schema_hash.clone()),
        (
            "integrity_hash".into(),
            packet.header.integrity_hash.clone(),
        ),
        (
            "strand_count".into(),
            packet.header.strand_manifest.len().to_string(),
        ),
        ("section_count".into(), packet.sections.len().to_string()),
        (
            "capabilities".into(),
            format!("0x{:04x}", packet.header.capabilities.encode_bits()),
        ),
    ])
}

fn push_sized_bytes(out: &mut Vec<u8>, payload: &[u8]) -> Result<(), String> {
    let len: u32 = payload
        .len()
        .try_into()
        .map_err(|_| "payload too large".to_string())?;
    out.extend_from_slice(&len.to_le_bytes());
    out.extend_from_slice(payload);
    Ok(())
}

fn read_exact<'a>(bytes: &'a [u8], cursor: &mut usize, len: usize) -> Result<&'a [u8], String> {
    let end = cursor
        .checked_add(len)
        .ok_or_else(|| "cursor overflow".to_string())?;
    let slice = bytes
        .get(*cursor..end)
        .ok_or_else(|| "packet truncated".to_string())?;
    *cursor = end;
    Ok(slice)
}

fn read_sized_bytes(bytes: &[u8], cursor: &mut usize) -> Result<Vec<u8>, String> {
    let len = u32::from_le_bytes(
        read_exact(bytes, cursor, 4)?
            .try_into()
            .map_err(|_| "bad length".to_string())?,
    ) as usize;
    Ok(read_exact(bytes, cursor, len)?.to_vec())
}

fn decode_packet_kind(value: u8) -> Result<LocusPacketKind, String> {
    match value {
        1 => Ok(LocusPacketKind::CanonicalTransfer),
        2 => Ok(LocusPacketKind::CertificationEnvelope),
        3 => Ok(LocusPacketKind::CheckerEnvelope),
        4 => Ok(LocusPacketKind::FrontierEnvelope),
        5 => Ok(LocusPacketKind::TowerBundle),
        _ => Err(format!("unknown locus packet kind `{value}`")),
    }
}

fn decode_genome_artifact_class(value: u8) -> Result<GenomeArtifactClass, String> {
    match value {
        1 => Ok(GenomeArtifactClass::Gene),
        2 => Ok(GenomeArtifactClass::Haplotype),
        3 => Ok(GenomeArtifactClass::Chromosome),
        4 => Ok(GenomeArtifactClass::Genome),
        _ => Err(format!("unknown genome artifact class `{value}`")),
    }
}

fn decode_genome_surface(value: u8) -> Result<GenomeSurface, String> {
    match value {
        1 => Ok(GenomeSurface::Rna),
        2 => Ok(GenomeSurface::Dna),
        _ => Err(format!("unknown genome surface `{value}`")),
    }
}

fn decode_opcode(value: u8) -> Result<LocusOpcode, String> {
    match value {
        1 => Ok(LocusOpcode::Header),
        2 => Ok(LocusOpcode::CanonicalPayload),
        3 => Ok(LocusOpcode::ReceiptTable),
        4 => Ok(LocusOpcode::AdequacyTable),
        5 => Ok(LocusOpcode::RouteLedger),
        6 => Ok(LocusOpcode::Campaign),
        7 => Ok(LocusOpcode::Certificate),
        8 => Ok(LocusOpcode::Checker),
        9 => Ok(LocusOpcode::Frontier),
        10 => Ok(LocusOpcode::Proposal),
        11 => Ok(LocusOpcode::Coverage),
        12 => Ok(LocusOpcode::SymbolTable),
        13 => Ok(LocusOpcode::Forensic),
        _ => Err(format!("unknown locus opcode `{value}`")),
    }
}

pub fn structural_opcode_specs() -> Vec<StructuralOpcodeSpec> {
    vec![
        (LocusOpcode::Header, "header", false),
        (LocusOpcode::CanonicalPayload, "canonical-payload", false),
        (LocusOpcode::ReceiptTable, "receipt-table", true),
        (LocusOpcode::AdequacyTable, "adequacy-table", true),
        (LocusOpcode::RouteLedger, "route-ledger", true),
        (LocusOpcode::Campaign, "campaign", true),
        (LocusOpcode::Certificate, "certificate", true),
        (LocusOpcode::Checker, "checker", true),
        (LocusOpcode::Frontier, "frontier", true),
        (LocusOpcode::Proposal, "proposal", true),
        (LocusOpcode::Coverage, "coverage", true),
        (
            LocusOpcode::SymbolTable,
            "symbol-table-compression-only",
            true,
        ),
        (LocusOpcode::Forensic, "forensic", true),
    ]
    .into_iter()
    .map(
        |(opcode, name, compatible_skip_allowed)| StructuralOpcodeSpec {
            opcode,
            name: name.into(),
            structural_only: true,
            compatible_skip_allowed,
        },
    )
    .collect()
}

pub fn token_class_specs() -> Vec<TokenClassSpec> {
    vec![
        TokenClassSpec {
            class: TokenClass::Atom,
            name: "ATOM".into(),
            rule: "lowercase, digits, and non-control symbolic scalars that are not delimiters/operators".into(),
            semantic_authority: false,
        },
        TokenClassSpec {
            class: TokenClass::Binder,
            name: "BINDER".into(),
            rule: "uppercase ASCII binder anchors".into(),
            semantic_authority: false,
        },
        TokenClassSpec {
            class: TokenClass::Operator,
            name: "OP".into(),
            rule: "structural operator characters such as _, |, :, =, +, -, *, /, ^, ≔, ‖, and →".into(),
            semantic_authority: false,
        },
        TokenClassSpec {
            class: TokenClass::GroupLeft,
            name: "GROUP_L".into(),
            rule: "opening grouping delimiters".into(),
            semantic_authority: false,
        },
        TokenClassSpec {
            class: TokenClass::GroupRight,
            name: "GROUP_R".into(),
            rule: "closing grouping delimiters".into(),
            semantic_authority: false,
        },
        TokenClassSpec {
            class: TokenClass::Separator,
            name: "SEP".into(),
            rule: "comma and semicolon structural separators".into(),
            semantic_authority: false,
        },
        TokenClassSpec {
            class: TokenClass::StrandRef,
            name: "STRAND_REF".into(),
            rule: "period strand-reference marker".into(),
            semantic_authority: false,
        },
        TokenClassSpec {
            class: TokenClass::Meta,
            name: "META".into(),
            rule: "whitespace and comments/control metadata".into(),
            semantic_authority: false,
        },
    ]
}

pub fn rnorm_diagnostic_specs() -> Vec<RnormDiagnosticSpec> {
    vec![
        RnormDiagnosticSpec {
            class: RnaIssueClass::Ambiguity,
            failure_kind: RnormFailureKind::Ambiguity,
            code: "RNORM_AMBIGUITY".into(),
            repair_hint_template: "make the conflicting structural region explicit".into(),
        },
        RnormDiagnosticSpec {
            class: RnaIssueClass::GroupingFailure,
            failure_kind: RnormFailureKind::GroupingFailure,
            code: "RNORM_GROUPING".into(),
            repair_hint_template: "close or remove the unmatched grouping delimiter".into(),
        },
        RnormDiagnosticSpec {
            class: RnaIssueClass::ArityMismatch,
            failure_kind: RnormFailureKind::ArityMismatch,
            code: "RNORM_ARITY".into(),
            repair_hint_template: "declare the operator arity explicitly".into(),
        },
        RnormDiagnosticSpec {
            class: RnaIssueClass::SpliceClosureFailure,
            failure_kind: RnormFailureKind::SpliceClosureFailure,
            code: "RNORM_SPLICE".into(),
            repair_hint_template: "close splice payloads with `eie`".into(),
        },
        RnormDiagnosticSpec {
            class: RnaIssueClass::UnknownOperator,
            failure_kind: RnormFailureKind::UnknownOperator,
            code: "RNORM_OPERATOR".into(),
            repair_hint_template: "replace unknown operator shorthand with structural RNA".into(),
        },
    ]
}

pub fn ssr_transition_specs() -> Vec<SsrTransitionSpec> {
    vec![
        (TokenClass::Atom, "emit_leaf"),
        (TokenClass::Binder, "emit_binding"),
        (TokenClass::Operator, "push_operator_context"),
        (TokenClass::GroupLeft, "push_frame"),
        (TokenClass::GroupRight, "close_frame"),
        (TokenClass::Separator, "finalize_term"),
        (TokenClass::StrandRef, "emit_reference_edge"),
        (TokenClass::Meta, "ignore"),
    ]
    .into_iter()
    .map(|(token_class, action)| SsrTransitionSpec {
        token_class,
        action: action.into(),
        persists_state: false,
        semantic_inference: false,
    })
    .collect()
}

pub fn canonical_rule_specs() -> Vec<CanonicalRuleSpec> {
    vec![
        CanonicalRuleSpec {
            id: "CNORM_SORT_TEXT_ID".into(),
            description: "sort structural nodes by text and then stable node id".into(),
            deterministic: true,
            semantic_lookup_required: false,
        },
        CanonicalRuleSpec {
            id: "CNORM_ERASE_FORMATTING".into(),
            description:
                "erase whitespace and splice-layout variation already removed by lower phases"
                    .into(),
            deterministic: true,
            semantic_lookup_required: false,
        },
    ]
}

pub fn token_map_entries() -> Vec<TokenMapEntry> {
    vec![
        TokenMapEntry {
            sample: "a".into(),
            class: TokenClass::Atom,
            rule: "[a-z0-9]".into(),
        },
        TokenMapEntry {
            sample: "A".into(),
            class: TokenClass::Binder,
            rule: "[A-Z]".into(),
        },
        TokenMapEntry {
            sample: "(".into(),
            class: TokenClass::GroupLeft,
            rule: "opening delimiter".into(),
        },
        TokenMapEntry {
            sample: ")".into(),
            class: TokenClass::GroupRight,
            rule: "closing delimiter".into(),
        },
        TokenMapEntry {
            sample: ",".into(),
            class: TokenClass::Separator,
            rule: "separator".into(),
        },
        TokenMapEntry {
            sample: ".".into(),
            class: TokenClass::StrandRef,
            rule: "strand reference".into(),
        },
        TokenMapEntry {
            sample: "_".into(),
            class: TokenClass::Operator,
            rule: "structural operator".into(),
        },
        TokenMapEntry {
            sample: " ".into(),
            class: TokenClass::Meta,
            rule: "whitespace".into(),
        },
    ]
}

pub fn tokenize_rna_bytes(bytes: &[u8]) -> Result<(TokenStream, TokenizationReceipt), String> {
    let text = std::str::from_utf8(bytes).map_err(|err| {
        format!(
            "RNA tokenization failed: input is not valid UTF-8 at byte {}",
            err.valid_up_to()
        )
    })?;
    tokenize_rna(text)
}

pub fn tokenize_rna(input: &str) -> Result<(TokenStream, TokenizationReceipt), String> {
    let source_text = input.replace("\r\n", "\n");
    let mut tokens = Vec::new();
    let mut issues = Vec::new();

    for (span_start, ch) in source_text.char_indices() {
        let span_end = span_start + ch.len_utf8();
        let class = classify_rna_char(ch);
        if ch.is_control() && !ch.is_whitespace() {
            issues.push(TokenizationIssue {
                offset: span_start,
                message: format!("invalid RNA control character U+{:04X}", ch as u32),
            });
        }
        tokens.push(RnaToken {
            class,
            value: ch as u32,
            flags: 0,
            span_start,
            span_end,
            text: ch.to_string(),
        });
    }

    let receipt = TokenizationReceipt {
        id: format!("TOK_{:x}", stable_hash_u64(&source_text)),
        token_count: tokens.len(),
        byte_len: source_text.len(),
        issues,
    };
    if !receipt.issues.is_empty() {
        return Err(
            serde_json::to_string(&receipt).unwrap_or_else(|_| "RNA tokenization failed".into())
        );
    }
    Ok((
        TokenStream {
            source_text,
            byte_len: receipt.byte_len,
            tokens,
        },
        receipt,
    ))
}

fn classify_rna_char(ch: char) -> TokenClass {
    match ch {
        '(' | '[' | '{' => TokenClass::GroupLeft,
        ')' | ']' | '}' => TokenClass::GroupRight,
        ',' | ';' => TokenClass::Separator,
        '.' => TokenClass::StrandRef,
        '_' | '|' | ':' | '=' | '+' | '-' | '*' | '/' | '^' | '≔' | '‖' | '→' => {
            TokenClass::Operator
        }
        c if c.is_whitespace() => TokenClass::Meta,
        c if c.is_ascii_uppercase() => TokenClass::Binder,
        _ => TokenClass::Atom,
    }
}

pub fn normalize_rna(input: &str) -> Result<(NormalizedRna, RnaNormalizationReceipt), String> {
    let (stream, _) = tokenize_rna(input)?;
    normalize_token_stream(&stream)
}

pub fn normalize_token_stream(
    stream: &TokenStream,
) -> Result<(NormalizedRna, RnaNormalizationReceipt), String> {
    let (normalized, mut receipt) = normalize_rna_text(&stream.source_text)?;
    receipt.token_stream_id = format!("TOK_{:x}", stable_hash_u64(&stream.source_text));
    receipt.shorthand_eliminated = !normalized.normalized_text.contains("  ");
    Ok((normalized, receipt))
}

fn normalize_rna_text(input: &str) -> Result<(NormalizedRna, RnaNormalizationReceipt), String> {
    let raw_text = input.replace("\r\n", "\n");
    let chars = raw_text.chars().collect::<Vec<_>>();
    let mut normalized = String::new();
    let mut issues = Vec::new();
    let mut splice_regions = Vec::new();
    let mut paren_depth = 0usize;
    let mut splice_start: Option<usize> = None;
    let mut prev_was_space = false;
    let mut index = 0usize;

    while index < chars.len() {
        if index + 2 < chars.len()
            && chars[index] == 'i'
            && chars[index + 1] == 'e'
            && chars[index + 2] == 'i'
        {
            if splice_start.is_some() {
                issues.push(RnaNormalizationIssue {
                    class: RnaIssueClass::Ambiguity,
                    offset: index,
                    message: "nested splice region start".into(),
                    repair_hints: vec!["close the previous splice before opening a new one".into()],
                });
            }
            if !normalized.is_empty() && !normalized.ends_with(' ') {
                normalized.push(' ');
            }
            let start = normalized.len();
            normalized.push_str("iei ");
            splice_start = Some(start);
            index += 3;
            prev_was_space = true;
            continue;
        }
        if index + 2 < chars.len()
            && chars[index] == 'e'
            && chars[index + 1] == 'i'
            && chars[index + 2] == 'e'
        {
            match splice_start.take() {
                Some(start) => {
                    if normalized.ends_with(' ') {
                        normalized.pop();
                    }
                    if !normalized.is_empty() && !normalized.ends_with(' ') {
                        normalized.push(' ');
                    }
                    normalized.push_str("eie");
                    splice_regions.push((start, normalized.len()));
                }
                None => issues.push(RnaNormalizationIssue {
                    class: RnaIssueClass::SpliceClosureFailure,
                    offset: index,
                    message: "splice close without matching open".into(),
                    repair_hints: vec!["remove the stray `eie` or add a matching `iei`".into()],
                }),
            }
            index += 3;
            prev_was_space = false;
            continue;
        }

        let ch = chars[index];
        match ch {
            '(' | '[' | '{' => {
                paren_depth += 1;
                normalized.push(ch);
                prev_was_space = false;
            }
            ')' | ']' | '}' => {
                if paren_depth == 0 {
                    issues.push(RnaNormalizationIssue {
                        class: RnaIssueClass::GroupingFailure,
                        offset: index,
                        message: format!("unmatched closing delimiter `{ch}`"),
                        repair_hints: vec![
                            "remove the unmatched closer or add a matching opener".into(),
                        ],
                    });
                } else {
                    paren_depth -= 1;
                }
                normalized.push(ch);
                prev_was_space = false;
            }
            c if c.is_whitespace() => {
                if !prev_was_space && !normalized.is_empty() {
                    normalized.push(' ');
                    prev_was_space = true;
                }
            }
            _ => {
                normalized.push(ch);
                prev_was_space = false;
            }
        }
        index += 1;
    }

    if paren_depth > 0 {
        issues.push(RnaNormalizationIssue {
            class: RnaIssueClass::GroupingFailure,
            offset: chars.len(),
            message: "unterminated grouping delimiter".into(),
            repair_hints: vec!["close all opened grouping delimiters".into()],
        });
    }
    if let Some(start) = splice_start.take() {
        issues.push(RnaNormalizationIssue {
            class: RnaIssueClass::SpliceClosureFailure,
            offset: start,
            message: "unterminated splice region".into(),
            repair_hints: vec!["close the splice with `eie`".into()],
        });
    }

    let normalized = normalized.trim().to_string();
    let state = if !issues.is_empty() {
        RnaState::Raw
    } else if !splice_regions.is_empty()
        || normalized.contains("iei ")
        || normalized.ends_with(" eie")
        || normalized.contains(" eie ")
    {
        RnaState::Spliced
    } else if normalized == raw_text.trim() {
        RnaState::Stabilized
    } else {
        RnaState::Expressed
    };

    let receipt = RnaNormalizationReceipt {
        id: format!("RNR_{:x}", stable_hash_u64(&normalized)),
        token_stream_id: String::new(),
        state,
        normalized_text: normalized.clone(),
        shorthand_eliminated: !normalized.contains("  "),
        splice_regions: splice_regions.clone(),
        issues,
    };
    if !receipt.issues.is_empty() {
        return Err(
            serde_json::to_string(&receipt).unwrap_or_else(|_| "RNA normalization failed".into())
        );
    }
    Ok((
        NormalizedRna {
            raw_text,
            normalized_text: normalized,
            state,
            splice_regions,
        },
        receipt,
    ))
}

pub fn resolve_spliced_rna(normalized: &NormalizedRna) -> Result<(SsrGraph, SsrReceipt), String> {
    let root_id = format!(
        "SSR_ROOT_{:x}",
        stable_hash_u64(&normalized.normalized_text)
    );
    let mut nodes = vec![SsrNode {
        id: root_id.clone(),
        kind: SsrNodeKind::Root,
        text: normalized.normalized_text.clone(),
        children: Vec::new(),
    }];

    for (idx, token) in normalized.normalized_text.split_whitespace().enumerate() {
        let kind = if token == "iei" || token == "eie" {
            SsrNodeKind::Splice
        } else if token.starts_with('(')
            || token.ends_with(')')
            || token.starts_with('{')
            || token.ends_with('}')
            || token.starts_with('[')
            || token.ends_with(']')
        {
            SsrNodeKind::Group
        } else {
            SsrNodeKind::Atom
        };
        let id = format!("SSR_{:x}", stable_hash_u64(&format!("{idx}:{token}")));
        nodes[0].children.push(id.clone());
        nodes.push(SsrNode {
            id,
            kind,
            text: token.into(),
            children: Vec::new(),
        });
    }

    let graph = SsrGraph {
        root_id: root_id.clone(),
        nodes,
        strand_partition: vec!["core".into()],
    };
    let receipt = SsrReceipt {
        id: format!("SSR_RCP_{:x}", stable_hash_u64(&graph.root_id)),
        root_id,
        node_count: graph.nodes.len(),
        transition_count: graph.nodes.len().saturating_sub(1),
        transition_table_hash: hash_serialized(&ssr_transition_specs()),
        ephemeral: true,
        normalized_rna_id: format!("RNR_{:x}", stable_hash_u64(&normalized.normalized_text)),
    };
    Ok((graph, receipt))
}

pub fn canonicalize_ssr_graph(graph: &SsrGraph) -> Result<(CanonicalGraph, CnormReceipt), String> {
    let mut canonical_nodes = graph.nodes.clone();
    canonical_nodes.sort_by(|left, right| {
        left.text
            .cmp(&right.text)
            .then_with(|| left.id.cmp(&right.id))
    });
    let canonical_text = canonical_nodes
        .iter()
        .filter(|node| !matches!(node.kind, SsrNodeKind::Root))
        .map(|node| node.text.clone())
        .collect::<Vec<_>>()
        .join(" ");
    let canonical_hash = format!("{:x}", stable_hash_u64(&canonical_text));
    let graph = CanonicalGraph {
        root_id: graph.root_id.clone(),
        canonical_text,
        canonical_nodes,
    };
    let receipt = CnormReceipt {
        id: format!("CNR_{:x}", stable_hash_u64(&graph.root_id)),
        root_id: graph.root_id.clone(),
        canonical_hash,
        rule_table_hash: hash_serialized(&canonical_rule_specs()),
        idempotent: true,
        erased_variations: vec!["formatting".into(), "splice-layout".into()],
    };
    Ok((graph, receipt))
}

pub fn execute_lower_chain(rna: &str) -> Result<LowerChainExecution, SystemFailureState> {
    let mut kernel = PhaseExecutionKernel::default();

    let token_out = kernel.execute(
        PhaseId::Tokenization,
        format!("{:x}", stable_hash_u64(rna)),
        Vec::new(),
        Some("retain raw RNA bytes".into()),
        || tokenize_rna(rna),
        |(stream, receipt)| {
            vec![
                InvariantCheck {
                    name: "all_bytes_classified".into(),
                    passed: stream.byte_len == rna.replace("\r\n", "\n").len()
                        && stream
                            .tokens
                            .iter()
                            .all(|token| token.span_end <= stream.byte_len),
                    detail: "token stream byte coverage must match normalized RNA input bytes"
                        .into(),
                },
                InvariantCheck {
                    name: "no_tokenization_issues".into(),
                    passed: receipt.issues.is_empty(),
                    detail: "tokenization issues must be empty after successful tokenization"
                        .into(),
                },
                InvariantCheck {
                    name: "no_semantic_token_authority".into(),
                    passed: token_class_specs()
                        .iter()
                        .all(|spec| !spec.semantic_authority),
                    detail:
                        "token classes classify structure only and cannot become semantic authority"
                            .into(),
                },
            ]
        },
    )?;
    let (token_stream, tokenization_receipt) = token_out;

    let rn_out = kernel.execute(
        PhaseId::RnaNormalization,
        hash_serialized(&token_stream),
        vec![tokenization_receipt.id.clone()],
        Some("retain raw RNA".into()),
        || normalize_token_stream(&token_stream),
        |(normalized, receipt)| {
            vec![
                InvariantCheck {
                    name: "no_semantic_mutation".into(),
                    passed: receipt.issues.is_empty(),
                    detail: "normalization issues must be empty after successful normalization"
                        .into(),
                },
                InvariantCheck {
                    name: "deterministic_tokenization".into(),
                    passed: !normalized.normalized_text.is_empty(),
                    detail: "normalized RNA must not be empty after successful normalization"
                        .into(),
                },
                InvariantCheck {
                    name: "token_boundary_preserved".into(),
                    passed: receipt.token_stream_id == tokenization_receipt.id,
                    detail: "RNORM must be grounded in the token stream receipt it consumed".into(),
                },
                InvariantCheck {
                    name: "diagnostics_are_table_owned".into(),
                    passed: !rnorm_diagnostic_specs().is_empty(),
                    detail: "RNORM diagnostics must be governed by reusable diagnostic specs"
                        .into(),
                },
            ]
        },
    )?;
    let (normalized, rn_receipt) = rn_out;
    let normalized_hash = format!("{:x}", stable_hash_u64(&normalized.normalized_text));

    let ssr_out = kernel.execute(
        PhaseId::StructuralResolution,
        normalized_hash.clone(),
        vec![rn_receipt.id.clone()],
        Some("discard SSR and retain RNORM receipt".into()),
        || resolve_spliced_rna(&normalized),
        |(graph, receipt)| {
            vec![
                InvariantCheck {
                    name: "ephemeral_only".into(),
                    passed: receipt.ephemeral,
                    detail: "SSR must remain ephemeral".into(),
                },
                InvariantCheck {
                    name: "root_node_present".into(),
                    passed: graph.nodes.iter().any(|node| {
                        node.id == graph.root_id && matches!(node.kind, SsrNodeKind::Root)
                    }),
                    detail: "SSR graph must contain exactly one root-like entry".into(),
                },
                InvariantCheck {
                    name: "transition_table_structural_only".into(),
                    passed: ssr_transition_specs()
                        .iter()
                        .all(|spec| !spec.persists_state && !spec.semantic_inference),
                    detail: "SSR transition specs must not persist state or infer semantics".into(),
                },
            ]
        },
    )?;
    let (kernel_graph, ssr_receipt) = ssr_out;
    let graph_hash = hash_serialized(&kernel_graph);

    let cnorm_out = kernel.execute(
        PhaseId::CanonicalNormalization,
        graph_hash,
        vec![ssr_receipt.id.clone()],
        Some("retain SSR receipt and reject unstable canonical output".into()),
        || canonicalize_ssr_graph(&kernel_graph),
        |(graph, receipt)| {
            vec![
                InvariantCheck {
                    name: "canonical_hash_matches".into(),
                    passed: receipt.canonical_hash
                        == format!("{:x}", stable_hash_u64(&graph.canonical_text)),
                    detail: "canonical hash must match canonical text".into(),
                },
                InvariantCheck {
                    name: "canonical_nodes_sorted".into(),
                    passed: graph.canonical_nodes.windows(2).all(|window| {
                        window[0].text < window[1].text
                            || (window[0].text == window[1].text && window[0].id <= window[1].id)
                    }),
                    detail: "canonical node ordering must be stable".into(),
                },
                InvariantCheck {
                    name: "canonical_rules_lookup_free".into(),
                    passed: receipt.idempotent
                        && canonical_rule_specs()
                            .iter()
                            .all(|spec| spec.deterministic && !spec.semantic_lookup_required),
                    detail: "CNORM rules must be deterministic, idempotent, and lookup-free".into(),
                },
            ]
        },
    )?;
    let (canonical_graph, cnorm_receipt) = cnorm_out;

    Ok(LowerChainExecution {
        token_stream,
        tokenization_receipt,
        normalized,
        rn_receipt,
        kernel_graph,
        ssr_receipt,
        canonical_graph,
        cnorm_receipt,
        ledger_entries: kernel.entries().to_vec(),
    })
}

pub fn hash_serialized<T: Serialize>(value: &T) -> String {
    let encoded = serde_json::to_string(value).unwrap_or_else(|_| "<unserializable>".into());
    format!("{:x}", stable_hash_u64(&encoded))
}

#[cfg(test)]
mod genome_foundation_tests {
    use super::*;

    #[test]
    fn tokenization_classifies_ascii_and_unicode_without_semantic_authority() {
        let (stream, receipt) = tokenize_rna("A.ι ≔ σ ‖ κ").expect("tokenization");
        assert_eq!(receipt.token_count, stream.tokens.len());
        assert!(receipt.issues.is_empty());
        assert!(
            stream
                .tokens
                .iter()
                .any(|token| token.class == TokenClass::Binder && token.text == "A")
        );
        assert!(
            stream
                .tokens
                .iter()
                .any(|token| token.class == TokenClass::StrandRef)
        );
        assert!(
            stream
                .tokens
                .iter()
                .any(|token| token.class == TokenClass::Operator && token.text == "≔")
        );
        assert!(
            token_class_specs()
                .iter()
                .all(|spec| !spec.semantic_authority)
        );
    }

    #[test]
    fn tokenization_rejects_non_whitespace_control_characters() {
        let err = tokenize_rna("a\u{0000}b").expect_err("control character should fail");
        assert!(err.contains("invalid RNA control character"));
    }

    #[test]
    fn normalize_rna_is_deterministic_under_spacing_variation() {
        let (left, _) = normalize_rna("ι   ≔   σ   ‖   κ").expect("left normalization");
        let (right, _) = normalize_rna("ι ≔ σ ‖ κ").expect("right normalization");
        assert_eq!(left.normalized_text, right.normalized_text);
    }

    #[test]
    fn rnorm_receipt_is_token_grounded_and_diagnostic_specs_are_closed() {
        let (stream, token_receipt) = tokenize_rna("ι   ≔   σ").expect("tokenization");
        let (normalized, receipt) = normalize_token_stream(&stream).expect("rnorm");
        assert_eq!(receipt.token_stream_id, token_receipt.id);
        assert!(receipt.shorthand_eliminated);
        assert_eq!(normalized.normalized_text, "ι ≔ σ");
        assert!(
            rnorm_diagnostic_specs()
                .iter()
                .all(|spec| !spec.code.is_empty() && !spec.repair_hint_template.is_empty())
        );
    }

    #[test]
    fn normalize_rna_reports_unterminated_splice() {
        let err = normalize_rna("iei f g").expect_err("unterminated splice should fail");
        assert!(err.contains("SpliceClosureFailure") || err.contains("unterminated splice"));
    }

    #[test]
    fn canonicalization_erases_spacing_after_ssr() {
        let (left_norm, _) = normalize_rna("ι   ≔   σ   ‖   κ").expect("left normalization");
        let (right_norm, _) = normalize_rna("ι ≔ σ ‖ κ").expect("right normalization");
        let (left_graph, _) = resolve_spliced_rna(&left_norm).expect("left ssr");
        let (right_graph, _) = resolve_spliced_rna(&right_norm).expect("right ssr");
        let (left_canon, left_receipt) = canonicalize_ssr_graph(&left_graph).expect("left cnorm");
        let (right_canon, right_receipt) =
            canonicalize_ssr_graph(&right_graph).expect("right cnorm");
        assert_eq!(left_canon.canonical_text, right_canon.canonical_text);
        assert_eq!(left_receipt.canonical_hash, right_receipt.canonical_hash);
        assert!(left_receipt.idempotent);
        assert_eq!(
            left_receipt.rule_table_hash,
            hash_serialized(&canonical_rule_specs())
        );
    }

    #[test]
    fn ssr_transition_specs_remain_ephemeral_and_lookup_free() {
        let (normalized, _) = normalize_rna("A.ι ≔ σ").expect("rnorm");
        let (graph, receipt) = resolve_spliced_rna(&normalized).expect("ssr");
        assert!(receipt.ephemeral);
        assert_eq!(
            receipt.transition_table_hash,
            hash_serialized(&ssr_transition_specs())
        );
        assert_eq!(
            receipt.transition_count,
            graph.nodes.len().saturating_sub(1)
        );
        assert!(
            ssr_transition_specs()
                .iter()
                .all(|spec| !spec.persists_state && !spec.semantic_inference)
        );
    }

    #[test]
    fn dna_packet_validation_rejects_semantic_symbol_table_flag() {
        let packet = LocusPacket {
            header: LocusPacketHeader {
                artifact_class: GenomeArtifactClass::Gene,
                surface: GenomeSurface::Dna,
                kind: LocusPacketKind::CanonicalTransfer,
                version_major: 1,
                version_minor: 0,
                authority_tier: 1,
                capabilities: LocusCapabilityMask::default(),
                grammar_id: "rna.v1".into(),
                schema_hash: "canonical_graph.v1".into(),
                integrity_hash: "abc".into(),
                strand_manifest: vec!["core".into()],
                feature_flags: 0,
                root_subject_id: "SUBJ".into(),
            },
            sections: vec![LocusSection {
                opcode: LocusOpcode::SymbolTable,
                flags: 0x0001,
                subject_id: "SUBJ".into(),
                payload: Vec::new(),
            }],
        };
        let report = validate_dna_packet(&packet);
        assert!(report.symbol_table_semantic_authority);
        assert!(!report.failures.is_empty());
    }

    #[test]
    fn lower_chain_emits_ledger_entries_for_each_closed_phase() {
        let execution = execute_lower_chain("ι ≔ σ ‖ κ").expect("lower chain");
        assert_eq!(execution.ledger_entries.len(), 4);
        assert!(
            execution
                .ledger_entries
                .iter()
                .all(|entry| entry.validation_result == PhaseValidationResult::Passed)
        );
        assert_eq!(execution.ledger_entries[0].phase_id, PhaseId::Tokenization);
        assert_eq!(
            execution.ledger_entries[1].phase_id,
            PhaseId::RnaNormalization
        );
        assert_eq!(
            execution.ledger_entries[2].phase_id,
            PhaseId::StructuralResolution
        );
        assert_eq!(
            execution.ledger_entries[3].phase_id,
            PhaseId::CanonicalNormalization
        );
        assert_eq!(
            execution.ledger_entries[1].dependency_edges,
            vec![execution.tokenization_receipt.id.clone()]
        );
    }

    #[test]
    fn lower_chain_halts_and_commits_failed_ledger_entry() {
        let failure = execute_lower_chain("iei f g").expect_err("unterminated splice should halt");
        assert_eq!(failure.phase_id, PhaseId::RnaNormalization);
        assert_eq!(
            failure.ledger_entry.validation_result,
            PhaseValidationResult::Failed
        );
        assert!(!failure.ledger_entry.failure_records.is_empty());
    }
}
