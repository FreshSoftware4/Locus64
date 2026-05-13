# Locus64 Escalating Linear Execution Rail

This document is the authoritative compounding change chain for Locus64.

It replaces:

- the older upper-stack-first rail
- the earlier narrative-only RNA/DNA rail
- the weaker “closed spec but not clearly escalating” form

This version is intentionally:

- escalating:
  each phase makes the next phase lawful, cheaper, and narrower
- linear:
  there is one primary rail and one promotion direction
- compounding:
  outputs are ledgered artifacts that reduce later work instead of isolated wins
- substrate-subordinate:
  lower machine bands close before upper bloom is allowed
- Rust-ready:
  each phase is a closed state transition contract with enforceable outputs
- end-to-end:
  the rail runs from symbolic ingestion through machine execution into governed reconnect

## Governing sentence

Humans author symbolic RNA, splice it into explicit RNA, compile it into compact DNA, and verify DNA by execution under kernel authority.

## Conversation anchors

This rail is pinned to the tail-end conclusions from the performance conversation:

- `RNA -> Normalize -> SSR -> Kernel Object Graph -> Canonical Form`
- `SSR` must remain trivial:
  - single-pass
  - no backtracking
  - no inference
  - no optimization
  - no persistence
- structural encoding remains the primary DNA substrate
- codebooks are rejected as semantic authority
- the true missing closure seam was:
  - phase transition semantics
  - compounding ledger
  - closure gate
- the last unstable lower layer is:
  - RNA normalization
  - SSR emission
  - canonicalization

Everything above those lower layers must remain subordinate until they close.

## Mission constraint

The mission is not:

- wider warm-host bloom
- more tower categories
- CLI sprawl
- packet cleverness for its own sake

The mission is:

- close the lower compiler chain
- make closure executable rather than aspirational
- reconnect existing upper artifacts to verified lower truth
- stop broadening whenever the next step ceases to reduce ambiguity or future cost

## Rust-native compounding law

Every new substrate must reduce future code edits in at least one of these ways:

- one owning enum/table replaces repeated local match ladders
- one trait or typed contract replaces repeated ad hoc constructor logic
- one generated artifact family replaces repeated hand-maintained projections
- one test harness verifies a family of cases instead of one example

If a new abstraction does not reduce real repeated work in the current codebase, it stays out of the rail.

### Preferred Rust shape

- closed enums for phase ids, token classes, opcodes, artifact classes, exactness classes, and failure kinds
- small immutable structs for receipts, ledger entries, token streams, graph nodes, and reuse decisions
- pure functions for phase transforms
- explicit result types for every fallible boundary
- builder helpers only where they prevent constructor drift across crates
- generated tables only when they are derived from a smaller authoritative grammar

### Generator admission rule

A generative substrate is allowed only when all are true:

1. the repeated burden already exists in the codebase
2. the generator has a smaller authoritative input than its emitted outputs
3. generated outputs are marked, testable, and reproducible
4. manual override pressure is measured rather than hidden
5. the generator cannot promote its own output to kernel authority

Allowed generator-owned seams:

- token classification table -> tokenizer + token tests
- structural opcode registry -> DNA codec dispatch + opcode tests
- phase contract registry -> ledger validators + closure tests
- report/reuse receipt schema -> report lowering + export/observe projections
- research host grammar -> governed research artifacts
- coverage/reuse grammar -> reuse decisions + residual verification receipts

Rejected generator seams:

- generators that define semantic truth
- generators that bypass closure gates
- generators that emit domain-heavy math hosts before the sealed computational contract exists
- generators whose output cannot be reproduced from committed inputs

## Potential user complaints and required rail responses

The rail must proactively close the most likely trust-breaking complaints.

### Complaint: “I cannot tell what is exact, approximate, proved, or only computed”

Required response:

- every execution-facing artifact must declare:
  - `Exact`
  - `Approximate`
  - `WitnessBacked`
  - `CounterexampleCandidate`
  - `Undischarged`
- no numeric output may be reported as proof by implication
- disproof candidates must remain distinct from verified contradictions under kernel authority

### Complaint: “A later stage can silently corrupt an earlier invariant”

Required response:

- all multi-stage execution paths must be sealed operator chains
- no shared mutable stage state across phase boundaries
- each boundary must emit:
  - typed input summary
  - typed output summary
  - invariant verdict
  - rollback pointer

### Complaint: “It keeps re-verifying the same thing and wastes time”

Required response:

- repeated verification may be skipped only through lawful reuse
- lawful reuse requires:
  - canonical identity match or declared transport-equivalence
  - closure-valid lineage
  - replay legality check
  - matching or explicitly admissible policy/tolerance envelope
- skipped work must still emit a reuse receipt
- “faster because we trusted a cache” is forbidden

### Complaint: “Replay and benchmarking are too weak to trust difficult computations”

Required response:

- computational runs must emit deterministic replay receipts where possible
- when exact replay is not possible, bounded nondeterminism must be declared explicitly
- benchmark and torture artifacts must preserve:
  - input lineage
  - execution regime
  - tolerance contract
  - failure envelope

### Complaint: “The system cannot distinguish proof witnesses from numerical evidence”

