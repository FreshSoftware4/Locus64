---
document_status: current
document_role: sole_markdown_project_authority
workspace_packages: 11
external_dependencies: 0
ambient_legacy_authority: deleted_tombstones_only
receipt_schema: L64PORT1
release: v0.1.3
---

# Locus64

Locus64 is a compact, dependency-free native authority and execution system.
Canonical `L64R1` RNA and canonical `L64D` DNA are its only source-authority
surfaces. `L64B` transports ordered DNA members without creating composite
authority. Execution evaluates current authority directly without mutation or
persistence. Certification, observation, change, and projection are verified
read-only derivatives.

This is the repository's sole Markdown document. It is the current user guide,
technical specification, package map, conformance contract, and release record.
Historical development causality is consolidated separately in `changelog.log`
and remains non-authoritative.

## Install

GitHub releases provide compact and performance-optimized x86-64 builds for
Windows and Linux. Each package contains both `l64` and `l64-cli`; keep them in
the same directory because `l64` delegates ordinary commands to its sibling
`l64-cli`.

After extracting a package:

```bash
./l64 authority-audit
./l64-cli --version
```

On Windows, use `l64.exe` and `l64-cli.exe`. Validate the archive against
`SHA256SUMS.txt` from the same release. The `compact` profile minimizes storage
footprint. The `perfopt` profile favors execution speed. Both use whole-program
LTO, one codegen unit, stripped binaries, and aborting panics.

## Build and verify

The workspace contains eleven dependency-free native packages and zero external
Rust dependencies.

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

Build native release packages from PowerShell on the matching host:

```powershell
./scripts/package-release.ps1 -Profile compact -Platform windows-x86_64 -Version v0.1.3
./scripts/package-release.ps1 -Profile perfopt -Platform windows-x86_64 -Version v0.1.3
```

Use `linux-x86_64` on Linux. Pushing a `v*` tag runs the GitHub release
workflow, tests Windows and Linux independently, builds all four packages,
generates checksums, and publishes one GitHub Release.

## Quick start

Use `l64-cli` for the complete command grammar. Use `l64` as the installed
convenience entrypoint. The wrapper adds one local diagnostic:

```bash
l64 authority-audit
```

The wrapper is not a second parser, execution layer, authority path, or
compatibility fallback.

Discover commands:

```bash
cargo run -p l64-cli -- --help
cargo run -p l64-cli -- help compile-rna
cargo run -p l64-cli -- --version
cargo run -p l64 -- authority-audit
```

Author, execute, derive, and transport:

```bash
cargo run -p l64-cli -- normalize-rna samples/native_triangle.rna
cargo run -p l64-cli -- run-rna samples/native_triangle.rna
cargo run -p l64-cli -- compile-rna samples/native_triangle.rna --out triangle.dna
cargo run -p l64-cli -- run-dna triangle.dna
cargo run -p l64-cli -- sequence-dna triangle.dna
cargo run -p l64-cli -- inspect-dna triangle.dna
cargo run -p l64-cli -- verify-roundtrip samples/native_triangle.rna
cargo run -p l64-cli -- certify-dna triangle.dna
cargo run -p l64-cli -- observe-dna triangle.dna
cargo run -p l64-cli -- compare-dna before.dna after.dna
cargo run -p l64-cli -- compile-bundle first.dna second.dna --out work.l64b
cargo run -p l64-cli -- run-bundle --file work.l64b
```

Run the complete offline demonstration:

```bash
./scripts/run-native-demo.sh /tmp/l64-demo
```

`compile-rna`, `compile-bundle`, and `export-genome-release` create outputs
atomically and refuse existing destinations. Wrong carrier formats fail at the
requested command contact. Malformed RNA/DNA diagnostics use stable
human-readable messages rather than Rust debug syntax.

## Authority law

1. Canonical `L64R1` RNA and canonical `L64D` DNA are the only
   source-authority surfaces.
2. Positive authority equality is exact equality of the domain-qualified
   canonical representation. A compact symbolic seal may prove fast inequality,
   but a matching seal never proves equality.
3. Canonical decoding revalidates structure, contexts, constraints, equality
   evidence, operation law, ordering, framing, and exact re-encoding before
   authority is accepted.
4. Admission is transactional: a failed transition does not mutate authority,
   context, route indexes, or journal state.
5. Names, projections, reports, process codes, CLI formatting, caches, package
   topology, receipts, and historical records are not authority.
