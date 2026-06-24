# Locus64 Linear Execution Rail

This is the authoritative linear execution rail for Locus64.

Compounding change chains are no longer separate top-level rails. A compounding change chain is a valid change to this rail that preserves the trajectory:

```text
syntax is transient
structure is authority
canonical form is identity
execution is lineage-bound
reuse is proven, not assumed
```

The first compounding change chain applied here is the consolidated unified substrate and execution specification. It replaces the prior substrate-inversion wording with a tighter system definition, phase model, and Rust implementation target.

## Rail Operating Model

A linear execution rail is a temporal dependency structure, not just an architecture document.

Its purpose is to prevent interruption, ambiguity, and drift from changing the implementation trajectory. The document must let a developer or agent resume from the last completed rail node, read only the local window needed for that node, and continue without rediscovering the architecture.

Each rail node must answer:

- what is the current input state?
- what exact codebase change happens next?
- what prior output does it depend on?
- what files or crates are likely touched?
- what invariant must become stronger?
- what test or inspection proves the node is complete?
- what downstream work becomes easier because this node landed?

The rail is intentionally verbose where ambiguity would otherwise be resolved during coding. It is still cheaper than letting every coding pass rediscover dependency order.

## Local Reading Window

When executing the rail, read only:

- the governing laws at the top of this file
- the current rail node
- the immediately previous completed node
- the immediately next planned node
- any referenced crate/file touched by the node

Do not reread the whole rail unless the trajectory changes.

## Change Chain Rule

A compounding change chain is valid only if it preserves or improves the temporal dependency structure.

Allowed change chains:

- split an oversized node into smaller ordered nodes
- insert a missing prerequisite before a blocked node
- remove a node that is already complete or made obsolete
- tighten an invariant without changing the public trajectory
- replace a vague step with Rust-specific file/crate actions
- add a conformance test that prevents future drift

Forbidden change chains:

- add upper-stack work before lower substrate closure
- introduce a parallel public surface model
- preserve obsolete compatibility formats without an actual ecosystem requirement
- promote compatibility formats into authority
- organize work primarily around file/crate deletion before the replacement authority path exists
- choose easy leaf deletion ahead of replacing a higher-fanout obsolete authority mechanism
- elevate an implementation representation such as a graph, arena, tree, or opcode stream into substrate authority
- hide unresolved ambiguity in future implementation details
- widen scope without reducing downstream cost
- reorder phases without stating the dependency reason

Every accepted change chain must leave the rail more executable than before.

## Load-Bearing Beam Strategy

Execution order is chosen by dependency pressure, not by which deletion is easiest.

When obsolete mechanisms remain, rank them by how much authority flow they still carry:

1. Replace the highest-fanout obsolete authority beam with a stronger RNA/DNA or lineage-native substrate.
2. Migrate the smallest representative command, fixture, and test through that replacement.
3. Expand the migration until the obsolete beam no longer carries major workflow responsibility.
4. Delete residual leaf projections, helper crates, samples, and docs only after the replacement path proves it can carry the load.

Leaf deletion is allowed early only when the leaf is genuinely isolated and does not delay a higher-pressure replacement.

The prior Q/projection cleanup is now historical. The current load-bearing beam is the RNA/DNA membrane:

```text
RNA source
 -> canonical structure
 -> DNA authority
 -> canonical RNA sequencing
 -> DNA fixed point
```

Any command, fixture, report, receipt, or projection that can cross that membrane without a role check is now the highest-priority debt source.

## Active Work Selector

When choosing the next implementation slice, optimize for burden reduction per development effort. The rail decides the next slice with this scoring order:

```text
highest score =
  authority effect
  * fanout
  * ambiguity removed
  * testability
  * reversibility
  / implementation size
```

Use the following decision order:

1. **Authority effect first**
   - Fix mechanisms that define, admit, validate, promote, or migrate authority before mechanisms that only display, cache, report, or document it.
2. **Fanout second**
   - Prefer one small substrate membrane used by many paths over a large fix inside one leaf workflow.
3. **Ambiguity removed third**
   - Prefer changes that make future code illegal or mechanically typed over changes that merely document intent.
4. **Testability fourth**
   - Prefer slices with direct unit or conformance tests over slices that require broad interpretation to verify.
5. **Reversibility fifth**
   - Prefer additive membranes and narrow reroutes before broad deletions unless the obsolete path is already proven non-load-bearing.
6. **Implementation size last**
   - If two slices reduce similar burden, choose the one with fewer files, fewer public API changes, and less fixture churn.

Current active selector output:

```text
Band F strategic migration membrane
-> route remaining transitional bundle/product/campaign paths through native authority products
-> remove or quarantine residual transitional fields once native products carry equivalent evidence
```

Rationale:

- Digest-role separation, DNA payload commitments, and authority decode modes have landed as the current packet membrane.
- Production authority paths now use current-authority decoding; migration/forensic decoding is explicit rather than ambient.
- Node 07A/08, canonical instruction, duplex local admission, domain closure, deterministic merge, and first bundle migration slices have landed enough lower-chain law for the first real structure family: distinction classes, transition law, equivalence coupling, versioned canonical instruction bytes, local semantic/complement pair validation, promotion-blocking domain closure, worker-count-independent authority merge, and bundle parity evidence over native authority products now exist.
- The next burden reducer is routing remaining transitional bundle/product/campaign paths through native authority products because bundle parity now carries duplex, closure, and deterministic merge evidence.
- Any migration slice must preserve capability but reject category errors exposed by the new role, distinction, duplex, closure, deterministic merge, and bundle parity law.

If a future pass finds a higher-scoring slice, it must update this selector before implementing it.

## External Proving Slice Rule

External proving projects are allowed only when they strengthen the main authority path.

A proving slice must enter through the same temporal dependency chain as the rest of the system:

```text
external authored structure
 -> RNA ingestion
 -> RNORM/SSR/CNORM
 -> DNA authority
 -> validation receipts
 -> derived projections
```

External source code, Markdown reports, DOT graphs, JSON, Julia objects, notebooks, or other documents are not authority. They may provide extraction material, fixtures, or derived projections, but the proving value is real only when the same structure can be represented as RNA, compiled to DNA, validated, replayed, and projected back out.

The CKI typed registry is the current best pilot candidate because it is small, dependency-explicit, validation-oriented, and mathematically interpretable. It must remain a proving slice for the authority path, not a new side quest or a new public surface.

Every serious external proving slice must be able to ship with coordinates:

- canonical genome artifact
- source sequence or canonical reconstruction
- dependency spine
- closure map
- claim pages
- closure frontier
- stress map
- lineage
- replay record
- generated views
- view receipts

These are not additional authority surfaces. They are release artifacts derived from the same genome so reviewers can inspect claims, dependencies, open edges, projections, and replay steps without guessing where the authority lives.

## Current Execution Ledger

This ledger records landed rail movement so future passes do not rediscover or accidentally reverse it.

Landed:

- `CanonicalGraph` authority has been removed from the lower chain.
- `CanonicalStructure` is now the CNORM output carried by lower-chain execution artifacts.
- DNA canonical payloads now use `canonical_structure.v1`.
- DNA validation checks canonical payload digest, canonical id, and canonical bytes consistency before sequencing.
- DNA sequencing uses one packet validation chokepoint before reconstructing a stabilized RNA-facing artifact.
- Research persistence writes and reads `.dna` packet artifacts only; `.locus` mirror writes and JSON fallback reads are removed from that path.
- Observe and surface cache loads no longer use silent JSON fallback helpers.
- The old `read_section_packet_or_json` convenience API has been removed.
- CLI tests now cover the primary `compile-rna` -> `.dna` -> `sequence-dna` authority path directly.
- Surface export commands no longer write textual Q projection cache files as a side channel.
- Surface cache root creation no longer creates an obsolete `exports` cache directory.
- QK0 active projection support has been removed from CLI arguments, `l64-surfaces`, and the workspace.
- The `l64-qk0` crate has been deleted; its only remaining value is historical.
- Bundle import has a surface-free `QaDocument` core path.
- Bundle file import can now consume `.dna` packets carrying native bundle documents before falling back to surface parsing.
- CLI bundle execution/certification now has a `.dna` bundle fixture path that bypasses QC0 parsing.
- Certification option bundle hashes are computed over file bytes, so binary `.dna` bundle files do not force UTF-8 text assumptions.
- Node 16 has been split into Node 16A primary obsolete beam replacement and Node 16B residual projection extraction/deletion.
- CLI chain-rule certification, promoted-operator reuse, broken-bridge blocking, and imported-claim certification inputs now use `.dna` bundle fixtures instead of `.qc0` files.
- Admin optimizer, evaluator, replay, lock, prediction, scheduler, obligation, and chain-rule workflow tests now generate `.dna` bundle fixtures for bundle inputs instead of `.qc0` files.
- Admin lock-bundle certification options hash bundle bytes, so binary `.dna` bundle files do not force UTF-8 text assumptions.
- Cert crate imported overlay, multi-campaign, imported-kernel-claim, and stress-gap unit fixtures now use `.dna` bundle packets instead of `.qc0` files.
- CLI `run-theorem --file` and `certify-derived --file` now have a native `.dna` bundle fixture path instead of exporting theorem/campaign `.qc0` files as their representative test input.
- CLI now exposes `compile-bundle`, a narrow migration command that compiles current bundle-entry projection text into native `.dna` bundle packets.
- Root sample bundles now have generated `.dna` artifacts and README sample certification commands point at those `.dna` files.
- Admin replay adequacy sample coverage now generates a `.dna` bundle fixture from the sample text before lock/replay execution.
- Language documentation now routes ChatGPT-generated bundle-entry text through `compile-bundle` before certification, rather than telling users to certify QC0 directly.
- Bundle execution/import now rejects silent projection parsing when no explicit projection kind is provided; projection bundle text must be compiled with `compile-bundle` before normal execution.
- Root `.qc0` sample bundles have been deleted after active `.dna` sample bundles replaced them.
- Torture-test and usage-guide bundle/lock flows now point at `.dna` samples.
- CLI projection leaf commands `transcode`, `normalize-surface`, `roundtrip-check`, `surface-capabilities`, `dump-transform-receipt`, `parse`, `normalize`, `validate`, `import`, and `export` have been deleted.
- Remaining projection import tests use neutral `.projection` filenames plus explicit `--as`, so extension hints no longer drive projection behavior.
- Torture harness no longer calls deleted projection capability commands.
- Standalone projection normalize/import/export is no longer a user command path.
- `l64-surfaces` projection utility leaves `transcode_text`, `normalize_surface`, `roundtrip_check`, `surface_capabilities`, and `dump_transform_receipt` have been deleted after CLI callers were removed.
- Report packet cache persistence now writes `.dna` as the primary report artifact instead of `.locus`; `.locus` packet reads remain fallback-only.
- CLI export-report regression tests now assert report packet cache authority lands at `reports/<report-id>.dna` and does not create a primary `.locus` report packet.
- Torture-test report packet export/import now uses `.dna` output paths instead of `.locus`.
- CLI now exposes `export-validation-dna-bundle`, which emits self-contained validation bundles as native `.dna` bundle packets instead of requiring QC0 textual projection output.
- CLI validation hints, language docs, usage docs, and torture validation-bundle flow now prefer `export-validation-dna-bundle` plus native `import-bundle --conflict-policy exact-match`.
- A CLI regression test proves exported validation `.dna` bundles decode as `QaDocument` payloads and import through the native bundle path without projection parsing.
- The obsolete textual `export-validation-bundle --to qc0|qa0|qm0` command has been deleted after the `.dna` validation bundle replacement landed.
- A CLI regression test proves `export-validation-bundle` is no longer accepted and points users toward `export-validation-dna-bundle`.
- Native report packet export/import commands are now named `export-report-dna` and `import-report-dna`, replacing the misleading `export-locus-packet`/`import-locus-packet` public route.
- Textual report projection export has been deleted; report export now uses `export-report-dna` or `export-validation-dna-bundle`.
- CLI regression tests prove `export-report` and `export-report-projection` are rejected and point users toward DNA report export.
- Admin textual artifact projection export has been deleted.
- Admin regression tests prove `export-artifact` and `export-artifact-projection` are rejected.
- QM0 active projection support has been removed from CLI/admin surface arguments, registry capabilities, policy defaults, `l64-surfaces`, and the workspace. `SurfaceKind::Qm0` remains as a tombstoned enum discriminant so existing `.dna` bincode payloads do not shift variant indexes.
- The `l64-qm0` crate has been deleted; its only remaining value is historical.
- Explicit bundle projection import has been removed from `l64-bundle`, `l64-cli`, and `l64-admin`; bundle execution inputs are `.dna` only.
- `compile-bundle --as`, `certify --file --as`, `run-bundle --as`, `certify-bundle --as`, `run-theorem --as`, `lock-bundle --as`, and `predict-impact --as` have been removed.
- Report projection sidecars and report-surface flags have been removed from CLI commands.
- Bundle substrate tests now use a shared theorem fixture constructor, reducing repeated migration-test boilerplate while keeping native `.dna` bundle behavior unchanged.
- Stale `SurfaceArg` and `AdminSurfaceArg` Q-surface command enums have been removed from `l64-command`; the shared command crate now carries only active bundle and optimizer arguments.
- Default report policy and observe fallback policy no longer request removed Q export surfaces; report/export defaults now remain neutral unless a native DNA/report path explicitly supplies behavior.
- CLI, admin, and cert scheduler/report policy fixtures now use neutral `export_surfaces: []`, so tests no longer preserve removed Q export-surface expectations.
- Atlas inline policy resolution also uses neutral report export surfaces, so route selection no longer seeds removed Q projection outputs by default.
- `l64-surfaces`, `l64-qc0`, and `l64-qa0` have been removed from workspace membership and deleted from the codebase.
- Bundle-entry text remains only as an authoring convenience compiled by `compile-bundle` into `.dna`; it is not a public authority surface.

Still active refinement targets:

- None for QC0/QA0/surfaces. Remaining references to removed projection command names are negative regression tests or historical rail notes only.
- Node 11 membrane confusion patch has landed: source RNA, canonical RNA, DNA authority, and inspection/report artifacts now have separate command behavior.
- `sequence-dna` emits canonical reconstructable RNA rather than inspection JSON.
- `inspect-dna` owns JSON inspection output.
- `compile-rna` rejects JSON/report/projection artifacts before lower-chain tokenization.
- `verify-roundtrip` exposes the public `RNA -> DNA -> canonical RNA -> DNA` fixed-point gate.
- Node 11B codon/lexon/macro-codon law has exited, but its symbolic law must now be enforced by downstream molecular, bundle, product, and campaign paths rather than treated as an isolated registry.
- Node 11D behavior-bearing reroute is intentionally paused until Node 11D0 freezes the architectural constitution and classifies current code as authority, scaffold, parity evidence, migration-only, deferred, or rejected.
- Node 07A distinction/transition law is the lower-chain counterpart to Node 11D0: it must define which differences survive or collapse before EQUIV/CNORM can claim canonical sameness.
- CKI `fixtures/cki_registry.genome.rna` remains the best regression fixture for source/canonical/projection separation because it is nontrivial, dependency-explicit, and already exposes the source/authority/projection distinction.

Current verified gate:

- `cargo test -p l64-cli` passes after `.dna` migration of representative certification bundle tests.
- `cargo test --workspace` passes after the Node 16A `.dna` migration of representative bundle/import/certification tests.
- `cargo test -p l64-admin` passes after `.dna` migration of admin-generated bundle workflow tests.
- `cargo test -p l64-cert` passes after `.dna` migration of cert imported-bundle unit tests.
- `cargo test -p l64-cli` passes after `.dna` migration of theorem/campaign file execution tests.
- `cargo test --workspace` passes after cert and CLI theorem/campaign `.dna` fixture migration.
- `cargo test -p l64-cli` passes after adding `compile-bundle` and verifying compiled `.dna` bundle certification.
- `cargo test -p l64-admin` passes after admin sample replay coverage reroutes through a generated `.dna` bundle fixture.
- All generated root `.dna` sample bundles certify through `l64-cli certify-bundle`.
- `cargo test --workspace` passes after `compile-bundle`, `.dna` sample generation, and documentation rerouting.
- `cargo test -p l64-bundle` passes after rejecting silent projection bundle import without an explicit projection kind.
- `cargo test -p l64-cli` and `cargo test -p l64-admin` pass after sample and bundle execution reroutes to `.dna`.
- `cargo test --workspace` passes after strict bundle execution seam and `.qc0` sample deletion.
- `scripts/torture-test.ps1 -OuterRounds 1 -InnerNamespaces 1` passes with `.dna` sample bundle execution and zero failures.
- `cargo test -p l64-cli` and `cargo check -p l64-cli` pass after projection leaf command deletion.
- `cargo test --workspace` and `scripts/torture-test.ps1 -OuterRounds 1 -InnerNamespaces 1` pass after projection leaf command deletion.
- `cargo test -p l64-bundle` passes after deleting projection bundle import and making bundle file import `.dna` only.
- `cargo test -p l64-cli --test cli` passes after deleting projection import, report projection export, report sidecars, and report-surface flags.
- `cargo test -p l64-admin --test admin` passes after deleting admin artifact projection export.
- `cargo test --workspace` passes after deleting `l64-surfaces`, `l64-qc0`, and `l64-qa0` from the workspace.
- `scripts/torture-test.ps1 -OuterRounds 1 -InnerNamespaces 1` passes after removing report projection export from the torture harness.
- `cargo test --workspace` passes after the Node 11 membrane patch.
- `scripts/torture-test.ps1 -OuterRounds 1 -InnerNamespaces 1` passes after adding canonical RNA sequencing, `inspect-dna`, `verify-roundtrip`, and inspection-output rejection checks.
- Direct release-wrapper reproduction proves `sequence-dna` output is not JSON, `sequence-dna -> compile-rna` preserves canonical hash, and `inspect-dna -> compile-rna` is rejected.
- Initial Node 17A release spine has landed:
  - `export-genome-release` creates genome, source sequence, claim page, dependency spine, closure map, closure frontier, stress map, replay record, overview view, and view receipt artifacts from RNA/DNA authority.
  - Generated non-source release artifacts carry explicit role/surface markers and use projection/record/receipt filenames rather than `.locus` or JSON authority hints.
  - CLI regression proves generated manifest, claim page, dependency spine, closure map, lineage, closure frontier, stress map, replay record, view, and view receipt artifacts are rejected by `compile-rna`, while generated source RNA and canonical RNA remain accepted.
- Residue scan for `l64_surfaces`, `l64-qc0`, `l64-qa0`, and `l64-surfaces` returns no source/workspace dependency references; remaining mentions are historical rail notes or negative regression tests.
- Manifest and bundle-lock packet persistence has moved from `l64-surfaces` to `l64-locus`, so execution artifact storage no longer depends on the projection adapter crate.
- Report packet cache ownership, report IDs, and report packet persistence have moved from `l64-surfaces` to `l64-cert`, so report artifact storage now lives beside certification/report packet encoding.
- Report-to-document derivation and validation-bundle document derivation have moved from `l64-surfaces` to `l64-cert`, so certification-derived report documents now live beside report packet/cache ownership.
- `l64-surfaces` has been deleted; there are no remaining workspace duties for that crate.
- Projection transform receipt persistence has been deleted with the projection adapter crate; no projection transform history is written into cache as pseudo-lineage.
- Reproduction gate: `compile-rna sample.gene.rna -> sequence-dna -> compile-rna sequence-output` now preserves canonical identity through canonical RNA output, and `inspect-dna -> compile-rna` is rejected.
- Legacy cache fallback reads have been removed from admin/cert lookup paths: JSON manifest/lock/report cache files, legacy report `.locus` lookup, and execution `reports.json` are no longer live discovery paths.
- Execution manifest and bundle-lock packet cache files now write/read `.dna` paths (`<id>.dna`, `<id>.lock.dna`) rather than `.locus` paths.
- Certification execution cache entries now write/read `.dna` paths rather than `.locus` paths; sibling CLI/admin binaries must be rebuilt together because admin tests invoke the CLI binary for cache-producing setup.
- Observe persistence records now write/read `.dna` packet paths rather than `.locus` paths for observations, diffs, predictions, plans, explanations, assessments, executions, and reconciliations.
- Bundle-entry authoring text now requires the native `!l64-bundle v1` header. Obsolete `!qc0`/`!qa0` bundle headers are rejected by `compile-bundle`; active bundle tests and public docs have moved to the native header.
- New planning correction: current `.record`/`.projection` release artifacts and `!l64-bundle v1` JSON bodies are transitional. The next authority reroute is Node 11B-11F: codon/lexon law, molecular substrate codec, bundle semantic reroute, witnessed expression products, then legacy quarantine.
- Node 11A readiness guard has been inserted before Node 11B so codon/lexon implementation cannot compensate for prior-node defects.
- Node 11B first slice has landed in `l64-core`: `CodonSpec`, `LexonSpec`, phase/admission helpers, alias resolution, generated structural word ban list, and law validation report.
- `cargo test -p l64-core` passes with Node 11B law-table, alias, phase, digest/memo, lexon binding, and generated-word-ban tests.
- Node 11C first slice has landed in `l64-core`: substrate primitives, molecular codec record validation, deterministic encode/decode, and derived witness construction.
- Node 11D preparatory slice has landed in `l64-bundle`: `QaDocument` entries can lower into substrate atom codec records without preserving old entry kind names as native lexons.
- Node 11D dependency parity slice has landed in `l64-bundle`: bundle dependency edges lower into substrate bond codec records and parity reports check entry count, dependency edge count, codec validation, and missing atom/bond failures.
- Node 11D namespace parity slice has landed in `l64-bundle`: substrate lowering follows existing namespace behavior by rewriting local bundle IDs while preserving external dependency references.
- Node 11D conflict parity slice has landed in `l64-bundle`: substrate-lowered bundle documents coexist with current `ExactMatch` positive behavior and current `Reject` negative behavior for seed overlaps.
- `cargo test -p l64-core` and `cargo test -p l64-bundle` pass after the first molecular substrate codec and bundle-lowering slices.
- Correction from latest pasted-text decision: these Node 11B/11C/11D code slices are not full substrate completion. They are first-slice scaffolds and tests only. The rail must still treat Node 11B's concrete codon/lexon law tables, opcode allocation, tombstone/versioning policy, phase/admission matrix, generated-word ban list, and representative lexon/macro-codon corpus as the next governing payload before deeper substrate or bundle reroute work proceeds.
- Do not make bundle substrate parity a mandatory import gate yet. The current decision is rail-first: prove the complete symbolic law and native molecular frame direction before turning transitional bundle import into a load-bearing substrate path.
- Sharpened scaffold classification before resuming:
  - current codon/lexon code is an experimental law scaffold
  - current substrate primitive code is a provisional substrate scaffold
  - current bundle lowering code is a parity probe
  - none of these are authority-bearing
  - none of these may gate runtime import, certification, execution, or promotion yet
