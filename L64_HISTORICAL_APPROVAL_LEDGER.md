---
document_status: historical
superseded_by: L64_APPROVAL_GATES.md
scope: legacy candidates, superseded gates, and migration approvals through Pass 27
preservation_reason: correction history and evidence lineage
---

# Locus64 Historical Approval Ledger

This ledger preserves the former mixed-era approval table as historical evidence. Candidate, rejected, deferred, superseded, and migration-era rows here do not define the live product. Current executable gates are listed in [`L64_APPROVAL_GATES.md`](L64_APPROVAL_GATES.md).

## Preserved prior approval ledger

# Locus64 Approval Gate Ledger

This file tracks candidate architectural law. A gate is only constitutional after it has executable evidence.

Status values:

- `Candidate`: accepted direction, not mechanically proven yet
- `Proven`: mechanically proven and eligible for constitution promotion
- `Deferred`: valid later concern outside the active dependency frontier
- `Rejected`: retained as an audited non-path

Proof classes:

- `CR`: compiler/category rejection
- `FP`: fixed-point reconstruction
- `DV`: DNA validation
- `MG`: strategic migration
- `PE`: deterministic parallel equivalence
- `PL`: projection-loss accounting
- `EA`: external-adapter witness
- `RI`: release/source integrity

| Gate | Candidate ruling | Proof | Depends on | Status | Evidence |
|---|---|---:|---|---|---|
| L64-G001 | Duplex pair is the smallest locally authoritative unit | CR,DV | canonical encoding | Proven | `cargo test -p l64-core duplex_pair_validation_requires_semantic_and_complement_strands` |
| L64-G002 | Neither strand is independently promotable | CR | G001 | Proven | `cargo test -p l64-core duplex_pair_validation_requires_semantic_and_complement_strands` |
| L64-G003 | Pair-local validity does not imply domain closure | CR,DV | G001 | Proven | `cargo test -p l64-core domain_closure_blocks_promotion_on_open_frontier` |
| L64-G004 | Complement encodes admissibility burdens; receipts only discharge them | CR | G001 | Proven | `cargo test -p l64-core authority_complement_must_carry_burdens_not_receipt_only_claims` |
| L64-G005 | Duplex is native DNA structure, not a third public format | CR,RI | artifact roles | Candidate | pending |
| L64-G006 | Current molecular nouns are provisional role mappings | CR,MG | role model | Candidate | pending |
| L64-G007 | Codons compile to structural instructions, not semantic codebooks | CR,DV | symbolic law | Proven | `cargo test -p l64-core` (`codon_lexon_law_tables_are_unique_and_complete`, `codon_header_phase_controls_source_admission`) |
| L64-G008 | Lexons are scoped receipted bindings, not global meaning | CR,DV | scope law | Proven | `cargo test -p l64-core` (`lexon_aliases_resolve_to_scoped_canonical_targets`, `representative_lexons_bind_scoped_targets_with_receipts`) |
| L64-G009 | Macro-codons compile to reaction law, not workflow labels | CR,DV | G007 | Proven | `cargo test -p l64-core macro_codons_are_separate_from_lexons` |
| L64-G010 | Graphs are resolver, plan, diagnostic, or projection representations only | CR | graph roles | Candidate | pending |
| L64-G011 | Duplex pairing is logical and variable-length, not fixed-width syntax | DV,FP | canonical encoding | Candidate | pending |
| L64-G012 | Domain theorem/claim/proof classes remain above root substrate | CR,MG | substrate roles | Candidate | pending |
| L64-G013 | Generated-word bans are regression tripwires, not authority law | CR | typed admission | Proven | `cargo test -p l64-core generated_structural_word_ban_detects_old_label_payloads` |
| L64-G014 | Bincode is cache/transport only, not canonical DNA | DV,FP | canonical encoding | Candidate | pending |
| L64-G015 | Persistent authority identity derives from canonical structural bytes | FP,DV | distinction/equivalence | Proven | `cargo test -p l64-core` (`canonical_structure_erases_spacing_but_preserves_order`, `dna_packet_validation_checks_canonical_structure_digest`) |
| L64-G016 | Explicit canonical instructions replace token-hash authority | FP,DV | G015 | Proven | `cargo test -p l64-core canonical_structure_erases_spacing_but_preserves_order` |
| L64-G017 | Every authority section commitment binds its payload | DV | packet codec | Proven | `cargo test -p l64-core generic_dna_packet_validates_section_payload_commitment` |
| L64-G018 | Authority payload decode requires complete packet validation | DV | G017 | Proven | `cargo test -p l64-core` (`dna_packet_validation_checks_canonical_structure_digest`, `generic_dna_packet_validates_section_payload_commitment`) |
| L64-G019 | Legacy decoding requires explicit migration or forensic mode | DV,MG | decode roles | Proven | `cargo test -p l64-core legacy_packet_decode_requires_explicit_migration_or_forensic_mode` |
| L64-G020 | Authority decoding is bounded before allocation | DV | packet codec | Proven | `cargo test -p l64-core locus_packet_decode_rejects_oversized_fields_before_payload_copy` |
| L64-G021 | Authority tiers are typed law at DNA validation | CR,DV | role model | Proven | `cargo test -p l64-core dna_packet_validation_rejects_unknown_authority_tier` |
| L64-G022 | Capability summaries require witness coordinates | CR,DV | witness model | Candidate | pending |
| L64-G023 | Generated caches and ambiguous reports do not ship as source | RI | release gate | Proven | `cargo test -p l64-cli --test cli genome_release_exports_coordinate_spine_and_rejects_views_as_source` |
| L64-G024 | Serial and parallel authoritative outputs are byte-equivalent | PE | closure scheduler | Proven | `cargo test -p l64-core deterministic_authority_merge_excludes_worker_count_and_input_order` |
| L64-G025 | Scheduler plans and telemetry are receipts, not authority | PE,CR | G024 | Proven | `cargo test -p l64-core deterministic_authority_merge_excludes_worker_count_and_input_order` |
| L64-G026 | Authored obligation status is intent, never evidence | CR,MG | evaluator model | Proven | `cargo test -p l64-cert authored_obligation_status_does_not_satisfy_evidence` |
| L64-G027 | Evaluator authority is explicit, named, and scoped | CR,MG | evaluator interface | Rejected | superseded by direct execution and deletion law L64-G068 |
| L64-G028 | Every certification verdict carries authority scope | CR,DV | scope law | Proven | `scripts/verify-native-carrier.sh` (`canonical_native_authority_certifies_from_direct_burdens`, native CLI membrane gate) |
| L64-G029 | Persisted report-derived lineage/readiness records are a valid authority path | CR,MG | artifact roles | Rejected | superseded by deletion law L64-G067 |
| L64-G030 | Policy precedence emits deterministic receipts | PE,DV | policy law | Rejected | superseded by deletion law L64-G068; no live policy registry remains |
| L64-G031 | Namespace import is a receipted bridge, not string rewriting | MG,DV | bridge law | Candidate | pending |
| L64-G032 | RNA and DNA remain the only public authority surfaces | CR,RI | artifact roles | Proven | `cargo test -p l64-cli --test cli` (`rna_dna_primary_authority_commands_work`, `standalone_projection_leaf_commands_are_removed`, `inspect_dna_output_is_not_rna_source`) |
| L64-G033 | Bundle-entry JSON is deletion-bound migration ingress only | MG,RI | native bundle path | Rejected | migration ingress and overlay world deleted by L64-G068 |
| L64-G034 | Remaining Qa/Qc types are migration ASTs, not core ontology | MG,CR | native bundle path | Candidate | pending |
| L64-G035 | Codebook/glyph/combo packs are projection or legacy machinery | MG,CR | projection boundary | Candidate | pending |
| L64-G036 | DNA reconstructs canonical RNA, not authored RNA | FP | canonical encoding | Proven | `cargo test -p l64-cli --test cli cki_registry_fixture_preserves_rna_dna_fixed_point` |
| L64-G037 | Grooves, folds, indexes, maps, and plans stay outside identity | FP,PE | G015 | Candidate | pending |
| L64-G038 | Release strings are renderers over witnessed products | MG,PL | expression layer | Proven | `cargo test -p l64-cli --test cli genome_release_exports_coordinate_spine_and_rejects_views_as_source` |
| L64-G039 | Seed JSON is bootstrap source and compiles to DNA | MG,RI | native seed path | Candidate | pending |
| L64-G040 | Every sample artifact has an explicit source/migration/projection role | RI,CR | artifact roles | Proven | `cargo test -p l64-cli --test cli genome_release_exports_coordinate_spine_and_rejects_views_as_source` |
| L64-G041 | External standards are dialects and witness providers | EA,CR | adapter law | Deferred | pending |
| L64-G042 | Every standards export carries a projection-loss receipt | PL,EA | G041 | Deferred | pending |
| L64-G043 | Proof assistants provide scoped external witnesses | EA,DV | evaluator interface | Deferred | pending |
| L64-G044 | SBOM/provenance standards are loss-accounted projections | PL,EA | G041 | Deferred | pending |
| L64-G045 | Constitutional rules require mechanical evidence | CR,FP,DV,MG,PE,PL,EA | this ledger | Proven | ledger promotion law |
| L64-G046 | New crates are justified only by enforceable authority membranes | CR | dependency audit | Candidate | pending |
| L64-G047 | Migration completes only after execution authority reroutes | MG | native execution path | Candidate | pending |
| L64-G048 | Reasoning cost and validation locality precede byte compactness | FP,DV | canonical path | Candidate | pending |
| L64-G049 | Authority tiers are earned through receipted transitions | CR,DV | G021 | Candidate | pending |
| L64-G050 | Product identity is witness-shaped; digest is its commitment | DV,MG | witness/product law | Candidate | pending |
| L64-G051 | Boollet is a transition-memory sidecar, not Locus64 authority | CR,MG | Bands B-E native authority path | Candidate | pending |
| L64-G052 | Boollet Rust embedding requires fixture parity with the Python reference | FP,MG | Boollet Rust port | Candidate | pending |
| L64-G053 | Boollet suggestions re-enter Locus64 only as proposals or witness coordinates | CR,MG | G051 | Candidate | pending |
| L64-G054 | Boollet replay identity may guide remediation ordering but cannot affect canonical identity | FP,CR | G015,G051 | Candidate | pending |
| L64-G055 | Locus64 admission/remediation sidecar fixtures prove usefulness without authority transfer | MG,DV | G051,G052,native Locus64 transition fixture | Candidate | pending |
| L64-G056 | No naked equality: structural equivalence requires explicit law specs and transition context | CR,FP | G015,distinction law | Proven | `cargo test -p l64-core` (`equivalence_law_makes_ordering_explicit_before_cnorm`, `distinction_law_classifies_collapse_before_equivalence`) |
| L64-G057 | K2 bridge/path comparability is a transport obligation over DNA authority, not root identity | CR,MG | G056,G031 | Candidate | pending |
| L64-G058 | K2 proof-shape success is a witness receipt and cannot override DNA admission or fixed-point failure | CR,DV | G018,G036,G056 | Candidate | pending |
| L64-G059 | Route selection is lexicographic by lawfulness, obligations, equivalence transport, loss, rollback, proof shape, complexity, then reuse payoff | MG,PE | domain closure | Candidate | pending |
| L64-G060 | Boollet replay identity is sidecar reproducibility evidence and cannot substitute for Locus64 canonical identity | CR,FP | G015,G051,G052 | Candidate | pending |
| L64-G061 | Native bundle transport frames canonical DNA members without creating composite authority | CR,DV,RI | G018,G032 | Proven | `scripts/verify-native-carrier.sh` (`bundle_is_exact_ordered_dna_transport`, `execution_verifies_each_projection_without_composite_authority`, native membrane gate) |
| L64-G062 | Native certification derives only from exact DNA and verified per-context burdens; bundles have no composite certificate | CR,DV,RI | G028,G061 | Proven | `scripts/verify-native-carrier.sh` (`canonical_native_authority_certifies_from_direct_burdens`, `invalid_child_context_invalidates_the_authority_certification`, `bundle_members_are_certified_independently`) |
| L64-G063 | Native observation is reconstructible non-authoritative projection over exact DNA; bundles have no composite observation | CR,DV,RI | G028,G061,G062 | Proven | `scripts/verify-native-carrier.sh` (`canonical_authority_observation_binds_verified_native_surfaces`, `invalid_child_context_remains_visible_in_report_and_certification`, `bundle_members_are_observed_independently`, native membrane gate) |