6. RNA normalization is deterministic, and
   `L64R1 -> L64D -> L64R1 -> L64D` is an exact fixed point.

### Symbolic commitment

For domain `D` and canonical structure `x`, exact identity is the
self-delimiting term:

```text
I(D, x) = L64S1 || |D| || |x| || D || x
```

Lengths are fixed-width little-endian integers. Equality is ordinary byte
equality and does not depend on collision probability.

The compact seal:

```text
S(D, x) = [D :: Sigma(x) * Pi(x) * Delta(x) * Omega(x)]
```

is a 256-bit projection used for fast inequality, corruption checks, indexing,
and diagnosis. Its coordinates are:

- `Sigma`: weighted aggregate content;
- `Pi`: ordered Horner composition;
- `Delta`: adjacent transition structure;
- `Omega`: nonlinear boundary closure.

Arithmetic is modulo the prime `2^64 - 59`. Ordered composition and canonical
set composition remain distinct. The seal is not a cryptographic hash,
signature, MAC, or authenticity mechanism. A matching seal only permits exact
comparison to continue.

### Native structure and proof

Routes, nodes, ports, contexts, constraints, judgments, witnesses, obligations,
and the transition journal share one native graph representation. The graph is
an implementation carrier, not a second authority surface.

Dimensions and guarded operations are checked at admission. Proven guards
receive native evidence, refuted guards reject, and unresolved guards create
native obligations. Equality is proof-producing and context-scoped:
reflexivity, exact identity, symmetry, transitivity, and congruence are
re-executed during decode.

Equivalence classes, representative caches, reverse indexes, route indexes, and
closure memoization are derived accelerators. They do not serialize into RNA or
DNA and do not participate in authority identity. Closure is context-relative
and may be `Closed`, `Open`, or `Invalid`; assumption changes create child
contexts rather than rewriting prior authority.

## Language and wire formats

### Authority surfaces

- `L64R1`: compact canonical textual RNA with one domain and strictly
  increasing local slots.
- `L64D`: bounded binary DNA with a fixed 44-byte header, exact canonical
  payload, and symbolic state seal.
- `L64S1`: exact self-delimiting identity envelope.

Projection, certification, observation, change, inspection, release metadata,
and reports are derived surfaces. They may reject or expose invalid authority,
but never become source authority.

### Transport

`L64B` is a versioned ordered frame containing one or more length-prefixed
canonical `L64D` members. It has a 16-byte header, a member count, and exact
length framing. It adds no names, hashes, manifests, registries, overlays,
conflict policies, receipts, caches, composite certificate, or composite
authority. Readers and writers process one canonical member at a time.

### `l64-cli` commands

- `normalize-rna`
- `compile-rna`
- `sequence-dna`
- `inspect-dna`
- `verify-roundtrip`
- `export-genome-release`
- `run-rna`
- `run-dna`
- `compile-bundle`
- `run-bundle`
- `certify-dna`
- `certify-bundle`
- `observe-dna`
- `observe-bundle`
- `compare-dna`
- `compare-bundle`

### `l64`-local command

- `authority-audit`

`authority-audit` belongs to the `l64` wrapper. All historical theorem,
campaign, research, registry, producer-host, tower, packet, policy, cache,
planning, administration, and `legacy` execution commands are permanently
deleted tombstones.

### Process semantics

Verdict-bearing commands emit their complete result and then return:

- `0`: success or `CERTIFIED`;
- `10`: `OPEN`;
- `11`: `INCOMPLETE`;
- `12`: `INVALID`;
- `2`: usage, input, filesystem, or processing failure.

These codes are process projections and do not alter authority. RNA syntax
failures carry an exact byte span `(line, column, length)` and render the source
excerpt and caret span.

## Package responsibilities

- `l64-symbolic`: exact identity envelope and non-authoritative compact seal.
- `l64-native`: canonical RNA/DNA authority, checked admission, equality,
  closure, and canonical codec.
- `l64-projection`: verified read-only views.
- `l64-execution`: direct non-persistent evaluation of RNA/DNA.
- `l64-release`: atomic four-contact release carrier.
- `l64-transport`: ordered `L64B` transport without composite authority.
- `l64-certification`: structural certification over exact authority.
- `l64-observation`: verified certification, replay, and report observation.
- `l64-change`: ephemeral exact-authority comparison.
- `l64-cli`: complete native command grammar and direct execution.
- `l64`: sibling wrapper adding `authority-audit` and otherwise forwarding
  unchanged to `l64-cli`.