- Required resume gate: run `cargo fmt --check` and `cargo test --workspace` before continuing implementation. Run the compact torture slice too if the next change touches CLI/runtime command surfaces.
- `cki_registry.genome.rna` must be classified before it becomes load-bearing: fixture candidate, local scratch artifact, external project artifact, or ignored generated sample. If fixture candidate, move it into a controlled fixture path; if scratch, ignore or remove it.
- CKI classification landed: `cki_registry.genome.rna` moved from ambiguous project-root artifact to `fixtures/cki_registry.genome.rna` as a controlled regression fixture candidate. Its only current load-bearing role is proving the public `RNA -> DNA -> canonical RNA -> DNA` fixed-point membrane over a nontrivial external source.
- CLI regression `cki_registry_fixture_preserves_rna_dna_fixed_point` proves the controlled CKI fixture compiles as genome RNA, sequences to canonical RNA, recompiles, and preserves DNA integrity hash.
- Node 11B phase/admission matrix has landed in `l64-core/src/codons.rs`: every codon phase has an explicit admission rule, and only source RNA plus canonical RNA admit source compilation.
- `codon_header_admits_source_compile` now makes header phase law the source-admission membrane for native codon headers; filename extension and command habit are not the law.
- Node 11B second slice has landed in `l64-core`: codon/lexon law is now physically modularized into `codons.rs`, `lexons.rs`, and `macro_codons.rs`; `lib.rs` exports the law surface rather than growing the compiled law table inline.
- The retired inline codon/lexon scaffold has been physically removed from `l64-core/src/lib.rs`; there is no disabled duplicate law block left in the compiled crate source.
- Node 11B now has a separate `MacroCodonSpec` registry for recurring reaction/product/receipt patterns. Roundtrip, closure frontier, view receipt, integration receipt, and expression receipt are no longer lexons.
- Node 11B now has explicit opcode range and tombstone law: active codon/macro-codon opcodes are checked against tombstoned opcodes, and tombstones cannot be reused.
- Node 11B generated-word bans are now structural-region aware: generated native structural regions reject banned words, while gloss/comment/human-view regions may carry explanatory words.
- Node 11B now has a codon header parser that derives phase from header truth rather than filename extension.
- `cargo test --workspace` passed before Node 11B resumed; `cargo test -p l64-core`, `cargo check -p l64-bundle`, `cargo check --workspace`, and `cargo fmt --check` pass after the Node 11B second slice. Targeted CKI CLI fixed-point regression also passes.
- Node 11B exit gate is satisfied: CodonSpec, LexonSpec, MacroCodonSpec, alias resolution, phase/admission matrix, opcode range/tombstone law, structural-region generated-word ban, representative lexon/macro-codon corpus, and codon header parser are implemented and tested. `cargo test --workspace` passes after the gate.
- Next implementation node is Node 11C audit and substrate re-keying. Current substrate primitives remain provisional until audited against completed Node 11B law.
- Node 11C first audit/re-key slice has landed in `l64-core`: substrate primitives and molecular codec records moved from `lib.rs` into `molecular.rs`, witness construction moved into `witness.rs`, and `lib.rs` now exports the molecular/witness surfaces without carrying the implementation block inline.
- Node 11C codec validation now checks primitive-internal codons against completed Node 11B law: atom lexon bindings must resolve through the lexon registry, bond/reaction/chassis codons must be canonical, phase admission rules must exist, and witness primitives must be derived rather than authored truth.
- Node 11C schema-law slice has landed: `substrate_primitive_kinds` and `molecular_codec_record_field_names` make the closed primitive inventory and no-generic-payload-bag rule testable.
- `cargo test -p l64-core`, `cargo check --workspace`, and `cargo fmt --check` pass after Node 11C molecular/witness module extraction and stricter codec validation.
- Node 11C witness-normal serialization slice has landed: derived substrate witnesses can be wrapped as molecular codec records and roundtrip through deterministic encode/decode, while authored witness truth remains rejected. `cargo test -p l64-core`, `cargo check --workspace`, and `cargo fmt --check` pass after this slice.
- Node 11C codec envelope slice has landed: `MolecularCodecEnvelope` binds a codon header to a molecular codec record, validates header/record phase and subject agreement, validates canonical/admitted header codons, and roundtrips deterministically. `cargo test -p l64-core`, `cargo check --workspace`, and `cargo fmt --check` pass after this slice.
- Node 11D first authority-reroute slice has landed: bundle document lowering now produces molecular codec envelopes, and substrate parity validates the envelope membrane rather than validating bare records only. `cargo test -p l64-bundle`, `cargo check --workspace`, and `cargo fmt --check` pass after this slice.
- Node 11D import-evidence slice has landed: `BundleWorld` and its persisted cache now carry native substrate parity evidence derived from bundle document lowering, including `.dna` import reload parity. `cargo test -p l64-bundle`, `cargo check --workspace`, and `cargo fmt --check` pass after this slice.
- Immediate audit-fix slice has landed: `LOCUS64_LANGUAGE_SPEC.md` no longer recommends QC0 as the indirect integration language, and the `l64` wrapper no longer routes removed admin verbs as live commands. `compare-executions` remains routed because it is still implemented.
- First hash-policy untangling slice has landed: `l64-core` now exposes domain-separated `cache_hash_v1_*` helpers, and CLI/admin/cert cross-binary cache/report/lock hashes use the shared helper instead of local `DefaultHasher`/FNV variants.
- First packet-integrity untangling slice has landed: generic `encode_section_packet` DNA packets now bind `integrity_hash` to serialized payload bytes instead of substituting the schema hash. `l64-locus` has a regression proving same-schema different-payload packets get different integrity hashes.
- `cargo test -p l64-locus -p l64-core -p l64-cli -p l64-admin -p l64-cert`, `cargo check --workspace`, and `cargo fmt --check` pass after the immediate fixes, shared cache digest helper, and generic packet integrity hardening.
- New architecture-freeze correction: deeper Node 11D behavior-bearing reroute is blocked until Node 11D0 creates `LOCUS64_ARCHITECTURAL_CONSTITUTION_V1.md`, maps current code to constitutional law, and classifies codon/lexon/molecular/bundle/product/campaign code as authority, scaffold, parity evidence, migration-only, deferred, or rejected.
- New lower-chain correction: Node 07A distinction/transition law now sits between structural form and equivalence. EQUIV/CNORM may not infer sameness from raw shape, source text, graph layout, or hash equality; they must consume explicit distinction, invariant, transition, and collapse law.
- First digest-role enforcement slice has landed: `l64-core` now exposes `DigestRole`, `RoleDigest`, and role-separated BLAKE3 digest helpers; substrate witness IDs use `WitnessId` domain separation; molecular codec validation digests use `CodecDigest` domain separation. `cargo test -p l64-core` and `cargo check --workspace` pass after this slice.
- DNA validation receipt-ID slice has landed: `validate_dna_packet` now derives `DnaValidationReport.id` through `DigestRole::ReceiptId`, with a regression proving the validation receipt id is distinct from `PayloadCommitment` and `AuthorityId` domains. `cargo fmt --check`, `cargo test -p l64-core`, and `cargo check --workspace` pass after this slice.
- DNA payload commitment and decode-mode slice has landed: generic packet payload integrity now uses `DigestRole::PayloadCommitment`; `DnaValidationReport` emits per-section payload commitments and validates generic single-section packet integrity against the section payload commitment; `LocusDecodeMode` separates current-authority, migration, and forensic packet decoding; production authority-facing locus/CLI/cert decode paths use current-authority mode instead of implicit migration fallback. Remaining migration-compatible decoder calls are test inspection helpers plus the compatibility wrapper. `cargo fmt --check`, `cargo test -p l64-core`, `cargo test -p l64-locus`, `cargo test -p l64-cli`, `cargo test -p l64-cert`, and `cargo check --workspace` pass after this slice.
- Node 07A first distinction/transition-law slice has landed: `DistinctionClass`, `DistinctionLawSpec`, and `TransitionLawSpec` define the first structural-form-to-CNORM distinction membrane; CNORM receipt rule-table hashes now bind equivalence, transition, and distinction law together; tests prove format-local/projection distinctions may collapse while authority-bearing item position/kind/value distinctions may not. `cargo test -p l64-core` and `cargo check --workspace` pass after this slice.
- Node 08 first equivalence-coupling slice has landed: `EquivalenceLawSpec` now cites the transition law it consumes and lists preserved versus collapsed distinction classes; `validate_structural_equivalence_laws` rejects unknown transition laws, illegal collapses, and preserve/collapse contradictions; CNORM invokes that validation before canonicalizing. `cargo test -p l64-core` and `cargo check --workspace` pass after this slice.
- Canonical instruction first slice has landed: `CanonicalInstructionTag` and `CanonicalInstruction` define the first versioned instruction stream for canonical structure; CNORM now derives `canonical_bytes` from explicit header/item instructions instead of inline ad hoc item-byte appends; tests prove canonical bytes equal encoded instructions while formatting collapses and order remains authority-bearing. `cargo fmt --check`, `cargo test -p l64-core`, `cargo check --workspace`, and `git diff --check` pass after this slice.
- Duplex local-admission first slice has landed: `SemanticStrand`, `AuthorityComplement`, `PairLaw`, `DuplexPair`, and `DuplexPairValidation` define the first paired-authority unit; pair commitments use `DigestRole::AuthorityId`; validation distinguishes `Unpaired`, `Malformed`, and `LocallyValid`; tests reject missing pairs and strand-only promotion while accepting a valid semantic/complement/law pair. `cargo test -p l64-core` and `cargo check --workspace` pass after this slice.
- Domain closure first slice has landed: `DomainClosureReport`, `OpenObligation`, `BridgeBurden`, and `CyclePolicyResult` distinguish local pair validity from promotable closed-domain authority; `evaluate_domain_closure` blocks promotion on open obligations, external burdens, rejected cycles, missing pairs, or malformed pairs, and requires a promotion receipt when promotable. `cargo fmt --check`, `cargo test -p l64-core`, `cargo check --workspace`, and `git diff --check` pass after this slice.
- Deterministic merge first slice has landed: `CanonicalWorkUnit`, `DeterministicAuthorityMerge`, and `deterministic_authority_merge` define canonical coordinate ordering and receipt digest derivation that excludes worker count, lane timing, and input completion order; tests prove worker counts `1`, `2`, and `16` plus reversed input order produce identical ordered authority reports. `cargo test -p l64-core` and `cargo check --workspace` pass after this slice.
- Strategic bundle migration first slice has landed: `BundleSubstrateParityReport` now carries duplex pair counts, locally valid pair counts, a `DomainClosureReport`, and a `DeterministicAuthorityMerge`; bundle molecular envelopes lower into semantic/complement duplex pairs under `PAIR_BUNDLE_SUBSTRATE_V1`; parity tests prove dependency bonds produce locally valid pairs, closed domain evidence, and deterministic merge coordinates, while empty/non-promotable parity remains blocked from domain promotion. Low-memory verification used `CARGO_BUILD_JOBS=1`, `CARGO_INCREMENTAL=0`, and debug-stripped profiles after rustc OOM under normal memory pressure; `cargo test -p l64-core --lib`, `cargo test -p l64-bundle`, `cargo check -p l64-bundle`, and `cargo fmt --check` pass.
- Strategic bundle admission slice has landed: bundle-entry authoring text now rejects header-only empty bundles; bundle import now rejects non-closed native substrate parity before persisting `BundleWorld`; active `.dna` bundle import and CLI certification flows still pass. Low-memory verification used `CARGO_BUILD_JOBS=1`, `CARGO_INCREMENTAL=0`, and `CARGO_PROFILE_TEST_DEBUG=0`; `cargo test -p l64-bundle`, `cargo test -p l64-cli`, `cargo fmt --check`, and `git diff --check` pass after this slice.
- Verification burden-reduction slice has landed: `scripts/verify-low-memory.ps1` centralizes the single-job, no-incremental, debug-stripped cargo settings required when `l64-core` rebuilds hit rustc OOM under low available RAM. The script supports `core`, `bundle`, `cli`, `workspace`, and `all` scopes and runs formatting plus `git diff --check` by default, so future passes do not need to reconstruct the low-memory command sequence manually.

## Convergence Refinement Spine

The recent documents converge on the same mechanism through different vocabularies: role separation, constitutional authority, symbolic law, molecular substrate, packet hardening, campaign governance, and projection quarantine. They must be aligned by dependency, not by whichever surface vocabulary is most recent.

Use this spine whenever a future change appears to fit multiple paths:

1. **Role/admission membrane**
   - source RNA, canonical RNA, DNA authority, reports, projections, receipts, products, and release views must be typed before any path consumes them
   - no file extension, command habit, or readable marker may decide authority by itself
   - admission must eventually move from marker rejection into header/role law
2. **Constitution freeze**
   - freeze the cross-cutting authority doctrine before deeper reroutes
   - classify existing code as authority, scaffold, parity evidence, migration-only, deferred, or rejected
   - do not turn provisional codon, lexon, molecular, bundle, or product scaffolds into behavior-bearing law by momentum
3. **Symbolic law enforcement**
   - codons define structural class, phase legality, arity/admission, and opcode allocation
   - lexons are scoped symbolic bindings to canonical targets, not aliases or gloss
   - macro-codons govern recurring transition/product/receipt patterns
   - authored text is one-way ingress into symbols; compiled artifacts do not owe original prose reconstruction
4. **Distinction/transition law**
   - structural form is a recoverable representation, not authority
   - distinction extraction determines which differences are authority-bearing, projection-only, format-local, collapse-eligible, or collapse-forbidden
   - transition law must precede equivalence and canonicalization
5. **Equivalence and CNORM**
   - EQUIV may collapse only distinctions permitted by transition law
   - CNORM produces canonical structure under declared law, not canonicalized presentation, graph layout, or raw hash equality
   - fixed-point gates must prove `RNA -> DNA -> canonical RNA -> DNA` stability plus canonical identity preservation
6. **DNA and packet authority hardening**
   - distinguish canonical DNA, validation bundle DNA, report DNA, record DNA, and product/package DNA by role and admission
   - every persisted authority or cross-binary digest must bind actual payload content, not schema identity or local process hash policy alone
   - generic section packets are containers until their payload role and validation law make them authority-bearing
7. **Molecular substrate and witness-normal authority**
   - molecular records must consume symbolic law and codec envelope law before they govern downstream behavior
   - witnesses are derived from substrate state and receipts, never authored as truth
   - public identity should be witness-shaped where products or campaigns are exposed
8. **Bundle reroute as parity path**
   - existing bundle workflows lower into substrate primitives for behavior parity
   - bundle object families do not become ontology or substrate primitives by default
   - no runtime import/certification/promotion path may depend on bundle substrate parity until constitution, symbolic law, packet hardening, and transition law are sufficiently enforced
9. **Campaign, reuse, and release products**
   - campaigns certify governed routes over lineage and witnesses, not generic theorem authority
   - reuse requires canonical identity, lineage validity, replay permission, and satisfied invariants
   - `.pep`, `.prot`, `.ptome`, `.cell`, claim pages, closure maps, stress maps, replay records, and views are products/projections unless explicitly reconstructed under source or canonical RNA role
10. **Deletion/quarantine**
   - delete or quarantine only after the replacement authority path carries the beam
   - use git history for old forms; keep no compatibility shim unless an active test proves a current required transition still depends on it

Priority rule:

```text
role membrane
-> constitution
-> symbolic law
-> distinction/transition law
-> equivalence/CNORM
-> packet authority hardening
-> molecular/witness authority
-> bundle parity reroute
-> campaign/reuse/product expression
-> deletion/quarantine
```

If two paths conflict, prefer the one that eliminates an authority bypass or prevents a scaffold from becoming law. If a path only improves naming, presentation, or ergonomics, it waits unless the current name is causing coordination failure or public authority confusion.

## Approval-Gated Duplex Authority Mutation

This mutation supersedes the prior assumption that Node 11D0 can freeze the architecture from documentation alone. Node 11D remains blocked until the authority chain below has executable evidence. Existing Node 11B/11C molecular work is retained as provisional scaffolding and extraction material; it is not constitutional authority.

Authoritative temporal sequence:

```text
verified checkpoint
-> approval-gate ledger
-> distinction/transition law
-> equivalence law
-> canonical instruction encoding and identity
-> bounded, role-validated DNA admission
-> symbolic-law compilation
-> duplex pair authority
-> domain closure
-> deterministic parallel equivalence
-> strategic bundle/certification migration
-> bounded Boollet sidecar pilot
-> constitutional promotion
-> legacy deletion/quarantine
-> Node 11D behavior-bearing reroute
```

Execution rules:

1. Every architectural ruling begins as a candidate gate in `L64_APPROVAL_GATES.md`.
2. A candidate gate becomes proven only when linked to a compiler rejection, fixed-point, DNA validation, migration, deterministic-parallel, projection-loss, or external-adapter test.
3. Only proven gates may enter `LOCUS64_ARCHITECTURAL_CONSTITUTION_V1.md`.
4. Functional capability is migrated strategically; legacy serialization, cache layout, graph shape, naming, and cross-compatibility are not parity requirements.
5. The duplex pair is the smallest locally authoritative unit. Neither strand is independently promotable.
6. Pair-local validity is only an admission precondition. Promotion requires a closed domain or genome slice and a promotion receipt.
7. Codon symbols and lexon names are authoring/registry handles. Canonical structural instructions, scoped bindings, and admission law carry authority.
8. Graphs, grooves, fold maps, indexes, scheduler plans, and diagnostic views are derived representations and remain outside canonical identity.
9. Parallel execution is lawful only when worker counts `1`, `2`, and `N` produce byte-identical canonical DNA, canonical RNA, ordered diagnostics, receipts, and report digests. Timing and worker telemetry are non-authoritative.
10. Bundle JSON and remaining Q-shaped records are migration ingress or migration ASTs only. Migration is complete only when native DNA execution carries their lawful functional capability and the old authority path is removed.
11. Boollet may enter only as a transition-memory sidecar. It can remember validated transitions, denials, challenges, remediations, operator decay, and replay identity; it cannot decide Locus64 authority, canonical identity, DNA validity, or promotion.

Immediate implementation bands:

### Band A - Candidate Gate Ledger And Maturity Classification

1. Verify and checkpoint the current worktree.
2. Create the 50-gate candidate ledger with stable IDs, proof classes, dependencies, code targets, and status.
3. Classify current molecular, bundle, DNA, canonicalization, certification, report, cache, and projection mechanisms as authority, scaffold, parity evidence, migration-only, derived, deferred, or rejected.
4. Mark every constitution section candidate until its proof reference exists.

Exit gate:

- no architectural law can be frozen by prose alone
- every behavior-bearing Node 11D dependency has an explicit candidate gate and proof obligation

Current audit findings:

| Mechanism | Current state | Rail classification | Required next move |
|---|---|---|---|
| `L64_APPROVAL_GATES.md` | present with 50 candidate gates | enforcement artifact, not constitution | keep first in sequence; promote gates only with executable evidence |
| `CanonicalStructureItem` / token-hash structure | still uses compact structural values in the lower chain | scaffold | replace with explicit canonical instructions before final identity law |
| `stable_hash_u64` / `hash_serialized` | still used for receipts, graph hashes, witness IDs, and some packet-related fields | display/cache/scaffold depending on call site | classify by `DigestRole`; remove from authority identity |
| `bincode` payloads | still used for canonical payloads, molecular records, reports, and cached packets | transport/cache scaffold | introduce canonical encoding before expanding authority-bearing DNA sections |
| `decode_section_payload` | decodes packets and deserializes payload by opcode | authority bypass risk | split authority/cache/projection decode APIs and require validation for authority payloads |
| raw `authority_tier`, `feature_flags`, `strand_manifest` | present in DNA packet headers | untyped scaffold | replace or wrap with typed authority tier, feature admission, and strand-role law |
| automatic legacy decode fallback | still present in packet decoding path | migration ambiguity | require explicit current-authority, migration, or forensic decode mode |
| `MolecularCodecRecord` / `MolecularCodecEnvelope` | useful typed substrate serialization spine | provisional scaffold and parity evidence | rekey into duplex pair/complement law; do not constitutionalize record names |
| `SubstrateWitness` | derived flag exists, but IDs still use display-style hashes | useful scaffold | make witness identity witness-shaped and authority-digest backed |
| `QaDocument` / `QaEntry` | still central to bundle/certification migration flows | migration AST | preserve functional capability only until native pair/domain execution replaces it |
| bundle JSON body format | still admitted through `!l64-bundle v1` migration text | migration ingress | require migration receipt and deletion condition |
| namespacing by string rewrite | still broad in bundle migration code | bridge-law gap | add receipted namespace bridge before treating rewritten IDs as authority |
| `l64-cert` threaded obligation waves | already sorts parallel results by wave offset | useful parity pattern | generalize to canonical work-unit waves and byte-equivalence tests |
| report/research derivation | reports can still generate research/cached records | projection risk | require DNA digest, authority scope, projection loss, and replay for promotion |
| Boollet Python reference | external transition-memory kernel with passing Python tests and non-compiling Rust skeleton | integration candidate and sidecar scaffold | port only after fixture parity; use to reduce remediation/replay/operator burden, not to define Locus64 authority |

Audit conclusion:

```text
The workspace is aligned in direction but not yet aligned in enforcement.
The rail must treat current molecular/bundle/DNA/certification code as useful scaffolding and parity evidence, not as completed substrate law.
The next efficient path is not deletion, naming, or broad crate surgery; it is turning the current authority-shaped seams into typed, tested membranes in dependency order.
```

Current formalization frontier:

The project is now past broad doctrine and before full implementation closure. The remaining work to formalize is the set of contracts that prevent implementation momentum from turning scaffolds into law.

| Frontier | Must become | Minimal end state | Blocks |
|---|---|---|---|
| distinction/transition law | Rust enums/tables/fixtures for preserved, erased, local, projection, and forbidden distinctions | equivalence consumes explicit transition law, not shape, text, layout, or hash equality | CNORM, canonical identity, DNA fixed point |
| equivalence law | table-driven relation law for the first real RNA/DNA structure family | each collapse is lawful, named, tested positive and negative, and tied to preserved invariants | CNORM, duplex pairing, reuse |
| canonical identity | domain-separated digest roles and canonical structural instruction bytes | `AuthorityId`, `PayloadCommitment`, `ReceiptId`, `CacheKey`, and display IDs cannot be substituted for each other | DNA admission, witness identity, release products |
| DNA admission | typed artifact role, decode mode, scope, feature, tier, and bounded validation contracts | authority decoding is impossible before complete packet validation and payload commitment checks | duplex DNA, migration, source/projection membrane |
| duplex local authority | semantic strand + authority complement + pair law + pair commitment types | no strand-only promotion and no receipt-only complement can compile or validate as authority | domain closure, compiled symbolic law |
| domain closure | closed-domain report over pairs, dependencies, burdens, cycles, open obligations, and promotion eligibility | local pair validity never promotes without domain closure receipt | certification, reuse, research records |
| deterministic parallelism | canonical work-unit waves and ordered writer merge law | workers `1`, `2`, and `N` produce byte-identical authority outputs and diagnostics | scalable validation, Boollet sidecar evidence |
| migration membrane | native capability map plus deletion-bound migration receipts | bundle/Q/JSON capabilities either re-express through native authority or become deletion targets | legacy removal, Node 11D reroute |
| Boollet sidecar boundary | fixture-parity Rust port and Locus64 adapter contract | Boollet ranks/remembers transitions only; Locus64 remains sole authority for verdicts and promotion | remediation acceleration, not substrate closure |

Deduction rule:

If a current mechanism is unclear, classify it by the strongest authority effect it can currently cause:

```text
defines identity
> admits authority
> validates authority
> promotes authority
> migrates into authority
> records authority history
> projects authority
> caches authority
> displays authority
```

Formalize higher-effect mechanisms first. Lower-effect mechanisms are allowed to remain ugly if they are boxed out of authority flow by typed gates.

Recovered K2 equivalence standard:

The old Math Framework docs contain one standard worth porting directly into Node 07A/08:

```text
no naked equality
only declared equivalence under an active regime/transition law
equivalent objects collapse to one canonical representative only under that declared law
bridge/path comparability requires explicit equivalence transport
equivalence transport must carry preservation, loss, reversibility, receipts, and rollback
commuting proof objects are emitted when alternate paths agree under the target equivalence
```

Locus64 translation:

- `EqLaw` is not global equality.
- `EqLaw` is a scoped law object consumed by EQUIV/CNORM.
- `EqLaw` must name its active regime or transition context.
- `EqLaw` must state preserved invariants, erased distinctions, forbidden collapses, loss class, reversibility class, required receipts, and rollback law.
- `EqLaw` may be used for canonical collapse only after Node 07A has classified the relevant distinctions.
- `target_equivalence` survives as a domain/campaign field, but root substrate equivalence remains structural and scoped.
- bridge/campaign equivalence is a higher-layer transport obligation over canonical DNA, not a substitute for root canonical identity.
- if a path cannot state equivalence transport explicitly, it is not promotable.

Recovered K2 standards worth porting, with corrections:

| Old K2 standard | Locus64 use | Required correction |
| --- | --- | --- |
| Canonical truth stays small | Authority lives in canonical DNA, scoped law objects, and receipts only. | Do not let reports, projections, labels, caches, or convenience views define authority. |
| Detail survives by facet | Place each implementation-bearing detail in the one rail node or contract that owns it. | Do not duplicate the same rule across docs, CLI help, fixtures, and crate comments as competing mini-specs. |
| Repeated algebra compiles into machinery | Shared law families should drive admission, equivalence, transport, closure, promotion, replay, and rollback. | Do not implement one-off route/campaign exceptions when a reusable law object can carry the burden. |
| Honest frontier | Every blocker must name the missing law, receipt, witness, obligation, or authority boundary. | Do not allow generic "unsupported" or "adequacy missing" when a sharper deficiency applies. |
| Discernment/stability/repair profile | Domain closure may use `(AdmAxes, EqAxes, PromAxes, Budgets, Failures, Priority)` as a compact scoped law-pack shape. | This is a domain-closure profile, not a root substrate replacement for RNA/DNA authority. |
| Bridge contract tuple | Transport contracts need source, target, identity preservation, equivalence preservation, loss, reversibility, receipts, and rollback. | Bridge law sits above canonical DNA; it cannot define canonical identity by itself. |
| Proof-shape batteries | Square, triangle, diamond, pentagon, and hexagon checks are useful witness forms for commutation and route adequacy. | Proof shapes are validation receipts, not semantic authority on their own. |
| Surface transform receipts | Import/export/round-trip transforms require explicit receipts, unsupported-feature failures, and capability disclosure. | Resurrect the receipt discipline, not QC0/QM0/QK0/QA0 as public surfaces. |

Route and campaign selection must be lexicographic when multiple lawful paths exist:

1. lawfulness and admission
2. satisfiable adequacy or complement obligations
3. explicit equivalence transport
4. typed loss classes below the active ceiling
5. rollback and replay viability
6. sufficient proof-shape battery
7. minimal route complexity
8. maximum lawful reuse payoff

Recovered obligation families should map into duplex complements and domain closure, not into root parser behavior:

- `OBL_EQ`: equivalence transport
- `OBL_ADM`: admissibility and typing
- `OBL_LOC`: locality or structural-context correctness
- `OBL_GLU`: gluing/descent or composition law
- `OBL_TOL`: aggregation/toll law
- `OBL_RED`: reduction and normalization preservation
- `OBL_BRG`: bridge contract correctness
- `OBL_RBK`: rollback viability
- `OBL_ADE`: adequacy assumptions
- `OBL_FIN`: finiteness, boundedness, and normalization side conditions
- `OBL_OBS`: observable-interface specification
- `OBL_KNT`: obstruction handling

Recovered deficiency leaves should become precise diagnostics where applicable:

- `D_eq`: equivalence law underspecified
- `D_loss`: loss classes untyped or above ceiling
- `D_commute`: expected commutation/proof-shape receipt missing
- `D_roundtrip`: round-trip law untested or failed
- `D_bad_eq_transport`: declared transport does not preserve required invariants
- `D_rollback_cliff`: rollback law is absent or too lossy for promotion
- `D_no_commuting_proof`: alternate paths lack a required agreement witness

Recovered limits to preserve in documentation and release gates:

- finite executable fragments are not full domain semantics
- semantic comparison is not proof-artifact identity
- prediction is not theorem-level simulation
- recomputation plans are not schedulers until executed and receipted
- sidecar projections are not source unless admitted through the surface-role gate

Porting rule:

Old standards may be ported only when they reduce ambiguity in the RNA/DNA authority chain, duplex complement burden, domain closure, or release conformance. Anything that primarily revives old surface names, old document authority, or old compatibility expectations remains history.

Compatibility membrane for K2 standards, duplex authority, and Boollet:

These three threads are compatible only if each occupies a different authority layer.

| Thread | Allowed authority effect | Forbidden authority effect | Integration point |
| --- | --- | --- | --- |
| Recovered K2 equivalence/bridge standards | Define scoped law packs, obligations, route-selection order, proof-shape receipts, and precise deficiencies. | Define root identity, revive Q surfaces, or make bridge/campaign equality substitute for canonical structure. | Bands B-E, especially EQUIV/CNORM, duplex complements, and domain closure. |
| Duplex authority | Bind semantic strand and authority complement into the smallest locally valid authority unit. | Promote a strand alone, promote receipt-only evidence, or collapse domain closure into pair-local validity. | Band D, then Band E for closure and deterministic parallel validation. |
| Boollet | Remember transitions, denials, stale validators, operator decay, and remediation candidates as sidecar memory. | Decide canonical identity, DNA validity, domain closure, promotion, or constitutional status. | Band F1, after Bands B-E produce native transition events that Boollet can observe. |

Conflict-resolution rules:

1. If K2 bridge logic and duplex local law disagree, duplex/local admission blocks promotion until a domain closure law states the bridge burden explicitly.
2. If Boollet ranks a remediation that violates a K2 obligation or duplex complement, Locus64 rejects the proposal and Boollet records the denial as sidecar memory.
3. If Boollet replay identity and Locus64 canonical identity disagree, Locus64 canonical identity wins; Boollet may only flag replay drift.
4. If a K2 proof-shape receipt succeeds but DNA admission or canonical fixed-point validation fails, the proof shape is projection evidence only.
5. If a recovered K2 standard would require old document, report, JSON, or Q-surface authority, port only its receipt/law discipline and reject the old surface role.

Implementation consequence:

- K2 value enters as law, obligation, route, and diagnosis structure.
- Duplex enters as native authority structure.
- Boollet enters as transition-memory and remediation-ordering structure.
- None of the three may become a third public surface, a global semantic registry, or a shortcut around `RNA -> DNA -> canonical RNA -> DNA`.

### Band B - Lower Authority Repair

1. Complete Node 07A distinction and transition law over the first real RNA/DNA structure family.
2. Make Node 08 equivalence consume transition law instead of raw shape, text, graph layout, or hash equality.
3. Replace `CanonicalStructureItem` token-hash authority with explicit canonical instructions.
4. Define versioned canonical encoding, bounded lengths, deterministic ordering, normalized strings, and domain-separated BLAKE3 commitments.
5. Separate authority IDs, payload commitments, receipt IDs, cache keys, and display IDs.

Band B implementation order for digest and identity debt:

1. Add role-separated digest APIs in `l64-core`.
2. Migrate witness-shaped and codec-shaped digests first because they are already behavior-facing and easy to confuse with authority IDs.
3. Migrate DNA payload commitments and validation report IDs next because they gate admission.
4. Migrate canonical structure identity only after Node 07A/08 distinction and equivalence law can say what bytes are canonical.
5. Leave display IDs, temporary cache keys, and historical hashes in place until their authority effect is classified; do not churn cosmetic IDs before authority IDs are safe.

Exit gate:

- canonical identity derives only from explicit canonical structural bytes under declared distinction/equivalence law
- `bincode`, JSON, `stable_hash_u64`, formatting, and scheduler state cannot define authority identity

### Band C - DNA Admission And Reconstruction

1. Type artifact roles, authority scopes, decode modes, graph roles, feature admission, and mechanically earned authority tiers.
2. Bind every authority section to its payload, schema, encoding version, role, and bounded length.
3. Require complete packet validation before authority payload decoding.
4. Replace automatic legacy fallback with explicit migration or forensic decode modes.
5. Enforce packet, section, string, strand, and obligation limits before allocation.
6. Prove `RNA -> DNA -> canonical RNA -> DNA` identity and byte fixed points.

Exit gate:

- malformed, truncated, oversized, role-confused, legacy, and tampered packets fail deterministically before authority admission
- canonical reconstruction promises canonical RNA only, never original authored RNA

### Band D - Compiled Symbolic And Duplex Law

1. Compile codons into opcode, phase, arity, admission, structural operation, and required complement burdens.
2. Compile lexons into scoped bindings with authority source, valid phase, loss law, and binding receipt.
3. Compile macro-codons into reaction law with input/output roles, pair-state requirements, witness forms, failure modes, canonical receipt shape, and rollback behavior.
4. Reclassify current molecular nouns as provisional mappings onto substrate roles.
5. Implement semantic strands, constitutive authority complements, pair laws, and canonical pair commitments.
6. Reject strand-only promotion, receipt-ID-only complements, invalid pair classes, and authored witness authority.

Exit gate:

- a pair is locally authoritative only when semantic structure and constitutive admissibility law are bound under one canonical commitment
- no symbol, label, receipt coordinate, or molecular implementation name can become authority independently

### Band E - Domain Closure And Deterministic Parallelism

1. Compute dependency-safe canonical work waves from immutable pair/domain inputs.
2. Decode and validate independent semantic/complement lanes and independent pairs concurrently.
3. Merge worker results through one canonical ordered writer.
4. Aggregate diagnostics by stable domain, pair, gate, and error-code coordinates rather than completion order.
5. Evaluate dependency closure, external bridge burdens, cycle law, open obligations, and promotion eligibility after local pair validation.
6. Exclude worker plans, lane assignments, timings, and telemetry from authority.

Exit gate:

- serial and parallel execution are byte-equivalent across canonical DNA, canonical RNA, receipts, diagnostics, and report digests
- locally valid but globally open domains remain non-promotable

### Band F - Strategic Capability Migration

1. Inventory lawful bundle, policy, namespace, dependency, conflict, certification, adequacy, replay, and execution capabilities.
2. Re-express those capabilities through native roles, duplex pairs, domain closure, witnesses, and DNA execution.
3. Preserve lawful success and lawful rejection; add rejection for category errors exposed by native authority law.
4. Require migration receipts for bundle JSON and Q-shaped migration ASTs.
5. Prove cold-cache operation and delete disposable graph/document cache dependencies from authority paths.
6. Split authored obligation intent from evaluator-produced evidence, attach authority scope to verdicts, and make evaluator implementations explicit and named.

Exit gate:

- native authority carries the required functional capability without preserving legacy wire forms or path-dependent object boundaries
- reports and research records are projections unless replayed from admitted DNA authority

### Band F1 - Boollet Transition-Memory Sidecar Pilot

Boollet is a useful warm-memory organism only if it remains subordinate to Locus64 authority. It enters after native authority, domain closure, deterministic parallelism, and strategic migration seams are explicit enough that Boollet cannot accidentally become the decision engine.

Integration rule:

- Boollet does not move earlier than Bands B-E in the Locus64 dependency order.
- Boollet work may proceed early only as an external Rust parity port against Boollet fixtures, with no Locus64 authority imports and no Locus64 public command surface.
- Locus64 integration begins only when there is a native Locus64 transition for Boollet to observe: admitted DNA, phase/gate identity, validator/evaluator identity, result, diagnostics, and replay identity.
- Boollet can reduce burden by remembering failed or repeated transition attempts, ranking remediation candidates, detecting stale validators/toolchains, and surfacing operator decay. It cannot reduce burden by skipping a Locus64 gate proof.
- If Boollet output would affect canonical state, domain closure, promotion, DNA validity, or constitutional status, it must re-enter as a Locus64 proposal or witness coordinate and pass the normal Locus64 gate.

Planned-change smoothing points:

1. During Band F migration, Boollet records repeated migration failures and ranks the next remediation candidate without changing migration verdicts.
2. During deterministic parallelism work, Boollet may compare replay identities across worker-count runs as sidecar evidence, while Locus64 byte-equivalence tests remain the authority.
3. During certification rekeying, Boollet may detect stale evaluator fingerprints and scope drift, while Locus64 receipts define authority scope.
4. During release/source integrity work, Boollet may remember projection-as-source category errors, while Locus64 admission gates perform rejection.
5. During legacy deletion, Boollet may preserve deletion rationale and failure history as transition memory, while git history remains the archival source and Locus64 tests prove replacement.

Allowed role:

- remember Locus64 transition attempts, validation receipts, denials, challenges, remediations, stale-validator warnings, operator promotion candidates, operator decay, and replay identities
- rank future remediation or tactic candidates under explicit authority ceilings
- expose repeated failures, negative scopes, stale toolchain/validator fingerprints, and functional pressure
- provide fixture-backed Rust parity targets for future embedding

Forbidden role:

- decide canonical identity
- validate DNA packets
- promote duplex pairs or domains
- define codon/lexon/macro-codon law
- replace Locus64 certification, adequacy, or replay authority
- introduce a third public surface or another canonical artifact language

Step sequence:

1. Treat `C:\Users\Fresh\Projects\boollet` as an external reference until its Rust port compiles and passes fixture parity.
2. Add a Boollet integration packet to the rail only after Band F has a native Locus64 transition to observe.
3. Define the adapter boundary:
   - input from Locus64: DNA digest, canonical ID, phase/gate, transition attempt, validator/evaluator identity, result, diagnostics, residue, remediation candidate, replay identity
   - output to Locus64: ranked proposal, challenge memory, remediation suggestion, stale-evidence warning, operator candidate, or decay warning
4. Require every Boollet output to re-enter Locus64 as a proposal or witness coordinate, never as authority.
5. Port the Boollet Rust core against fixture hashes before embedding:
   - `BoolState`
   - `StateDelta`
   - `TransitionContract`
   - `ValidationReceipt`
   - `CommitRecord`
   - `ChallengeRecord`
   - `RemediationRecord`
   - `OperatorCandidate`
   - `OperatorDecayRecord`
   - `rail_digest`
   - `replay_identity`
6. Keep proof-search, patch/test, command execution, and Locus64 contact as adapters over the Boollet core.
7. Add one Locus64 remediation-loop fixture:
   - admission failure from admitted DNA authority
   - Boollet challenge/remediation memory
   - later Locus64 result
   - Boollet improves ranking without changing Locus64 verdict logic
8. Use Boollet to reduce migration burden only where it classifies repeated failures or suggests ordered remediation. Do not use it to skip required Locus64 gate proofs.

Exit gate:

- Boollet Rust core passes fixture parity against the Python reference
- one Locus64 sidecar fixture demonstrates improved remediation ordering while Locus64 remains the only authority
- Boollet artifacts are classified as sidecar memory, proposal, challenge, remediation, or projection; none are authority-bearing DNA
- all Boollet-derived suggestions require Locus64 validation before affecting canonical state

### Band G - Constitutional Promotion And Removal

1. Promote mechanically proven candidate gates into the constitution.
2. Keep failed or unproved gates candidate, deferred, or rejected with explicit residue.
3. Delete legacy paths after replacement capability and negative tests pass.
4. Reject generated caches, ambiguous JSON, reports, and projections from source releases.
5. Resume Node 11D only against the proven constitution and native authority chain.

Exit gate:

- the constitution is an index of executable law rather than architectural aspiration
- no legacy mechanism remains load-bearing merely because it existed before the native replacement

## Rail Node Template

Each implementation node should be shaped like this:

```text
Node:
  id:
  purpose:
  depends_on:
  code_targets:
  step_sequence:
  actions:
  invariants:
  tests:
  exit_condition:
  downstream_payoff:
```

If a node cannot be written in that form, it is still a concept, not executable rail material.

## Rust Development Optimization Rules

The rail must be executed in Rust-native increments, not architecture-sized rewrites.

Rules:

- Prefer type boundaries over prose boundaries: every important phase state should become a Rust type or enum before broad logic moves.
- Prefer newtypes for authority-sensitive IDs: `CanonicalId`, `DnaDigest`, `PhaseId`, `LedgerEntryId`, `LineageId`.
- Prefer table-driven law over scattered matches for token classes, opcodes, phase IDs, artifact classes, failure kinds, and exactness classes.
- Prefer `Result<T, E>` with domain errors over stringly `anyhow` at substrate boundaries; `anyhow` is acceptable at CLI edges.
- Keep substrate law physically modular. Do not continue growing `l64-core/src/lib.rs` as a new schema junk drawer; move codon, lexon, molecular substrate, and witness-normal logic into focused modules once the next Node 11B slice edits them.
- Treat naming as coordination hygiene, not execution. Rename only when the name itself creates authority confusion, public API drift, dependency cleanup risk, or repeated implementation mistakes.
- When a rename is necessary, prefer mechanical rename commits over mixed semantic commits.
- Prefer crate-local tests before workspace-wide tests during deletion or justified naming-hygiene passes.
- Prefer `cargo check -p <crate>` after local edits, then `cargo test -p <crate>`, then workspace tests only at phase exit.
- Do not refactor `release/src`, `target`, or generated release payloads as source of truth.
- Do not remove a crate and change dependent names in the same step unless the dependency graph proves it is isolated.
- Do not introduce macros/generators until the hand-written shape has repeated at least twice and the generator input is smaller than generated output.
- Do not add `unsafe` for parser/codec performance until conformance and fuzz tests exist around the safe version.

