---
document_status: historical
superseded_by: LOCUS64_NATIVE_CONSTITUTION.md
scope: pre-native and migration-era architectural laws through Pass 27
preservation_reason: correction history and forensic reconstruction
---

# Locus64 Historical Architectural Law Ledger

This ledger preserves the former mixed-era constitution exactly as trajectory evidence. It is not a current architecture source. Current authority law lives in [`LOCUS64_NATIVE_CONSTITUTION.md`](LOCUS64_NATIVE_CONSTITUTION.md); current executable gates live in [`L64_APPROVAL_GATES.md`](L64_APPROVAL_GATES.md).

## Preserved prior constitution

# Locus64 Architectural Constitution v1

This file is an index of mechanically demonstrated architectural law. It is not a wishlist, roadmap, branding document, or substitute for executable evidence.

Rules enter this constitution only after their corresponding approval gate is marked `Proven` in `L64_APPROVAL_GATES.md`.

## Authority Rule

Runtime authority is earned by closed transitions, validated packet structure, stable commitments, deterministic merge behavior, and receipted lineage. Names, projections, reports, caches, legacy formats, display IDs, scheduler choices, and migration objects do not become authority by existing in the repository.

## Proven Gates

### L64-G001 - Duplex Pair Local Authority Unit

Duplex pair is the smallest locally authoritative unit.

Evidence:

- `cargo test -p l64-core duplex_pair_validation_requires_semantic_and_complement_strands`

Constitutional effect:

- A semantic strand without its authority complement is not locally valid.
- Local authority begins at the paired structure, not at either strand alone.

### L64-G002 - No Strand-Only Promotion

Neither strand is independently promotable.

Evidence:

- `cargo test -p l64-core duplex_pair_validation_requires_semantic_and_complement_strands`

Constitutional effect:

- Semantic-only or complement-only material must fail local admission.
- Promotion logic must consume paired authority structure rather than single-lane meaning.

### L64-G003 - Local Validity Is Not Domain Closure

Pair-local validity does not imply domain closure.

Evidence:

- `cargo test -p l64-core domain_closure_blocks_promotion_on_open_frontier`

Constitutional effect:

- A locally valid pair may still be non-promotable when open obligations or external burdens remain.
- Domain closure is a separate gate above pair admission.

### L64-G004 - Complement Burden Law

Complement encodes admissibility burdens; receipts only discharge them.

Evidence:

- `cargo test -p l64-core authority_complement_must_carry_burdens_not_receipt_only_claims`

Constitutional effect:

- Authority complements must carry invariant and witness-form requirements.
- Receipt-only complement claims are malformed and cannot become local authority.

### L64-G007 - Codon Structural Law

Codons compile to structural instructions, not semantic codebooks.

Evidence:

- `cargo test -p l64-core`
- Proving tests: `codon_lexon_law_tables_are_unique_and_complete`, `codon_header_phase_controls_source_admission`

Constitutional effect:

- Codon tables define structural phase/admission law.
- Codon names are not semantic authority and must not become lookup-mediated meaning.

### L64-G008 - Lexon Scoped Binding Law

Lexons are scoped receipted bindings, not global meaning.

Evidence:

- `cargo test -p l64-core`
- Proving tests: `lexon_aliases_resolve_to_scoped_canonical_targets`, `representative_lexons_bind_scoped_targets_with_receipts`

Constitutional effect:

- Lexon aliases resolve into scoped canonical targets.
- Bindings require receipts and cannot become global semantic aliases.

### L64-G009 - Macro-Codon Reaction Law

Macro-codons compile to reaction law, not workflow labels.

Evidence:

- `cargo test -p l64-core macro_codons_are_separate_from_lexons`

Constitutional effect:

- Macro-codons are separate reaction-law structures.
- Workflow labels do not define executable law.

### L64-G013 - Generated-Word Regression Boundary

Generated-word bans are regression tripwires, not authority law.

