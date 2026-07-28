# Source Reference Manifest

## Live source

- `l64-symbolic`
- `l64-native`
- `l64-projection`
- `l64-release`
- `l64-execution`
- `l64-transport`
- `l64-certification`
- `l64-observation`
- `l64-change`
- `l64-cli`
- `l64`

## Governing closure

- `LOCUS64_LEGACY_AUTHORITY_ISLAND_DELETION_CHANGE_CHAIN.athens`
- `LOCUS64_NATIVE_EXECUTION_CHANGE_CHAIN.athens`
- `LOCUS64_NATIVE_CLI_HARDENING_CHANGE_CHAIN.athens`
- `LOCUS64_NATIVE_PROCESS_CONTRACT_CHANGE_CHAIN.athens`
- `LOCUS64_NATIVE_SCALE_CHANGE_CHAIN.athens`
- `LOCUS64_DOCUMENTATION_COHERENCE_CHANGE_CHAIN.athens`
- `LOCUS64_GOLDEN_PORTABILITY_CHANGE_CHAIN.athens`
- `LOCUS64_RESIDUE_REMOTE_PROOF_CHANGE_CHAIN.athens`
- `LOCUS64_EXECUTION_COHERENCE_RAIL.athens`

## Restore and historical recovery

The removed legacy source is external to the live workspace in the Pass 19–23 forensic archives. The complete Pass 29 source archive is the restore boundary for Pass 30. The removed pre-native torture harness is retained only in the external Pass 30 forensic backup; it is not a live verification contact.

## Direct verification

- `scripts/verify-legacy-authority-island-deletion.sh`
- `scripts/verify-native-carrier.sh`
- `scripts/verify-cli-hardening.sh`
- `scripts/verify-process-contract.sh`
- `scripts/verify-native-scale.sh`
- `scripts/verify-documentation-coherence.sh`
- `scripts/verify-golden-portability.sh`
- `scripts/verify-portability-receipts.sh --self-test`
- `scripts/run-native-demo.sh`

## Current product evidence

- `samples/native_triangle.rna`
- `samples/native_equality.rna`
- `samples/golden/*.rna`
- `samples/golden/expected/*`
- `LOCUS64_GOLDEN_PORTABILITY_CONTRACT.md`
- `LOCUS64_PORTABILITY_RECEIPT_CONTRACT.md`

## Pass 30 residue and remote-proof surfaces

- `scripts/write-portability-receipt.sh` emits one source-bound host receipt after the complete host-neutral workspace test passes.
- `scripts/verify-portability-receipts.sh` requires the exact Linux, macOS, and Windows receipt set and provides a local contract self-test.
- `.github/workflows/native-core.yml` retains per-host and combined receipt artifacts.
- `scripts/verify-documentation-coherence.sh` rejects a returned torture harness, retired-command use in current scripts, stale restore-boundary prose, and missing receipt contacts.
- `LOCUS64_RESIDUE_REMOTE_PROOF_CHANGE_CHAIN.athens` records the bounded closure.
