# Locus64

Locus64 is a compact, dependency-free native authority and execution system.

Current authority is canonical `L64R1` RNA and `L64D` DNA. `L64B` transports ordered DNA members without creating composite authority. Execution evaluates current authority directly without mutation or persistence. Certification, observation, change, and projection are verified read-only derivatives.

## Install a release

GitHub releases provide four x86-64 packages: compact and
performance-optimized builds for Windows and Linux. Each package contains both
`l64` and `l64-cli`; keep them in the same directory because `l64` delegates
non-audit commands to its sibling `l64-cli`.

After extracting a package, verify the command surface:

```bash
./l64 authority-audit
./l64-cli --version
```

On Windows, use `l64.exe` and `l64-cli.exe`. Validate the downloaded archive
against `SHA256SUMS.txt` from the same GitHub release.

`compact` minimizes storage footprint. `perfopt` favors execution speed. Both
profiles use whole-program LTO, one codegen unit, stripped binaries, and aborting
panics.

## Build release packages

Native packages can be reproduced from PowerShell on the matching host:

```powershell
./scripts/package-release.ps1 -Profile compact -Platform windows-x86_64 -Version v0.1.3
./scripts/package-release.ps1 -Profile perfopt -Platform windows-x86_64 -Version v0.1.3
```

Use `linux-x86_64` on Linux. Pushing a `v*` tag runs the GitHub release workflow,
which tests Linux and Windows independently, builds all four packages on native
runners, generates checksums, and publishes one GitHub Release.

## Verify the workspace

```bash
cargo test --locked --offline --workspace
cargo clippy --locked --offline --workspace --all-targets -- -D warnings
cargo fmt --all --check
./scripts/verify-legacy-authority-island-deletion.sh
./scripts/verify-native-carrier.sh
./scripts/verify-cli-hardening.sh
./scripts/verify-process-contract.sh
./scripts/verify-native-scale.sh
./scripts/verify-documentation-coherence.sh
./scripts/verify-golden-portability.sh
```

## Command surfaces

`l64-cli` owns the complete native command grammar. `l64` is a small sibling-binary wrapper: it adds `authority-audit`, forwards every other argument unchanged to `l64-cli`, and preserves the child exit code. It is not a second parser, execution layer, or compatibility route.

```bash
cargo run -p l64-cli -- --help
cargo run -p l64-cli -- help run-rna
cargo run -p l64-cli -- --version
cargo run -p l64 -- authority-audit
```

Primary commands:

```bash
cargo run -p l64-cli -- run-rna samples/native_triangle.rna
cargo run -p l64-cli -- compile-rna samples/native_triangle.rna --out triangle.dna
# New authority and bundle outputs are created atomically and never overwrite existing paths.
cargo run -p l64-cli -- run-dna triangle.dna
cargo run -p l64-cli -- certify-dna triangle.dna
cargo run -p l64-cli -- observe-dna triangle.dna
cargo run -p l64-cli -- inspect-dna triangle.dna
```

The legacy theorem, campaign, research, registry, producer-host, tower, policy, overlay, cache, planning, and administration execution island was historically exported and deleted. No live compatibility dispatcher remains.

Run the complete offline demonstration:

```bash
./scripts/run-native-demo.sh /tmp/l64-demo
```

## Process and memory contract

Verdict-bearing commands keep their output and return stable process codes: `0` certified/success, `10` open, `11` incomplete, `12` invalid, and `2` for usage/input/filesystem/processing failure. RNA parse failures include exact line, column, source excerpt, and caret span. Bundle creation and bundle-facing commands process one canonical DNA member at a time rather than retaining the complete transport and every graph simultaneously.

## Measured scale contract

Large-authority work is optimized only after profiling. Canonical RNA source compilation uses a bulk graph-construction path that suppresses per-instruction whole-state symbol and journal recomputation, then derives the exact final state once. Direct interactive graph mutations remain journaled. Route lookup is indexed per node; closure analysis is shared per context; equality canonicalization traverses real equality edges; retained/external projections remain explicitly reverified.


## Golden workload and portability contract

Five representative authority fixtures bind exact normalized RNA, canonical DNA bytes, execution text, certification text, mixed-verdict transport behavior, and native release contents. The host-neutral Rust tests also prove LF/CRLF equivalence and paths containing spaces and Unicode. CI runs the complete Rust workspace on Linux, macOS, and Windows; Unix shell gates remain release orchestration rather than runtime dependencies.

Run the direct local gate:

```bash
./scripts/verify-golden-portability.sh
```

## Current source-of-truth documents

- [`LOCUS64_NATIVE_CONSTITUTION.md`](LOCUS64_NATIVE_CONSTITUTION.md): live architecture law.
- [`L64_APPROVAL_GATES.md`](L64_APPROVAL_GATES.md): live executable gates.
- [`LOCUS64_LANGUAGE_SPEC.md`](LOCUS64_LANGUAGE_SPEC.md): authority, transport, binary, command, and process specification.
- [`LOCUS64_STACK.md`](LOCUS64_STACK.md): current eleven-package contact map.
- [`LOCUS64_GOLDEN_PORTABILITY_CONTRACT.md`](LOCUS64_GOLDEN_PORTABILITY_CONTRACT.md): exact workload and host-portability law.
- [`HANDOFF_STATUS.md`](HANDOFF_STATUS.md): current pass boundary.

Historical trajectory is consolidated in [`changelog.log`](changelog.log) and
the explicitly marked historical ledgers. It is not a current architecture
source.