Evidence:

- `cargo test -p l64-core generated_structural_word_ban_detects_old_label_payloads`

Constitutional effect:

- Generated-word bans may prevent old label payloads from leaking into structural regions.
- The ban list is diagnostic enforcement, not a source of meaning.

### L64-G015 - Canonical Structural Identity

Persistent authority identity derives from canonical structural bytes.

Evidence:

- `cargo test -p l64-core`
- Proving tests: `canonical_structure_erases_spacing_but_preserves_order`, `dna_packet_validation_checks_canonical_structure_digest`

Constitutional effect:

- Formatting variation cannot define authority identity.
- Authority identity must bind canonical structure, not authored source text.

### L64-G016 - Canonical Instruction Stream

Explicit canonical instructions replace token-hash authority.

Evidence:

- `cargo test -p l64-core canonical_structure_erases_spacing_but_preserves_order`

Constitutional effect:

- Canonical structure is encoded through versioned instructions.
- Token hash shortcuts cannot be promoted as authority identity.

### L64-G017 - Section Payload Commitment

Every authority section commitment binds its payload.

Evidence:

- `cargo test -p l64-core generic_dna_packet_validates_section_payload_commitment`

Constitutional effect:

- DNA validation must compare section payloads against payload commitments before authority decode can be trusted.
- Payload commitments are authority-adjacent validation material, not display identifiers.

### L64-G018 - Complete Authority Decode Validation

Authority payload decode requires complete packet validation.

Evidence:

- `cargo test -p l64-core`
- Proving tests: `dna_packet_validation_checks_canonical_structure_digest`, `generic_dna_packet_validates_section_payload_commitment`

Constitutional effect:

- Authority decode must validate payload commitment and canonical structure digest requirements.
- Partial packet success cannot promote a payload into authority.

### L64-G019 - Explicit Legacy Decode Mode

Legacy decoding requires explicit migration or forensic mode.

Evidence:

- `cargo test -p l64-core legacy_packet_decode_requires_explicit_migration_or_forensic_mode`

Constitutional effect:

- Current authority decode must not silently fall back to legacy packet decoding.
- Migration and forensic decode paths are intentional subordinate modes, not ambient compatibility.

### L64-G020 - Bounded Authority Decode

Authority decoding is bounded before allocation.

Evidence:

- `cargo test -p l64-core locus_packet_decode_rejects_oversized_fields_before_payload_copy`

Constitutional effect:

- Sized packet fields must pass a hard length cap before payload copy.
- Authority decoding cannot allocate unbounded field payloads from attacker-controlled lengths.

### L64-G021 - Authority Tier Validation

Authority tiers are typed law at DNA validation.

Evidence:

- `cargo test -p l64-core dna_packet_validation_rejects_unknown_authority_tier`

Constitutional effect:

- DNA validation must reject unknown authority tier values.
- The packet byte remains layout-compatible, but valid tier meaning is closed by the typed tier table.

### L64-G023 - Source Release Hygiene

Generated caches and ambiguous reports do not ship as source.

Evidence:

- `cargo test -p l64-cli --test cli genome_release_exports_coordinate_spine_and_rejects_views_as_source`

Constitutional effect:

- Release products must distinguish source, projection, record, receipt, and view roles.
- Generated views and reports may be shipped as products, but not as source authority.

### L64-G024 - Deterministic Parallel Equivalence

Serial and parallel authoritative outputs are byte-equivalent.

Evidence:

- `cargo test -p l64-core deterministic_authority_merge_excludes_worker_count_and_input_order`

Constitutional effect:

- Worker count and input completion order must not affect authoritative merge output.
- Canonical ordering must be derived from structural coordinates and payload commitments.

### L64-G025 - Scheduler Non-Authority

Scheduler plans and telemetry are receipts, not authority.

Evidence:

- `cargo test -p l64-core deterministic_authority_merge_excludes_worker_count_and_input_order`