Required response:

- witness artifacts and numeric evidence artifacts must remain separate families
- promotion gates may combine them, but may not collapse them into one status silently
- exact symbolic closure must always outrank approximate numerical support

### Complaint: “Forbidden lookup semantics are sneaking back in through runtime interners or symbol tables”

Required response:

- local interning is allowed only as compression or indexing support
- no interner, symbol table, or packet section may be required to reconstruct semantic meaning
- any local symbol support must be:
  - optional
  - local in scope
  - removable without semantic loss

### Complaint: “It is still better at architecture than at actually solving or checking hard math”

Required response:

- difficult math flows must become first-class sealed execution chains
- stage-local invariants, witnesses, replay traces, and counterexample candidates must be emitted as artifacts
- the rail must improve:
  - solving flow
  - checking flow
  - proving/disproving flow
  - auditability
  without requiring domain-specialized kernel mutation first

## Global closure law

A phase is executable iff it is a closed state transition system.

```text
PHASE := (
  InputState,
  Transform,
  Invariants,
  OutputState,
  FailureModes,
  ValidationGate,
  PromotionCondition,
  Rollback
)
```

No silent transitions.
No doctrine-only phases.
No upper-phase promotion without a committed ledger entry.

## Authority model

The following are hard constraints:

- only the **Locus Kernel** is semantic authority
- only `.rna` and `.dna` are public representation surfaces
- AST-equivalent structure exists only as an **ephemeral SSR**
- no semantic lookup layer
- no codebooks
- no global alias systems
- opcodes are structural only
- bitfields are control metadata only
- projections are derived only
- extension is hint, header is truth
- upper growth remains blocker-driven only

### Enforcement rule

Any semantic resolution must originate from structural transformation, never from external lookup.

## Core execution chain

```text
RNA -> RNORM -> SSR -> KGRAPH -> CNORM -> DNA -> EXEC -> RECONNECT
```

Where:

- `RNA` = human-authored symbolic surface
- `RNORM` = deterministic normalized RNA
- `SSR` = ephemeral structural resolution
- `KGRAPH` = kernel-ready structural graph
- `CNORM` = canonical graph + canonical identities
- `DNA` = machine structural encoding
- `EXEC` = verification/certification under kernel authority
- `RECONNECT` = research-host and tower reattachment to verified lineage

## Escalation law

The rail is not merely ordered. It escalates.

Each promoted phase must satisfy all three conditions:

1. **Closure escalation**
   it removes a lower ambiguity that previously forced interpretation
2. **Artifact escalation**
   it emits a new stable artifact class that later phases can consume directly
3. **Cost escalation**
   it lowers the branching factor or implementation burden of every remaining downstream phase

If a phase does not satisfy all three, it is not yet worthy of promotion.

## Compounding ledger

Every phase transition must emit exactly one ledger entry.

```text
CHANGE_LEDGER_ENTRY := (
  phase_id,
  input_state_hash,
  output_state_hash,
  dependency_edges,
  invariant_checks,
  failure_records,
  validation_result,
  promotion_signal,
  rollback_pointer
)
```

### Ledger rules

- no phase promotion without a committed entry
- no downstream phase may consume an output without its ledger entry
- failure records must be explicit and typed
- rollback pointer may be minimal, but it must exist
- ledger truth outranks ad hoc comments, debug prints, or prose-only status
- downstream artifacts must preserve lineage to the ledger entries they depend on

## Closure kernel

```text
PHASE_VALID :=
  all_invariants_hold
  AND failure_records == empty
  AND ledger_entry_exists
  AND output_is_consumable_by_next_phase
```

If `PHASE_VALID == false`, propagation halts.

No “mostly done” promotion.
No interpretive carry-forward.

## Global failure state

When closure fails, the system enters:

```text
SYSTEM_FAILURE_STATE := (
  failing_phase,
  disposition,
  failure_records,
  rollback_pointer,
  last_committed_ledger_entry
)
```

### Allowed dispositions

- `HaltPropagation`
- `RollbackLocal`

There is no silent partial success state.

## Artifact ladder

The rail escalates by artifact stabilization:

1. `RNA_ARTIFACT`
2. `RNORM_ARTIFACT`
3. `SSR_RECEIPT + KGRAPH`
4. `CNORM_ARTIFACT`
5. `DNA_ARTIFACT`
6. `EXEC_REPORT`
7. `LINEAGE_ARTIFACT`
8. `RESEARCH_RECONNECT_ARTIFACT`
9. `TOWER_REUSE_ARTIFACT`

Every rung must be:

- serializable where appropriate
- ledger-linked
- consumable by the next rung without interpretation drift

## Promotion ladder

Each phase has exactly four state outcomes:

- `Unstarted`
- `Active`
- `ClosureBlocked`
- `Promoted`

Promotion is only lawful when:

- the phase’s validation gate passes
- the closure kernel evaluates true
- the ledger entry is committed
- the next phase can consume the produced artifact without ad hoc repair

## Rail graph

The execution rail is linear, but compounding:

1. `P0` Tokenization Closure
2. `P1` RNORM Closure
3. `P2` SSR Closure
4. `P3` CNORM Closure
5. `P4` DNA Compaction and Freeze
6. `P5` DNA Rollout and Surface Tightening
7. `P6` Execution / Certification Ledger Closure
8. `P7` Research Reconnect Closure
9. `P8` Reuse / Coverage / Tower Closure

## Computational pressure-test verdict

The Julia pipeline pressure-test is useful because it exposes a general substrate need:

- sealed staged computation
- invariant-gated composition
- proof/disproof separation
- deterministic replay where possible
- stronger numeric audit trails

### What Locus64 can already do better than a plain Julia script

- preserve phase lineage and replay-oriented receipts
- separate semantic authority from execution convenience
- attach typed witnesses to specific execution bands
- halt promotion when closure gates fail
- reconnect higher artifacts through provenance rather than trust by convention

### What Locus64 does not yet do better than Julia

- high-throughput numerical linear algebra
- sparse spectral computation at mature library depth
- Fourier / eigen / Hessian / RG style stage families as reusable sealed hosts
- automatic proof/disproof bridging from numerical evidence into kernel-governed verdicts

### Correct conclusion

The mission is not to replace Julia as a general numerical runtime overnight.

The worthwhile improvement is to make Locus64 better at:

- structuring difficult mathematical computations as sealed operator chains
- verifying stage-local invariants
- emitting witnesses, evidence, replay traces, and counterexample candidates
- governing when numerical support is strong enough to promote, block, or request more exact proof

This strengthens the general substrate and can later host physics-like, optimization-like, or algebraic computational bands without rewriting the kernel around one domain.

## Subordinate computational host extension

This extension is subordinate to the main rail. It does not add a second primary mission.

It exists to ensure the main rail can support hard staged computations more robustly than ad hoc scripts.

### CH0 — Sealed computational stage contract

Any heavy computation admitted above `P4` must declare:

- typed inputs
- typed outputs
- local invariants
- failure envelope
- replay contract
- exactness class
- tolerance contract if approximate

This applies to future stage families such as:

- lattice/geometry construction
- operator assembly
- gauge normalization
- Hessian assembly
- spectral analysis
- propagator/scattering reduction
- RG trend extraction

### CH1 — Execution witness and evidence split

Every admitted computational stage must emit one or more of:

- `ExecutionWitness`
- `NumericEvidenceReceipt`
- `CounterexampleCandidate`
- `ReplayTraceReceipt`

Rules:

- witnesses describe stage-closed structural success
- evidence receipts describe measured or computed support
- counterexample candidates describe possible falsifiers
- none of them alone become semantic authority

### CH2 — Approximation discipline

Approximate computation must be explicitly governed.

Required fields:

- tolerance model
- stability class
- conditioning notes
- backend dependency notes
- reproducibility grade

No approximate result may silently inhabit an exact artifact family.

### CH3 — Proof/disproof bridge

The computational host must support three distinct downstream outcomes:

- `SupportsClaim`
- `FailsToSupport`
- `SuggestsCounterexample`

Only the kernel may promote any of these into:

- certified proof support
- explicit block
- governed disproof

This keeps numerical evidence useful without letting it impersonate semantic closure.

### CH4 — Backend-subordinate optimization

Performance work is allowed only when subordinate to the rail:

- sparse backends
- FFT backends
- linear algebra accelerators
- external solver interop

but only if:

- stage contracts remain sealed
- receipts remain reproducible
- backend choice cannot change semantic meaning silently
- fallback behavior remains explicit

### CH4.5 — Verification amortization and reuse

The system should aggressively avoid redundant work, but only by proving reuse is lawful.

Allowed fast paths:

- exact canonical identity reuse
- subsuming proof coverage reuse
- transport-equivalent witness reuse
- replayable obligation reuse
- stage-local computational reuse where inputs, tolerances, and invariants match

Required gates:

- canonical id equality or declared equivalence witness
- lineage continuity
- invariant compatibility
- replay legality
- cache policy legality
- approximation compatibility when approximate evidence is involved

Required artifacts:

- `ReuseLegalityReceipt`
- `ReuseDecisionReceipt`
- `ResidualVerificationReceipt`

Rules:

- reuse may skip recomputation, not accountability
- any residual unchecked obligations must remain explicit
- approximate evidence may never discharge an exact obligation silently
- a failed reuse gate must fall back to full verification, not partial trust

### CH5 — Domain-general adoption rule

A new computational host seam is worth adding only if it strengthens more than one future domain.

Accepted examples:

- sealed graph/operator assembly
- spectral decomposition witnesses
- replayable approximate execution
- counterexample candidate extraction
- exact/approximate bridge discipline

Rejected examples:

- one-off physics-specialized kernel mutation
- domain-specific fast path that bypasses lineage, witnesses, or closure gates

## Phase 0 — Tokenization Closure

Implementation status:

- landed in `mf-core` as the first ledger-visible lower-chain phase
- public artifacts now include `TokenClass`, `TokenClassSpec`, `TokenMapEntry`, `RnaToken`, `TokenStream`, `TokenizationIssue`, and `TokenizationReceipt`
- `execute_lower_chain` now runs `Tokenization -> RNORM -> SSR -> CNORM`
- workspace tests verify Unicode-preserving tokenization, control-character rejection, tokenization ledger insertion, and compile/sequence ledger parity
- remaining optional cleanup:
  factor the landed token substrate into a smaller file/module only if `mf-core/src/lib.rs` splitting becomes an active maintenance bottleneck

