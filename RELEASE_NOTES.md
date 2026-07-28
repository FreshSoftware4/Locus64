# Locus64 v0.1.3

This is the first release of the dependency-free native Locus64 authority
system. It replaces the `unversioned` May 2026 build rather than preserving its
legacy execution paths.

## Complete improvement inventory

### Native authority substrate

- Replaced the document-oriented Math Framework workspace with an eleven-package
  dependency-free `l64-*` Rust workspace organized around native authority flow.
- Established canonical `L64R1` RNA and `L64D` DNA as the only public authority
  surfaces, with deterministic parsing, normalization, compilation, sequencing,
  and canonical reconstruction.
- Added precise source diagnostics with spans, locations, excerpts, and repair
  coordinates.
- Added compact structural storage with explicit contexts, dimensions, symbols,
  routes, constraints, typed operations, transactional validation, and
  receipted commits.
- Added canonical byte encoding, domain-separated identities, and bounded DNA
  decoding with fixed-point reconstruction, structural validation, tamper
  rejection, and resource limits.
- Added streaming `L64B` transport for ordered DNA members without creating
  composite bundle authority.

### Proof, equality, and closure

- Added proof-carrying native operations, kernel judgments, witness validation,
  and proof-producing equality congruence.
- Added exact and incremental closure evaluation with dependency indexes, open
  frontiers, context-sensitive state, and equivalence to full recomputation.
- Separated authored obligation intent from evaluated evidence and added duplex
  complement burdens, scoped evaluators, deterministic policy precedence, and
  explicit equivalence laws.

### Execution and command surfaces

- Replaced the legacy CLI dispatcher with one in-process native command membrane
  and stable process outcomes.
- Added `run-rna`, `compile-rna`, `sequence-dna`, `run-dna`, `certify-dna`,
  `observe-dna`, `inspect-dna`, comparison, bundle, and native release flows.
- Added the `l64` sibling wrapper with `authority-audit` and exact delegation to
  `l64-cli`.
- Added atomic output creation and refusal to overwrite existing authority or
  bundle files.
- Added direct non-persistent execution with exactness-aware witnesses and
  deterministic rendered outcomes.

### Derived products and transport

- Added read-only certification, observation, change, projection, release,
  transport, atlas, analysis, replay, report, research, source, set, and ordering
  products over native authority.
- Required derived products to verify against their source authority.
- Added native release export containing canonical RNA, canonical DNA, verified
  projection, and a release record with explicit authority roles.
- Added streaming bundle ingestion and member-at-a-time execution to bound
  retained memory.
- Classified reports, research records, views, and receipts as projections and
  prevented them from silently becoming source authority.

### Authority hardening and legacy removal

- Removed the QK/QA/QM/QC0 surface model and its JSON/document authority paths.
- Removed the legacy theorem, campaign, research, registry, selector, policy,
  tower, administration, runtime, and surface execution island.
- Removed legacy caches, plan storage, overlays, import receipts, pseudo-lineage,
  numeric-hash authority, and presentation-driven routing.
- Rejected deprecated schemas and non-source artifacts at every authority
  boundary; any remaining document input is explicitly migration ingress.
- Added authority-tier validation, packet resource bounds, and category-error
  rejection for reports and projections presented as source.
- Promoted only mechanically proven authority, substrate, duplex, public
  surface, migration, evaluator, and release gates into the constitution.

### Performance and resource behavior

- Added bulk RNA graph construction, indexed routes, shared closure analysis,
  exact equality traversal, preallocated decoding, and compact locality-oriented
  storage.
- Added measured native complexity budgets and scale tests.
- Added low-memory verification and streaming bundle checks.
- Kept the complete workspace dependency-free for deterministic offline builds.
- Added dedicated `compact` and `perfopt` Cargo profiles with LTO, one codegen
  unit, aborting panics, and stripped binaries.

### Portability, conformance, and release quality

- Added full-workspace tests on Linux, macOS, and Windows.
- Added exact source-bound portability receipts and combined receipt
  verification.
- Added golden RNA, DNA, execution, certification, observation, comparison,
  bundle, and native-release fixtures across certified, open, and invalid cases.
- Added LF/CRLF equivalence tests and paths containing spaces and Unicode.
- Added CLI, process, scale, documentation, dependency, authority-deletion,
  portability, and native-carrier conformance gates.
- Added deterministic source formatting through `.gitattributes` and
  rustfmt/clippy enforcement.
- Added source-reference manifests, historical ledgers, the executable
  development rail, native constitution, language specification, stack map,
  approval gates, and portability contract.
- Removed generated binaries, archives, caches, logs, and temporary artifacts
  from source authority and source releases.
- Added reproducible GitHub-native release builds for compact and
  performance-optimized Windows and Linux packages.
- Added SHA-256 release checksums, build provenance, bundled usage/language
  documentation, and the complete MIT license text.

## Breaking changes from `unversioned`

- Legacy `mf-*` executables and crates are gone.
- QC0 and the former Q-surface formats are not accepted.
- Legacy JSON bundles, registry objects, reports, caches, and research documents
  are not authority inputs.
- New authority must enter as canonical RNA or validated DNA.
- Projection and inspection outputs cannot be recompiled as source.
- Existing automation must use `l64-cli` or the adjacent `l64` wrapper.

## Release packages

Each package contains both `l64` and `l64-cli` plus this release record, the
usage guide, language specification, license, and non-authoritative build
provenance.

- `locus64-v0.1.3-windows-x86_64-compact.zip`
- `locus64-v0.1.3-windows-x86_64-perfopt.zip`
- `locus64-v0.1.3-linux-x86_64-compact.tar.gz`
- `locus64-v0.1.3-linux-x86_64-perfopt.tar.gz`
- `SHA256SUMS.txt`

The release page records every net improvement while collapsing intermediate
implementation passes that produced the same final result.