Constitutional effect:

- Worker count, timing, lane assignment, and scheduling plan data are excluded from authority identity.
- Scheduler details may be recorded as diagnostics or receipts only.

### L64-G026 - Authored Obligation Status Is Intent

Authored obligation status is intent, never evidence.

Evidence:

- `cargo test -p l64-cert authored_obligation_status_does_not_satisfy_evidence`

Constitutional effect:

- An authored `Certified` obligation does not satisfy evidence by itself.
- Unsupported obligations must remain unsupported or blocked unless executable evaluation or native stored evidence exists.

### L64-G027 - Scoped Evaluator Authority (Superseded)

The registry-scoped evaluator-policy path is rejected. Direct certification owns one explicit in-process execution profile and emits no policy object, binding, resolution ID, precedence trace, or persistent policy receipt. See L64-G068.

### L64-G030 - Deterministic Policy Precedence Receipts (Superseded)

Policy precedence is no longer a live authority path. The policy registry and stored resolution machinery were deleted after historical export. Determinism now belongs to direct execution and exact native authority, not ordering policy records. See L64-G068.

### L64-G032 - RNA/DNA Public Authority Surfaces

RNA and DNA remain the only public authority surfaces.

Evidence:

- `cargo test -p l64-cli --test cli`
- Proving tests: `rna_dna_primary_authority_commands_work`, `standalone_projection_leaf_commands_are_removed`, `inspect_dna_output_is_not_rna_source`

Constitutional effect:

- Public authority workflows must pass through source/canonical RNA or DNA.
- Projection, inspection, and removed legacy commands cannot define public authority surfaces.

### L64-G033 - Bundle-Entry Migration Ingress (Superseded)

Bundle-entry JSON, `BundleWorld`, overlay merging, namespace import, and conflict-policy execution were deleted after historical export. Current bundle transport is exact ordered `L64B` over canonical `L64D` members. See L64-G061 and L64-G068.

### L64-G036 - Canonical RNA Reconstruction

DNA reconstructs canonical RNA, not authored RNA.

Evidence:

- `cargo test -p l64-cli --test cli cki_registry_fixture_preserves_rna_dna_fixed_point`

Constitutional effect:

- DNA-to-RNA reconstruction targets canonical RNA.
- Original authored RNA remains lineage/source material, not a required inverse of DNA.

### L64-G038 - Release Renderer Law

Release strings are renderers over witnessed products.

Evidence:

- `cargo test -p l64-cli --test cli genome_release_exports_coordinate_spine_and_rejects_views_as_source`

Constitutional effect:

- Release strings, claim pages, maps, views, and rendered records must be derived from witnessed authority products.
- Rendered release artifacts do not become source authority.

### L64-G040 - Artifact Role Explicitness

Every sample artifact has an explicit source, migration, projection, record, receipt, or view role.

Evidence:

- `cargo test -p l64-cli --test cli genome_release_exports_coordinate_spine_and_rejects_views_as_source`

Constitutional effect:

- Release and sample artifacts must advertise their role through type, filename, content marker, or command boundary.
- Ambiguous artifacts must fail source admission or be classified before use.

### L64-G056 - Explicit Structural Equivalence Law

Structural equivalence requires explicit law specs and transition context.

Evidence:

- `cargo test -p l64-core`
- Proving tests: `equivalence_law_makes_ordering_explicit_before_cnorm`, `distinction_law_classifies_collapse_before_equivalence`

Constitutional effect:

- CNORM cannot claim sameness from naked equality strings, graph shape, or source text alone.
- Equivalence must reference transition law and distinction law before collapse is allowed.

### L64-G045 - Mechanical Evidence Requirement

Constitutional rules require mechanical evidence.

Evidence:

- `L64_APPROVAL_GATES.md` promotion law

Constitutional effect:

- Candidate rulings remain outside this constitution until their executable proof is landed and recorded.
- Documentation alone cannot promote runtime law.

