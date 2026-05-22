# Locus64 Substrate Inversion Execution Rail

This document fully replaces the prior compounding change chain.

The prior rail correctly named the desired direction, but it treated too much of the existing lower chain as closed. The current codebase is better classified as transitional canonical graph persistence with RNA/DNA wrapping, not yet deterministic structural execution infrastructure.

The new mission is substrate inversion:

```text
from: symbolic graph persistence with machine-format wrappers
to:   deterministic structural execution substrate with semantic systems derived above it
```

## Governing Law

Locus64 must converge on this public model:

```text
RNA -> TOKENIZE -> RNORM -> SSR -> CNORM -> DNA -> EXEC -> RECONNECT -> REUSE
```

Every arrow is a Rust-enforced phase transition with typed input, deterministic transform, invariant checks, failure records, receipts, rollback semantics, conformance tests, and a promotion gate.

No phase is complete until the next phase can consume its output without interpretation.

## Current Reality Classification

| Layer | Current state | Target state |
| --- | --- | --- |
| RNA surface | partial success | only symbolic authoring/input surface |
| RNORM | mostly real, likely string-heavy | byte/token-grounded deterministic normalization |
| SSR | too persistent and graph-like | ephemeral structural lowering machine |
| CNORM | partial graph canonicalization | canonical execution topology |
| DNA | proto-DNA graph packet | compact structural opcode artifact |
| Execution | graph/document-oriented in places | opcode/topology traversal |
| QC0/JSON/document persistence | drifted semantic authority | legacy import/export projection only |
| Research/cert/tower | extensive but too high-rooted | derived overlays keyed to DNA lineage |

## Non-Negotiable Architecture Laws

- Kernel authority remains lower than prose, reports, JSON, QC0, and document workflows.
- RNA and DNA are the only target public representation surfaces.
- SSR is temporary lowering state only. It must not be serialized, exported as authority, or reused as an identity layer.
- CNORM produces canonical execution topology, not a named semantic graph snapshot.
- DNA is structural binary authority. Human-readable semantic text must not be required for DNA execution or validation.
- Opcodes are structural. They select shape, traversal, arity, framing, and compatibility behavior; they do not encode ontology by lookup.
- Codebooks and symbol tables are allowed only as local deterministic compression/debug aids and must be removable without semantic loss.
- Semantic systems reattach only after lower-chain closure and only as derived structures keyed by canonical DNA lineage.
- Every compatibility path must be marked as compatibility, not current doctrine.

## Global Phase Contract

Each phase must implement this shape:

```text
Phase := (
  input_type,
  transform,
  output_type,
  invariants,
  failure_modes,
  rollback_pointer,
  receipts,
  conformance_tests,
  promotion_gate
)
```

A phase is valid iff:

```text
all invariants pass
AND no failure records remain
AND one immutable ledger entry exists
AND output is consumable by the next phase
AND conformance corpus passes
```

Failure halts propagation. Partial downstream artifacts are invalid.

## Phase 0 - Authority Quarantine And Inventory

Purpose: stop legacy authority leakage before rebuilding lower layers.

Rust deliverables:

- Classify modules and commands as `lower_substrate`, `derived_semantic`, `compatibility_projection`, or `legacy_authority_risk`.
- Add explicit authority flags to packet/report/import paths where absent.
- Mark QC0/JSON/document-derived workflows as non-authoritative compatibility or projection surfaces.
- Add a single audit command that reports active authority sources and fails if more than RNA/DNA/lower receipts claim substrate authority.

Invariants:

- No JSON/QC0/document object can become canonical authority without passing through the lower chain.
- No CLI command silently bypasses the phase ledger.
- Compatibility code is named as compatibility in help text and receipts.

Failure gates:

- Any non-DNA artifact accepted as machine authority.
- Any SSR object persisted as identity.
- Any semantic label required to validate machine structure.

Promotion condition:

- Authority audit passes in CI and torture tests.

Compounding payoff:

- Prevents upper-stack work from hardening the current inversion.

## Phase 1 - Freeze Token Algebra And RNA Grammar

Purpose: replace string-centric parsing assumptions with a fast, deterministic byte/token substrate.

Rust deliverables:

- `TokenClass`: `Atom`, `Binder`, `Op`, `GroupL`, `GroupR`, `Sep`, `StrandRef`, `Meta`, `Escape`, `Literal`.
- Fixed byte classification table with precedence.
- Explicit UTF-8 policy, Unicode normalization policy, invalid byte policy, control character policy, escape semantics, and string literal semantics.
- Frozen grammar for RNA term/sequence/group/operator/reference legality.
- Deterministic error codes and machine-readable diagnostics with source spans.
- Golden corpus for valid RNA, invalid bytes, escapes, grouping, separators, binders, strand references, and arity failures.

Invariants:

- Every byte is either classified or rejected deterministically.
- Tokenization is O(n), no semantic lookup, no alias resolution.
- RNORM receives tokens, not raw strings.

Failure gates:

- Ambiguous token boundary.
- Platform-dependent classification.
- Silent Unicode normalization.
- Unspecified control character accepted.

Promotion condition:

- Token golden corpus and fuzz corpus pass across repeated runs.

Compounding payoff:

- All later parsing, diagnostics, and canonicalization stop duplicating local string rules.

## Phase 2 - Freeze Structural Opcode Law

Purpose: make structure encodable before rebuilding SSR/CNORM/DNA.

Rust deliverables:

- `Opcode` namespace with reserved ranges, compatibility classes, versioning rules, arity table, and validation table.
- Core structural opcodes: `Leaf`, `Bind`, `Apply`, `Group`, `Seq`, `Ref`.
- Optional structural modifiers only where lawfully structural: `Commute`, `Assoc`, `Inline`, `ReduceHint`.
- Varint strategy, endianness guarantees, integer overflow policy, deterministic numeric encoding policy, and opcode decode failure policy.
- Generated or table-driven validation instead of scattered match ladders.

Invariants:

- Opcode meaning is structural, never semantic.
- Arity validation is table-driven and deterministic.
- Unknown compatible opcodes skip safely; unknown incompatible opcodes fail.

Failure gates:

- Semantic opcode.
- Opcode requiring symbol-table meaning.
- Platform-dependent numeric encoding.

Promotion condition:

- Opcode conformance tests validate packing, arity, compatibility, and failure behavior.

Compounding payoff:

- DNA, executor, validator, and diagnostics share one structural law.

## Phase 3 - Rebuild SSR As Ephemeral Structural Lowering

Purpose: replace persistent graph-like SSR with a streaming structural reducer.

Rust deliverables:

- Deterministic pushdown reducer over RNORM tokens.
- Bounded stack state with explicit maximum depth and stack exhaustion behavior.
- Temporary arena/lifetime model that prevents SSR identity persistence.
- Transition table for token class actions.
- Receipts that summarize lowering without exposing SSR as authority.
- Source-map emission for diagnostics without making source text part of machine authority.

Invariants:

- Single pass unless an explicit diagnostic recovery mode is requested.
- No backtracking, semantic inference, optimization, or persistence.
- SSR emits only the next structural input plus receipt metadata.

Failure gates:

- Stack underflow/overflow.
- Reference legality violation.
- Arity mismatch.
- SSR ID exported or stored as canonical identity.

Promotion condition:

- Tests prove SSR output determinism, no serialization path, and bounded failure behavior.

Compounding payoff:

- Removes the largest current inversion point.

## Phase 4 - Define True CNORM

Purpose: canonicalize execution topology, not presentation graphs.

Rust deliverables:

- `CanonicalTopology` type separate from legacy graph snapshots.
- Canonical ordering law for nodes, edges, references, groups, associative flattening, and commutative ordering.
- Structural equivalence partition rules and conflict handling.
- Canonical hash law with collision policy and deterministic serialization law.
- Idempotence tests: `CNORM(CNORM(x)) == CNORM(x)`.
- Cross-platform hash stability tests.

Invariants:

- Equivalent structures produce identical canonical IDs.
- Presentation labels, source strings, and debug names do not affect canonical identity.
- Canonicalization does not discover new semantics at runtime.

Failure gates:

- Hash drift.
- Text residue in canonical identity.
- Unspecified reference cycle or recursive structure behavior.

Promotion condition:

- Golden equivalence corpus converges and non-equivalence corpus remains separated.

Compounding payoff:

- Reuse, ledger, execution, research lineage, and certification can share one identity root.

## Phase 5 - Rebuild DNA As True Structural Encoding

Purpose: replace proto-DNA graph serialization with compact structural machine authority.

Rust deliverables:

- Fixed DNA header: magic, protocol version, artifact class, feature flags, section count, root subject, integrity digest, canonical topology digest.
- Section table with alignment rules, partial-read behavior, truncation behavior, corruption behavior, and forward/backward compatibility policy.
- Compact opcode stream over canonical topology.
- Deterministic local compression policy. String/debug sections optional and non-authoritative.
- Streaming decoder and staged validator: header, section table, opcode stream, topology digest, canonicality proof.
- `.locus` compatibility import remains readable but cannot be emitted as preferred authority.

Invariants:

- DNA reconstructs canonical topology without semantic string lookup.
- Human-readable aliases are optional debug/projection data only.
- DNA validation does not require RNA, QC0, JSON, or report text.

Failure gates:

- Required semantic string in DNA.
- Non-reconstructible encoding.
- Incompatible unknown section accepted.
- Digest/canonicality mismatch.

Promotion condition:

- `.dna` encode/decode/canonicality corpus passes; readable Unicode is absent from required structural sections.

Compounding payoff:

- Establishes actual machine substrate and removes graph-packet ambiguity.

## Phase 6 - Rebuild Execution Over DNA

Purpose: move execution from deserialized semantic graphs to structural opcode/topology traversal.

Rust deliverables:

- DNA executor/traverser over canonical topology and opcode stream.
- Explicit exact vs approximate evidence classification.
- Deterministic scheduling policy and deterministic multithreading rule.
- Resource budgets, sandbox boundaries, malicious DNA handling, structural bomb detection, hash-flood resistance, and memory exhaustion policy.
- Replay trace format keyed to DNA digest and canonical topology ID.

Invariants:

- Execution does not require graph/document reconstruction.
- Approximate evidence cannot satisfy exact obligations.
- Resource exhaustion fails closed with replayable diagnostics.

Failure gates:

- Hidden semantic lookup during execution.
- Platform-nondeterministic result for deterministic workload.
- Unsafe unbudgeted traversal.

Promotion condition:

- Executor conformance and torture suites pass over valid, invalid, adversarial, and replay cases.

Compounding payoff:

- Certification can become execution-native instead of document-native.

## Phase 7 - Phase Engine, Ledger, Replay, And Cache Law

Purpose: enforce the rail uniformly instead of scattering closure checks.

Rust deliverables:

- Global `PhaseEngine` that owns transition execution, invariant checks, ledger commit, rollback pointer, and downstream consumability checks.
- Immutable ledger format with storage backend, replay semantics, dependency edge derivation, DAG legality, cycle handling, transaction boundaries, interruption policy, and recovery semantics.
- Cache invalidation law keyed by canonical IDs, protocol versions, feature flags, and dependency edges.
- Incremental and streaming compilation semantics only after full replay correctness exists.

Invariants:

- Exactly one ledger entry per successful phase transition.
- Failed phases emit failure state and do not emit downstream artifacts.
- Reuse requires canonical ID match, valid lineage, replay permission, and invariant satisfaction.

Failure gates:

- Missing ledger entry.
- Mutable ledger rewrite.
- Cycle in dependency DAG unless explicitly legal and handled.
- Blind cache hit.

Promotion condition:

- Replay and rollback tests reproduce success/failure states deterministically.

Compounding payoff:

- Every later feature gets closure enforcement without custom plumbing.

## Phase 8 - CLI Phase Enforcement And Diagnostics

Purpose: make the executable interface obey the substrate rather than bypass it.

Rust deliverables:

- CLI commands mapped to explicit phase transitions.
- `compile-rna`, `inspect-dna`, `validate-dna`, `execute-dna`, `replay-ledger`, and `authority-audit` use the phase engine.
- Deterministic error codes, JSON diagnostics, human diagnostics, source maps, ambiguity diagnostics, dependency-aware repair hints, and trace output.
- Help text states RNA/DNA doctrine and labels legacy import/export paths.

Invariants:

- CLI trace equals phase trace.
- No command emits final artifacts from unpromoted intermediate states.
- Header truth outranks extension hint.

Failure gates:

- Noncanonical representation leaked as authoritative output.
- CLI bypasses ledger or validation.
- Ambiguous error without machine-readable failure code.

Promotion condition:

- CLI conformance tests pass and all public examples use RNA/DNA phase vocabulary.

Compounding payoff:

- Users interact with one coherent language and one coherent machine path.

## Phase 9 - Semantic Quarantine Migration

Purpose: downgrade legacy semantic persistence into projections and imports.

Rust deliverables:

- QC0/JSON/report object paths become import/export adapters, not authority roots.
- Existing semantic objects rekey to canonical topology ID and DNA digest.
- Research host objects reject floating reports without DNA lineage.
- Compatibility receipts record origin, lowering path, and projection status.