| L64-G064 | Native change analysis compares exact current authorities without creating diff authority or composite bundle verdict | CR,DV,RI | G061,G062,G063 | Proven | `scripts/verify-native-carrier.sh` (`identical_dna_is_exactly_unchanged`, `structural_and_verdict_movement_is_directly_visible`, `bundle_members_remain_independent_change_contacts`, native membrane gate) |

| L64-G065 | Campaign/report cache storage may be deleted once native certification, observation, and change carry every productive contact | MG,RI | G062,G063,G064 | Proven | `scripts/verify-legacy-cache-deletion.sh` plus packaged historical export |
| L64-G066 | Prediction/recompute/execution-plan records and bundle-lock/execution-manifest storage may be deleted once direct native change and current carriers replace their productive contacts | MG,RI | G061,G062,G063,G064,G065 | Proven | `scripts/verify-legacy-plan-storage-deletion.sh` plus packaged historical export |
| L64-G067 | Report-derived lineage, readiness, promotion-queue, and handoff stores may be deleted once native certification, observation, and change carry their productive facts | MG,RI | G062,G063,G064,G065,G066 | Proven | `scripts/verify-legacy-lineage-readiness-deletion.sh` plus packaged historical export |
| L64-G068 | Registry-overlay bundle execution and stored policy resolution may be deleted once current transport and direct execution carry every productive contact | MG,RI | G061,G062,G063,G064,G065,G066,G067 | Proven | `scripts/verify-legacy-registry-overlay-policy-deletion.sh` plus packaged historical export |

