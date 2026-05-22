# Locus64

Locus64 is a Rust command-line certification framework for routed mathematical campaigns, lower-chain RNA/DNA compilation, adequacy checking, replayable execution, proof coverage, and governed research artifacts.

The primary binary is `mf`. It routes commands to the CLI or admin implementation as needed. `mf-cli` and `mf-admin` are still shipped for compatibility and scripting.

## Current Architecture

- **Locus Kernel**: cold semantic authority for certification, adequacy, replay, and promotion decisions.
- **RNA/DNA lower chain**: transitional `TOKENIZE -> RNORM -> SSR -> CNORM -> DNA` support exists, but the active rail now treats SSR/CNORM/DNA as substrate-inversion targets rather than fully closed final architecture.
- **Locus Genome**: current `.dna` artifacts are proto-DNA during the inversion; `.locus` remains readable during compatibility rollout.
- **Research Host**: governed task, signature, review, challenge, lineage, promotion, handoff, and remediation surfaces.
- **Tower/Coverage**: proof coverage dispatch, lawful reuse receipts, residual verification, distress/help, recipes, and promotion candidates.

## Quick Start

```powershell
cargo test -q
cargo build --release -p mf -p mf-cli -p mf-admin
.\target\release\mf.exe clear-cache --scope all
.\target\release\mf.exe certify-derived --campaign CPG_CHAIN_RULE
.\target\release\mf.exe observe-run --report REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
```

Expected seeded verdicts:

- `Integrated`: `CPG_CHAIN_RULE`, `CPG_CHAIN_RULE_RECIPE`, `CPG_CHAIN_RULE_TRANSPORT`
- `Certified`: `CPG_BAYES_BRACE`, `CPG_EXEC_INFER`, `CPG_PROB_JUDG`, `CPG_CERT_PROP`, `CPG_CH_NORM`, `CPG_CH_INH`

## RNA/DNA Flow

```powershell
Set-Content .\sample.gene.rna "ι ≔ σ ‖ κ" -Encoding UTF8
.\target\release\mf.exe normalize-rna .\sample.gene.rna
.\target\release\mf.exe compile-rna .\sample.gene.rna --out .\sample.gene.dna --artifact-class gene --persist-lineage
.\target\release\mf.exe sequence-dna .\sample.gene.dna
```

RNA is the human symbolic surface. DNA is the machine artifact surface. SSR is ephemeral and must not be treated as public authority.

## Sample Bundles

```powershell
.\target\release\mf.exe certify-bundle --file samples/chain_rule_bundle.qc0 --conflict-policy exact-match
.\target\release\mf.exe certify-bundle --file samples/chain_rule_integrated_bundle.qc0 --conflict-policy exact-match
.\target\release\mf.exe certify-bundle --file samples/imported_claim_bundle.qc0 --conflict-policy exact-match
.\target\release\mf.exe certify-bundle --file samples/imported_claim_stress_gap_bundle.qc0 --conflict-policy exact-match
```

The imported-claim samples exercise evidence contracts, benchmark receipts, challenge receipts, reproducibility packets, and sharp stress-gap blocking.

## Torture Test

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\torture-test.ps1
```

The harness builds release binaries, runs the full test suite, exercises seeded campaigns, sample bundles, report export/import, lock/replay, research reconnect, and RNA/DNA normalize/compile/sequence checks. Output lands in `release\torture` unless overridden.

## Release Layout

Prepared release artifacts are generated under:

- `release\perfopt`: speed-optimized Windows and Linux binaries plus docs
- `release\compact`: footprint-minimized Windows and Linux binaries plus docs
- `release\src`: source release with usage and developer docs

Zip files for all five release packages are placed directly under `release`.

## Development References

- `LINEAR_EXECUTION_RAIL.md`: authoritative linear rail; compounding change chains are trajectory-preserving changes to this file
- `LOCUS64_LANGUAGE_SPEC.md`: concrete command/RNA/QC0 syntax and record schema for interacting with Locus64
- `USAGE_GUIDE.md`: command guide
- `SEMANTIC_USAGE_GUIDE.md`: semantic/claim-governance guide for research frameworks and indirect ChatGPT workflows
- `HANDOFF_STATUS.md`: developer handoff and verification notes
- `LOCUS64_STACK.md`: stack overview