### Derived carrier law

Execution validates exact authority and derives verified projections without
mutating authority or persisting scheduler or run state. Certification
classifies each burden as discharged, open, invalid, or missing evidence.
Observation binds direct certification, verified replay, and verified
closure/opcode reports. Change reports structural, count, certification, and
verified per-context movement between exact authorities.

Fresh in-process projections may be trusted by construction only inside the
deriving operation. Retained or externally supplied projections require
explicit rederive-and-compare verification. None of these packages creates a
registry, policy graph, cache, campaign, receipt store, alternate graph, or
composite bundle verdict.

### Release carrier law

A native release contains exactly four direct contacts:

1. canonical RNA;
2. canonical DNA;
3. one verified projection;
4. one small release record binding them.

Release rendering creates no second authority schema. File-producing commands
stage output atomically and refuse to overwrite existing destinations.

## Performance and resource law

Performance changes require a measured structural cause. Optimization may
remove repeated derivation, allocation, scanning, or transient journaling only
when exact authority, wire bytes, member order, verdict law, stale/forged
projection rejection, and public mutation semantics remain unchanged.

Canonical source compilation uses private bulk construction, suppresses
transient construction journaling, and derives the final state once. Public
graph mutation remains fully journaled. Primary routes, reverse dependencies,
context-local nodes, and closure memoization are derived from canonical
authority and rebuilt after decode.

No generic digest service, schema router, cache bureaucracy, alternate graph,
or semantic lookup system may conceal local work. Large-authority work is
optimized only after profiling. The workspace must remain buildable and
testable with locked offline Cargo.

## Golden workload and portability contract

Golden fixtures are executable comparison evidence, not authority.

| Workload | Contact | Required verdict | Distinction protected |
|---|---|---:|---|
| `certified_triangle.rna` | typed function composition | `CERTIFIED` / `0` | proof-producing operation and fixed point |
| `equality_chain.rna` | equality evidence | `CERTIFIED` / `0` | equality replay and canonical ordering |
| `open_obligation.rna` | unresolved square-root guard | `OPEN` / `10` | visible non-fatal burden |
| `invalid_child_context.rna` | refuted child guard | `INVALID` / `12` | local invalidity without root rewrite |
| `matrix_multiply.rna` | matrix shape admission | `CERTIFIED` / `0` | typed shape law and witness |

The repository retains exact normalized RNA, lowercase DNA hexadecimal,
execution text, and certification text for every workload. Product-journey
snapshots additionally bind comparison, mixed-verdict bundle execution,
certification, observation, and the four-file native release.

Portability law:

1. Canonical bytes and stable output are identical on Linux, macOS, and Windows.
2. LF and CRLF RNA normalize to identical LF RNA and DNA.
3. Stable textual output uses `\n`.
4. Paths containing spaces and valid Unicode remain native paths.
5. Host width and endianness cannot enter authority.
6. `l64` forwards the exact argument vector and exit code.
7. Complete workspace tests execute on every supported host.
8. Each host emits one `L64PORT1` receipt bound to the exact source commit.
9. The combined verifier requires the exact three-host set from one source.
10. Receipts remain external evidence and never enter native authority.
11. Shell orchestration is not a runtime dependency.
12. Build-and-execute gates honor `CARGO_TARGET_DIR`.

Each receipt records source, repository, workflow run and attempt, runner label,
Rust host triple, compiler and Cargo versions, package count, dependency count,
test command, and passed result. The supported runner set is exactly
`ubuntu-latest`, `macos-latest`, and `windows-latest`.

Direct evidence:

- `samples/golden/*.rna`;
- `samples/golden/expected/*`;
- `l64-cli/src/golden_tests.rs`;
- `scripts/verify-golden-portability.sh`;
- `scripts/write-portability-receipt.sh`;
- `scripts/verify-portability-receipts.sh`;
- `.github/workflows/native-core.yml`.

## Current approval gates