```text
InputState:
  raw RNA bytes

Transform:
  explicit byte / character classification
  direct token emission
  token artifact stabilization
  generator-owned token table extraction

OutputState:
  TOKEN_STREAM
```

### Invariants

- every input byte is classified deterministically
- token classes remain disjoint
- no keyword lookup is required in the hot path
- token emission remains O(n)

### FailureModes

- invalid byte class
- ambiguous direct mapping
- tokenizer requiring semantic lookup to proceed

### ValidationGate

- tokenization is deterministic across repeated runs
- token output is sufficient for RNORM without reparsing raw bytes

### PromotionCondition

- RNORM consumes token stream directly
- token artifact is stable enough to become the only lower-band ingest input
- tokenizer implementation and tokenizer tests derive from one authoritative token-class table

### Rollback

- retain raw RNA bytes
- do not mutate source RNA during tokenization

### Rust substrate

- target split:
  - `tokenizer.rs`
- generator-owned source:
  - `TokenClassSpec`
  - `TokenMapEntry`
  - `TokenizationFixture`

### Escalation payoff

- removes raw-byte ambiguity from the active lower chain
- gives RNORM a stable, explicit substrate instead of folded lexical behavior
- future token additions touch one table instead of tokenizer, docs, tests, and diagnostics separately

## Phase 1 — RNORM Closure

Implementation status:

- RNORM now consumes explicit `TokenStream` through `normalize_token_stream`
- `RnaNormalizationReceipt` records the consumed token stream id and shorthand-elimination status
- reusable `RnormFailureKind` and `RnormDiagnosticSpec` now own the diagnostic/failure seam
- lower-chain validation checks token -> RNORM grounding instead of allowing hidden lexical behavior

```text
InputState:
  TOKEN_STREAM

Transform:
  splice normalization
  shorthand expansion into explicit structural form
  separator and grouping normalization

OutputState:
  RNORM_RNA
```

### Invariants

- no semantic mutation
- deterministic splice ordering
- normalization remains O(n)
- all shorthand eliminated before SSR

### FailureModes

- ambiguous tokens
- malformed splice graph
- grouping failure
- arity mismatch
- nondeterministic normalization

### ValidationGate

- `hash(RNORM(RNA))` is deterministic across runs
- invalid RNA fails with explicit structural diagnostics
- no hidden tokenization remains inside normalization

### PromotionCondition

- corpus-wide RNORM stability passes
- all known shorthand-bearing inputs lower deterministically
- token -> RNORM boundary is mechanically sharp in code

### Rollback

- preserve token stream and raw RNA
- no in-place rewrite without explicit confirmation

### Output artifact

- `RNORM_ARTIFACT`
- `RNORM_RECEIPT`

### Rust substrate

- `mf-core::normalize_rna`
- target split:
  - `rna_lexer.rs`
  - `rna_normalizer.rs`
- generator-owned source:
  - `RnormFailureKind`
  - `RnormDiagnosticSpec`
  - `RnormFixture`

### Escalation payoff

- SSR no longer has to solve lexical, byte-class, or shorthand ambiguity
- all later compiler work sees one homogeneous input class
- invalid-input handling becomes consistent enough to extend without rewriting CLI/user-facing diagnostics

## Phase 2 — SSR Closure

Implementation status:

- `SsrTransitionSpec` now defines the transition seam as table-owned structural behavior
- `SsrReceipt` records transition count and transition-table hash
- lower-chain validation enforces no persistence and no semantic inference in the transition table
- SSR remains ephemeral and is still discarded after canonical lowering

```text
InputState:
  RNORM_RNA

Transform:
  single-pass structural resolution
  stack-based graph construction
  strand/reference/group construction only
  transition-table-owned state handling

OutputState:
  SSR_GRAPH
```

### Invariants

- no persistence
- no semantic inference
- no reordering
- no optimization
- no backtracking
- O(n), constant-factor small

### FailureModes

- stack underflow
- arity violation
- unresolved grouping boundary
- graph persistence leak
- semantic enrichment

### ValidationGate

- identical RNORM input yields identical SSR output
- SSR state is discarded after output generation
- no public export path surfaces SSR as authority

### PromotionCondition

- SSR receipts are emitted
- SSR artifacts remain ephemeral across CLI, cache, and persistence paths
- SSR transition behavior is defined by one table and verified by generated transition fixtures

### Rollback

- discard SSR output
- retain RNORM input and receipt only

### Output artifact

- `KGRAPH`
- `SSR_RECEIPT`

### Rust substrate

- `mf-core::resolve_spliced_rna`
- target split:
  - `ssr.rs`
  - `kernel_graph.rs`
- generator-owned source:
  - `SsrTransitionSpec`
  - `SsrInvariantSpec`
  - `SsrFixture`

### Escalation payoff

