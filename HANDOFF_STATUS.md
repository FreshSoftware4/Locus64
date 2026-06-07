# Locus64 Handoff Status

## Current State

The workspace builds and tests as a Rust workspace with the unified `l64` wrapper, direct `l64-cli`, and direct `l64-admin` binaries.

The source crate/package/binary prefix rename from `mf` to `l64` has been executed. Remaining `mf` mentions should be limited to historical rail notes or explicitly documented transitional residue.

The active lower chain is implemented as a transitional path:

```text
Tokenization -> RnaNormalization -> StructuralResolution -> CanonicalNormalization -> DNA emission
```

Implemented transitional substrates include:

- token classes, token streams, tokenization receipts, token specs
- RNORM diagnostic specs and token-grounded normalization receipts
- SSR transition specs and ephemeral SSR receipts
- CNORM rule specs, idempotence receipts, canonical hashes
- DNA header receipts and validation reports
- execution exactness and execution closure receipts
- proof coverage dispatch with reuse legality, reuse decisions, and residual verification receipts
- research lineage records that carry canonical hash, lowering receipt id, phase ids, and phase ledger

The current `LINEAR_EXECUTION_RAIL.md` supersedes earlier completion framing. It classifies the current system as graph-persistence/proto-DNA infrastructure that must be inverted into true structural execution substrate before upper-stack expansion continues.

## Shipping Entry Points

- `l64`: canonical wrapper
- `l64-cli`: direct CLI compatibility binary
- `l64-admin`: direct admin compatibility binary

Release packages include all three because `l64` dispatches to sibling `l64-cli` or `l64-admin`.

## Verification Commands

```powershell
cargo fmt --check
cargo test -q
cargo build --profile perfopt -p l64 -p l64-cli -p l64-admin
cargo build --profile compact -p l64 -p l64-cli -p l64-admin
powershell -ExecutionPolicy Bypass -File .\scripts\torture-test.ps1 -OuterRounds 1 -InnerNamespaces 1
```

## Release Profiles

- `perfopt`: `opt-level=3`, fat LTO, one codegen unit, stripped, abort panic
- `compact`: size optimization, LTO, one codegen unit, stripped, abort panic

Linux target attempted by this release pass: `x86_64-unknown-linux-gnu`. The Rust target is installed, but packaging Linux binaries is blocked locally because the GNU linker `cc` is missing.

Windows target used by this release pass: `x86_64-pc-windows-msvc`.

## Known Constraints

- The repo is a Git repository connected to GitHub.
- `.locus` decode remains as a compatibility path; `.dna` is the preferred machine artifact language.
- Q-surface crates have been removed from the workspace. Public doctrine is RNA/DNA; remaining Q-surface mentions are historical notes, negative regressions, or documented tombstones only.
- SSR is intentionally ephemeral and must not become a persisted semantic authority layer.
- `sequence-dna` emits canonical reconstructable RNA; `inspect-dna` owns JSON inspection output.
- `compile-rna` rejects inspection reports, release projections, release records, receipts, and JSON report/projection text as source.
- Generated genome release artifacts now use source/canonical `.rna`, genome `.dna`, and role-specific `.projection`, `.record`, or receipt filenames. They no longer emit `.locus` projection files or an on-disk JSON manifest.
- Admin/cert artifact discovery no longer reads legacy JSON manifest/lock/report caches, legacy report `.locus` caches, or execution `reports.json`.
- Execution manifest and bundle-lock packet caches now use `.dna` filenames rather than `.locus` filenames.
- Certification execution cache entries now use `.dna` filenames rather than `.locus` filenames. Rebuild `l64-cli` and `l64-admin` together before running admin tests that use CLI-produced cache entries.
- Observe persistence entries now use `.dna` filenames rather than `.locus` filenames for observation/diff/prediction/plan/explanation/assessment/execution/reconciliation records.
- Bundle-entry authoring text now uses the native `!l64-bundle v1` header. `compile-bundle` rejects obsolete `!qc0`/`!qa0` headers.

## Cleanup Policy Used

Generated caches, old zips, transient scratch files, and previous release payloads are moved to:

```text
C:\Users\Fresh\Projects\Locus64 Garbage
```

They are not hard-deleted.

## Most Important Files

- `README.md`: concise project overview and release layout
- `LOCUS64_LANGUAGE_SPEC.md`: RNA/DNA command and language reference, including source/inspection membrane rules
- `USAGE_GUIDE.md`: operator command guide
- `SEMANTIC_USAGE_GUIDE.md`: semantic usage guide for claim governance, branch/challenge tracking, and research-framework integration
- `LINEAR_EXECUTION_RAIL.md`: authoritative linear execution rail and phase sequence
- `scripts/torture-test.ps1`: regression/torture harness
- `Cargo.toml`: workspace members and release profiles
