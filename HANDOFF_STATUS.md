# Locus64 Handoff Status

## Current State

The workspace builds and tests as a Rust workspace with the unified `mf` wrapper, direct `mf-cli`, and direct `mf-admin` binaries.

The active lower chain is implemented as:

```text
Tokenization -> RnaNormalization -> StructuralResolution -> CanonicalNormalization -> DNA emission
```

Implemented closure substrates include:

- token classes, token streams, tokenization receipts, token specs
- RNORM diagnostic specs and token-grounded normalization receipts
- SSR transition specs and ephemeral SSR receipts
- CNORM rule specs, idempotence receipts, canonical hashes
- DNA header receipts and validation reports
- execution exactness and execution closure receipts
- proof coverage dispatch with reuse legality, reuse decisions, and residual verification receipts
- research lineage records that carry canonical hash, lowering receipt id, phase ids, and phase ledger

## Shipping Entry Points

- `mf`: canonical wrapper
- `mf-cli`: direct CLI compatibility binary
- `mf-admin`: direct admin compatibility binary

Release packages include all three because `mf` dispatches to sibling `mf-cli` or `mf-admin`.

## Verification Commands

```powershell
cargo fmt --check
cargo test -q
cargo build --profile perfopt -p mf -p mf-cli -p mf-admin
cargo build --profile compact -p mf -p mf-cli -p mf-admin
powershell -ExecutionPolicy Bypass -File .\scripts\torture-test.ps1 -OuterRounds 1 -InnerNamespaces 1
```

## Release Profiles

- `perfopt`: `opt-level=3`, fat LTO, one codegen unit, stripped, abort panic
- `compact`: size optimization, LTO, one codegen unit, stripped, abort panic

Linux target used by this release pass: `x86_64-unknown-linux-gnu`.

Windows target used by this release pass: `x86_64-pc-windows-msvc`.

## Known Constraints

- The repo has no git metadata in this working copy, so cleanup is conservative and destructive source pruning is avoided unless tests prove it safe.
- `.locus` decode remains as a compatibility path; `.dna` is the preferred machine artifact language.
- Q-surface crates remain in the workspace as compatibility/implementation crates. Public doctrine is RNA/DNA, not a Q-surface product model.
- SSR is intentionally ephemeral and must not become a persisted semantic authority layer.

## Cleanup Policy Used

Generated caches, old zips, transient scratch files, and previous release payloads are moved to:

```text
C:\Users\Fresh\Projects\Locus64 Garbage
```

They are not hard-deleted.

## Most Important Files

- `README.md`: concise project overview and release layout
- `USAGE_GUIDE.md`: operator command guide
- `COMPOUNDING_CHANGE_CHAIN.md`: architecture rail and phase status
- `scripts/torture-test.ps1`: regression/torture harness
- `Cargo.toml`: workspace members and release profiles