Adversarial audit checklist for every node:

- Which mechanism is still acting as semantic authority after this node?
- Does this node make structure more authoritative, or only delete/rename artifacts around the old authority path?
- Does this node confuse structure with one representation of structure?
- Could this node make later deletion harder?
- Could this node preserve an obsolete format out of inertia?
- Could this node hide semantic authority behind a debug field, label, or string table?
- Could this node create or preserve names that mislead implementation, public usage, or authority boundaries?
- Is any proposed rename doing real coordination work, or is it just cosmetic churn?
- Could this node create a second path around the phase engine?
- Could this node pass tests only because stale Q-surface samples still exercise the old architecture?
- Could this node edit generated release snapshots instead of source crates?
- Could this node broaden scope without reducing downstream ambiguity?

## 0. System Identity

Locus64 is a deterministic structural transformation substrate.

The system is designed for:

- deterministic transformation
- replayability
- machine-efficient execution
- human-editable symbolic interaction
- structural reuse and amortization
- canonical equivalence collapse
- semantically disciplined compilation

Core identity law:

```text
meaning := structure
```

Meaning does not come from labels, aliases, comments, strings, or codebooks.

## 1. Primary Ontology

Public surfaces:

```text
RNA := human interaction surface
DNA := machine authority surface
```

RNA is symbolic, editable, spliceable, and structurally recoverable.

DNA is compact, executable, reconstructive, and canonical.

Transformation law:

```text
RNA
 -> TOKENIZE
 -> RNORM
 -> SSR
 -> STRUCTURAL_FORM
 -> CNORM
 -> DNA
 -> EXEC
 -> LINEAGE
 -> REUSE
```

Each phase must reduce ambiguity, increase authority, preserve structure, and reduce downstream cost.

## 2. Core Axioms

Single semantic truth:

- all representations must collapse to one invariant structure
- syntax is disposable after structural recovery
- presentation never outranks canonical structure

Determinism:

```text
same input
-> same canonical structure
-> same canonical hash
-> same DNA
```

No semantic leakage:

- DNA must not require semantic names
- DNA must not require symbolic aliases
- DNA must not require human-readable operator labels
- semantic systems must emerge from structure, not annotate structure into authority

No silent mutation:

- all rewrites are explicit
- all rewrites are receipted
- all rewrites are reversible or rollback-addressable
- all rewrites are ledgered

Canonical authority:

```text
canonical_structure := identity
```

## 3. File Model

Public authoring and machine artifact extensions:

```text
.gene.rna
.locus.rna
.genome.rna

.dna
```

Extensions are hints. Headers are truth.

Artifact classes:

```text
GENE    atomic unit
LOCUS   grouped structure
GENOME  composite bundle
DNA     compiled substrate
```

Mandatory artifact header fields:

```text
type
encoding
version
canonical_id
grammar_id
integrity_hash
```

Optional header fields:

```text
dependencies
lineage
context
```

## 4. Global Identity Model

Identity:

```text
ID := blake3(canonical_binary_structure)
```

Identity rules:

- metadata-independent
- machine-stable
- replay-stable
- canonicalization-dependent

Collision policy:

```text
hash collision -> structural equality fallback
```

## 5. Token Algebra

Closed token set:

```text
ATOM
BINDER
OP
GROUP_L
GROUP_R
SEP
STRAND_REF
META
```

No dynamic token classes are allowed in the substrate.

Byte-level classification:

```text
[a-z0-9] -> ATOM
[A-Z]    -> BINDER

(        -> GROUP_L
)        -> GROUP_R

, ;      -> SEP
.        -> STRAND_REF

_ | :    -> OP

space    -> META
newline  -> META
```

Tokenization is single-pass classifiable and does not parse keywords.

Token structure:

```text
TOKEN := (
  class: u8,
  value: u32,
  flags: u8,
  arity_hint: u8
)
```

Token algebra:

```text
compose
group
bind
apply
reference
```

No hidden semantic operations are allowed.

Rust target:

- one owning `TokenClass` enum
- one byte classification table
- one diagnostic code family for invalid byte, boundary, grouping, arity, and scope failures
- conformance corpus for classification and normalization

## 6. RNA Surface

RNA is strict symbolic structural notation.

RNA properties:

- parseable at all times when valid
- splice-capable
- structurally reducible
- deterministic under normalization

RNA grammar:

```text
SEQ := TERM (SEP TERM)*

TERM :=
  ATOM
  BINDER
  OP TERM*
  GROUP

GROUP :=
  ( SEQ )
```

RNA states:

```text
RAW
SPLICE_BEARING
EXPRESSED
NORMALIZED
```

Splice payloads are transient shorthand fragments. They must be locally parsable, globally reducible, and deterministically expandable.

Example shorthand class:

```text
iei i>l1.l2|0|w eie
```

Splice payloads are never persisted after normalization.

## 7. RNORM

RNORM transforms symbolic RNA into explicit structural sequence.

RNORM responsibilities:

- expand shorthand
- resolve grouping
- resolve separators
- assign arity
- resolve binding structure
- remove ambiguity

Guarantee:

```text
RNORM(x) == RNORM(y)
iff structurally equivalent under the RNA grammar
```

Complexity:

```text
O(n)
```

RNORM must not perform:

- semantic inference
- canonicalization
- execution
- graph optimization

Rust target:

- `RnormInput(TokenStream)`
- `RnormOutput(NormalizedTokenStream)`
- explicit `RnormReceipt`
- deterministic diagnostics with source spans

## 8. SSR

SSR means Structural State Resolution.

SSR is a deterministic pushdown structural machine.

SSR is not:

- a recursive parser
- an AST builder
- a semantic interpreter
- a persistent graph authority layer

SSR state:

```text
SSR_STATE := (
  stack,
  frame_stack,
  graph_builder,
  scope_stack
)
```

Transition table:

| Token | Action |
| --- | --- |
| ATOM | emit leaf |
| BINDER | register binding |
| OP | push operator |
| GROUP_L | push frame |
| GROUP_R | close frame |
| SEP | finalize term |
| STRAND_REF | emit edge |
| META | ignore |

SSR constraints:

- single-pass
- no backtracking
- no semantic lookup
- no persistence
- no canonicalization
- no mutation of prior committed state

SSR output:

```text
STRUCTURAL_FORM
SSR_RECEIPT
```

Rust target:

- stack-bounded reducer
- ephemeral arenas or lifetimes that prevent SSR identity persistence
- source maps allowed for diagnostics only
- no SSR serialization path

## 9. Structural Form Transfer Representation

SSR emits a temporary structural form consumed by CNORM.

This structural form is a recoverable structural arrangement. It may be implemented as an arena graph, tree, DAG, term structure, opcode arena, compressed structural tape, or another efficient representation. The representation is not authority. The recovered structure is what later phases canonicalize and encode.

If the Rust code keeps a `KGraph` or `KernelGraph` name during migration, that name means "one internal representation of structural form," not public identity and not substrate authority.

Minimal structural item shape:

```text
ITEM := (
  opcode,
  relation_inputs[],
  aux,
  flags
)
```

Implementation options:

```text
arena graph
tree
DAG
term form
opcode arena
compressed structural tape
```

Representation legality is an implementation policy until the equivalence law declares a structural reason to require or forbid a shape. DAG-by-default, cycle handling, storage layout, and traversal strategy are not substrate law by themselves.

Structural form constraints:

- no semantic payloads
- contiguous storage preferred
- locality-oriented layout
- temporary representation only

Rust target:

- `KGraph` is a lower-chain transfer object, not public authority
- semantic labels must not appear in node identity
- persistence, if any, is debug-only and non-authoritative

## 10. Opcode System

Absolute rule:

```text
opcodes define structure only
```

Core opcode set:

```text
LEAF
BIND
APPLY
GROUP
SEQ
REF
CONST
PROJ
ANNOT
```

Structural modifiers:

```text
COMMUTE
ASSOC
INLINE
REDUCE
```

Forbidden semantic opcodes:

```text
CHAIN_RULE
DERIVATIVE
THEOREM
CERTIFIED_PROOF
```

Those concepts must emerge structurally above the substrate.

Rust target:

- one `Opcode` enum
- one arity table
- one compatibility table
- one decode/validation failure table
- reserved opcode ranges and version negotiation policy

## 10A. Discernment And Transition Law

Structural form is not yet canonical authority. It is recovered representation.

Before equivalence and CNORM may collapse anything, the system must account for the distinctions that matter under the active transition regime.

This layer defines:

- which distinctions are authority-bearing
- which distinctions are projection-only
- which distinctions are format-local
- which distinctions are collapse-eligible
- which distinctions are collapse-forbidden
- which transitions preserve identity
- which transitions change identity
- which invariants must survive a transition
- which failures halt before equivalence

Correct chain:

```text
structural form
-> distinction extraction
-> transition/invariant law
-> equivalence
-> CNORM
```

Equivalence is therefore not raw shape comparison. It is compiled sameness-discernment under declared transition law.

Rust target:

- `DistinctionClass`
- `DiscernmentCarrier`
- `TransitionLaw`
- `InvariantSet`
- `CollapsePolicy`
- transition-law fixtures consumed by equivalence tests

## 11. CNORM

CNORM collapses structurally equivalent forms into identical canonical structures.

CNORM does not canonize the graph layout, arena layout, parse tree, or any other representation. It canonizes the recovered structure itself.

CNORM is not defined until the structural equivalence law is defined.

The equivalence law must answer, at minimum:

- when ordering is significant
- when ordering is erased
- when grouping is significant
- when grouping is erased
- when associative flattening is lawful
- when commutative ordering is lawful
- when references remain distinct
- when redundant structure collapses
- what minimal invariant set determines identity
- what canonical ordering rule is used after equivalence is known

Canonicalization operations:

```text
normalize ordering
flatten associative groups
collapse redundant nesting
deduplicate equivalent nodes
stabilize identity
```

Canonical hash:

```text
HASH(node) :=
  hash(
    opcode,
    ordered(child_hashes),
    flags,
    aux
  )
```

Idempotence:

```text
CNORM(CNORM(x)) == CNORM(x)
```

Deduplication:

- hash-consing is permitted only after canonical ordering
- hash collisions fall back to structural equality

Rust target:

- `CanonicalStructure`
- canonical ordering law
- deterministic binary canonical form
- cross-platform hash-stability tests

## 12. DNA Substrate

DNA is canonical reconstructive machine encoding.

Global layout:

```text
HEADER
SECTION*
```

Header:

```text
magic
version
flags
section_count
root_id
node_count
integrity_hash
grammar_id
```

Node encoding:

```text
[OPCODE][ARITY][CHILDREN...]
```

Atom encoding:

```text
varint(u32)
```

Optional sections:

```text
STRING_TABLE
DEBUG_INFO
LOCAL_COMPRESSION
```

Optional sections must never affect meaning.

DNA guarantees:

- endian stable
- streaming decodable
- reconstructive
- deterministic
- semantically closed

Forbidden DNA content in required authority sections:

- symbolic aliases
- semantic labels
- proof prose
- operator names

Rust target:

- staged DNA validator
- compact structural sections
- local compression only
- proto-DNA and `.locus` are migration observations only, not authority paths

## 13. Codebook Policy

Final decision:

```text
structure = authority
codebooks = optional compression
```

Allowed:

- local deduplication
- symbol compression
- payload compression

Forbidden:

- semantic dependency
- mandatory codebook resolution
- meaning reconstruction from a codebook

## 14. Execution Model

Execution input:

```text
DNA
```

Execution output types:

```text
ExecutionWitness
NumericEvidence
CounterexampleCandidate
ReplayTrace
ResidualObligation
```

Output families must remain distinct.

Execution guarantees:

- exactness explicit
- approximation explicit
- deterministic replay when exact
- residual obligations explicit

Rust target:

- executor over DNA/canonical structure, not reconstructed semantic documents
- deterministic scheduling policy
- resource budgeting
- malicious DNA handling
- structural bomb detection

## 15. Ledger System

Ledger entry:

```text
ENTRY := (
  phase_id,
  input_hash,
  output_hash,
  invariants,
  failures,
  validation,
  rollback_ptr,
  metrics
)
```

Ledger laws:

- append-only
- immutable
- lineage preserving
- mandatory for promotion

Rust target:

- phase engine owns ledger writes
- exactly one ledger entry per successful phase transition
- failure states record last valid ledger entry

## 16. Reuse Law

Reuse conditions:

```text
canonical_id match
AND lineage valid
AND replay permitted
AND invariants satisfied
```

Forbidden reuse:

- blind cache hits
- approximate to exact substitution
- lineage-free reuse

Reuse outputs:

```text
ReuseReceipt
ResidualObligation
```

## 17. Failure Model

Failure types:

```text
StructuralError
ArityError
ScopeError
CanonicalizationError
EncodingError
ValidationError
ExecutionError
ReuseViolation
```

Failure state:

```text
FAILURE_STATE := (
  phase,
  reason,
  rollback_pointer,
  last_valid_ledger
)
```

Allowed responses:

```text
HALT
ROLLBACK
```

No partial propagation is allowed.

## 18. Phase Execution Kernel

Global rail:

```text
P0 TOKENIZE
P1 RNORM
P2 SSR
P3 CNORM
P4 DNA_ENCODE
P5 DNA_VALIDATE
P6 EXECUTE
P7 RECONNECT
P8 REUSE
```

Core execution loop:

```rust
for phase in EXECUTION_CHAIN {
    let output = phase.transform(input)?;

    assert!(phase.invariants(&output));
    assert!(output.failures.is_empty());

    ledger.commit(...);

    input = output;
}
```

Escalation law:

1. every phase reduces ambiguity
2. every phase increases authority
3. every phase reduces downstream cost
4. every phase produces reusable artifacts

## 19. Performance Model

Performance sources:

```text
byte-level tokenization
single-pass SSR
zero semantic lookup
canonical reuse
streaming decode
contiguous arenas
hash-consing
```

Complexity:

| Phase | Complexity |
| --- | --- |
| TOKENIZE | O(n) |
| RNORM | O(n) |
| SSR | O(n) |
| CNORM | O(n log n) worst |
| DNA | O(n) |
| EXEC | variable |
| REUSE | O(1) expected |

Forbidden performance killers:

- recursive parsing
- semantic lookup tables
- string-heavy hot paths
- runtime alias resolution
- mutable AST passes
- backtracking parsers

## 20. CLI Surface Model

Target commands:

```text
import
splice
fold
compile
validate
sequence
execute
trace
certify
reuse
```

RNA UX contract:

```text
Detected: gene.rna
Status: RNA contains splice payload fragments

Planned actions:
- splice RNA
- rewrite file in place
- create backup
```

Safety guarantees:

- atomic writes
- explicit confirmation
- rollback support
- backup before mutation

## 21. Current Codebase Alignment

The current codebase is not yet the final rail implementation.

Current codebase classification:

- RNA surface exists as partial success.
- RNORM is mostly real but still needs byte/token-grounded closure.
- SSR exists but must be forced into ephemeral lower-chain form.
- CNORM exists partially but must become canonical structure rather than graph snapshot persistence.
- DNA exists as proto-DNA and must be rebuilt into compact structural encoding.
- CLI exists but must be phase-engine enforced.
- Research, certification, tower, and coverage systems are valuable but must be reattached as derived overlays.
- QC0/QA0/QK0/QM0/JSON/document workflows are extraction sources and deletion targets, not compatibility commitments or final public authority.

Authority-flow finding:

- the bottleneck is not primarily Q, `.locus`, crate names, or documentation
- the bottleneck is that structure is not yet the sole authority
- the immediate substrate bottleneck is the missing structural equivalence law under CNORM
- current DNA is encoded graph persistence rather than encoded canonical structure
- current SSR preserves representation identities too easily
- current CNORM still depends on presentation-rich structures
- current tests still assert old representations more than structural invariants

Mechanism-first correction:

- deletion is delayed until the replacement authority path exists
- first establish `RNA -> RNORM -> SSR -> EQUIV -> CNORM -> DNA -> DNA_DECODE -> CNORM_RECONSTRUCTION`
- then move execution, lineage, and reuse onto that authority path
- then classify every remaining subsystem as feeding the authority path, consuming it, or irrelevant
- only irrelevant or authority-competing subsystems are deleted

Observed codebase facts that affect the rail:

- The active workspace has already largely moved to `l64` naming.
- Remaining stale names matter only when they affect public usage, authority boundaries, dependency cleanup, or deletion sequencing.
- Q-surface crates are no longer workspace members; `l64-qc0`, `l64-qa0`, and `l64-surfaces` have been deleted.
- `l64-qm0` has been deleted as an active crate. Its remaining presence is limited to the tombstoned `SurfaceKind::Qm0` enum discriminant required to keep existing bincode-encoded `.dna` artifacts stable until a stable numeric surface encoding replaces enum-index persistence.
- CLI/admin tests now keep only negative regressions for removed projection commands.
- Root bundle samples are now `.dna`; old `.qc0` sample bundles have been deleted.
- Release source snapshots under `release/src` may be stale and should not be treated as authoritative source during refactors.
- Naming is not execution; stale names are plan-altering only when they cause coordination failure or preserve obsolete architecture.

Plan-altering conclusion:

- The rail must not begin with deletion. It must first make canonical structure the sole authority path.
- The phase contract skeleton must exist before lower-chain work, because otherwise token/RNORM/SSR/CNORM/DNA changes will each invent local validation and failure semantics.
- Authority audit must classify mechanisms and authority flows before it classifies crate deletion targets.
- After replacement begins, extraction/deletion order must attack load-bearing obsolete authority beams before low-fanout leaves.
- Naming hygiene remains subordinate and is applied only where names hide authority, dependency, or public-use mistakes.
- End-to-end closure must include release packaging and documentation verification, because the rail was created to carry the project to a shippable endpoint.

Adversarial audit result:

- Highest rework risk: allowing misleading names to survive where they hide authority or dependency edges.
- Highest architecture risk: allowing a new non-DNA persistence path to replace the deleted Q-surface compatibility path.
- Highest mechanism risk: deleting legacy crates before the RNA/DNA authority path can replace their load-bearing behavior.
- Highest sequencing risk: pursuing leaf cleanup before replacing the next high-fanout non-DNA persistence beam.
- Highest Rust risk: moving logic between crates without first establishing typed phase/error/ID contracts in `core`/command substrate.
- Highest testing risk: workspace tests staying green while generated `.dna` artifacts still contain presentation-rich payloads that should become leaner structural encodings.
- Highest release risk: stale `release/src` snapshots or old docs being mistaken for current source truth.

## 22. First Compounding Change Chain

This change chain is now applied to the rail:

1. Rename the authoritative rail from `COMPOUNDING_CHANGE_CHAIN.md` to `LINEAR_EXECUTION_RAIL.md`.
2. Define compounding change chains as trajectory-preserving changes to the linear execution rail.
3. Consolidate the substrate ontology into one sequence: RNA, token algebra, RNORM, SSR, structural form, CNORM, DNA, EXEC, LINEAGE, REUSE.
4. Make DNA explicitly reconstructive structural encoding, not graph persistence.
5. Make every SSR structural-form representation explicitly ephemeral and temporary.
6. Add `CONST`, `PROJ`, and `ANNOT` as structural opcodes while keeping semantic opcodes forbidden.
7. Make BLAKE3 over canonical binary structure the identity target.
8. Add mandatory/optional file header requirements.
9. Add the phase execution kernel as the implementation anchor.
10. Preserve the old substrate inversion finding as current codebase alignment, not as the main rail text.
11. Apply the mechanism-first correction: build the replacement authority path before deleting legacy mechanisms.
12. Replace topology-as-authority wording with canonical-structure wording.
13. Insert structural equivalence law before CNORM, because canonical structure is not defined until equivalence is defined.
14. Strengthen the first authority-chain gate from roundtrip reconstruction to fixed-point stability.

## 23. Temporal Implementation Rail

This is the current path-optimized implementation sequence. Execute it in order unless a compounding change chain proves that a missing prerequisite must be inserted.

### Node 00 - Rail Rename And Reference Closure

Purpose:

- make `LINEAR_EXECUTION_RAIL.md` the only authoritative rail file
- make compounding change chains subordinate edits to this rail

Depends on:

- repository documentation baseline

Code targets:

- `README.md`
- `HANDOFF_STATUS.md`
- `EXPORT_STRESS_REMEDIATION_LEDGER.md`
- old `COMPOUNDING_CHANGE_CHAIN.md` path

Actions:

- remove the old rail file
- update references to the new filename
- state that change chains preserve rail trajectory

Invariants:

- no active documentation points to `COMPOUNDING_CHANGE_CHAIN.md` as the current rail
- no separate top-level rail competes with this file

Tests:

- `rg COMPOUNDING_CHANGE_CHAIN`
- `git status --short`

Exit condition:

- docs consistently name `LINEAR_EXECUTION_RAIL.md`

Downstream payoff:

- future planning edits target one stable document

Status:

- complete

### Node 01 - Authority Audit Baseline

Purpose:

- identify every path that can currently create, validate, import, export, or promote artifacts
- classify those paths by authority level before lower-chain changes begin
- establish the current workspace dependency map before naming-hygiene or deletion work begins

Depends on:

- Node 00

Code targets:

- current `l64-*` crates and binary names
- any stale aliases or historical names that still affect public usage or deletion
- root `Cargo.toml`
- `l64-cli`
- `l64-admin`
- `l64-command`
- `l64-locus`
- `l64-qc0`
- `l64-qa0`
- `l64-research`
- `l64-cert`
- `l64-runtime`

Step sequence:

1. Run `cargo metadata` or equivalent workspace inspection and record crate membership.
2. Build a dependency fanout map for Q-surface crates, CLI/admin binaries, surfaces, bundle, cert, and tests.
3. Classify each command/import/export/promote path by authority category.
4. Mark generated release snapshots and `target` output as non-source.
5. Add or plan the `authority-audit` command only after the static classification is clear.
6. Add residue searches for Q-surface authority leakage and misleading public names.
7. Run targeted checks for the crates touched by the audit.

Actions:

- add an authority classification enum or equivalent static table
- classify paths as `SubstrateAuthority`, `DerivedSemantic`, `ExtractionSource`, or `DeletionTarget`
- expose `l64 authority-audit` or equivalent admin command
- make the audit fail if QC0/QA0/QK0/QM0 tombstone paths, JSON, report text, or SSR identities claim substrate authority
- record crate dependency fanout so later naming-hygiene/deletion order is mechanical

Invariants:

- RNA and DNA are the only target public representation surfaces
- Q-surface paths are not compatibility commitments
- semantic persistence cannot silently become substrate authority
- release snapshots are not edited as authoritative source

Tests:

- `cargo metadata` or equivalent workspace inventory succeeds
- command smoke test for authority audit
- assertion that QC0/QA0/QK0/QM0/JSON/report paths are not classified as substrate authority
- regression test that DNA/lower receipts remain authority-capable

Exit condition:

- developers can see all authority-bearing paths before changing the lower chain

Downstream payoff:

- prevents substrate work from being invalidated by hidden upper-stack authority leakage

### Node 02 - Phase Contract Skeleton And Canonical Identity Foundation

Purpose:

- establish the Rust phase contract before individual phases invent incompatible validation/error/receipt shapes
- make canonical identity precise before token/opcode/DNA work depends on it

Depends on:

- Node 01

Code targets:

- `l64-core`
- `l64-canon`
- `l64-locus`
- `l64-command`

Step sequence:

1. Define closed enums or equivalent stable IDs for `PhaseId`, `ArtifactClass`, `FailureKind`, `ExactnessClass`, and `AuthorityClass`.
2. Define newtypes for `CanonicalId`, `DnaDigest`, `LineageId`, and `LedgerEntryId`.
3. Define minimal `PhaseInput`, `PhaseOutput`, `PhaseReceipt`, and `PhaseFailure` traits or structs.
4. Define canonical identity as BLAKE3 over canonical binary structure.
5. Add collision fallback semantics as structural equality, even if collision tests use an injected/fake hash.
6. Add tests around metadata independence and source formatting independence.
7. Only after these types exist, let later nodes add phase-specific payloads.

Actions:

- add minimal phase contract types without forcing every phase through the engine yet
- verify or add BLAKE3 support for canonical binary structure
- define `CanonicalId` as hash over canonical binary structure, not metadata
- add structural equality fallback for hash collision handling
- isolate debug labels and source text from canonical identity input

Invariants:

- metadata does not affect identity
- source formatting does not affect identity
- canonical binary structure is the only identity input
- later phase receipts share the same phase/failure/authority vocabulary

Tests:

- compile test for shared phase contract types
- same structure with different metadata yields same ID
- different structure yields different ID or falls through to structural comparison on forced collision test
- canonical ID stable across repeated runs

Exit condition:

- all later phases can depend on one identity law and one minimal phase contract vocabulary

Downstream payoff:

- tokenization, RNORM, SSR, CNORM, DNA, execution, reuse, and semantic rekeying do not each invent local contract shapes

### Node 03 - Token Algebra Freeze

Purpose:

- replace scattered/string-centric lexical behavior with one closed token substrate

Depends on:

- Node 02

Code targets:

- `l64-core`
- `l64-cli`
- `l64-locus`
- existing RNA normalization code

Step sequence:

1. Locate all token-like classification currently embedded in RNA, CLI, locus, and tests.
2. Create the closed `TokenClass` and byte classification table in the lowest appropriate crate.
3. Redirect one existing tokenizer path to the shared table.
4. Add fixtures for accepted bytes, rejected bytes, boundaries, whitespace, and newlines.
5. Remove duplicated local token rules only after the shared tests pass.
6. Run crate-local checks before wider workspace tests.

Actions:

- create or consolidate `TokenClass`
- implement one byte classification table
- define invalid byte, control character, whitespace, and newline handling
- define token boundary rules
- add token conformance fixtures

Invariants:

- no dynamic token classes
- no keyword parsing in the substrate tokenizer
- every accepted byte is classified deterministically
- every rejected byte has a deterministic diagnostic

Tests:

- token fixture corpus
- invalid byte tests
- repeated-run determinism tests

Exit condition:

- RNORM can consume token streams without re-tokenizing by private string rules

Downstream payoff:

- parsing, diagnostics, SSR, and RNA UX stop duplicating lexical logic

### Node 04 - RNA Grammar And RNORM Closure

Purpose:

- make normalized RNA an explicit structural sequence rather than a presentation cleanup result

Depends on:

- Node 03

Code targets:

- `l64-core`
- `l64-locus`
- `l64-cli`

Step sequence:

1. Introduce RNA state types or equivalent wrappers without changing behavior.
2. Make RNORM accept the shared `TokenStream`.
3. Move shorthand/splice expansion behind explicit RNORM functions.
4. Emit `RnormReceipt` using shared phase/failure vocabulary.
5. Add idempotence and failure fixtures.
6. Remove old string-only normalization paths after tests prove equivalent or intentionally stricter behavior.

Actions:

- define `RawRna`, `SpliceBearingRna`, `ExpressedRna`, and `NormalizedRna` or equivalent state types
- make RNORM consume `TokenStream`
- expand shorthand into explicit structural form
- remove splice payloads after normalization
- emit `RnormReceipt`
- add deterministic repair-locus diagnostics

Invariants:

- RNORM is O(n)
- RNORM does not canonicalize
- RNORM does not execute
- RNORM does not infer semantics
- normalized output has no unresolved splice fragments

Tests:

- shorthand expansion fixtures
- grouping/separator/arity failure fixtures
- idempotence test for already normalized RNA

Exit condition:

- SSR receives normalized structural tokens only

Downstream payoff:

- SSR can be a mechanical reducer instead of a parser plus repair engine

### Node 05 - Structural Opcode Law Freeze

Purpose:

- define the structural vocabulary before SSR emits graph nodes or DNA encodes them

Depends on:

- Node 04

Code targets:

- `l64-core`
- `l64-locus`
- `l64-runtime`

Step sequence:

1. Inventory existing opcode-like enums, packet tags, and structural node kind matches.
2. Define the shared `Opcode` enum and arity table.
3. Redirect SSR structural-form output first, DNA second, execution third.
4. Add forbidden semantic opcode tests.
5. Replace scattered local matches with table lookups where this reduces duplication.
6. Run targeted tests for each consumer before deleting old local definitions.

Actions:

- define one `Opcode` enum
- add `LEAF`, `BIND`, `APPLY`, `GROUP`, `SEQ`, `REF`, `CONST`, `PROJ`, and `ANNOT`
- add structural modifiers `COMMUTE`, `ASSOC`, `INLINE`, and `REDUCE`
- define arity and compatibility tables
- reserve forbidden semantic opcode names in tests to ensure they cannot enter the substrate

Invariants:

- opcodes describe structure only
- semantic operations emerge above the substrate
- arity validation is table-driven

Tests:

- opcode arity corpus
- decode/validation failure tests
- forbidden semantic opcode tests

Exit condition:

- structural form and DNA can share the same structural opcode law

Downstream payoff:

- removes duplicated structural meaning from representation, codec, and executor code

### Node 06 - SSR Ephemerality Refactor

Purpose:

- force SSR into a deterministic pushdown structural machine

Depends on:

- Node 05

Code targets:

- `l64-core`
- `l64-locus`
- any current structural resolution module

Step sequence:

1. Identify every persisted SSR-like ID, graph, receipt, or export path.
2. Add the bounded SSR reducer while leaving old path behind a testable comparison if needed.
3. Emit `SsrReceipt` without authority identity.
4. Switch CNORM input to the new structural-form path.
5. Delete or demote SSR persistence/export.
6. Add tests that fail if SSR becomes serializable authority again.

Actions:

- implement SSR as a reducer over normalized token streams
- introduce bounded `SSR_STATE`
- create `SsrReceipt` that records transition facts without preserving authority identity
- remove or demote any persistent SSR graph identity
- ensure source maps are diagnostic-only

Invariants:

- single pass
- no backtracking
- no semantic lookup
- no persistence
- no canonicalization
- no mutation of prior committed state

Tests:

- stack underflow/overflow tests
- reference legality tests
- no SSR serialization path test
- repeated-run deterministic structural-form output test

Exit condition:

- SSR produces structural form plus receipt, and only that structural form is consumable by CNORM

Downstream payoff:

- closes the largest current architecture inversion point

### Node 07 - Structural Form Transfer Boundary

Purpose:

- make the SSR output representation temporary, explicit, and non-authoritative

Depends on:

- Node 06

Code targets:

- `l64-core`
- `l64-canon`
- `l64-locus`

Step sequence:

1. Define the structural-form storage shape in the lower crate.
2. Add DAG-by-default validation and explicit cycle representation if needed.
3. Remove semantic payloads from node identity.
4. Make CNORM consume only the structural-form type.
5. Add representation property tests.
6. Remove legacy graph snapshot inputs from CNORM once callers move.

Actions:

- define explicit structural-form node/storage shape
- enforce DAG-by-default with explicit cycle representation if cycles are allowed
- prefer contiguous node storage
- remove semantic payloads from node identity
- add debug-only projection if needed

Invariants:

- structural form is not a public identity layer
- structural form does not persist as authority
- structural form contains structure, not semantic prose
- graph/tree/arena layout is not the canonical structure

Tests:

- representation property tests
- cycle legality tests
- semantic-label exclusion tests

Exit condition:

- CNORM has a single well-scoped structural-form input type

Downstream payoff:

- canonicalization stops depending on legacy graph persistence assumptions

### Node 07A - Distinction And Transition Law

Purpose:

- define the discernment/transition layer that tells equivalence which distinctions may be collapsed and which must survive
- prevent CNORM from treating raw structural shape, token text, graph layout, or hash equality as semantic sameness

Depends on:

- Node 07

Code targets:

- `l64-core`
- `l64-canon`
- `l64-testkit`

Step sequence:

1. Inventory where current lower-chain code treats normalization, child order, grouping, source text, or hash value as identity-relevant.
2. Define closed distinction classes:
   - authority-bearing
   - projection-only
   - format-local
   - structural
   - semantic
   - validation
   - collapse-eligible
   - collapse-forbidden
3. Define transition-law records:
   - consumed state
   - produced state
   - active regime
   - preserved invariant set
   - changed distinction set
   - lawful collapse policy
   - rejection condition
   - receipt emitted
4. Add first fixtures for:
   - order-preserving transition
   - order-erasing transition
   - grouping-preserving transition
   - grouping-erasing transition
   - projection-only distinction ignored by authority
   - authority-bearing distinction preserved
5. Make Node 08 equivalence consume distinction/transition law instead of raw structural form.
6. Mark any current equivalence fixture that lacks transition law as provisional.

Actions:

- introduce the minimal `DistinctionClass`, `DiscernmentCarrier`, `TransitionLaw`, `InvariantSet`, and `CollapsePolicy` types or equivalents
- add receipts proving which distinction classes were preserved or collapsed
- add negative diagnostics for attempted collapse of authority-bearing distinctions
- avoid vocabulary-heavy generality; implement only the first structure family needed by current RNA/DNA and CKI fixtures

Invariants:

- raw shape equality is not equivalence
- hash equality is not equivalence
- source text equality is not equivalence
- transition law decides what may collapse
- projection-only distinctions may not become authority-bearing accidentally
- authority-bearing distinctions may not be erased by formatting, sorting, grouping, or product rendering

Tests:

- distinction classification fixtures
- transition-law positive/negative fixtures
- projection-only distinction collapse fixture
- authority-bearing distinction preservation fixture
- attempted illegal collapse emits typed failure before CNORM

Exit condition:

- Node 08 equivalence consumes explicit distinction/transition law and no longer has to infer sameness from raw structure

Downstream payoff:

- CNORM becomes canonicalization of preserved discernment, not presentation-rich structure hashing

### Node 08 - Structural Equivalence Law

Purpose:

- define what structural equivalence means before CNORM claims to canonicalize anything

Depends on:

- Node 07A

Code targets:

- `l64-core`
- `l64-canon`
- `l64-testkit`

Step sequence:

1. Inventory every place the current code assumes equivalence through sorting, grouping, text normalization, or hash equality.
2. Consume Node 07A distinction/transition law as the source of permitted collapse.
3. Define the smallest explicit equivalence-law table for the first real structure family.
4. Mark each relation as ordered, unordered, associative, commutative, identity-preserving, or non-collapsible.
5. Define the minimal invariant set for canonical identity.
6. Define canonical ordering only after equivalence classes are known.
7. Add positive and negative fixtures proving what collapses and what remains distinct.
8. Refuse additional equivalence families until the smallest one passes conformance.

Actions:

- introduce `EquivalenceLaw` or equivalent table-driven law
- define ordering significance per opcode/relation
- define grouping significance per opcode/relation
- define reference identity preservation rules
- define which redundant structures can collapse
- define canonical ordering from the equivalence law, not from representation layout

Invariants:

- equivalence is explicit, not inferred from representation shape
- equivalence preserves Node 07A authority-bearing distinctions
- graph/tree/arena/tape layout does not define equivalence
- sorting is allowed only where the equivalence law permits it
- flattening is allowed only where the equivalence law permits it
- hash equality is not the definition of equivalence

Tests:

- equivalence-positive corpus
- equivalence-negative corpus
- ordering-significant fixture
- ordering-erased fixture
- grouping-significant fixture
- grouping-erased fixture
- reference-preservation fixture
- forced hash-collision structural equality fallback

Exit condition:

- CNORM has an explicit equivalence law to implement against

Downstream payoff:

- canonical structure becomes a mechanism rather than a named placeholder

### Node 09 - CNORM Canonical Structure

Purpose:

- make canonicalization produce canonical structure and stable identity

Depends on:

- Node 08

Code targets:

- `l64-canon`
- `l64-core`
- `l64-locus`

Step sequence:

1. Introduce `CanonicalStructure` beside existing canonical graph code.
2. Consume the Node 08 equivalence-law table.
3. Implement canonical ordering for the smallest real structure family first.
4. Add idempotence and equivalence fixtures.
5. Compute canonical binary form and canonical ID through Node 02 identity types.
6. Redirect DNA encoding to consume `CanonicalStructure`.
7. Remove presentation-dependent identity inputs.

Actions:

- introduce `CanonicalStructure`
- implement canonical ordering from the equivalence law
- flatten associative structures where declared
- order commutative structures where declared
- deduplicate only after canonical ordering
- compute canonical binary form and canonical ID

Invariants:

- `CNORM(CNORM(x)) == CNORM(x)`
- equivalent structures converge
- non-equivalent structures remain separated
- equivalence law determines collapse, not representation shape
- presentation data does not affect identity

Tests:

- equivalence corpus
- non-equivalence corpus
- idempotence tests
- hash stability tests

Exit condition:

- DNA encoder consumes canonical structure, not legacy graph snapshots

Downstream payoff:

- DNA, execution, ledger, and reuse become identity-stable

### Node 10 - DNA Structural Encoding

Purpose:

- replace proto-DNA graph packet behavior with reconstructive structural machine encoding

Depends on:

- Node 09

Code targets:

- `l64-locus`
- `l64-core`

Step sequence:

1. Define DNA header and section structs using fixed-size/varint primitives.
2. Encode a minimal canonical structure without optional strings.
3. Add optional debug/string sections only after required sections validate alone.
4. Add staged decode and validation helpers.
5. Keep proto-DNA import only long enough to migrate tests.
6. Add binary inspection tests proving required sections do not need semantic text.

Actions:

- implement required DNA header fields
- encode required authority sections as canonical structural data
- encode atoms as varints
- permit string/debug/local-compression sections only as optional non-authority sections
- add staged validator for header, section table, structural digest, and canonicality

Invariants:

- required DNA sections contain no semantic text needed for validation or execution
- DNA is endian-stable
- DNA is streaming-decodable
- DNA reconstructs canonical structure

Tests:

- encode/decode roundtrip
- semantic text absence test for required sections
- truncation/corruption tests
- optional section removal test

Exit condition:

- valid DNA can be validated without RNA, QC0, JSON, or report text

Downstream payoff:

- creates the actual machine authority surface

### Node 11 - Artifact Membrane And Fixed-Point Authority Gate

Purpose:

- close the source/authority/projection membrane failure before any more upper-stack work
- make `sequence-dna` mean canonical RNA sequencing, not inspection-report emission
- prevent reports, receipts, traces, JSON, or inspection output from being compiled as RNA source
- make fixed-point identity the first user-visible authority proof

Depends on:

- Node 10
- current reproduced membrane failure: `compile-rna RNA -> sequence-dna JSON -> compile-rna JSON` succeeds but produces different DNA

Code targets:

- `l64-core`
- `l64-locus`
- `l64-cli`
- `l64-cli/tests`
- `l64-locus` tests
- `scripts/torture-test.ps1`
- CKI fixture path `cki_registry.genome.rna`
- `LOCUS64_LANGUAGE_SPEC.md`

Step sequence:

1. Add role/surface distinctions at the lowest shared boundary:
   - `ArtifactRole`: `Source`, `Authority`, `Projection`, `Receipt`, `Trace`, `Failure`
   - `SurfaceKind`: `RNA`, `DNA`, `CanonicalRNA`, `InspectionReport`, `Receipt`
   - use existing `GenomeSurface` only where it still accurately describes packet header truth
2. Add a compile input gate:
   - `compile-rna` accepts only `role=Source` and `surface=RNA`
   - obvious report/projection/receipt/trace formats fail before tokenization
   - JSON object/array text is rejected as RNA source unless an explicit future import command owns conversion into RNA
3. Split sequencing from inspection:
   - `sequence-dna` emits canonical reconstructable RNA text only
   - `inspect-dna` emits JSON inspection/report output
   - current JSON artifact output moves from `sequence-dna` to `inspect-dna`
4. Add canonical RNA reconstruction:
   - DNA decode reconstructs canonical structure
   - canonical structure projects to canonical RNA deterministically
   - canonical RNA can recompile to the same canonical structure and DNA digest
5. Add `verify-roundtrip` or equivalent first-class command:
   - `RNA0 -> DNA0 -> CanonicalRNA1 -> DNA1`
   - require `CanonicalId0 == CanonicalId1`
   - require `CanonicalHash0 == CanonicalHash1`
   - require DNA digest equality or explicit canonical-packet equality if packet metadata is intentionally variable
6. Add fixed-point stabilization:
   - repeated `DNA -> sequence-dna -> compile-rna -> DNA` stabilizes
   - repeated `DNA -> inspect-dna` never feeds the compile path
7. Promote CKI as a regression fixture:
   - compile `cki_registry.genome.rna`
   - sequence to canonical RNA
   - recompile
   - verify fixed-point identity
   - keep CKI as a fixture, not as a third surface
8. Update torture test:
   - replace “sequence output exists” checks with fixed-point verification
   - include a negative test that compiling `inspect-dna` JSON output fails
9. Update docs:
   - `compile-rna`: source RNA only
   - `sequence-dna`: canonical RNA only
   - `inspect-dna`: reports only
   - `verify-roundtrip`: first correctness command for operators

Actions:

- add role/surface types or equivalent command-boundary classifiers
- route `compile-rna` through the source gate before lower-chain execution
- change `sequence-dna` output from JSON artifact serialization to canonical RNA text
- add `inspect-dna` for the current JSON artifact view
- add fixed-point verification helper in `l64-locus` and expose it through CLI
- wire CKI fixture into targeted tests
- remove JSON/report fixture assumptions from lower-chain golden tests

Invariants:

- source RNA, DNA authority, canonical RNA, inspection reports, receipts, traces, and failures are distinct roles
- projection output cannot silently become source input
- inspection output cannot silently become source input
- `sequence-dna` output is reconstructable canonical RNA, not a report
- fixed-point identity is relational, not merely local validation
- local validity never substitutes for roundtrip authority preservation
- JSON is not a native source, authority, or golden lower-chain fixture format

Tests:

- CLI test: `sequence-dna` stdout can be passed to `compile-rna` and produces the same canonical ID/hash as the original DNA
- CLI test: `inspect-dna` stdout passed to `compile-rna` fails with a role/surface diagnostic
- library test: `RNA -> DNA -> canonical RNA -> DNA` fixed point
- library test: repeated fixed-point cycle stabilizes
- CKI fixture fixed-point test
- torture test includes sequence/inspect membrane checks
- docs command examples match the new command roles

Exit condition:

- `sequence-dna` is no longer an inspection command
- `inspect-dna` owns report/JSON output
- `compile-rna` rejects projection/report/receipt/trace text
- CKI fixed-point test passes
- the reproduced membrane failure is impossible through the public CLI