- canonicalization now operates on an explicit structural machine output rather than syntax
- the kernel boundary becomes mechanically sharp
- structural parser changes become table updates plus generated invariant tests, not scattered parser edits

## Phase 3 — CNORM Closure

Implementation status:

- `CanonicalRuleSpec` now owns the canonical-rule seam
- `CnormReceipt` records rule-table hash and idempotence status
- lower-chain validation checks deterministic lookup-free canonical rules
- canonical hashes remain the lower identity consumed by DNA, research, and reuse layers

```text
InputState:
  KGRAPH

Transform:
  canonical graph normalization
  structural ordering normalization
  context normalization
  canonical identity hashing
  canonical-rule-owned equivalence collapse

OutputState:
  CNORM_GRAPH
```

### Invariants

- structural equivalence collapses deterministically
- formatting variation disappears
- lawful splice variation disappears
- canonical hashing remains stable
- canonicalization is idempotent

### FailureModes

- canonical drift
- hash instability
- undefined equivalence
- inconsistent context

### ValidationGate

- structurally equivalent graphs hash identically
- repeated canonicalization is idempotent

### PromotionCondition

- equivalence classes converge under tests
- canonical identities are consumable by packet, research, and tower layers
- canonicalization rules are registered once and generate idempotence/equivalence fixtures

### Rollback

- discard unstable canonical outputs
- retain KGRAPH and failure receipt

### Output artifact

- `CNORM_ARTIFACT`
- `CNORM_RECEIPT`
- `CANONICAL_ID`

### Rust substrate

- `mf-core::canonicalize_ssr_graph`
- target split:
  - `canonicalizer.rs`
  - `hash.rs`
- generator-owned source:
  - `CanonicalRuleSpec`
  - `CanonicalFixture`

### Escalation payoff

- DNA encoding no longer needs to solve structural variance
- upper reconnect phases gain stable lineage keys
- new equivalence rules gain deterministic tests and hash fixtures by construction

## Phase 4 — DNA Compaction and Freeze

Implementation status:

- `StructuralOpcodeSpec` now owns the structural opcode registry seam
- `DnaHeaderReceipt` and `DnaValidationReport` validate header truth, structural opcode law, and symbol-table non-authority
- `mf-locus::compile_rna_to_dna_packet` rejects invalid DNA packets before emission
- compile/sequence artifacts now expose DNA header and validation receipts

```text
InputState:
  CNORM_GRAPH

Transform:
  encode canonical graph into structural machine artifact
  remove unnecessary readable payload dependence
  constrain optional symbol/string support to compression-only
  opcode-registry-owned codec dispatch

OutputState:
  DNA_ARTIFACT
```

### DNA header contract

```text
DNA_HEADER := (
  artifact_class,
  surface,
  grammar_id,
  structural_format_version,
  integrity_hash,
  strand_manifest,
  feature_flags,
  authority_tier,
  root_subject_id
)
```

### Section contract

```text
[OPCODE][LENGTH][PAYLOAD]
```

### Invariants

- no semantic lookup allowed
- no codebook authority
- no global alias authority
- opcodes describe structure only
- bitfields encode control only
- header truth outranks extension
- readable text is not required for semantic reconstruction

### FailureModes

- hidden semantic dependency
- missing structural header fields
- structure requiring lookup for meaning
- symbol-table opcode used semantically
- DNA packet remaining payload-readable for meaning rather than debug/support only

### ValidationGate

- `decode(DNA)` reconstructs the canonical graph
- unknown incompatible opcodes fail deterministically
- unknown compatible opcodes skip safely

### PromotionCondition

- reversible encoding is proven
- `.dna` artifacts carry complete structural truth
- structure remains reconstructible if optional string/symbol support is removed
- opcode encode/decode dispatch is generated from one structural opcode registry

### Rollback

- preserve CNORM graph
- reject invalid packet emission

### Output artifact

- `DNA_ARTIFACT`
- `DNA_HEADER_RECEIPT`

### Rust substrate

- `mf-core::LocusPacketHeader`
- `mf-core::encode_locus_packet`
- `mf-core::decode_locus_packet`
- `mf-locus::compile_rna_to_dna_packet`
- `mf-locus::sequence_dna_to_rna`
- generator-owned source:
  - `StructuralOpcodeSpec`
  - `DnaSectionSpec`
  - `DnaCodecFixture`

### Repo-specific constraint

`LocusOpcode::SymbolTable` may survive only as:

- optional local compression support
- non-semantic
- non-authoritative
- unnecessary for meaning reconstruction

### Escalation payoff

- machine execution becomes stable against surface noise
- report/replay/persistence can all share one authority substrate
- canonical identities become strong enough to support lawful reuse rather than blind cache hits
- codec evolution becomes registry-driven and test-generated instead of hand-synchronized across encoder, decoder, docs, and fixtures

## Phase 5 — DNA Rollout and Surface Tightening

Implementation status:

- `.dna` compile/sequence path remains active and `.locus` legacy decode remains readable as compatibility fallback
- DNA validation now runs at compile emission, making header truth outrank extension hints in the active path
- roundtrip tests assert ledger parity and DNA validation success
- remaining work is CLI/help-text cleanup and broader artifact-classification table generation, not core packet validity