Invariants:

- No theorem, campaign, adequacy, route, bridge, operator, or coverage record is authority without canonical lineage.
- Semantic labels are projections over structure.
- Legacy data can be imported, challenged, or recompiled, but not silently promoted.

Failure gates:

- Orphan semantic object.
- Report-derived truth without DNA lineage.
- QC0/JSON treated as current public language.

Promotion condition:

- Existing seeded workflows pass through compatibility adapters and produce lineage-grounded artifacts.

Compounding payoff:

- Preserves valuable upper stack while removing document-oriented authority.

## Phase 10 - Reattach Certification, Adequacy, Research, And Tower

Purpose: rebuild upper value on the correct root.

Rust deliverables:

- Certification consumes execution-native receipts and canonical lineage.
- Adequacy clauses attach to DNA-backed obligations and evidence classes.
- Route, bridge, operator, proof-shape, replay, and coverage systems derive from canonical topology IDs.
- Tower/generator/distress systems become blocker-driven overlays and cannot create substrate authority.
- Promotion executor and promotion timing semantics are made explicit.

Invariants:

- Upper systems are derived, replayable, and challengeable.
- Promotion is evaluated by the kernel/phase engine, not by report text.
- Generator growth requires blocker signal and closure-valid lineage.

Failure gates:

- Semantic object persistence becomes load-bearing authority.
- Certification bypasses DNA execution.
- Tower growth not grounded in blocker or lineage.

Promotion condition:

- Seeded campaigns, imported claims, coverage reuse, and research reconnect all pass with DNA-backed lineage.

Compounding payoff:

- The sophisticated current upper stack survives as governed capability instead of corrupting the substrate.

## Phase 11 - Governance, Security, Migration, And Release Freeze

Purpose: make the protocol sustainable.

Rust deliverables:

- Protocol version freeze process, migration governance, deprecation semantics, feature-flag negotiation, extension registration, and capability isolation model.
- Unsafe Rust boundary policy, arena/allocation strategy, zero-copy guarantees, host/runtime API boundaries, plugin isolation, and untrusted execution policy.
- Formal docs: language spec, transition algebra, binary protocol, opcode law, canonicalization proof sketch, execution semantics, conformance document, threat model.
- Release gates: format conformance, fuzzing, torture test, cross-platform determinism, docs accuracy, migration tests.

Invariants:

- Extensions cannot alter substrate authority.
- Unsafe code is isolated and justified.
- Releases cannot ship protocol changes without conformance artifacts.

Failure gates:

- Unversioned protocol change.
- Undocumented unsafe boundary.
- Missing migration path for public artifact change.

Promotion condition:

- Release candidate passes conformance, torture, docs, and migration gates.

Compounding payoff:

- Future changes become protocol-governed rather than intuition-governed.

## Immediate Implementation Slice

The next path-optimized slice is:

1. Add the authority audit and mark QC0/JSON/report workflows as compatibility projections.
2. Freeze token classification and RNA grammar in Rust tables plus conformance corpus.
3. Freeze opcode law in a single Rust module with generated/table-driven validation.
4. Refactor SSR so its identities cannot persist and its receipts cannot become authority.
5. Introduce `CanonicalTopology` and route DNA validation toward it.
6. Replace DNA required structural sections with compact opcode/topology encoding while keeping legacy `.locus`/proto-DNA import.

This slice reduces future work because it replaces scattered local behavior with shared substrate tables and one phase engine path.

## Removed From The Old Rail

- Any claim that current DNA is final DNA.
- Any claim that current SSR is fully correct merely because receipts exist.
- Any implication that QC0 or JSON is a public representation peer to RNA/DNA.
- Any upper-stack expansion before lower substrate closure.
- Any release criterion based only on graph serialization roundtrip.

## Definition Of Done

The substrate inversion is complete when all of these are true:

- Required DNA structural sections contain no semantic text needed for validation or execution.
- SSR has no persistence/export path and cannot provide canonical identity.
- CNORM identity is independent of presentation strings and source formatting.
- Execution traverses DNA/canonical topology directly, not reconstructed semantic documents.
- Every public CLI command maps to a phase-engine transition and ledger entry.
- Semantic systems are rekeyed to canonical DNA lineage.
- Compatibility imports are clearly marked and cannot become authority without recompilation.
- Fuzz, conformance, torture, replay, migration, and cross-platform determinism tests pass.
