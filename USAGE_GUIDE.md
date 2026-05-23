# Locus64 Usage Guide

Use `l64` as the primary command. On Windows release bundles, run `l64.exe` from the release directory. On Linux release bundles, run `./l64`.

## Verify Installation

```powershell
l64 surface-capabilities
l64 dump-runtime-roots
l64 clear-cache --scope all
```

If running from source on Windows:

```powershell
cargo build --release -p l64 -p l64-cli -p l64-admin
.\target\release\l64.exe surface-capabilities
```

## Campaign Certification

```powershell
l64 certify-derived --campaign CPG_CHAIN_RULE
l64 certify-derived --campaign CPG_CHAIN_RULE_RECIPE
l64 certify-derived --campaign CPG_CHAIN_RULE_TRANSPORT
l64 certify-derived --campaign CPG_BAYES_BRACE
l64 certify-derived --campaign CPG_EXEC_INFER
l64 certify-derived --campaign CPG_PROB_JUDG
l64 certify-derived --campaign CPG_CERT_PROP
l64 certify-derived --campaign CPG_CH_NORM
l64 certify-derived --campaign CPG_CH_INH
```

Use `observe-run` after certification:

```powershell
l64 observe-run --report REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
```

## Bundle Certification

```powershell
l64 certify-bundle --file samples/chain_rule_bundle.qc0 --conflict-policy exact-match
l64 certify-bundle --file samples/chain_rule_integrated_bundle.qc0 --conflict-policy exact-match
l64 certify-bundle --file samples/imported_claim_bundle.qc0 --conflict-policy exact-match
l64 certify-bundle --file samples/imported_claim_stress_gap_bundle.qc0 --conflict-policy exact-match
```

## RNA/DNA Commands

```powershell
l64 normalize-rna sample.gene.rna
l64 compile-rna sample.gene.rna --out sample.gene.dna --artifact-class gene --persist-lineage
l64 sequence-dna sample.gene.dna
```

The lower chain is ledgered as:

```text
Tokenization -> RnaNormalization -> StructuralResolution -> CanonicalNormalization
```

DNA emission validates header truth, structural opcode law, and symbol-table non-authority before writing the artifact.

## Export, Import, Validate

```powershell
l64 export-report --id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE --to qc0
l64 export-validation-bundle --id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE --to qc0
l64 export-locus-packet --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE --out chain-rule.dna
l64 import-locus-packet chain-rule.dna
```

## Lock and Replay

```powershell
l64 lock-bundle samples/chain_rule_integrated_bundle.qc0 --optimizer-policy conservative --conflict-policy exact-match
l64 replay-with-lock <LOCK_ID> --parallel-obligations --max-obligation-workers 3
```

## Research Host

```powershell
l64 research-import --kind task samples/research/task_operational_truth.json
l64 research-import --kind signature samples/research/signature_operational_truth.json
l64 research-route --task-id TASK_OPERATIONAL_TRUTH_HARDENING --signature-id SIG_OPERATIONAL_TRUTH_HARDENING
l64 research-derive-from-report --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE --persist
l64 research-promotion-readiness REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
l64 research-status
```

For exact syntax and record shapes, see `LOCUS64_LANGUAGE_SPEC.md`.

For semantic claim governance, branch tracking, derivation lineage, and cosmology-style workflows, see `SEMANTIC_USAGE_GUIDE.md`.

## Coverage and Tower

```powershell
l64 dispatch-coverage --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
l64 derive-frontier --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
l64 tower-step --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
l64 derive-distress --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
```

Reuse is lawful only when canonical identity or declared transport-equivalence, closure-valid lineage, replay legality, and residual-obligation accounting all pass.

## Torture Harness

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\torture-test.ps1
```

Useful smaller smoke pass:

```powershell
powershell -ExecutionPolicy Bypass -File .\scripts\torture-test.ps1 -OuterRounds 1 -InnerNamespaces 1
```

Artifacts land in `release\torture` by default.

## Cache Isolation

Set `MF_CACHE_NAMESPACE` to isolate runs:

```powershell
$env:MF_CACHE_NAMESPACE='demo'
l64 clear-cache --scope all
```

Release users can delete `.l64-cache` safely when no run needs cached reports or research artifacts.