| L64-G073 | Large-authority optimization is admitted only from measured structural hot paths and may not weaken exact authority or retained-projection verification | FP,DV,PE | G069,G070,G071,G072 | Proven | `scripts/verify-native-scale.sh`, profile corpus, full offline workspace tests and Clippy |

## Promotion Law

1. Add or identify executable evidence.
2. Record the exact test, fixture, or migration artifact in `Evidence`.
3. Verify all listed dependencies are `Proven`.
4. Change the row to `Proven` in the same change that lands the proof.
5. Promote the ruling into the constitution only after workspace conformance passes.

No bulk status promotion is allowed. Evidence is gate-specific even when one test exercises multiple gates.

## G069 — Legacy authority island deletion

Approved. The complete historical theorem/campaign/research/registry/tower execution island was externally exported and deleted. The live workspace has eleven dependency-free packages and passes full offline tests and Clippy. Evidence: `changelog.log` and `scripts/verify-legacy-authority-island-deletion.sh`.

## G070 — Direct native execution carrier

Approved. `l64-execution` provides direct, non-persistent RNA/DNA structural evaluation; bundle execution reuses the same member carrier. Full offline workspace tests, Clippy, and the end-to-end native demo pass. Evidence: `changelog.log` and execution tests.

## G071 — Direct Native CLI Hardening