```text
InputState:
  DNA_ARTIFACT + legacy .locus compatibility path

Transform:
  dual-write / dual-read migration
  semantic equivalence enforcement
  surface-command behavior derived from artifact classification

OutputState:
  stable .dna machine surface
```

### Invariants

- `.dna` and `.locus` remain semantically equivalent during rollout
- no divergence is tolerated
- `.dna` is the target machine surface
- CLI and surface behavior identify RNA/DNA truth consistently

### FailureModes

- format drift
- partial compatibility collapse
- header mismatch causing semantic divergence

### ValidationGate

- roundtrip `.dna` == canonical graph
- roundtrip `.locus` == canonical graph
- cross-compat import remains stable

### PromotionCondition

- parity test suite passes
- `.dna` becomes preferred emission target without breaking compatibility
- public surface behavior no longer depends on legacy surface metaphors
- import/export/classify command behavior is generated from one artifact-classification table where practical

### Rollback

- keep `.locus` read support
- revert default emission preference if parity breaks

### Output artifact

- `MIGRATION_RECEIPT`
- `PARITY_REPORT`

### Rust substrate

- `mf-research` dual persistence
- legacy decoder fallback in `mf-core`
- generator-owned source:
  - `ArtifactClassificationSpec`
  - `SurfaceCommandSpec`

### Escalation payoff

- machine substrate is now usable across the existing repo without a flag day
- later reconnect phases can rely on one primary executable surface
- parity and replay truth become strong enough to amortize repeated verification safely
- adding a new artifact class stops requiring parallel edits across CLI, surfaces, and research persistence

## Phase 6 — Execution / Certification Ledger Closure

Implementation status:

- `EvidenceExactness` separates exact, approximate, witness-backed, counterexample-candidate, and undischarged outputs
- `ExecutionClosureReceipt` makes execution/certification closure visible without merging proof witnesses and numeric evidence
- `mf-cert::derive_execution_closure_receipt` derives closure status from report verdicts, reuse visibility, residual visibility, and promotion-gate visibility
- lower-chain ledger remains the ground truth for RNA/DNA provenance

```text
InputState:
  stable lower RNA/DNA chain plus certifier execution

Transform:
  extend ledger, validation, and promotion enforcement above the lower chain
  make execution and certification artifacts closure-visible end to end
  report and receipt lowering from shared schema

OutputState:
  closure-governed execution / certification substrate
```

### Invariants

- lower-chain ledger remains authoritative
- certification and execution artifacts emit explicit closure-visible receipts
- promotion/validation is not local ad hoc logic outside the lower kernel
- reuse and skip decisions are surfaced rather than hidden

### FailureModes

- upper execution path without ledger linkage
- promotion decision without explicit gate result
- certification artifact emitted without closure-visible reuse / residual truth

### ValidationGate

- certification and execution artifacts carry committed closure-visible receipts
- reused vs recomputed work is reported explicitly
- upper phases can consume execution artifacts without inferred provenance

### PromotionCondition

- execution/certification outputs are closure-ready inputs for reconnect phases
- promotion gates are explicit enough to stop floating upper-phase logic
- report/export/observe projections use shared receipt schema rather than bespoke lowering paths

### Rollback

- keep lower-chain ledger as ground truth
- reject upper artifacts that bypass closure-visible execution lineage

### Output artifact

- `EXEC_REPORT`
- `EXECUTION_CLOSURE_RECEIPT`
- `REUSE_TRACE_RECEIPT`

### Rust substrate

- `mf-cert`
- `mf-runtime`
- `mf-cli`
- report / replay / execution receipts
- generator-owned source:
  - `ReceiptFamilySpec`
  - `ReportLoweringSpec`
  - `ClosureGateSpec`

### Escalation payoff

- reconnect no longer depends on partially implicit certifier/runtime behavior
- upper artifacts can be grounded in closure-visible execution truth
- adding a receipt family becomes a schema extension with generated report/export/observe coverage

## Phase 7 — Research Reconnect Closure

Implementation status:

- existing `ResearchLineageRecord` already carries canonical hash, lowering receipt id, phase ids, and phase ledger
- report-derived lineage already gates readiness/handoff flows through lineage requirements
- current changes strengthen upstream DNA/lower-chain receipts that research reconnect consumes
- remaining work is reducing repeated host/handoff constructors into grammar-owned helpers where it clearly lowers maintenance burden

```text
InputState:
  DNA-backed execution outputs

Transform:
  reconnect warm-host persistence to canonical lower truth
  emit research artifacts from a smaller host grammar where repeated patterns exist

OutputState:
  governed research substrate
```

### Invariants

- research artifacts trace to canonical semantic truth plus lowering receipts
- no floating report semantics
- producer-host derivation remains explicitly transitional until grammar-owned replacement lands
- lineage records exist for reconnect artifacts

### FailureModes

- orphaned warm objects
- report-shape dependency without canonical lineage
- persistence that bypasses lowering receipts

### ValidationGate

- all persisted research artifacts trace to DNA lineage and kernel truth
- `.dna`-backed persistence remains stable

### PromotionCondition