Downstream payoff:

- execution, lineage, reuse, certification, and external proving slices can trust that source, authority, and projection roles do not collapse at the command boundary

### Node 11A - Pre-Codon Authority Readiness Audit

Purpose:

- prove Nodes 00-11 are actually complete enough to support codon/lexon law
- prevent Node 11B from compensating for unresolved lower-chain or membrane defects
- make the transition from RNA/DNA authority into molecular substrate work explicit

Depends on:

- Node 11

Code targets:

- `l64-core`
- `l64-locus`
- `l64-cli`
- `l64-bundle`
- `LINEAR_EXECUTION_RAIL.md`
- current targeted tests

Step sequence:

1. Verify the lower chain still exposes the required substrate:
   - token classes are closed
   - RNORM is deterministic
   - SSR remains ephemeral
   - structural equivalence law exists before CNORM
   - CNORM emits canonical structure
   - DNA encodes canonical structure
2. Verify the artifact membrane still holds:
   - `compile-rna` accepts source/canonical RNA only
   - `sequence-dna` emits canonical reconstructable RNA
   - `inspect-dna` emits inspection/report output only
   - `verify-roundtrip` proves fixed-point identity
3. Verify bundle and release transitional paths are correctly classified:
   - bundle v1 remains transitional authoring/ingress
   - generated release artifacts are rejected as source
   - JSON remains inspection/foreign/debug, not source or authority
4. Verify the next active implementation target is only Node 11B:
   - do not implement product files first
   - do not implement bundle reroute first
   - do not delete legacy paths first

Invariants:

- Node 11B may only add codon/lexon law; it must not patch lower-chain deficits indirectly
- if any lower-chain or membrane gate fails, return to the owning prior node before proceeding
- prior-node completion is verified by current tests or direct code inspection, not by historical confidence alone

Tests:

- run the smallest targeted tests that cover lower-chain fixed point and artifact membrane
- run `cargo test -p l64-core` after Node 11B law code lands
- run targeted CLI membrane tests if Node 11B touches admission behavior

Exit condition:

- the lower chain and artifact membrane are confirmed stable enough that Node 11B can proceed without backfilling prior-node defects

Downstream payoff:

- codon/lexon law work starts from a verified substrate instead of an assumed one

### Node 11B - Codon/Lexon Law Gate

Purpose:

- establish native symbolic law before molecular substrate implementation
- prevent generated products, bundle records, and future codecs from fossilizing word-shaped temporary names
- define codons as finite structural operators/classes with phase and arity laws, not semantic codebooks
- define lexons as scoped symbolic bindings, not global opcodes

Depends on:

- Node 11A
- current `!l64-bundle v1` header migration
- current genome release artifact spine
- CKI proving slice

Code targets:

- `l64-core`
  - `l64-core/src/codons.rs`
  - `l64-core/src/lexons.rs`
  - `l64-core/src/macro_codons.rs`
- `l64-locus`
- `l64-bundle`
- `l64-cli`
- `LOCUS64_LANGUAGE_SPEC.md`

Step sequence:

1. Define `CodonSpec`:
   - symbol
   - ASCII aliases
   - codon class
   - arity law
   - phase/admission law
   - opcode
   - canonical/tombstone status
2. Define `LexonSpec`:
   - symbol
   - scope
   - class
   - canonical target
   - aliases
   - gloss
   - binding/admission receipt
3. Define `MacroCodonSpec` separately from `LexonSpec`:
   - symbol
   - ASCII aliases
   - pattern class
   - reaction/transition shape
   - arity law
   - phase/admission law
   - witness requirements
   - opcode or opcode-range allocation
4. Produce the phase/admission matrix:
   - source RNA and canonical RNA admission
   - product/view/receipt rejection
   - memo/hash codon restrictions
   - foreign/debug artifact quarantine
5. Allocate opcode ranges and tombstone/version policy before substrate implementation:
   - never reuse an opcode
   - deprecate by tombstone
   - migrate by receipt
   - keep stable numeric allocation independent of Rust enum variant order
6. Add alias normalization rules:
   - aliases are ingress-only
   - generated native artifacts emit symbols, not structural words
   - gloss is human explanation only
7. Produce the first representative lexon corpus:
   - chain rule
   - derivative
   - composition
   - triadic bridge sector
   - spectral generation bridge
8. Produce the first representative macro-codon corpus:
   - roundtrip
   - closure frontier
   - view receipt
   - integration receipt
   - expression receipt
9. Produce generated-word ban list for native structural regions:
   - `claim_page`
   - `dependency_spine`
   - `closure_map`
   - `roundtrip_report`
   - `view_receipt`
   - `artifact_class`
   - `payload_json`
   - `metadata`
   - `theorem`
   - `campaign`
   - `adequacy`
10. Make the generated-word ban structural-region aware:
   - reject banned words in generated native headers, codon bodies, structural tokens, and role fields
   - allow banned words only in authored shorthand before normalization, explicit gloss/comment fields, or human Markdown/views
   - do not use a raw whole-file string grep as the final authority test
11. Add a codon header parser:
   - header truth outranks extension hints
   - source/product/receipt/view/foreign admission comes from codon phase law
   - no call site may invent an independent source/admission rule once the parser lands

Actions:

- add native Rust tables or generated tables for codons, lexons, and macro-codons
- add uniqueness checks for codon symbols, aliases, opcodes, and tombstones
- add compile/admission checks from the phase matrix
- add structural-region generated-output word-ban tests before any product/bundle rewrite
- split current `l64-core/src/lib.rs` scaffold into focused modules as part of the next code touch:
  - `codons.rs`
  - `lexons.rs`
  - `macro_codons.rs`
  - leave `lib.rs` as exports and integration glue

Invariants:

- codons are finite structural operators/classes with phase and arity laws
- codons are not semantic codebooks
- lexons are scoped symbolic bindings, not global opcodes
- macro-codons are recurring structural/reaction patterns, not scoped semantic-object bindings
- lexons and macro-codons must not share the same registry bucket
- meaning cannot be reconstructed from a global dictionary
- generated native structure emits symbols, not structural aliases as words
- structural words may appear in gloss/comments/views but not in generated native structural regions
- machine digest/memo codons cannot act as public proof identity
- products, views, receipts, and projections reject as source unless explicitly reconstructed into canonical RNA
- codon phase/admission law is the membrane; filenames, JSON shape, and command-specific heuristics are not authority

Tests:

- codon symbols are unique
- codon opcodes are unique
- aliases cannot collide across active codons/lexons unless an explicit scope rule permits it
- tombstoned opcodes cannot be reused
- phase/admission violations reject deterministically
- generated native structural regions do not contain banned structural words
- banned words remain allowed in explicit gloss/comment/view regions
- products/views/receipts reject as source
- hash/memo identity cannot be promoted as public witness identity
- codon header phase controls source admission independently of filename extension
- macro-codons and lexons occupy distinct registries and cannot collide silently

Exit condition:

- Node 11B has produced and tested:

```text
CodonSpec table
LexonSpec table
MacroCodonSpec table
alias normalization table
phase/admission matrix
opcode allocation and tombstone policy
structural-region generated-word ban list
first representative lexon corpus
first representative macro-codon corpus
codon header parser
```

Downstream payoff:

- future substrate and product work cannot smuggle semantic meaning through labels, aliases, JSON fields, or digest-only identity

### Node 11C - Molecular Substrate Codec Spine

Purpose:

- define the native substrate units that carry semantic structure before product expression or bundle deletion
- serialize substrate primitives through codec records without making codec records the ontology
- prevent a generic family/payload record from replacing JSON with a differently shaped object soup

Depends on:

- Node 11B
- full Node 11B exit gates, not merely the current first-slice scaffold

Code targets:

- `l64-core`
  - `l64-core/src/molecular.rs`
  - `l64-core/src/witness.rs`
- `l64-locus`
- `l64-bundle`
- `l64-cli/tests`

Step sequence:

1. Define substrate primitives:
   - `Locus`
   - `Atom`
   - `Bond`
   - `Reaction`
   - `Witness`
   - `Chassis`
   - `Cassette`
   - `Plasmid`
2. Define codec record envelope:
   - codon header
   - phase/admission role
   - origin DNA or canonical source
   - subject
   - substrate primitive kind
   - deterministic field order
3. Add hard codec law:
   - frames/records are serialization units, not ontology
   - no arbitrary native payload bag
   - unknown fields reject unless explicitly namespaced by extension law
4. Add witness derivation:
   - witnesses derive from substrate state, lineage, obligations, receipts, and closure status
   - witnesses are not accepted as authored truth
   - hashes remain machine memo bindings only
5. Add first certification-slice substrate serialization:
   - one valid seeded flow
   - one old-invalid flow
   - one old-allowed-but-now-category-wrong flow

Invariants:

- current substrate primitives remain provisional until audited against completed Node 11B law
- old bundle objects lower into substrate primitives; they do not become substrate primitives by default
- campaign, adequacy, theorem, policy, and report names are not native ontology unless structurally forced
- witness forms replace public digest identity
- codec records cannot validate solely by carrying a recognizable family name
- codec records serialize substrate primitives; codec records are not ontology
- no bundle import, certification, execution, or promotion path may depend on provisional substrate scaffolding before this node exits

Tests:

- substrate primitives serialize and deserialize deterministically
- generic payload escape hatch is absent
- witness-authored-as-truth rejects or downgrades to claim intent
- digest/memo identity used as proof identity rejects or quarantines
- first certification-slice witness derivation is stable

Exit condition:

- molecular substrate codec exists, has no generic payload bag, and emits derived witnesses from substrate state

Downstream payoff:

- bundle rerouting can target substrate mechanics instead of copying old object boundaries

### Node 11D0 - Approval-Gated Constitution Promotion

Purpose:

- promote only mechanically demonstrated symbol/substrate/codec/campaign/product laws before Node 11D becomes behavior-bearing
- prevent provisional codon, lexon, molecular, bundle, product, and campaign scaffolds from becoming authority by implementation momentum
- keep unproven architectural rulings in `L64_APPROVAL_GATES.md` rather than freezing them as constitution

Depends on:

- Node 11C
- `L64_APPROVAL_GATES.md`
- current audit findings
- pre-formulation consolidation material

Document targets:

- `LOCUS64_ARCHITECTURAL_CONSTITUTION_V1.md`
- `LINEAR_EXECUTION_RAIL.md`
- `LOCUS64_LANGUAGE_SPEC.md`
- `HANDOFF_STATUS.md`

Step sequence:

1. Verify `L64_APPROVAL_GATES.md` covers the active approval rulings and marks unproven rulings as `Candidate` or `Deferred`.
2. Create `LOCUS64_ARCHITECTURAL_CONSTITUTION_V1.md` only as a promoted-law index, not as an aspiration document.
3. Define the authority doctrine from proven gates:
   - source RNA
   - canonical DNA
   - canonical symbolic RNA
   - products
   - projections
   - receipts
   - traces
   - failures
   - foreign/import material
4. Define the genesis and promotion stack:
   - variation
   - retention
   - recurrence
   - discernment
   - canonicalization
   - objecthood
   - operatorization
   - promotion
   - law
   - constitution
5. Define native symbol language:
   - authored text as one-way capture material
   - symbolic output as durable native form
   - source retention as non-authority lineage
   - no obligation to reconstruct authored prose from DNA
6. Promote codon, lexon, macro-codon, and generated-word law only where compiler/category tests prove the ruling.
7. Promote Node 07A distinction/transition law only where fixed-point or illegal-collapse tests prove the ruling.
8. Define substrate role law before molecular noun law:
   - unit
   - relation
   - transformation
   - witness
   - container
   - integration
   - expression
   - scope
   - admission
9. Classify current molecular names as provisional mappings unless a later proof promotes one:
   - locus
   - atom
   - bond
   - reaction
   - chassis
   - cassette
   - plasmid
   - genome
10. Define codec spine law:
   - records serialize primitives
   - envelopes bind codon headers to records
   - payload digests bind content
   - no generic payload ontology
11. Define witness/public identity:
   - witnesses are derived
   - hashes are machine memo bindings
   - product identity is witness-shaped
12. Define bundle semantic reroute as a strategic migration/proving path, not ontology source.
13. Define campaign/certification model before expanding campaign runtime.
14. Define projection/inspection/foreign-output quarantine.
15. Define CKI/theory fixture law:
   - fixture role
   - dependency closure
   - parent-before-child constructive order
   - open obligations
   - projection exports as non-authority
16. Build a compliance matrix:
   - current code modules -> constitution sections
   - rail nodes -> constitution gates
   - tests -> laws proved
   - scaffolds -> maturity classification
17. Build a defer/reject ledger:
   - old K2 concepts ported as law
   - concepts deferred
   - concepts rejected
   - concepts quarantined as migration-only

Invariants:

- no runtime gate is promoted by documentation alone
- parity evidence is not certification
- bundle object boundaries do not define ontology
- products are not source
- projections are not source
- source capture text is not durable authority
- current code is classified by maturity, not described as final architecture
- old K2 surfaces are ported, deferred, or rejected explicitly
- no `Candidate` gate may appear in the constitution as proven law
- current molecular names are role mappings unless promoted by evidence

Tests / inspections:

- documentation scan proves QC0/QA0/QM0/QK0 are not recommended as public languages
- compliance matrix covers every active crate
- defer/reject ledger covers old surface concepts and unresolved K2 concepts
- constitution acceptance checklist maps to rail Definition of Done
- every promoted constitution rule links to a proven gate in `L64_APPROVAL_GATES.md`

Exit condition:

- `LOCUS64_ARCHITECTURAL_CONSTITUTION_V1.md` contains only proven gate law or clearly marked deferred residue
- Node 11D can resume against explicit executable law rather than scattered chat-derived intent or provisional scaffolding

Downstream payoff:

- future implementation slices can be tested against a constitution instead of reinterpreting the chat history or overpromoting scaffolds

### Node 11D - Bundle Semantic Reroute

Purpose:

- reroute existing `QaDocument`/bundle workflows through the molecular substrate without making the old bundle ontology sovereign
- prove behavior parity before deleting JSON-body bundle paths

Depends on:

- Node 11D0
- Node 11C
- completed witness-normal derivation from Node 11C

Code targets:

- `l64-bundle`
- `l64-cert`
- `l64-cli`
- `l64-admin`
- `l64-cli/tests`
- `l64-admin/tests`

Step sequence:

1. Inventory current bundle object families and map each to substrate primitives:
   - atom
   - bond
   - reaction
   - witness
   - receipt
   - policy
   - obligation
   - chassis relation
2. Add lowering adapter:
   - `QaDocument`/bundle DNA -> molecular substrate
   - substrate -> temporary old workflow structs only where existing cert/runtime code still requires them
3. Prove positive behavior parity:
   - dependency extraction
   - conflict behavior
   - namespace rewrites
   - certification verdicts
   - execution receipts
   - adequacy outcomes
4. Prove negative parity:
   - same old-invalid inputs remain invalid
   - old rejection/failure behavior remains explicit
   - old-allowed-but-now-category-wrong inputs are newly rejected
5. Move representative docs/fixtures to substrate-backed bundle DNA path.

Invariants:

- behavior parity includes success, rejection, and failure modes
- behavior parity includes same old valid behavior, same old invalid rejection, and new rejection of known category errors
- bundle object boundaries do not define final substrate primitive boundaries
- temporary old workflow structs are adapters, not authority
- parity probes are not runtime authority gates until this node exits

Tests:

- representative valid bundle flow produces same certification outcome through substrate path
- representative invalid bundle flow remains invalid
- report/product/witness-as-source category error rejects
- namespace/conflict/dependency behavior matches old path where old behavior was valid

Exit condition:

- representative bundle/certification workflows pass through molecular substrate with positive and negative behavior parity

Downstream payoff:

- JSON-body bundle paths can be quarantined from a position of replacement, not hope

### Node 11E - Expression Product Layer

Purpose:

- express release products from witnessed substrate/DNA authority after codon/lexon law and substrate codec exist
- evolve the current release spine in place rather than creating a second release generator

Depends on:

- Node 11D
- witnessed DNA authority from Node 11C/11D, not raw DNA or raw object fields

Code targets:

- `l64-locus`
- `l64-cli`
- `l64-cli/tests`
- `l64-observe`
- `l64-cert`
- `USAGE_GUIDE.md`
- `SEMANTIC_USAGE_GUIDE.md`
- `scripts/torture-test.ps1`

Format family:

```text
.rna    source sequence
.dna    canonical genome / machine authority
.pep    small expressed product
.prot   structured expressed product
.ptome  expressed product index
.cell   redistributable release organism directory/archive
```

Step sequence:

1. Re-key the current release spine before adding a broad product framework:
   - modify `export_genome_release_from_rna`
   - keep existing fixed-point, source/canonical RNA, view, and membrane tests
   - make `.pep`/`.prot`/`.ptome` output a direct evolution of current `.record`/`.projection` outputs
2. Add product record writers:
   - `.pep`: claim, view receipt, roundtrip receipt, build receipt, small stress point
   - `.prot`: dependency spine, closure map, closure frontier, stress map, replay record, build record
   - `.ptome`: complete product index mapping product path, kind, subject, witness form, digest/cache key, origin DNA
3. Add `.cell` directory/archive writer:
   - genome DNA
   - canonical RNA
   - product index
   - products
   - views
   - receipts
   - replay
4. Add `express` only as wrapper/rename over the proven release product path:
   - no divergent release generator
   - generated Markdown/DOT views remain projections and require `.pep` receipts
5. Move native `inspect-dna` toward product-record output only after product substrate exists:
   - JSON remains explicit foreign/debug mode only

Invariants:

- `.pep`, `.prot`, and `.ptome` are products, not sources
- `.cell` is a package, not an authority replacement
- every product declares role before payload
- every product carries origin DNA
- every public identity has a derived witness form
- every digest is subordinate to a witness form
- hash/digest identity is represented as a machine memo binding only, not public proof identity
- products express witnessed authority, not raw field exports
- product path and timestamps never affect canonical identity
- no generated product can be ingested by `compile-rna`

Tests:

- product parser rejects missing role/origin/schema fields
- product canonicalization erases comments and normalizes whitespace
- product digest is stable under path changes
- `.pep` products include witness form, closure status, dependencies, frontier, and receipt coordinate
- `.prot` products include witness forms and origin DNA
- `.ptome` maps every digest/cache key to a witness form
- `.cell` contains genome DNA, canonical RNA, ptome, products, views, and receipts
- generated views without `.pep` receipts fail release conformance
- `compile-rna` rejects `.pep`, `.prot`, `.ptome`, and `.cell` product text

Exit condition:

```text
RNA -> DNA -> canonical RNA -> fixed-point receipt -> express -> .cell
```

- `.cell` products use `.pep`, `.prot`, and `.ptome`
- products expose canonical witness forms rather than digest-only identity

Downstream payoff:

- release artifacts become witnessed products rather than paperwork exports

### Node 11F - Legacy Quarantine

Purpose:

- remove or quarantine transitional release/bundle paths after replacement authority is proven

Depends on:

- Node 11E

Code targets:

- `l64-bundle`
- `l64-cli`
- `l64-admin`
- `l64-cert`
- docs
- samples
- torture/conformance tests

Step sequence:

1. Quarantine `!l64-bundle v1` JSON-body input:
   - require explicit legacy flag or delete once substrate reroute carries representative workflows
   - normal `compile-bundle` rejects JSON-body bundle records after the replacement path carries the beam
2. Clip transitional release artifacts:
   - `release_manifest.record` -> `<subject>.ptome`
   - claim pages -> `.pep`
   - view receipts -> `.pep`
   - dependency spine -> `.prot`
   - closure map -> `.prot`
   - closure frontier -> `.prot`
   - stress map -> `.prot`
   - replay record -> `.prot`
3. Move JSON inspection to explicit foreign/debug mode:
   - default native inspection emits product records where applicable
   - JSON requires `--json` or `--foreign json`
4. Delete docs/samples that teach transitional outputs as public authority.

Invariants:

- legacy paths cannot silently become authority
- JSON is foreign/debug unless explicitly admitted by a legacy adapter
- compatibility imports are removed unless backed by a concrete active requirement and deletion condition

Tests:

- normal `compile-bundle` rejects quarantined JSON-body bundle input
- transitional `.record`/`.projection` release artifacts are absent from native release output
- JSON inspection requires explicit foreign/debug mode
- docs/examples use source RNA, DNA, substrate-backed bundle DNA, or expressed products only
- workspace, torture, and representative certification tests pass

Exit condition:

- transitional release artifacts and JSON-body bundle authoring are clipped without removing load-bearing behavior

Downstream payoff:

- old authority paths become history rather than living coordination cost

### Node 12 - Execution Over DNA

Purpose:

- move execution from graph/document workflows to structural traversal

Depends on:

- Node 11F

Code targets:

- `l64-runtime`
- `l64-cert`
- `l64-core`

Step sequence:

1. Add execution input type that requires validated DNA or canonical structure plus validation receipt.
2. Implement the smallest structural traversal path before optimizing.
3. Split exact witness, numeric evidence, counterexample candidate, replay trace, and residual obligation outputs.
4. Add resource budget and structural bomb checks.
5. Redirect certification to consume execution-native receipts.
6. Remove graph/document execution shortcuts after parity tests pass.

Actions:

- execute over DNA or canonical structure
- keep `ExecutionWitness`, `NumericEvidence`, `CounterexampleCandidate`, `ReplayTrace`, and `ResidualObligation` separate
- define exactness and approximation fields
- add replay trace keyed to DNA/canonical ID
- add resource budget and structural bomb checks

Invariants:

- exact and approximate artifacts do not collapse
- execution does not require reconstructed semantic documents
- replay is deterministic when exactness claims determinism

Tests:

- execution smoke tests
- exactness classification tests
- resource exhaustion tests
- replay determinism tests

Exit condition:

- certification can consume execution-native receipts

Downstream payoff:

- proof and adequacy work can become execution-native

### Node 13 - Phase Engine And Ledger Enforcement

Purpose:

- enforce the linear rail uniformly

Depends on:

- Node 12

Code targets:

- `l64-core`
- `l64-command`
- `l64-cli`
- `l64-admin`

Step sequence:

1. Upgrade Node 02 phase contract skeleton into an actual phase engine.
2. Route one low-risk command through the engine.
3. Add ledger commit and failure-state behavior.
4. Route compile/validate/execute paths through the engine.
5. Add replay/trace command support.
6. Remove direct phase bypasses after tests prove equivalent behavior.

Actions:

- upgrade the early phase contract skeleton into the global execution kernel
- make phase transitions emit exactly one ledger entry
- add failure state with rollback pointer and last valid ledger
- define transaction boundaries and interruption behavior
- expose trace/replay command paths

Invariants:

- no successful phase transition without ledger entry
- no failed phase emits downstream artifact
- ledger is append-only and lineage-preserving

Tests:

- successful phase ledger test
- failed phase halt test
- rollback pointer test
- replay test

Exit condition:

- CLI commands can be routed through one phase engine contract

Downstream payoff:

- later semantic systems inherit closure instead of reimplementing it

### Node 14 - CLI Surface Alignment

Purpose:

- make user-facing commands match the rail

Depends on:

- Node 13

Code targets:

- public command name `l64`
- crate prefix target `l64-*`
- current `l64-cli`
- current `l64-admin`
- current `l64-command`
- `README.md`
- `USAGE_GUIDE.md`
- `LOCUS64_LANGUAGE_SPEC.md`

Step sequence:

1. Keep the current command surface stable unless a name actively misleads usage or authority.
2. Move docs examples to `l64`.
3. Route `l64` commands through the phase engine.
4. Remove Q-surface command examples before Q crate deletion.
5. Run command smoke tests and stale wording scans.
6. Remove any obsolete alias in Node 15 only when it is confirmed to be active and misleading.

Actions:

- align commands around `import`, `splice`, `fold`, `compile`, `validate`, `sequence`, `execute`, `trace`, `certify`, and `reuse`
- choose `l64` as the public binary name; use uppercase `L64` only for prose/product identity where appropriate
- avoid command or crate renames unless the stale name creates a public or implementation problem
- remove, not merely label, obsolete Q-surface commands unless still required as an extraction source during the active migration node
- ensure extension is hint and header is truth
- implement RNA mutation UX with backup and explicit confirmation where applicable

Invariants:

- CLI trace equals phase trace
- user-facing docs do not imply QC0/QA0/QK0/QM0/JSON are public authority or supported ecosystem formats
- mutation is never silent

Tests:

- CLI smoke suite
- help text scan for stale authority wording
- residue scan for misleading public names and Q-surface authority wording
- RNA backup/rollback test where implemented

Exit condition:

- users can interact with Locus64 through the rail vocabulary

Downstream payoff:

- documentation and CLI stop fighting the architecture

### Node 15 - Naming Hygiene Gate

Purpose:

- prevent obsolete or misleading names from creating coordination failure without treating naming churn as execution

Depends on:

- Node 14

Code targets:

- `Cargo.toml`
- every crate `Cargo.toml`
- crate directories or imports whose names still contradict current public identity
- command docs and release scripts
- generated release source snapshots only after source-of-truth crates change

Step sequence:

1. Inventory remaining stale, misleading, or authority-confusing names in package names, directory names, binary names, import paths, docs, scripts, and tests.
2. Classify each name as `Harmless`, `HistoricalNote`, `CoordinationRisk`, `PublicApiRisk`, or `DeletionBlocker`.
3. Do nothing for `Harmless` and `HistoricalNote` names except keep them out of active public examples.
4. Rename only `CoordinationRisk`, `PublicApiRisk`, and `DeletionBlocker` names.
5. Freeze source-of-truth scope: exclude `target`, `release/src`, generated zips, and old release payloads from rename edits until final packaging.
6. For naming-only cleanup, rename lowest-fanout items first only when the rename is independent of authority replacement.
7. Do not use leaf-first naming or deletion work to postpone a higher-fanout obsolete authority replacement.
8. Rename shared crates or public APIs only when tests prove the old name still shapes usage or architecture.
9. Remove temporary aliases once tests pass through the current name.
10. Run targeted tests plus a help-text scan for stale public identity usage.

Actions:

- preserve behavior while changing required names
- prefer mechanical rename patches over semantic edits when a rename is justified
- keep one checkpoint per rename band if implementation is split
- do not rename working internals merely because names are old
- reserve name changes for cases where the name creates a concrete implementation, authority, API, or deletion problem

Invariants:

- no public command or document uses an obsolete name as the active project identity
- no obsolete name remains where it causes authority confusion, public API drift, or dependency cleanup risk
- any temporary alias has an explicit deletion condition
- Q-surface crates are not deleted in this node unless name cleanup exposes a trivial isolated removal
- generated release snapshots are regenerated after source changes rather than hand-edited as source truth

Tests:

- `cargo test`
- `cargo build -p l64` or final binary equivalent
- search for obsolete public identity names returns only historical notes or non-authoritative internals
- release script dry-run or smoke inspection

Exit condition:

- names no longer mislead implementation, public usage, authority boundaries, or deletion sequencing

Downstream payoff:

- later substrate and Q-surface deletion work is not blocked or distorted by misleading names

### Node 16A - Primary Obsolete Beam Replacement

Purpose:

- replace the highest-pressure obsolete authority beam with a stronger RNA/DNA or lineage-native path
- collapse QC0-style bundle/import/certification authority pressure before residual projection deletion
- prove representative commands, fixtures, and certification flows can run without QC0 parsing

Depends on:

- Node 15
- the Node 11 authority-chain fixed-point test must pass

Code targets:

- deleted historical crates `l64-qc0`, `l64-qa0`, `l64-surfaces`
- `l64-bundle`
- `l64-cli`
- `samples`
- future or existing RNA/DNA homes:
  - `l64-rna` or current lower-chain module home
  - `l64-dna` or current locus/DNA module home
  - `l64-command`
  - `l64-research`
- external proving fixtures only if they reduce primary-beam ambiguity:
  - `cki_registry` as a CKI typed-registry RNA/DNA pilot candidate

Step sequence:

1. Start from the passing authority-chain test, not from the Q crate list.
2. Classify every Q/surface/document mechanism by authority role: `LoadBearingAuthorityBeam`, `FeedsAuthorityPath`, `ConsumesAuthorityPath`, `BypassesAuthorityPath`, `ExtractionSource`, `LeafProjection`, or `Delete`.
3. Rank active obsolete beams by fanout across fixtures, commands, tests, samples, cert/admin flows, and upper-stack consumers.
4. Treat QC0-style bundle/import/certification behavior as the current primary beam unless a fresh audit proves another mechanism carries more authority pressure.
5. Build or extend the native replacement seam first: direct document import, `.dna` packet import, RNA/DNA fixture generation, lineage-native receipts, or canonical-structure-backed records.
6. Migrate one representative command/test pair through the replacement seam and verify that the old beam is no longer required for that path.
7. Expand migration across remaining tests, fixtures, samples, and commands that depend on the same beam.
8. Move required data structures and logic into the selected RNA/DNA/research home only when they feed or consume the authority path.
9. Track remaining `.qc0` and `.qa0` fixture pressure by command/test/sample owner, and separately track any `Qm0` tombstone references that exist only for binary compatibility.
10. If using an external proving slice, choose one small dependency-explicit registry and require it to enter as RNA/DNA, not as Markdown/DOT/JSON authority.
11. Run targeted beam-migration tests and workspace tests.

Actions:

- inventory all Q-surface crates by used type, parser, fixture, sample, command, and receipt
- maintain a beam-rank table during execution so work follows dependency pressure rather than deletion convenience
- continue replacing QC0-style bundle/import/certification flows with native `.dna`, RNA/DNA, or lineage-backed equivalents before spending cycles on lower-fanout projection deletion
- extract reusable logic into RNA/DNA or shared substrate modules only when still required
- redirect commands and tests from `.qc0` and `.qa0` files to `.rna` and `.dna` artifacts
- replace Q-surface samples with rail-native `.gene.rna`, `.locus.rna`, `.genome.rna`, and `.dna` samples
- use CKI-style typed registry data only as a bounded proof-of-value fixture for RNA ingestion, DNA authority, validation receipts, and derived projection export
- preserve no compatibility shim unless an active test proves a still-required internal transition cannot yet be completed
- document any temporary shim with an owner node and deletion condition

Invariants:

- replacement is mechanism-driven, not name-driven
- replacement is beam-first: remove the load-bearing obsolete authority path before optimizing around easy residual leaves
- leaf deletion cannot be used as evidence that the main obsolete authority beam is solved
- Q-surface artifacts cannot become substrate authority
- public doctrine remains RNA/DNA
- Q-surface support is not treated as a user-facing ecosystem because no real ecosystem exists yet
- extracted code must be smaller and better-rooted than the obsolete surface crate
- external proving slices cannot introduce a third public surface or bypass the RNA/DNA authority path
- derived Markdown/DOT/report outputs are projections, not authority

Tests:

- primary-beam migration tests prove representative bundle/import/certification commands pass through `.dna`, RNA/DNA, or lineage-native artifacts without QC0 parsing
- migrated fixture count is tracked until `.qc0` bundle fixtures no longer dominate coverage
- sample certification regression tests pass through RNA/DNA or lineage-native replacements
- external registry pilot tests, if added, prove parent/dependency closure before export projection generation
- promotion rejection test for lineage-free extracted/imported object
- `cargo test -p l64-bundle`
- `cargo test -p l64-cli`
- `cargo test --workspace`

Exit condition:

- the primary obsolete bundle/import/certification beam has native `.dna`, RNA/DNA, or lineage-backed command and test coverage, and remaining Q-surface work is residual rather than load-bearing

Downstream payoff:

- broad deletion can proceed without trimming a branch the workspace still depends on

### Node 16B - Residual Projection Extraction And Deletion

Purpose:

- delete mechanisms that bypass or compete with the RNA/DNA authority path after stronger replacement seams exist
- extract only required implementation value from obsolete Q-surface crates, redirect surviving behavior into RNA/DNA substrate modules, and delete the Q-surface crates instead of preserving fake compatibility

Depends on:

- Node 16A

Code targets:

- `l64-qc0`
- `l64-qa0`
- `l64-surfaces`
- `samples`
- `Cargo.toml`
- `Cargo.lock`
- docs and command examples that still expose Q-surface workflow

Step sequence:

1. Re-run the authority role classification after Node 16A.
2. Split `l64-surfaces` by actual mechanism ownership before deleting more code.
3. Delete `BypassesAuthorityPath`, `Delete`, and `LeafProjection` items after their parent beam is replaced, or earlier only when they are genuinely isolated.
4. Delete legacy projection utility commands that are not required for `.dna` bundle execution: `transcode`, `normalize-surface`, `roundtrip-check`, `surface-capabilities`, `dump-transform-receipt`, `parse`, `normalize`, `validate`, `import`, and `export`.
5. Delete remaining projection import/export paths after validation/report DNA replacements carry the workflow.
6. Remove `l64-surfaces` after report/export/cache duties and explicit bundle import duties are absorbed into rail-native modules. Status: complete.
7. Remove Q sample files and replace them with rail-native samples. Status: complete.
8. Remove Q-surface exports from user documentation. Status: complete for public docs.
9. Run full workspace tests and residue searches. Status: complete; remaining mentions are historical rail notes or negative regression tests.

#### Node 16B.1 - Poorly Mapped Territory Burden Chart

Purpose:

- make the remaining deletion path tactical rather than interpretive
- prevent `l64-surfaces` from remaining a mixed authority/projection/cache/document crate
- reduce future work by moving each still-useful mechanism to its natural owner exactly once

Current poorly mapped territory:

| Territory | Current home | Current consumers | Authority role | Target owner | High-efficiency action | Deletion gate |
|---|---|---|---|---|---|---|
| QC0/QA0 parser/render adapter | deleted | none | deleted historical projection | none | deleted after `.dna` bundle/document/report paths carried active tests | no source dependency on `l64-surfaces`, `l64-qc0`, or `l64-qa0` |
| Bundle projection ingestion | deleted | none | deleted authority bypass | `l64-bundle` native DNA/RNA ingest | `compile-bundle` accepts bundle-entry text without `--as`; bundle execution imports `.dna` only | bundle tests pass without `l64_surfaces::import_file` |
| Report-to-document derivation | `l64-cert` | `l64-cli`, `l64-admin`, validation bundle export | `ConsumesAuthorityPath` | complete for current certification/report owner | moved `report_to_document_with_registry`, `report_to_validation_bundle_with_registry`, `load_report_document_with_registry`, and related report document construction beside certification/report logic | validation bundle and report DNA tests pass without report helpers in `l64-surfaces` |
| Report packet cache paths | `l64-cert` | `l64-cli`, `l64-admin` | `ConsumesAuthorityPath` | complete for current report owner | moved `report_cache_root`, `report_cache_path`, `legacy_report_cache_path`, `report_id`, and `persist_report_document` to report/cert ownership | replay/export/import report tests pass without report cache functions in `l64-surfaces` |
| Execution manifest and bundle lock packet persistence | `l64-locus` | `l64-admin`, `l64-cli`, report derivation helpers | `ConsumesAuthorityPath` | complete for current packet-storage owner | moved `load_execution_manifest`, `persist_execution_manifest`, `load_bundle_lock`, `persist_bundle_lock`, `manifest_cache_root` to `l64-locus` | admin lock/replay and CLI report tests pass without manifest/lock helpers in `l64-surfaces` |
| Registry object projection lookup | deleted projection path | none | deleted leaf projection | none | deleted with `export-report-projection` and `export-artifact-projection` | negative CLI/admin regressions prove removed commands are rejected |
| Transform receipt store | deleted | projection import/export only | `Delete` | none | deleted because it only recorded obsolete projection transforms; import/export still returns non-persisted receipts | no active tests require `persist_receipt` or `load_receipt_store` |
| `SurfaceKind::Qm0` tombstone | `l64-core` | bincode-encoded existing `.dna` payload compatibility | `BinaryCompatibilityTombstone` | stable numeric encoding, then delete | do not remove until serialized enum-index dependency is replaced or all fixtures are regenerated under stable encoding | residue scan shows no Qm0 except documented tombstone, and bincode fixture stability no longer depends on enum variant position |

Burden-reduction order:

1. Move manifest/lock persistence first because it is high fanout and not intrinsically projection-related. Status: complete, now owned by `l64-locus`.
2. Move report cache and report-to-document derivation second because it lets `l64-surfaces` stop owning report authority-adjacent behavior. Status: complete in `l64-cert`.
3. Move or delete registry object projection lookup third because it is only needed by explicit projection exports.
4. Delete transform receipt store fourth unless lineage migration proves a non-projection use. Status: complete; no non-projection use existed.
5. Isolate `l64-surfaces` to QC0/QA0 adapter code only. Status: complete before deletion.
6. Replace `compile-bundle --as` and `certify --file --as` fixtures with `.dna` or RNA/DNA fixtures. Status: complete; those flags are removed.
7. Delete `l64-qc0`, `l64-qa0`, and the remaining `l64-surfaces` adapter after consumers are gone. Status: complete.
8. Remove Q-surface docs and residue notes except approved historical rail notes. Status: complete for public docs; rail retains historical notes only where they explain prior dependency decisions.

Per-slice verification:

- after each ownership move, run the smallest owning-crate test first
- after every public command removal, add a negative CLI/admin regression proving the old command is rejected
- after every dependency deletion, run `cargo check -p <former-consumer>` and `cargo test -p <former-consumer>`
- after every adapter deletion, run `rg "l64_surfaces|l64-qc0|l64-qa0|\\.qc0|\\.qa0"` and `cargo test --workspace`
- after every report/cache move, run report export/import, lock/replay, validation bundle, and torture compact slice

High-risk traps:

- moving report helpers into `l64-bundle` would couple certification reporting to bundle import; prefer `l64-cert` or a small report artifact module
- deleting `l64-surfaces` before manifest/lock/report helpers move would remove non-projection cache behavior by accident
- deleting QC0/QA0 before `compile-bundle --as` is replaced would break the remaining migration seam
- treating transform receipts as lineage receipts without rekeying them to DNA authority would preserve projection authority under a new name
- deleting the `Qm0` enum tombstone before stable surface encoding lands can silently break existing bincode `.dna` payloads

Exit condition for 16B.1:

- `l64-surfaces` is deleted
- all non-projection report/cache/manifest/lock helpers live with their owning authority modules
- the Q-surface deletion list is empty

Actions:

- remove Q-surface crates from the workspace once consumers are redirected
- remove Q-surface exports from user documentation
- delete parser/render code that no longer feeds or consumes the authority path
- aggressively delete projection leaf commands once their only remaining purpose is compatibility demonstration
- preserve no compatibility shim unless an active test proves a still-required internal transition cannot yet be completed
- document any temporary shim with an owner node and deletion condition

Invariants:

- deletion is mechanism-driven, not name-driven
- no deletion is allowed to remove the only implementation of a still-required authority-chain function
- Q-surface artifacts cannot become substrate authority
- public doctrine remains RNA/DNA
- Q-surface support is not treated as a user-facing ecosystem because no real ecosystem exists yet
- no Q-surface crate remains in the workspace after this node exits

Tests:

- `rg "l64-qc0|l64-qa0|\\.qc0|\\.qa0"` returns only allowed historical notes, if any
- `rg "l64-qm0|\\.qm0|Qm0"` returns only allowed historical notes and the documented enum tombstone, if still required for bincode stability
- workspace membership no longer includes Q-surface crates
- sample certification regression tests pass through RNA/DNA or lineage-native replacements
- build and test after each removed crate or command group
- `cargo test --workspace`

Exit condition:

- Q-surface crates are gone, useful logic has been absorbed into RNA/DNA substrate homes, and user-facing workflow is RNA/DNA only

Downstream payoff:

- reduces code hoarding, removes false compatibility pressure, and prevents obsolete surface names from shaping future architecture

### Node 17 - Semantic Rekeying And Upper Reattachment

Purpose:

- attach research, certification, adequacy, tower, and coverage to canonical lineage

Depends on:

- Node 16B

Code targets:

- `l64-research`
- `l64-cert`
- `l64-selector`
- `l64-atlas`
- `l64-registry`
- `l64-observe`

Step sequence:

1. Inventory semantic records that currently derive from reports, Q bundles, or JSON.
2. Add canonical ID and DNA digest fields where missing.
3. Require lineage for claim, route, adequacy, bridge, operator, proof-shape, and coverage records.
4. Convert former Q semantic payloads into RNA/DNA-backed lineage records or native Rust records.
5. Add typed-registry lineage rules for dependency-explicit external structures: node declarations, typed edges, proof dependencies, open obligations, and projection exports.
6. Add rejection tests for lineage-free semantic promotion.
7. Rerun seeded campaigns and imported-claim equivalents through lineage-native paths.

Actions:

- rekey semantic records to canonical ID and DNA digest
- require lineage for claim, route, adequacy, bridge, operator, proof-shape, and coverage records
- make generator/tower growth blocker-driven and lineage-grounded
- preserve challenge/remediation records as derived overlays
- ensure any useful semantic payload formerly expressed in QC0 exists as RNA/DNA-backed lineage objects or native Rust records, not as Q-surface text
- treat registry Markdown/DOT/report exports as derived projections that must be regenerated from DNA-backed lineage

Invariants:

- no floating report semantics
- no theorem/campaign object becomes authority without DNA lineage
- upper systems can be replayed or challenged
- no upper system depends on Q-surface crates
- dependency order, parent closure, endpoint closure, and open-obligation status are validation requirements for registry-like semantic structures

Tests:

- seeded campaign regression
- imported claim regression
- lineage-required rejection tests
- coverage reuse tests
- Q-surface dependency absence test
- registry-like fixture test proving parent-before-child constructive order and zero undeclared endpoints before projection export

Exit condition:

- upper stack survives as derived capability above the substrate

Downstream payoff:

- the extensive current semantic system becomes an asset rather than substrate debt

### Node 17A - Genome Release Artifact Spine

Purpose:

- produce redistributable review artifacts from canonical DNA and lineage without creating another authority surface
- make every claim, view, open edge, and replay step coordinate-addressable
- force status downgrade and open-obligation visibility before public release packaging

Depends on:

- Node 17
- Node 11 surface-role admission gate
- Node 11 DNA-to-canonical-RNA reconstruction gate
- Node 11 fixed-point membrane gate:
  - `compile-rna` rejects reports, views, receipts, traces, closure maps, claim pages, stress maps, replay records, and inspection output
  - `sequence-dna` emits canonical reconstructable RNA
  - `inspect-dna` emits non-authority inspection output
  - `verify-roundtrip` proves `RNA -> DNA -> canonical RNA -> DNA` fixed point