Approved. The current CLI has no boolean compatibility fallback, rejects wrong carrier formats at the requested contact, exposes command-specific help and version output, emits stable human-readable diagnostics, rejects dead artifact-class routing, and creates authority/bundle files atomically without overwriting existing paths. Evidence: `changelog.log` and `scripts/verify-cli-hardening.sh`.


## G072 — Verdict-Aware Process and Bounded Transport

Approved. Certification-bearing commands emit complete output and use stable member-local verdict process codes; RNA parser failures carry exact token spans; bundle creation and bundle-facing CLI operations process members sequentially while preserving exact ordered `L64B` transport. Evidence: `changelog.log` and `scripts/verify-process-contract.sh`.

## G073 — Measured Native Scale Law

Approved. Profiling identified per-mutation whole-state recomputation, repeated closure memo allocation, whole-route scans, and whole-graph equality scans as structural hot paths. Canonical source compilation now bulk-constructs and derives its final state symbol once; routes are indexed per node; closure analysis is shared per context; equality traversal is edge-local; and freshly derived in-process projections are not immediately rederived. Exact RNA/DNA fixed points, tracked mutation journals, stale/forged projection rejection, packet bytes, bundle framing, and member-local authority remain unchanged. Evidence: `changelog.log`, `scripts/verify-native-scale.sh`, the Pass 27 benchmark corpus, and full offline tests/Clippy.