- traceability graph is complete enough for report-derived and route-derived warm artifacts
- reconnect becomes lineage-required rather than merely lineage-capable
- repeated producer/handoff/readiness artifacts are emitted from a grammar-owned source where it reduces manual drift

### Rollback

- preserve prior persisted artifacts
- reject new persistence paths that bypass lineage requirements

### Output artifact

- `LINEAGE_ARTIFACT`
- `RESEARCH_RECONNECT_ARTIFACT`

### Rust substrate

- `mf-research`
- current reconnect points:
  - `TaskEnvelope`
  - `DerivationSignature`
  - `ReviewReceipt`
  - `ChallengeRecord`
  - `ResearchLineageRecord`
  - producer-host and handoff derivations
- generator-owned source:
  - `HostSpec`
  - `OrganSpec`
  - `ContractGrammar`
  - `ResearchArtifactEmissionSpec`

### Escalation payoff

- upper artifacts become traceable products of verified execution
- tower work no longer floats above implied provenance
- warm-host additions stop duplicating host id, purpose, refs, readiness, handoff, and lineage logic by hand

## Phase 8 — Reuse / Coverage / Tower Closure

Implementation status:

- existing proof coverage dispatch already emits reuse legality, reuse decision, and residual verification receipts
- execution closure exactness now gives reuse/tower surfaces a sharper proof-vs-evidence distinction
- coverage/tower growth remains blocker-driven and lineage-required in existing readiness flows
- remaining work is generated projection coverage for reuse/tower reports if repeated projection edits become the active bottleneck

```text
InputState:
  governed research substrate

Transform:
  close lawful verification amortization
  reconnect coverage, payoff, distress, and tower growth to verified lineage and residual-obligation truth
  derive reuse/tower projections from closure-valid schemas

OutputState:
  closure-safe growth system
```

### Invariants

- reuse only from verified DNA lineage
- no speculative tower bloom
- no generator object depends on lookup-mediated meaning
- coverage fast paths must remain closure-valid under replay and equivalence law

### FailureModes

- ungrounded generator growth
- non-canonical reuse
- tower artifact detached from closure-valid lineage
- skipped verification without lawful reuse receipts

### ValidationGate

- tower artifacts trace to canonical semantic identities and DNA-backed execution
- coverage/payoff receipts remain replayable
- proof coverage reuse decisions remain explainable and reproducible
- residual verification remains explicit when reuse does not fully discharge obligations

### PromotionCondition

- reuse graph remains stable under load and torture runs
- coverage/reuse/tower projections are generated from closure-valid lineage and residual-obligation inputs where practical

### Rollback

- freeze tower promotion
- retain prior verified tower artifacts without widening the family

### Output artifact

- `TOWER_REUSE_ARTIFACT`
- `GROWTH_RECEIPT`

### Rust substrate

- proof coverage dispatch
- vertical compounding bundles
- distress / promotion candidate scaffolding
- generator-owned source:
  - `CoverageReuseSpec`
  - `ResidualObligationSpec`
  - `TowerProjectionSpec`

### Escalation payoff

- the system can compound from prior truth instead of rediscovering it
- upper growth becomes cheaper because it is constrained by lineage and closure
- tower growth becomes schema-produced from verified evidence instead of repeated custom bundle assembly

## Verification amortization law

Skipping redundant verification is feasible and worthwhile, but only under a closed law.

### Lawful skip conditions

A verification step may be skipped iff all are true:

1. the current subject has:
   - identical canonical identity
   - or a declared subsuming / transport-equivalent coverage relation
2. the reused artifact has closure-valid lineage
3. replay legality permits reuse in the active policy context
4. all invariants required by the skipped phase are already discharged or reduced to explicit residuals
5. any approximation/tolerance envelope is compatible with the current demand

### Required outputs

Every skip decision must emit:

- why reuse was lawful
- what was reused
- what residual work, if any, remains
- what full-verification path would have run otherwise

### Forbidden shortcuts

- extension-only cache hits
- symbol/interner-based semantic reuse
- approximate evidence silently satisfying exact proof obligations
- reusing artifacts across incompatible policy or tolerance envelopes
- skipping residual obligation checks because a parent artifact looked similar

### Best fit in the current repo

The substrate already contains the beginnings of this law:

- proof coverage dispatch
- reused artifact tracking
- replay legality checks
- replay barrier receipts
- replay merge receipts
- obligation cache shards
- replay divergence records

So this is feasible now.

The right next step is not inventing a new cache system. It is tightening the existing replay/coverage substrate until every reuse decision is:

- canonical-id aware
- lineage aware
- replay-law aware
- residual-obligation aware

## Cross-phase dependency law

The dependency graph is linear at the phase level and acyclic at the artifact level.

### Allowed dependency directions

- `P(n)` may depend only on:
  - outputs from `P(n-1)`
  - committed ledger entries from `P(<n)`
  - explicitly versioned compatibility artifacts

### Forbidden dependency directions

- lower phases depending on upper phases
- research reconnect defining lower compiler truth
- tower artifacts retroactively defining canonical identity

## Rail-wide stop rule

Work must stop escalating upward when either condition is true:

1. the next phase cannot satisfy all escalation laws
2. the next step would widen object families without reducing closure ambiguity or future cost

This keeps the rail from drifting into elegant but non-compounding expansion.

## Current implementation status

Already complete enough to remove from the active chain:

- RNA/DNA authority reset is in place
- lower-chain phase kernel exists for:
  - normalization
  - SSR
  - canonicalization
- `.dna` compile/sequence exists
- lineage artifacts exist
- lawful reuse receipts now exist on the certification / coverage path

Active completion gaps:

- tokenization is still folded into normalization rather than closed as its own band
- token classification is not yet owned by a smaller table/spec that can generate tokenizer tests
- SSR and CNORM exist, but their boundaries are not yet fully split into dedicated lower-band modules
- SSR and CNORM do not yet have table/spec-owned fixture generation
- DNA is still more payload-readable than the intended compact structural substrate
- DNA opcode/section behavior is not yet registry-generated from one structural source
- `.dna` rollout is still transitional alongside `.locus`
- execution / certification closure is stronger than before, but not yet uniformly enforced above the lower chain
- report/export/observe lowering still risks drift when new receipt families are added
- research reconnect is materially real, but still needs stronger lineage-required enforcement
- repeated research host/handoff/readiness emission is not yet owned by a compact grammar
- reuse / coverage / tower closure is only partly complete outside the current certification path
- coverage/tower projection logic is still more hand-assembled than schema-produced

## Immediate execution priority

The next highest-leverage seam is:

1. extract tokenization into an explicit lower-band artifact and table-owned module seam
2. keep RNORM but re-ground it on token stream input and reusable diagnostic/failure specs
3. harden SSR as a transition-table-owned structural machine boundary
4. harden CNORM around registered canonical rules and generated equivalence/idempotence fixtures
5. compact and constrain DNA through a structural opcode/section registry so readable payload is optional support, not practical meaning substrate
6. continue tightening execution / certification closure while moving report/export/observe lowering toward shared receipt-family schemas
7. make research reconnect lineage-required, then factor repeated research artifact emission into `HostSpec` / `OrganSpec` / `ContractGrammar`
8. tighten lawful verification amortization and close coverage / tower reuse with schema-produced reuse and residual verification projections
9. only then introduce `CH0` / `CH1` computational host contracts in one domain-general seam
10. defer domain-heavy numerical bands until the substrate can seal, replay, generate, and govern them cleanly

This stays faithful to the conversation tail:

- define the lower compiler boundary strictly
- stop expanding above unstable seams
- reconnect higher layers only through verified lineage
- skip repeated work only when canonical identity, lineage, replay legality, and residual obligations all line up
- use generation only where it replaces repeated Rust maintenance with smaller verified source truth
- let hard computations in only after the substrate can seal, replay, and govern them

## Generator execution order

Generative substrates must be introduced in dependency order, not wherever repetition is visible first.

1. `TokenClassSpec`
   - owns byte/token classification
   - emits tokenizer tables and tokenization fixtures
2. `RnormDiagnosticSpec`
   - owns normalization failure classes and repair loci
   - emits diagnostic fixtures and CLI-safe error payloads
3. `SsrTransitionSpec`
   - owns token-class-to-transition behavior
   - emits transition fixtures and invariant checks
4. `CanonicalRuleSpec`
   - owns declared structural equivalence and canonical ordering rules
   - emits idempotence and hash-stability fixtures
5. `StructuralOpcodeSpec`
   - owns DNA structural opcode/section behavior
   - emits encode/decode dispatch and codec fixtures
6. `ReceiptFamilySpec`
   - owns report/reuse/execution receipt families
   - emits report/export/observe lowering coverage
7. `ResearchArtifactEmissionSpec`
   - owns repeated research host/handoff/readiness emission
   - emits governed research artifacts from lineage-backed inputs
8. `CoverageReuseSpec`
   - owns lawful reuse, residual obligation, and tower projection generation
   - emits reuse/tower projections from closure-valid inputs

No generator later in this list may define inputs for an earlier one.

## Minimal Rust execution shape

The intended runtime reduction is:

```text
for each phase in linear_chain:
    output = phase.transform(input)

    assert invariants_hold(output)
    assert failure_records(output).is_empty()

    ledger.commit(phase, input, output)

    assert next_phase_can_consume(output)

    input = output
```

In practical crate terms, continued convergence should go toward:

- `mf-core`
  - phase contracts
  - receipts
  - ledger entry types
  - closure gate helpers
  - promotion gate helpers
- `mf-locus`
  - DNA encode/decode and phase receipts
- `mf-cli`
  - explicit phase tracing
  - contract-visible persistence commands
- `mf-research`
  - lineage-aware persistence
  - reconnect gating
- `mf-cert`
  - closure-valid execution + verification integration

## Acceptance condition for this rail

This rail is only successful when:

- lower bands (`RNORM`, `SSR`, `CNORM`, `DNA`) are phase-closed
- every transition emits a ledger entry
- promotion is gate-controlled rather than inferred
- research-host persistence traces to canonical lineage
- tower growth consumes only closure-valid outputs

Until then, no further upper-stack bloom should be treated as authoritative completion.
