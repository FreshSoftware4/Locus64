# Locus64

Locus64 is a Rust command-line certification framework for routed mathematical campaigns, lower-chain RNA/DNA compilation, adequacy checking, replayable execution, proof coverage, and governed research artifacts.

The primary binary is `l64`. It routes commands to the CLI or admin implementation as needed. `l64-cli` and `l64-admin` are still shipped as direct entry points for automation and lower-level debugging.

## Current Architecture

- **Locus Kernel**: cold semantic authority for certification, adequacy, replay, and promotion decisions.
- **RNA/DNA lower chain**: `TOKENIZE -> RNORM -> SSR -> CNORM -> DNA` is the active authority path. SSR remains ephemeral; CNORM now consumes explicit distinction/equivalence law and emits versioned canonical instructions.
- **Locus Genome**: `.dna` is the machine authority artifact. Legacy packet decoding is explicit migration/forensic behavior, not an ambient authority fallback.
- **Bundle substrate migration**: `.dna` bundle imports must pass native molecular-envelope parity, duplex local validation, domain closure, and deterministic merge evidence before persistence.
- **Research Host**: governed task, signature, review, challenge, lineage, promotion, handoff, and remediation surfaces.
- **Tower/Coverage**: proof coverage dispatch, lawful reuse receipts, residual verification, distress/help, recipes, and promotion candidates.

## Quick Start

```powershell
cargo test -q
cargo build --release -p l64 -p l64-cli -p l64-admin
.\target\release\l64.exe clear-cache --scope all
.\target\release\l64.exe certify-derived --campaign CPG_CHAIN_RULE
.\target\release\l64.exe observe-run --report REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
```

If the local machine is under memory pressure during a full rebuild, use the low-memory verifier:

```powershell
.\scripts\verify-low-memory.ps1 -Scope workspace
.\scripts\verify-low-memory.ps1 -Scope all
```

Expected seeded verdicts:

- `Integrated`: `CPG_CHAIN_RULE`, `CPG_CHAIN_RULE_RECIPE`, `CPG_CHAIN_RULE_TRANSPORT`
- `Certified`: `CPG_BAYES_BRACE`, `CPG_EXEC_INFER`, `CPG_PROB_JUDG`, `CPG_CERT_PROP`, `CPG_CH_NORM`, `CPG_CH_INH`

## RNA/DNA Flow

```powershell
Set-Content .\sample.gene.rna "ι ≔ σ ‖ κ" -Encoding UTF8
.\target\release\l64.exe normalize-rna .\sample.gene.rna
.\target\release\l64.exe compile-rna .\sample.gene.rna --out .\sample.gene.dna --artifact-class gene --persist-lineage
.\target\release\l64.exe sequence-dna .\sample.gene.dna
.\target\release\l64.exe inspect-dna .\sample.gene.dna
.\target\release\l64.exe verify-roundtrip .\sample.gene.rna --artifact-class gene
.\target\release\l64.exe export-genome-release --rna .\sample.gene.rna --out .\sample-release --artifact-class gene
```

`sequence-dna` emits canonical reconstructable RNA. `inspect-dna` emits inspection JSON. `compile-rna` rejects inspection/report/projection text as source.
`export-genome-release` emits a coordinate-bearing release spine from RNA/DNA authority: genome, source sequence, claim page, dependency spine, closure map, frontier, stress map, replay record, views, and view receipts. Only generated source/canonical RNA are re-compilable; the other release artifacts use projection/record/receipt roles and extensions.

RNA is the human symbolic surface. DNA is the machine artifact surface. SSR is ephemeral and must not be treated as public authority.

## Sample Bundles

```powershell
.\target\release\l64.exe certify-bundle --file samples/chain_rule_bundle.dna --conflict-policy exact-match
.\target\release\l64.exe certify-bundle --file samples/chain_rule_integrated_bundle.dna --conflict-policy exact-match
.\target\release\l64.exe certify-bundle --file samples/imported_claim_bundle.dna --conflict-policy exact-match
.\target\release\l64.exe certify-bundle --file samples/imported_claim_stress_gap_bundle.dna --conflict-policy exact-match
```

The imported-claim samples exercise evidence contracts, benchmark receipts, challenge receipts, reproducibility packets, and sharp stress-gap blocking.

If you have bundle-entry text, compile it into a `.dna` packet before using it as a certification input:

```powershell
.\target\release\l64.exe compile-bundle .\bundle.locus.rna --out .\bundle.dna
```

Bundle-entry text must start with `!l64-bundle v1` and contain at least one entry. Obsolete `!qc0`/`!qa0` headers and header-only bundles are rejected.

## Torture Test

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\torture-test.ps1
```

The harness builds release binaries, runs the full test suite, exercises seeded campaigns, sample bundles, report DNA export/import, validation DNA bundle export/import, lock/replay, research reconnect, and RNA/DNA normalize/compile/sequence checks. Output lands in `release\torture` unless overridden.

## Release Layout

Prepared release artifacts are generated under:

- `release\perfopt`: speed-optimized Windows and Linux binaries plus docs
- `release\compact`: footprint-minimized Windows and Linux binaries plus docs
- `release\src`: source release with usage and developer docs

Zip files for all five release packages are placed directly under `release`.

## Development References

- `LINEAR_EXECUTION_RAIL.md`: authoritative linear rail; compounding change chains are trajectory-preserving changes to this file
- `LOCUS64_LANGUAGE_SPEC.md`: RNA/DNA command and language reference
- `USAGE_GUIDE.md`: command guide
- `SEMANTIC_USAGE_GUIDE.md`: semantic/claim-governance guide for research frameworks and indirect ChatGPT workflows
- `HANDOFF_STATUS.md`: developer handoff and verification notes
- `LOCUS64_STACK.md`: stack overview