### L64-G028 - Certification Scope Law

Every certification verdict carries an explicit authority scope.

Evidence:

- `scripts/verify-native-carrier.sh`
- Proving tests: `canonical_native_authority_certifies_from_direct_burdens`, native CLI membrane gate

Constitutional effect:

- A verdict cannot silently broaden from native structural closure into theorem, campaign, benchmark, policy, or external truth.
- The authority bytes and the non-authoritative diagnostic symbol must remain distinguishable in certification output.

### L64-G061 - Native Bundle Transport Law

Native bundle transport frames canonical DNA members without creating composite authority.

Evidence:

- `scripts/verify-native-carrier.sh`
- Proving tests: `bundle_is_exact_ordered_dna_transport`, `execution_verifies_each_projection_without_composite_authority`

Constitutional effect:

- `L64B` may preserve order and exact member boundaries only.
- Bundle transport cannot merge registries, resolve semantic conflicts, or invent bundle-level truth.

### L64-G062 - Native Certification Derivation Law

Native certification derives only from exact DNA and verified per-context burdens; bundles have no composite certificate.

Evidence:

- `scripts/verify-native-carrier.sh`
- Proving tests: `canonical_native_authority_certifies_from_direct_burdens`, `invalid_child_context_invalidates_the_authority_certification`, `bundle_members_are_certified_independently`

Constitutional effect:

- Every native context must be evaluated; an invalid child context invalidates the authority certification.
- `OPEN`, `INCOMPLETE`, and `INVALID` remain distinct from `CERTIFIED`.
- `L64B` members retain independent verdicts; no bundle-level verdict or certificate may be emitted.

### L64-G063 - Native Observation Projection Law

Native observation is reconstructible non-authoritative projection over exact DNA; bundles have no composite observation.

Evidence:

- `scripts/verify-native-carrier.sh`
- Proving tests: `canonical_authority_observation_binds_verified_native_surfaces`, `invalid_child_context_remains_visible_in_report_and_certification`, `bundle_members_are_observed_independently`, native CLI membrane gate

Constitutional effect:

- Observation may bind direct certification to verified replay and closure/opcode report projections only.
- Observation cannot persist a second authority, campaign, report cache, policy result, or fabricated replay history.
- `L64B` members remain independently observed; no bundle-level observation or verdict may be emitted.

### L64-G064 - Native Change Projection Law

Native change analysis compares exact current authorities without creating diff authority or a composite bundle verdict.

Evidence:

- `scripts/verify-native-carrier.sh`
- Proving tests: `identical_dna_is_exactly_unchanged`, `structural_and_verdict_movement_is_directly_visible`, `bundle_members_remain_independent_change_contacts`, native CLI membrane gate

Constitutional effect:

- Exact canonical DNA bytes determine equality; symbolic section differences and projection movements remain diagnostics.
- Change analysis cannot persist predictions, plans, locks, manifests, policy resolutions, report caches, campaigns, or receipts.
- Ordered bundle members are compared independently by transport index; no bundle-level change authority or verdict may be emitted.

### L64-G065 - Legacy Cache Deletion Law

Campaign/report storage may be deleted after every productive current contact is carried directly by native authority projections and an external historical export exists.

Evidence:

- `scripts/verify-legacy-cache-deletion.sh`
- Pass 19 historical export archive and Pass 18 restore source
- packaged native carrier verification after deletion

Constitutional effect:

- `l64-observe`, `l64-admin`, certification execution caches, report-cache paths, report-ID replay, and invalidation lookup are not live runtime surfaces.
- Certification computes fresh from its current inputs. Native observation and change remain reconstructible and non-persistent.
- Deleted storage schemas must not be recreated inside native crates. Historical decoding belongs to external archives or explicit file contacts only.

## Explicit Non-Promotions

The following are not constitutional authority in v1:

- `QaDocument` as ontology source.
- Bundle-entry JSON as public authority syntax.
- Bincode as canonical DNA.
- Projection/report/view artifacts as source.
- Legacy Q-surface syntax or Q-surface policy objects.
- Graph, arena, map, index, fold, or scheduler representation as substrate authority.
- Current molecular names unless promoted by a future proven gate.

### L64-G066 - Legacy Plan, Lock, and Manifest Deletion Law

Stored prediction, recomputation, execution-plan, bundle-lock, and execution-manifest records are obsolete once exact native authorities and direct projections carry their productive contacts.

Evidence:

- `scripts/verify-legacy-plan-storage-deletion.sh`
- Pass 20 historical export archive and Pass 19 restore source
- packaged dependency-free native carrier verification after deletion

Constitutional effect:

- Prediction, semantic-drift, recomputation-plan, plan-execution, assessment, reconciliation, bundle-lock, replay-lock, lock-receipt, lock-diff, policy-manifest, and execution-manifest record types are not live schemas.
- QA/registry admission, bundle import/export, packet-store paths, policy builders, certification enrichment, and command routes for those records are deleted.
- Numeric packet-kind tombstones may remain only to preserve unrelated discriminant numbering; they may have no constructors or live references.
- Deleted storage schemas must not be recreated inside native crates. Historical reconstruction belongs to the external archive.


### L64-G067 - Legacy Lineage, Readiness, Queue, and Handoff Deletion Law

Persisted report-derived lineage, promotion-readiness, promotion-queue, and governed-handoff records are obsolete once exact native certification, observation, and change expose their productive facts directly.

Evidence:

- `scripts/verify-legacy-lineage-readiness-deletion.sh`
- Pass 21 historical export archive and Pass 20 restore source
- packaged dependency-free native carrier verification after deletion

Constitutional effect:

- `ResearchLineageRecord`, `PromotionReadinessReport`, `PromotionQueueEntry`, `PromotionQueueStatus`, and `HandoffPacket` are not live schemas.
- Research bundle fields, report-derived constructors, persistence paths, import/export/list routes, status aggregation, and compile-RNA lineage persistence for those records are deleted.
- Live certification output must not manufacture references to deleted lineage records.
- Deleted schemas must not be recreated inside native crates. Historical reconstruction belongs to the external archive.


### L64-G068 - Registry Overlay and Stored Policy Deletion Law

Registry-overlay bundle worlds and stored policy resolution are obsolete once current native transport, certification, observation, and change carry every productive contact directly.

Evidence:

- `scripts/verify-legacy-registry-overlay-policy-deletion.sh`
- Pass 22 historical export archive and Pass 21 restore source
- packaged dependency-free native carrier verification after deletion

Constitutional effect:

- `l64-bundle` and `l64-policy` are not live crates.
- `BundleWorld`, overlay registries, bundle manifests, merge reports, conflict policies, namespace import, and bundle-world caches are not live schemas or execution paths.
- `MechanizationPolicyObject`, `PolicyBinding`, `PolicyResolution`, policy precedence traces, policy IDs, and policy hashes are not live schemas or report coordinates.
- The seed registry may retain surviving theorem/campaign records temporarily, but it admits no evaluator-policy objects or overlay-local authority.
- Certification uses one direct in-process execution profile and emits no persistent policy object, binding, resolution, or receipt.
- Current `L64B` transport orders independent canonical `L64D` authorities and creates no composite authority.
- Deleted overlay and policy machinery must not be recreated inside native crates. Historical reconstruction belongs to the external archive.

## Current Blockers

The following gates remain candidate or deferred and must not be treated as proven law:

- Duplex-pair promotion law beyond local first-slice tests.
- Full canonical structural identity replacing every token-hash fallback.
- The historical theorem/campaign/research/registry authority island is deleted; no compatibility runtime remains.
- Seed JSON replacement by native DNA bootstrap.
- Boollet integration as a sidecar transition-memory module.
- External standards and proof assistants as scoped witness providers.

