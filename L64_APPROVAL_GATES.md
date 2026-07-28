---
document_status: current
view: live executable gates only
historical_ledger: L64_HISTORICAL_APPROVAL_LEDGER.md
---

# Locus64 Current Approval Gates

This file contains only gates that constrain the live native workspace. The complete candidate, rejection, deferral, supersession, and migration history is preserved in [`L64_HISTORICAL_APPROVAL_LEDGER.md`](L64_HISTORICAL_APPROVAL_LEDGER.md).

| Gate | Contact protected | Direct evidence | Current consequence |
|---|---|---|---|
| `native-constraint-core-green` | dimensional and guarded admission | `LOCUS64_CONSTRAINT_CORE_CHANGE_CHAIN.athens`; native constraint tests | inconsistent dimensions and refuted guards cannot commit |
| `proof-producing-congruence-green` | equality and congruence | `LOCUS64_PROOF_CONGRUENCE_CHANGE_CHAIN.athens`; equality tests | equality requires checked evidence and context scope |
| `incremental-closure-green` | context-relative closure | `LOCUS64_INCREMENTAL_CLOSURE_CHANGE_CHAIN.athens`; closure tests | change impact is derived without a second authority store |
| `native-upper-projection-green` | read-only upper views | `LOCUS64_NATIVE_UPPER_PROJECTION_CHANGE_CHAIN.athens`; projection tests | views remain reconstructible and non-authoritative |
| `legacy-authority-island-deletion-green` | deleted compatibility authority | `LOCUS64_LEGACY_AUTHORITY_ISLAND_DELETION_CHANGE_CHAIN.athens`; `scripts/verify-legacy-authority-island-deletion.sh` | deleted crates and executable legacy routes cannot return |
| `native-carrier-green` | RNA/DNA execution, release, transport, certification, observation, change | `scripts/verify-native-carrier.sh` | every product contact derives from exact native authority |
| `native-cli-hardening-green` | public command contact | `LOCUS64_NATIVE_CLI_HARDENING_CHANGE_CHAIN.athens`; `scripts/verify-cli-hardening.sh` | wrong formats fail locally; outputs are atomic and non-overwriting |
| `native-process-contract-green` | shell status, diagnostics, bounded bundle work | `LOCUS64_NATIVE_PROCESS_CONTRACT_CHANGE_CHAIN.athens`; `scripts/verify-process-contract.sh` | verdict output and process status remain stable and member-local |
| `native-scale-green` | measured structural performance | `LOCUS64_NATIVE_SCALE_CHANGE_CHAIN.athens`; `scripts/verify-native-scale.sh` | optimization cannot weaken authority or retained-projection verification |
| `documentation-coherence-green` | current architecture reconstruction | `LOCUS64_DOCUMENTATION_COHERENCE_CHANGE_CHAIN.athens`; `scripts/verify-documentation-coherence.sh` | current docs, Cargo membership, live help, scripts, and historical markers agree |
| `golden-portability-green` | representative product behavior and host invariance | `LOCUS64_GOLDEN_PORTABILITY_CHANGE_CHAIN.athens`; `scripts/verify-golden-portability.sh` | canonical bytes, stable outputs, verdicts, bundles, releases, line endings, paths, and host builds cannot drift silently |
| `residue-remote-proof-green` | live-source residue and retained host proof | `LOCUS64_RESIDUE_REMOTE_PROOF_CHANGE_CHAIN.athens`; `scripts/verify-documentation-coherence.sh`; `scripts/verify-portability-receipts.sh`; combined CI receipt artifact | deleted commands cannot return through scripts and three-host success must remain bound to one exact source |
| `workspace-green` | complete source boundary | locked-offline workspace tests, Clippy, formatting, ShellCheck | no local gate may substitute for complete workspace proof |

## Promotion law

1. Name the operational contact and the law being changed.
2. Land or identify direct executable evidence.
3. Verify every dependent current gate.
4. Update [`LOCUS64_NATIVE_CONSTITUTION.md`](LOCUS64_NATIVE_CONSTITUTION.md) when the law changes.
5. Preserve superseded reasoning in the historical ledgers when it remains reconstructively useful.
6. Pass `documentation-coherence-green` and `workspace-green` in the same release boundary.

No bulk promotion is allowed. A historical gate name, prior test, or narrative claim cannot authorize current behavior by resemblance.
