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
| L64-G027 | Evaluator authority is explicit, named, and scoped | CR,MG | evaluator interface | Candidate | pending |
| L64-G028 | Every certification verdict carries authority scope | CR,DV | scope law | Candidate | pending |
| L64-G029 | Report-derived research records are projections until replayed | CR,MG | artifact roles | Proven | `cargo test -p l64-research report_derivation_emits_lineage_record` |
| L64-G030 | Policy precedence emits deterministic receipts | PE,DV | policy law | Candidate | pending |
| L64-G031 | Namespace import is a receipted bridge, not string rewriting | MG,DV | bridge law | Candidate | pending |
| L64-G032 | RNA and DNA remain the only public authority surfaces | CR,RI | artifact roles | Proven | `cargo test -p l64-cli --test cli` (`rna_dna_primary_authority_commands_work`, `standalone_projection_leaf_commands_are_removed`, `inspect_dna_output_is_not_rna_source`) |
| L64-G033 | Bundle-entry JSON is deletion-bound migration ingress only | MG,RI | native bundle path | Proven | `cargo test -p l64-bundle` (`bundle_document_import_can_cross_explicit_migration_ingress`, `bundle_entry_text_rejects_deprecated_surface_schema_entries`) |
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

## Promotion Law

1. Add or identify executable evidence.
2. Record the exact test, fixture, or migration artifact in `Evidence`.
3. Verify all listed dependencies are `Proven`.
4. Change the row to `Proven` in the same change that lands the proof.
5. Promote the ruling into the constitution only after workspace conformance passes.

No bulk status promotion is allowed. Evidence is gate-specific even when one test exercises multiple gates.