## Maintenance Rule

Every future constitution edit must include:

1. The promoted gate ID.
2. The exact evidence command, fixture, or proof artifact.
3. The authority effect.
4. The non-authority boundary it preserves.

If any of those are missing, the ruling belongs in `L64_APPROVAL_GATES.md`, not here.

## L64-G069 — Delete the Superseded Authority Island

Once every productive current contact is carried directly by exact native authority and verified projections, historical theorem/campaign/research/registry execution machinery must be exported and removed rather than retained as a compatibility runtime.

Proving boundary:
- `scripts/verify-legacy-authority-island-deletion.sh`
- `cargo test --locked --offline --workspace`
- `cargo clippy --locked --offline --workspace --all-targets -- -D warnings`

Authority effect:
- the live workspace is eleven dependency-free native packages;
- no seed registry, research store, producer host, tower store, campaign engine, or legacy dispatcher remains;
- deleted commands fail as permanent tombstones and cannot touch the filesystem.

## Native execution carrier

Direct execution is non-persistent structural authority evaluation. `run-rna` compiles canonical RNA transiently; `run-dna` validates canonical DNA. Both verify the root projection, certify every context, mutate no authority, and create no run record. Ordered bundle execution reuses the same member carrier and creates no composite execution authority.

## L64-G071 — Direct Native CLI Boundary

The native operator boundary must dispatch current commands directly. A wrong carrier format is an error at the requested command contact, not a compatibility-fallback signal.

Proving boundary:
- `scripts/verify-cli-hardening.sh`
- `cargo test --locked --offline --workspace`
- `cargo clippy --locked --offline --workspace --all-targets -- -D warnings`

Constitutional effect:
- no `Result<bool, _>` command membrane or `Ok(false)` fallback may return;
- public carrier errors must render stable human-readable diagnostics rather than Rust debug syntax;
- `compile-rna` and `compile-bundle` create new files atomically and refuse replacement;
- no-op legacy selectors such as `--artifact-class` must fail explicitly;
- command discovery must include `--help`, `help <command>`, and `--version` without external dependencies.


## L64-G072 — Verdict-Aware Process and Bounded Transport Law

Native commands that compute certification must preserve the full derived result while returning a stable process status for the worst member-local verdict. RNA parse failures must identify the exact source token span. Ordered bundle processing must not require retaining the complete transport and all decoded member graphs simultaneously.

Proving boundary:
- `scripts/verify-process-contract.sh`
- `streaming_encoder_matches_exact_transport_and_decoder_releases_members_sequentially`
- `streaming_decoder_never_requests_the_whole_bundle_buffer`
- `rna_diagnostic_renders_exact_token_span`
- full locked offline workspace tests and Clippy

Constitutional effect:
- `0`, `10`, `11`, and `12` represent `CERTIFIED`, `OPEN`, `INCOMPLETE`, and `INVALID` successful computations; operational failure remains `2`;
- process status is a projection over certification and never authority;
- RNA diagnostics preserve exact line, column, and token length without introducing a diagnostic registry;
- bundle encoding and bundle-facing CLI operations process one member at a time and preserve exact `L64B` bytes, member order, and member-local authority.

## L64-G073 — Measured Native Scale Law

Performance work must begin with a reproducible authority-shaped workload and a measured causal hot path. Canonical source compilation may suppress intermediate journals and state-symbol recomputation only while constructing a transient graph that is finalized once before publication. Public graph mutations remain tracked. Derived indexes and shared analysis may accelerate exact queries but cannot enter canonical authority bytes. Fresh in-process projections may be accepted by construction; any retained, transported, cached, or externally supplied projection must still be explicitly verified against exact authority.

Proving boundary:
- `scripts/verify-native-scale.sh`
- exact RNA/DNA fixed-point and journal tests
- stale and forged projection rejection tests
- full locked offline workspace tests and Clippy