Code targets:

- `l64-research`
- `l64-cert`
- `l64-observe`
- `l64-cli`
- `l64-locus`
- `LOCUS64_LANGUAGE_SPEC.md`
- `USAGE_GUIDE.md`
- CKI fixture outputs

Step sequence:

1. Define release artifact roles as derived outputs:
   - `CanonicalGenome`
   - `SourceSequence`
   - `DependencySpine`
   - `ClosureMap`
   - `ClaimPage`
   - `ClosureFrontier`
   - `StressMap`
   - `Lineage`
   - `ReplayRecord`
   - `View`
   - `ViewReceipt`
2. Add status propagation rules:
   - closed claim with conditional parent becomes conditional
   - closed claim with bridge-dependent parent becomes bridge-qualified
   - claim with open parent remains open-dependent
   - observable without benchmark/test attachment remains hypothesis-class
   - generated view remains projection-only
3. Add claim-page schema:
   - claim ID
   - claim class
   - status
   - assumptions
   - proof or derivation route
   - dependencies
   - open dependencies
   - stress points
   - authority coordinates: DNA hash, canonical ID, source node IDs, receipt IDs
   - generated view links
4. Add dependency-spine and closure-map extraction from lineage records.
5. Add closure-frontier extraction from residual obligations and open dependencies.
6. Add stress-map extraction from known failure classes:
   - undefined term
   - hidden assumption
   - invalid dependency
   - overstated status
   - circular construction
   - missing proof
   - projection mistaken for source
   - non-reproducible computation
   - empirical mismatch
   - ambiguous symbol reuse
7. Add replay record generation:
   - commands
   - input/output hashes
   - protocol versions
   - environment details where deterministic replay depends on them
   - regenerated projection hashes
8. Add view receipts for every Markdown, DOT, paper skeleton, diagram, or reviewer-facing output.
9. Build the first CKI genome release fixture:
   - `genome/`
   - `spine/`
   - `claims/`
   - `frontier/`
   - `replay/`
   - `views/`
10. Add CLI generation command only after the data contracts are testable.

Actions:

- introduce release artifact structs or records in the smallest owner crate
- generate CKI claim pages from RNA/DNA lineage rather than from Markdown or report text
- generate dependency spine and closure map from canonical lineage
- generate frontier and stress map from typed residual/open records
- generate replay record from fixed-point and validation receipts
- generate view receipts for derived views
- keep all generated human-facing views projection-only

Invariants:

- every claim has coordinates
- every view traces to a genome
- every open edge remains visible
- every release can be replayed
- every change has lineage
- closure status propagates through dependencies
- generated views cannot become source or authority
- no release artifact is accepted as source unless its role is `Source` and its surface is `RNA` or `CanonicalRNA`
- claim pages, closure maps, stress maps, replay records, lineages, receipts, and views remain projection/record artifacts
- known limits are represented as frontier/stress entries, not hidden prose

Tests:

- CKI release fixture contains canonical genome and source sequence
- every claim page has DNA hash, canonical ID, dependencies, status, and open-dependency fields
- status propagation downgrades claims with conditional, bridge-dependent, or open parents
- generated views have view receipts and are projection-only
- replay record reproduces the fixed-point command path
- closure frontier is non-empty when open obligations exist
- stress map routes common criticism classes to exact coordinates or open records
- `inspect-dna` output passed to `compile-rna` is rejected
- generated view, claim page, closure map, stress map, replay record, and receipt outputs passed to `compile-rna` are rejected
- `sequence-dna` output passed to `compile-rna` preserves canonical hash and DNA fixed point

Exit condition:

- CKI can emit a complete genome release fixture whose review artifacts are all derived from canonical DNA, lineage, receipts, and fixed-point replay

Downstream payoff:

- conformance and release packaging can verify public-facing artifacts by structure rather than relying on prose claims

### Node 18 - Conformance, Torture, And Release Gates

Purpose:

- turn the rail into a shipping standard

Depends on:

- Node 17

Code targets:

- `l64-testkit`
- `scripts`
- release docs
- CI or local release commands

Step sequence:

1. Build a conformance corpus for token, RNORM, SSR, CNORM, DNA, execution, lineage, and reuse.
2. Add randomized or fuzz-style stress where practical.
3. Update torture tests to exercise the full rail through the current public command surface.
4. Add release-artifact conformance for claim pages, dependency spine, closure map, frontier, stress map, replay record, and view receipts.
5. Add membrane conformance for derived release artifacts:
   - `inspect-dna` output -> `compile-rna` must reject
   - generated view -> `compile-rna` must reject
   - claim page -> `compile-rna` must reject
   - closure map -> `compile-rna` must reject
   - stress map -> `compile-rna` must reject
   - replay record -> `compile-rna` must reject
   - receipt -> `compile-rna` must reject
   - `sequence-dna` output -> `compile-rna` must preserve canonical hash
6. Add residue scans for misleading names, Q surfaces, proto-DNA claims, and stale docs.
7. Add release smoke tests for Windows, Linux, compact, perfopt, and source packages.
8. Make failed conformance block release generation.

Actions:

- create conformance corpus for token/RNORM/SSR/CNORM/DNA/execution/reuse
- create conformance corpus for claim pages, dependency spine, closure map, frontier, stress map, replay record, and view receipts
- add fuzzing or randomized stress where practical
- update torture test to exercise the full rail
- add cross-platform determinism checks where available
- make release docs state current protocol version and compatibility status

Invariants:

- release cannot pass with stale docs
- release cannot pass with proto-DNA represented as final DNA
- release cannot pass with uncoordinateable claims, unreceipted views, hidden open edges, or missing replay records
- release cannot pass if any view, receipt, closure map, claim page, stress map, replay record, or inspection report can be ingested as RNA source without an explicit source/reconstruction role
- conformance failures block release

Tests:

- `cargo test`
- rail conformance suite
- genome release artifact conformance suite
- release artifact membrane rejection suite
- torture test
- release package smoke tests

Exit condition:

- project is shippable against this rail

Downstream payoff:

- future work has a regression net and clear resume point

### Node 19 - End-To-End Delivery Closure

Purpose:

- ensure the rail endpoint satisfies the requirements that caused the rail to be created, not merely internal architecture cleanup

Depends on:

- Node 18

Code targets:

- `README.md`
- `USAGE_GUIDE.md`
- `LOCUS64_LANGUAGE_SPEC.md`
- `SEMANTIC_USAGE_GUIDE.md`
- `HANDOFF_STATUS.md`
- `release`
- release scripts
- GitHub repository metadata

Step sequence:

1. Verify the implemented system matches the final rail definition: RNA/DNA public surface, structural authority, canonical identity, lineage-bound execution, lawful reuse.
2. Verify user-facing commands use `l64` and documented examples run.
3. Verify no Q-surface or misleading naming residue remains outside approved historical notes.
4. Generate or refresh usage documentation for actual interaction language, command syntax, RNA authoring, DNA validation, execution, tracing, certification, and reuse.
5. Run conformance, torture, and release smoke tests.
6. Produce Windows and Linux binary releases for performance and compact profiles.
7. Produce a source release without stale generated cache, target output, or obsolete release payloads.
8. Zip release artifacts and verify archive contents.
9. Commit, tag or document release version, and push to GitHub.
10. Write final handoff with exact commands run, tests passed, known limits, and next rail node if any remains.

Actions:

- close documentation and release packaging as first-class rail outputs
- treat stale docs as release failures
- treat missing source/binary release artifacts as endpoint failure
- keep known limitations explicit rather than hidden behind marketing language

Invariants:

- final endpoint is usable by a new operator without knowing the chat history
- release contents match docs
- GitHub state matches local release state
- all unresolved limitations are documented with next-step location

Tests:

- full conformance suite
- torture test
- release package smoke test
- archive content inspection
- docs command scan
- `git status --short`

Exit condition:

- the project is shipped end-to-end against the rail requirements with reproducible docs, releases, and handoff

Downstream payoff:

- the rail terminates in a product-ready state, not an unfinished implementation plan

## 24. Second Compounding Change Chain

This change chain is now applied to the rail:

1. Reclassify Q-surface crates as extraction-and-deletion targets, not compatibility surfaces.
2. Reframe Node 15 as a naming hygiene gate, not a naming-centered implementation phase.
3. Replace the former compatibility-demotion phase with Node 16 Q-surface extraction and deletion.
4. Require useful code from extraction sources to move into RNA/DNA or shared substrate modules before deletion; `l64-qc0`, `l64-qa0`, and `l64-surfaces` have now been deleted, while `Qm0` remains only as a temporary binary-compatibility tombstone if still required.
5. Require commands, samples, and docs to redirect to `.rna` and `.dna` artifacts instead of `.qc0` or `.qa0`.
6. Permit naming changes only when they prevent authority confusion, public API drift, dependency cleanup risk, or repeated implementation mistakes.
7. Permit temporary shims only when they have an owner node and explicit deletion condition.
8. Add search/build/test gates proving Q-surface residue is gone rather than merely documented.
9. Add an end-to-end delivery closure node so the rail endpoint includes docs, releases, source archive, GitHub state, and handoff.
10. Add per-node step sequences for new and plan-altering nodes to keep execution temporally honest.

## 25. Third Compounding Change Chain

This change chain optimizes the rail for Rust-specific, high-efficiency execution:

1. Add Rust development optimization rules so implementation proceeds through typed contracts, crate-local tests, table-driven law, and mechanical edits when naming hygiene is truly required.
2. Add an adversarial audit checklist for each node.
3. Record plan-altering codebase facts: Q-surface crates are active dependencies, misleading names can hide deletion risk, and release snapshots are not source truth.
4. Move the phase contract skeleton earlier by merging it into Node 02 with canonical identity foundation.
5. Keep the full phase engine later as Node 13, but make it an upgrade of the early skeleton rather than a late invention.
6. Add nested step sequences to lower-chain nodes that previously had only actions.
7. Tighten naming sequencing to exclude generated release snapshots and isolate necessary naming hygiene from Q-surface deletion.
8. Add local Rust test strategy: check crate, test crate, then run workspace tests at phase exit.

## 26. Fourth Compounding Change Chain

This change chain corrects the rail's treatment of names:

1. Names are allowed and useful when they coordinate work.
2. Names are not mechanism and must not become implementation objectives by themselves.
3. A name change is justified only when the name creates authority confusion, public API drift, dependency cleanup risk, or repeated implementation mistakes.
4. Existing names may remain when they are harmless implementation handles.
5. Planning should state a name only when doing so prevents coordination failure or points to a concrete code target.
6. Node 15 is now a naming hygiene gate rather than a rename-centered phase.
7. Renaming must stay subordinate to substrate closure, Q-surface extraction/deletion, DNA authority, execution, lineage, reuse, testing, and release.

## 27. Fifth Compounding Change Chain

This change chain corrects the rail's treatment of deletion and representation:

1. The rail is organized around authority collapse, not named-object deletion.
2. The first hard implementation commitment is the complete authority path: `RNA -> RNORM -> SSR -> CNORM -> DNA -> DNA_DECODE -> CNORM_RECONSTRUCTION`.
3. Deletion begins only after that replacement authority path exists and can classify remaining systems mechanically.
4. Remaining systems are classified as load-bearing obsolete beams, feeding the authority path, consuming it, bypassing it, extraction-only, leaf projections, or deletable.
5. Bypassing and deletable mechanisms are removed; feeding and consuming mechanisms are reattached.
6. Graphs, arenas, trees, DAGs, term structures, and opcode streams are implementation representations, not substrate authority.
7. "Canonical topology" wording is replaced with "canonical structure" to avoid turning one mathematical representation into the substrate.

## 28. Sixth Compounding Change Chain

This change chain corrects the rail's extraction/deletion ordering:

1. The rail now chooses extraction order by dependency pressure, not deletion convenience.
2. Load-bearing obsolete authority beams are replaced before residual leaf projections are removed.
3. The QC0-style bundle/import/certification flow is the current primary obsolete beam unless a fresh authority audit proves otherwise.
4. The completed QK0 deletion is recorded as a valid isolated-leaf cut, not as the strategy template for the remaining work.
5. Node 16A now requires a beam-rank table and a representative `.dna`, RNA/DNA, or lineage-native replacement seam before broad Q deletion.
6. Fixture, command, sample, and certification migrations now prove the beam is collapsing before crate deletion is counted as progress.
7. Naming-only leaf-first cleanup remains allowed only when it does not delay beam replacement.

## 29. Seventh Compounding Change Chain

This change chain incorporates the CKI registry proving-project lesson without changing the main task:

1. External proving projects are permitted only when they strengthen the RNA/DNA authority path.
2. The required external proving flow is `external authored structure -> RNA ingestion -> RNORM/SSR/CNORM -> DNA authority -> validation receipts -> derived projections`.
3. Markdown, DOT, Julia objects, JSON, notebooks, and reports are projection or extraction material, not authority.
4. CKI-style typed registries are recognized as useful pilot fixtures because they expose dependency closure, parent ordering, typed edges, proof dependencies, and open obligations.
5. Node 16A may use one bounded CKI-style registry pilot only if it reduces primary-beam ambiguity and does not create a third public surface.
6. Node 17 now carries the semantic rekeying requirements for registry-like structures.
7. Release discipline gates now explicitly include dependency closure, parent-before-child constructive order, explicit open obligations, validation receipts, and projection/authority separation.

## 30. Eighth Compounding Change Chain

This change chain incorporates the artifact-membrane failure found by inspecting `sequence-dna`, `compile-rna`, and the CKI proving path:

1. The active load-bearing beam is no longer Q/projection cleanup. That cleanup is historical; the current beam is source/authority/projection membrane correctness.
2. Local validity is insufficient. The authority gate is relational identity preservation across `RNA -> DNA -> canonical RNA -> DNA`.
3. `sequence-dna` must mean canonical RNA sequencing and must not emit report/inspection JSON.
4. Inspection/report output belongs to `inspect-dna` or an equivalent projection command and cannot be fed back into `compile-rna`.
5. `compile-rna` must accept only source RNA and reject obvious projection, report, receipt, trace, failure, and JSON artifacts before lower-chain tokenization.
6. `verify-roundtrip` becomes the operator-facing proof command for fixed-point authority.
7. CKI `cki_registry.genome.rna` becomes the first nontrivial regression fixture for source/authority/projection separation.
8. Torture tests must prove both positive sequencing fixed point and negative inspection-output rejection.
9. JSON/YAML/report formats are projections or import materials only; they are not native lower-chain golden surfaces.
10. Documentation must teach actual command roles instead of implying every emitted text artifact is valid source.

## 31. Ninth Compounding Change Chain

This change chain adds genome release discipline without making public-facing artifacts into authority:

1. Public release artifacts are function-named, not virtue-named.
2. The release spine is: canonical genome, source sequence, dependency spine, closure map, claim pages, closure frontier, stress map, lineage, replay record, views, and view receipts.
3. These artifacts are derived from DNA, canonical structure, lineage, receipts, obligations, and fixed-point replay.
4. Every claim must ship with coordinates: DNA hash, canonical ID, source node IDs, receipt IDs, dependencies, status, open dependencies, and stress points.
5. Closure status propagates through dependencies and downgrades rather than overclaims.
6. Generated Markdown, DOT, diagrams, paper skeletons, and reviewer summaries are views with receipts, never authority.
7. The CKI proving slice becomes the first genome release fixture after the membrane patch and semantic rekeying.
8. Conformance now checks coordinate completeness, visible open edges, replayability, view receipts, and status propagation.
9. Release packaging fails if public-facing artifacts are not traceable to the genome or if open obligations are hidden.
10. Node 17A explicitly depends on surface-role admission, DNA-to-canonical-RNA reconstruction, and fixed-point roundtrip gates.
11. Release artifacts are rejected by `compile-rna` unless explicitly admitted as `Source` over `RNA` or `CanonicalRNA`.
12. Conformance includes positive canonical RNA fixed-point tests and negative ingestion tests for inspection output, views, receipts, claim pages, closure maps, stress maps, and replay records.

## 32. Tenth Compounding Change Chain

This change chain corrects the next-order substrate mistake before implementation drifts into product-first syntax cleanup:

1. The current `!l64-bundle v1` header migration is useful but not sufficient. It removes the old Q marker while preserving JSON-shaped bundle bodies.
2. The next route is not a prettier identifier scheme and not product-first packaging. The next route is a law-first substrate reroute.
3. Node 11B must produce codon/lexon law before substrate implementation:
   - `CodonSpec`
   - `LexonSpec`
   - alias normalization
   - phase/admission matrix
   - opcode ranges
   - tombstone/versioning policy
   - generated-word ban list
   - representative lexon/macro-codon corpus
4. Codons define structural class, operation, arity, phase, admission, and opcode identity. Codons are not semantic codebooks.
5. Lexons bind scoped semantic names to canonical targets with aliases, gloss, and binding receipts. Lexons are not global opcodes.
6. Node 11C must define the molecular substrate codec spine:
   - locus
   - atom
   - bond
   - reaction
   - witness
   - chassis
   - cassette
   - plasmid
7. Codec records serialize substrate primitives. Codec records are not ontology and must not contain a generic native payload bag.
8. Node 11D must reroute bundle semantics through substrate primitives and prove behavior parity:
   - same successful outcomes
   - same old-valid behavior
   - same old-invalid rejection behavior
   - new rejection of known category errors
9. Bundle object boundaries do not define substrate primitive boundaries; old workflow structs are temporary adapters.
10. Node 11E expresses `.pep`, `.prot`, `.ptome`, and `.cell` products only after witnessed substrate/DNA authority exists.
11. `.pep`, `.prot`, and `.ptome` are not source formats. They are expressed products and must be rejected by `compile-rna`.
12. `.cell` is not a new authority surface. It is a package containing DNA, canonical RNA, products, views, receipts, replay, and index.
13. Witnesses are derived from substrate state, not authored as truth. Public product identity becomes canonical witness form; hashes remain memo/cache bindings.
14. `express` becomes the product-generation command only as a wrapper/rename over the proven release product path.
15. Node 11F quarantines/deletes transitional `.record`, `.projection`, default JSON inspection, and JSON bundle-body paths after replacement behavior is proven.

## 33. Definition Of Done

The rail is implemented when:

- `LOCUS64_ARCHITECTURAL_CONSTITUTION_V1.md` exists and freezes authority doctrine, native symbol language, codon/lexon/macro law, molecular substrate law, codec spine law, bundle reroute law, campaign/certification law, projection quarantine, CKI fixture law, codebase compliance status, and the defer/reject ledger
- every behavior-bearing reroute after Node 11D consumes the architectural constitution rather than carrying scaffold momentum forward as implicit authority
- distinction/transition law exists between structural form and EQUIV/CNORM
- EQUIV consumes explicit distinction, invariant, transition, and collapse law instead of inferring sameness from raw shape, source text, graph layout, or hash equality
- CNORM canonicalizes preserved structure under that law and never erases authority-bearing distinctions
- required DNA sections contain no semantic text needed for validation or execution
- SSR cannot be serialized as authority
- structural-form representations cannot become public identity layers
- CNORM identity is independent of presentation strings and source formatting
- DNA validates from structural sections alone
- generic DNA packet integrity binds payload bytes, not schema identity alone
- authored text capture is one-way source ingress; compiled DNA and products do not owe prose reconstruction
- `compile-rna` accepts source RNA only and rejects projection/report/receipt/trace/failure text
- `sequence-dna` emits canonical reconstructable RNA only
- `inspect-dna` owns inspection/report output
- `verify-roundtrip` proves `RNA -> DNA -> canonical RNA -> DNA` fixed-point identity
- CKI fixed-point regression passes through the public command path
- execution traverses DNA or canonical structure directly
- every public CLI command maps to phase-engine transitions and ledger entries
- semantic systems are lineage-keyed derived overlays
- Q-surface crates are removed and no longer shape the architecture
- public command/crate naming does not mislead users, implementation, authority boundaries, or deletion sequencing
- compatibility imports are removed unless backed by a concrete active requirement and deletion condition
- persisted cross-binary cache/report/lock keys use the shared cache-hash policy rather than ad hoc local hashers
- public documentation does not recommend QC0, QA0, QM0, or QK0 as public languages or active authority routes
- wrapper commands do not route deleted or retired commands as live behavior
- conformance, fuzz, torture, replay, migration, and cross-platform determinism tests pass
- external proving slices demonstrate that authored registry-like structures can be ingested through RNA/DNA and exported as projections without making the projection authoritative
- genome release artifacts are expressed as `.pep`, `.prot`, `.ptome`, and `.cell` products after Node 11E lands
- every claim/product has canonical witness identity: kind, status, subject, dependencies, proof route, open frontier, and receipt coordinate
- every digest/cache key resolves to a witness form in the product index
- no release artifact can be ingested by `compile-rna` unless it has an explicit source/reconstruction role over RNA or canonical RNA
- inspection reports, generated views, receipts, `.pep`, `.prot`, `.ptome`, `.cell` product text, claim pages, closure maps, stress maps, replay records, and lineages are rejected as source
- release gates include dependency closure, parent-before-child constructive order, explicit open obligations, validation receipts, replay records, status propagation, and projection/authority separation

## 34. Final System Definition

Locus64 is a deterministic structural substrate in which symbolic interaction surfaces are normalized into canonical structure, encoded into reconstructive machine form, executed under lineage-preserving authority, and amortized through lawful structural reuse.

Compact form:

```text
syntax is transient
structure is authority
canonical form is identity
reuse is proven rather than assumed
```