| Gate | Protected contact | Direct evidence |
|---|---|---|
| `native-constraint-core-green` | dimensional and guarded admission | `l64-native/tests/constraints.rs` |
| `proof-producing-congruence-green` | equality and congruence | `l64-native/tests/equality.rs` |
| `incremental-closure-green` | context-relative closure | `l64-native/tests/closure.rs` |
| `native-upper-projection-green` | read-only upper views | `l64-projection/tests/projection.rs` |
| `legacy-authority-island-deletion-green` | deleted compatibility authority | `scripts/verify-legacy-authority-island-deletion.sh` |
| `native-carrier-green` | native product contacts | `scripts/verify-native-carrier.sh` |
| `native-cli-hardening-green` | public command contact | `scripts/verify-cli-hardening.sh` |
| `native-process-contract-green` | statuses, diagnostics, bounded transport | `scripts/verify-process-contract.sh` |
| `native-scale-green` | measured performance | `scripts/verify-native-scale.sh` |
| `documentation-coherence-green` | sole-document reconstruction | `scripts/verify-documentation-coherence.sh` |
| `golden-portability-green` | host-invariant product behavior | `scripts/verify-golden-portability.sh` |
| `residue-remote-proof-green` | residue and source-bound host proof | documentation and receipt gates |
| `workspace-green` | complete source boundary | tests, Clippy, formatting, ShellCheck |

No bulk promotion is allowed. A historical gate name, prior test, snapshot, or
narrative claim cannot authorize current behavior by resemblance.

## Historical and source boundary

The legacy theorem, campaign, research, registry, selector, policy, tower,
administration, runtime, packet, surface, and compatibility execution island
was externally recovered and deleted. No compatibility dispatcher remains.
The token `legacy` is a rejection tombstone only.

Historical implementations and superseded documents remain recoverable through
Git history and forensic source archives. `changelog.log` consolidates their
development causality. Historical familiarity is not evidence and cannot
authorize reintroduction.

Current live source consists of the eleven packages listed above, the scripts,
golden fixtures, samples, CI workflows, Cargo manifests, this document,
`changelog.log`, and `LICENSE`. Generated caches, unclassified JSON, reports,
projection artifacts, binaries, and archives are not source authority.

## Documentation and promotion law

1. `LOCUS64.md` is the only tracked Markdown document.
2. Current behavior, commands, package membership, gates, portability, usage,
   and release information must be reconstructible from this file.
3. Historical material belongs in `changelog.log` or Git history, not a second
   Markdown authority.
4. Every command listed here must appear in live CLI help, except wrapper-local
   `authority-audit`.
5. Current scripts may not invoke deleted crates or removed commands.
6. Documentation validation remains a direct check over this file, Cargo
   membership, scripts, workflow contacts, and live help. It may not become a
   document registry.

A new architectural law becomes current only when:

1. its operational contact is named;
2. executable evidence is identified;
3. dependent laws are preserved or explicitly replaced;
4. the relevant direct gate passes;
5. full locked-offline tests and Clippy pass;
6. this document and `changelog.log` are updated in the same change.

Documentation alone cannot promote behavior. Passing code cannot leave
contradictory current documentation behind.

## Release record: v0.1.3

Version `v0.1.3` is the first release of the dependency-free native authority
system. It replaces the unversioned May 2026 build rather than retaining its
legacy execution paths.

Net improvements:

- replaced the document-oriented Math Framework workspace with the eleven
  dependency-free native packages;
- established deterministic RNA, bounded DNA, fixed-point reconstruction,
  exact diagnostics, typed operations, and streaming transport;
- added proof-carrying operations, equality congruence, context-relative
  closure, duplex burdens, scoped evaluators, and deterministic policy law;
- replaced the legacy dispatcher with direct native commands and stable process
  outcomes;
- added verified certification, observation, change, projection, release, and
  transport products without granting them authority;
- deleted Q surfaces, JSON authority, legacy research/campaign/registry/tower
  paths, caches, overlays, pseudo-lineage, and presentation-driven routing;
- added bulk construction, indexed routes, shared closure, exact equality
  traversal, resource bounds, and compact/performance build profiles;
- added golden fixtures, cross-platform tests, source-bound portability
  receipts, conformance gates, release checksums, and reproducible GitHub-native
  packages;
- consolidated project history into `changelog.log` and all current Markdown
  guidance into this file.

Release assets:

- `locus64-v0.1.3-windows-x86_64-compact.zip`;
- `locus64-v0.1.3-windows-x86_64-perfopt.zip`;
- `locus64-v0.1.3-linux-x86_64-compact.tar.gz`;
- `locus64-v0.1.3-linux-x86_64-perfopt.tar.gz`;
- `SHA256SUMS.txt`.

Legacy executables, Q surfaces, JSON bundles, registry objects, reports, caches,
and research documents are not accepted as authority inputs. New authority must
enter as canonical RNA or validated DNA. Projection and inspection outputs
cannot be recompiled as source.
