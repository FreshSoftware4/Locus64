# Locus64 Usage Guide

Use `mf` as the primary command. On Windows release bundles, run `mf.exe` from the release directory. On Linux release bundles, run `./mf`.

## Verify Installation

```powershell
mf surface-capabilities
mf dump-runtime-roots
mf clear-cache --scope all
```

If running from source on Windows:

```powershell
cargo build --release -p mf -p mf-cli -p mf-admin
.\target\release\mf.exe surface-capabilities
```

## Campaign Certification

```powershell
mf certify-derived --campaign CPG_CHAIN_RULE
mf certify-derived --campaign CPG_CHAIN_RULE_RECIPE
mf certify-derived --campaign CPG_CHAIN_RULE_TRANSPORT
mf certify-derived --campaign CPG_BAYES_BRACE
mf certify-derived --campaign CPG_EXEC_INFER
mf certify-derived --campaign CPG_PROB_JUDG
mf certify-derived --campaign CPG_CERT_PROP
mf certify-derived --campaign CPG_CH_NORM
mf certify-derived --campaign CPG_CH_INH
```

Use `observe-run` after certification:

```powershell
mf observe-run --report REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
```

## Bundle Certification

```powershell
mf certify-bundle --file samples/chain_rule_bundle.qc0 --conflict-policy exact-match
mf certify-bundle --file samples/chain_rule_integrated_bundle.qc0 --conflict-policy exact-match
mf certify-bundle --file samples/imported_claim_bundle.qc0 --conflict-policy exact-match
mf certify-bundle --file samples/imported_claim_stress_gap_bundle.qc0 --conflict-policy exact-match
```

## RNA/DNA Commands

```powershell
mf normalize-rna sample.gene.rna
mf compile-rna sample.gene.rna --out sample.gene.dna --artifact-class gene --persist-lineage
mf sequence-dna sample.gene.dna
```

The lower chain is ledgered as:

```text
Tokenization -> RnaNormalization -> StructuralResolution -> CanonicalNormalization
```

DNA emission validates header truth, structural opcode law, and symbol-table non-authority before writing the artifact.

## Export, Import, Validate

```powershell
mf export-report --id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE --to qc0
mf export-validation-bundle --id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE --to qc0
mf export-locus-packet --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE --out chain-rule.dna
mf import-locus-packet chain-rule.dna
```

## Lock and Replay

```powershell
mf lock-bundle samples/chain_rule_integrated_bundle.qc0 --optimizer-policy conservative --conflict-policy exact-match
mf replay-with-lock <LOCK_ID> --parallel-obligations --max-obligation-workers 3
```

## Research Host

```powershell
mf research-import --kind task samples/research/task_operational_truth.json
mf research-import --kind signature samples/research/signature_operational_truth.json
mf research-route --task-id TASK_OPERATIONAL_TRUTH_HARDENING --signature-id SIG_OPERATIONAL_TRUTH_HARDENING
mf research-derive-from-report --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE --persist
mf research-promotion-readiness REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
mf research-status
```

## Coverage and Tower

```powershell
mf dispatch-coverage --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
mf derive-frontier --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
mf tower-step --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
mf derive-distress --report-id REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE
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
mf clear-cache --scope all
```

Release users can delete `.mf-cache` safely when no run needs cached reports or research artifacts.
