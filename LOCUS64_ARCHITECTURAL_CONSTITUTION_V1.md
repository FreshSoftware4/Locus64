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

### L64-G032 - RNA/DNA Public Authority Surfaces

RNA and DNA remain the only public authority surfaces.

Evidence:

- `cargo test -p l64-cli --test cli`
- Proving tests: `rna_dna_primary_authority_commands_work`, `standalone_projection_leaf_commands_are_removed`, `inspect_dna_output_is_not_rna_source`

Constitutional effect:

- Public authority workflows must pass through source/canonical RNA or DNA.
- Projection, inspection, and removed legacy commands cannot define public authority surfaces.

### L64-G033 - Bundle-Entry Migration Ingress

Bundle-entry JSON is deletion-bound migration ingress only.

Evidence:

- `cargo test -p l64-bundle`
- Proving tests: `bundle_document_import_can_cross_explicit_migration_ingress`, `bundle_entry_text_rejects_deprecated_surface_schema_entries`

Constitutional effect:

- Bundle-entry text may cross an explicit migration membrane.
- Bundle-entry JSON cannot become a public authority syntax or native ontology source.

### L64-G036 - Canonical RNA Reconstruction

DNA reconstructs canonical RNA, not authored RNA.

Evidence:

- `cargo test -p l64-cli --test cli cki_registry_fixture_preserves_rna_dna_fixed_point`

Constitutional effect:

- DNA-to-RNA reconstruction targets canonical RNA.
- Original authored RNA remains lineage/source material, not a required inverse of DNA.

### L64-G045 - Mechanical Evidence Requirement

Constitutional rules require mechanical evidence.

Evidence:

- `L64_APPROVAL_GATES.md` promotion law

Constitutional effect:

- Candidate rulings remain outside this constitution until their executable proof is landed and recorded.
- Documentation alone cannot promote runtime law.

## Explicit Non-Promotions

The following are not constitutional authority in v1:

- `QaDocument` as ontology source.
- Bundle-entry JSON as public authority syntax.
- Bincode as canonical DNA.
- Projection/report/view artifacts as source.
- Legacy Q-surface syntax or Q-surface policy objects.
- Graph, arena, map, index, fold, or scheduler representation as substrate authority.
- Current molecular names unless promoted by a future proven gate.

## Current Blockers

The following gates remain candidate or deferred and must not be treated as proven law:

- Duplex-pair promotion law beyond local first-slice tests.
- Full canonical structural identity replacing every token-hash fallback.
- Bundle semantic reroute as native authority rather than parity scaffold.
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
