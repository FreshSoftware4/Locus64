# Locus64 Linear Execution Rail

This is the authoritative linear execution rail for Locus64.

Compounding change chains are no longer separate top-level rails. A compounding change chain is a valid change to this rail that preserves the trajectory:

```text
syntax is transient
structure is authority
canonical form is identity
execution is lineage-bound
reuse is proven, not assumed
```

The first compounding change chain applied here is the consolidated unified substrate and execution specification. It replaces the prior substrate-inversion wording with a tighter system definition, phase model, and Rust implementation target.

## Rail Operating Model

A linear execution rail is a temporal dependency structure, not just an architecture document.

Its purpose is to prevent interruption, ambiguity, and drift from changing the implementation trajectory. The document must let a developer or agent resume from the last completed rail node, read only the local window needed for that node, and continue without rediscovering the architecture.

Each rail node must answer:

- what is the current input state?
- what exact codebase change happens next?
- what prior output does it depend on?
- what files or crates are likely touched?
- what invariant must become stronger?
- what test or inspection proves the node is complete?
- what downstream work becomes easier because this node landed?

The rail is intentionally verbose where ambiguity would otherwise be resolved during coding. It is still cheaper than letting every coding pass rediscover dependency order.

## Local Reading Window

When executing the rail, read only:

- the governing laws at the top of this file
- the current rail node
- the immediately previous completed node
- the immediately next planned node
- any referenced crate/file touched by the node

Do not reread the whole rail unless the trajectory changes.

## Change Chain Rule

A compounding change chain is valid only if it preserves or improves the temporal dependency structure.

Allowed change chains:

- split an oversized node into smaller ordered nodes
- insert a missing prerequisite before a blocked node
- remove a node that is already complete or made obsolete
- tighten an invariant without changing the public trajectory
- replace a vague step with Rust-specific file/crate actions
- add a conformance test that prevents future drift

Forbidden change chains:

- add upper-stack work before lower substrate closure
- introduce a parallel public surface model
- preserve obsolete compatibility formats without an actual ecosystem requirement
- promote compatibility formats into authority
- hide unresolved ambiguity in future implementation details
- widen scope without reducing downstream cost
- reorder phases without stating the dependency reason

Every accepted change chain must leave the rail more executable than before.

## Rail Node Template

Each implementation node should be shaped like this:

```text
Node:
  id:
  purpose:
  depends_on:
  code_targets:
  step_sequence:
  actions:
  invariants:
  tests:
  exit_condition:
  downstream_payoff:
```

If a node cannot be written in that form, it is still a concept, not executable rail material.

## Rust Development Optimization Rules

The rail must be executed in Rust-native increments, not architecture-sized rewrites.

Rules:

- Prefer type boundaries over prose boundaries: every important phase state should become a Rust type or enum before broad logic moves.
- Prefer newtypes for authority-sensitive IDs: `CanonicalId`, `DnaDigest`, `PhaseId`, `LedgerEntryId`, `LineageId`.
- Prefer table-driven law over scattered matches for token classes, opcodes, phase IDs, artifact classes, failure kinds, and exactness classes.
- Prefer `Result<T, E>` with domain errors over stringly `anyhow` at substrate boundaries; `anyhow` is acceptable at CLI edges.
- Prefer mechanical rename commits over mixed semantic commits.
- Prefer crate-local tests before workspace-wide tests during rename/deletion passes.
- Prefer `cargo check -p <crate>` after local edits, then `cargo test -p <crate>`, then workspace tests only at phase exit.
- Do not refactor `release/src`, `target`, or generated release payloads as source of truth.
- Do not remove a crate and rename its dependents in the same step unless the dependency graph proves it is isolated.
- Do not introduce macros/generators until the hand-written shape has repeated at least twice and the generator input is smaller than generated output.
- Do not add `unsafe` for parser/codec performance until conformance and fuzz tests exist around the safe version.

Adversarial audit checklist for every node:

- Could this node make later deletion harder?
- Could this node preserve an obsolete format out of inertia?
- Could this node hide semantic authority behind a debug field, label, or string table?
- Could this node make `l64` renaming harder by adding new `mf` references?
- Could this node create a second path around the phase engine?
- Could this node pass tests only because stale Q-surface samples still exercise the old architecture?
- Could this node edit generated release snapshots instead of source crates?
- Could this node broaden scope without reducing downstream ambiguity?

## 0. System Identity

Locus64 is a deterministic structural transformation substrate.

The system is designed for:

- deterministic transformation
- replayability
- machine-efficient execution
- human-editable symbolic interaction
- structural reuse and amortization
- canonical equivalence collapse
- semantically disciplined compilation

Core identity law:

```text
meaning := structure
```

Meaning does not come from labels, aliases, comments, strings, or codebooks.

## 1. Primary Ontology

Public surfaces:

```text
RNA := human interaction surface
DNA := machine authority surface
```

RNA is symbolic, editable, spliceable, and structurally recoverable.

DNA is compact, executable, reconstructive, and canonical.

Transformation law:

```text
RNA
 -> TOKENIZE
 -> RNORM
 -> SSR
 -> KGRAPH
 -> CNORM
 -> DNA
 -> EXEC
 -> LINEAGE
 -> REUSE
```

Each phase must reduce ambiguity, increase authority, preserve structure, and reduce downstream cost.

## 2. Core Axioms

Single semantic truth:

- all representations must collapse to one invariant structure
- syntax is disposable after structural recovery
- presentation never outranks canonical structure

Determinism:

```text
same input
-> same canonical graph
-> same canonical hash
-> same DNA
```

No semantic leakage:

- DNA must not require semantic names
- DNA must not require symbolic aliases
- DNA must not require human-readable operator labels
- semantic systems must emerge from structure, not annotate structure into authority

No silent mutation:

- all rewrites are explicit
- all rewrites are receipted
- all rewrites are reversible or rollback-addressable
- all rewrites are ledgered

Canonical authority:

```text
canonical_structure := identity
```

## 3. File Model

Public authoring and machine artifact extensions:

```text
.gene.rna
.locus.rna
.genome.rna

.dna
```

Extensions are hints. Headers are truth.

Artifact classes:

```text
GENE    atomic unit
LOCUS   grouped structure
GENOME  composite bundle
DNA     compiled substrate
```

Mandatory artifact header fields:

```text
type
encoding
version
canonical_id
grammar_id
integrity_hash
```

Optional header fields:

```text
dependencies
lineage
context
```

## 4. Global Identity Model

Identity:

```text
ID := blake3(canonical_binary_structure)
```

Identity rules:

- metadata-independent
- machine-stable
- replay-stable
- canonicalization-dependent

Collision policy:

```text
hash collision -> structural equality fallback
```

## 5. Token Algebra

Closed token set:

```text
ATOM
BINDER
OP
GROUP_L
GROUP_R
SEP
STRAND_REF
META
```

No dynamic token classes are allowed in the substrate.

Byte-level classification:

```text
[a-z0-9] -> ATOM
[A-Z]    -> BINDER

(        -> GROUP_L
)        -> GROUP_R

, ;      -> SEP
.        -> STRAND_REF

_ | :    -> OP

space    -> META
newline  -> META
```

Tokenization is single-pass classifiable and does not parse keywords.

Token structure:

```text
TOKEN := (
  class: u8,
  value: u32,
  flags: u8,
  arity_hint: u8
)
```

Token algebra:

```text
compose
group
bind
apply
reference
```

No hidden semantic operations are allowed.

Rust target:

- one owning `TokenClass` enum
- one byte classification table
- one diagnostic code family for invalid byte, boundary, grouping, arity, and scope failures
- conformance corpus for classification and normalization

## 6. RNA Surface

RNA is strict symbolic structural notation.

RNA properties:

- parseable at all times when valid
- splice-capable
- structurally reducible
- deterministic under normalization

RNA grammar:

```text
SEQ := TERM (SEP TERM)*

TERM :=
  ATOM
  BINDER
  OP TERM*
  GROUP

GROUP :=
  ( SEQ )
```

RNA states:

```text
RAW
SPLICE_BEARING
EXPRESSED
NORMALIZED
```

Splice payloads are transient shorthand fragments. They must be locally parsable, globally reducible, and deterministically expandable.

Example shorthand class:

```text
iei i>l1.l2|0|w eie
```

Splice payloads are never persisted after normalization.

## 7. RNORM

RNORM transforms symbolic RNA into explicit structural sequence.

RNORM responsibilities:

- expand shorthand
- resolve grouping
- resolve separators
- assign arity
- resolve binding structure
- remove ambiguity

Guarantee:

```text
RNORM(x) == RNORM(y)
iff structurally equivalent under the RNA grammar
```

Complexity:

```text
O(n)
```

RNORM must not perform:

- semantic inference
- canonicalization
- execution
- graph optimization

Rust target:

- `RnormInput(TokenStream)`
- `RnormOutput(NormalizedTokenStream)`
- explicit `RnormReceipt`
- deterministic diagnostics with source spans

## 8. SSR

SSR means Structural State Resolution.

SSR is a deterministic pushdown structural machine.

SSR is not:

- a recursive parser
- an AST builder
- a semantic interpreter
- a persistent graph authority layer

SSR state:

```text
SSR_STATE := (
  stack,
  frame_stack,
  graph_builder,
  scope_stack
)
```

Transition table:

| Token | Action |
| --- | --- |
| ATOM | emit leaf |
| BINDER | register binding |
| OP | push operator |
| GROUP_L | push frame |
| GROUP_R | close frame |
| SEP | finalize term |
| STRAND_REF | emit edge |
| META | ignore |

SSR constraints:

- single-pass
- no backtracking
- no semantic lookup
- no persistence
- no canonicalization
- no mutation of prior committed state

SSR output:

```text
KGRAPH
SSR_RECEIPT
```

Rust target:

- stack-bounded reducer
- ephemeral arenas or lifetimes that prevent SSR identity persistence
- source maps allowed for diagnostics only
- no SSR serialization path

## 9. KGRAPH

KGRAPH is an ephemeral structural graph emitted by SSR and consumed by CNORM.

Node shape:

```text
NODE := (
  opcode,
  inputs[],
  aux,
  flags
)
```

Default graph property:

```text
DAG
```

Cycles are explicit only.

KGRAPH constraints:

- no semantic payloads
- contiguous storage preferred
- locality-oriented layout
- temporary graph authority only

Rust target:

- `KGraph` is a lower-chain transfer object, not public authority
- semantic labels must not appear in node identity
- persistence, if any, is debug-only and non-authoritative

## 10. Opcode System

Absolute rule:

```text
opcodes define structure only
```

Core opcode set:

```text
LEAF
BIND
APPLY
GROUP
SEQ
REF
CONST
PROJ
ANNOT
```

Structural modifiers:

```text
COMMUTE
ASSOC
INLINE
REDUCE
```

Forbidden semantic opcodes:

```text
CHAIN_RULE
DERIVATIVE
THEOREM
CERTIFIED_PROOF
```

Those concepts must emerge structurally above the substrate.

Rust target:

- one `Opcode` enum
- one arity table
- one compatibility table
- one decode/validation failure table
- reserved opcode ranges and version negotiation policy

## 11. CNORM

CNORM collapses structurally equivalent graphs into identical canonical forms.

Canonicalization operations:

```text
normalize ordering
flatten associative groups
collapse redundant nesting
deduplicate equivalent nodes
stabilize identity
```

Canonical hash:

```text
HASH(node) :=
  hash(
    opcode,
    ordered(child_hashes),
    flags,
    aux
  )
```

Idempotence:

```text
CNORM(CNORM(x)) == CNORM(x)
```

Deduplication:

- hash-consing is permitted only after canonical ordering
- hash collisions fall back to structural equality

Rust target:

- `CanonicalTopology`
- canonical ordering law
- deterministic binary canonical form
- cross-platform hash-stability tests

## 12. DNA Substrate

DNA is canonical reconstructive machine encoding.

Global layout:

```text
HEADER
SECTION*
```

Header:

```text
magic
version
flags
section_count
root_id
node_count
integrity_hash
grammar_id
```

Node encoding:

```text
[OPCODE][ARITY][CHILDREN...]
```

Atom encoding:

```text
varint(u32)
```

Optional sections:

```text
STRING_TABLE
DEBUG_INFO
LOCAL_COMPRESSION
```

Optional sections must never affect meaning.

DNA guarantees:

- endian stable
- streaming decodable
- reconstructive
- deterministic
- semantically closed

Forbidden DNA content in required authority sections:

- symbolic aliases
- semantic labels
- proof prose
- operator names

Rust target:

- staged DNA validator
- compact structural sections
- local compression only
- compatibility import for proto-DNA and `.locus`, not preferred authority emission

## 13. Codebook Policy

Final decision:

```text
structure = authority
codebooks = optional compression
```

Allowed:

- local deduplication
- symbol compression
- payload compression

Forbidden:

- semantic dependency
- mandatory codebook resolution
- meaning reconstruction from a codebook

## 14. Execution Model

Execution input:

```text
DNA
```

Execution output types:

```text
ExecutionWitness
NumericEvidence
CounterexampleCandidate
ReplayTrace
ResidualObligation
```

Output families must remain distinct.

Execution guarantees:

- exactness explicit
- approximation explicit
- deterministic replay when exact
- residual obligations explicit

Rust target:

- executor over DNA/canonical topology, not reconstructed semantic documents
- deterministic scheduling policy
- resource budgeting
- malicious DNA handling
- structural bomb detection

## 15. Ledger System

Ledger entry:

```text
ENTRY := (
  phase_id,
  input_hash,
  output_hash,
  invariants,
  failures,
  validation,
  rollback_ptr,
  metrics
)
```

Ledger laws:

- append-only
- immutable
- lineage preserving
- mandatory for promotion

Rust target:

- phase engine owns ledger writes
- exactly one ledger entry per successful phase transition
- failure states record last valid ledger entry

## 16. Reuse Law

Reuse conditions:

```text
canonical_id match
AND lineage valid
AND replay permitted
AND invariants satisfied
```

Forbidden reuse:

- blind cache hits
- approximate to exact substitution
- lineage-free reuse

Reuse outputs:

```text
ReuseReceipt
ResidualObligation
```

## 17. Failure Model

Failure types:

```text
StructuralError
ArityError
ScopeError
CanonicalizationError
EncodingError
ValidationError
ExecutionError
ReuseViolation
```

Failure state:

```text
FAILURE_STATE := (
  phase,
  reason,
  rollback_pointer,
  last_valid_ledger
)
```

Allowed responses:

```text
HALT
ROLLBACK
```

No partial propagation is allowed.

## 18. Phase Execution Kernel

Global rail:

```text
P0 TOKENIZE
P1 RNORM
P2 SSR
P3 CNORM
P4 DNA_ENCODE
P5 DNA_VALIDATE
P6 EXECUTE
P7 RECONNECT
P8 REUSE
```

Core execution loop:

```rust
for phase in EXECUTION_CHAIN {
    let output = phase.transform(input)?;

    assert!(phase.invariants(&output));
    assert!(output.failures.is_empty());

    ledger.commit(...);

    input = output;
}
```

Escalation law:

1. every phase reduces ambiguity
2. every phase increases authority
3. every phase reduces downstream cost
4. every phase produces reusable artifacts

## 19. Performance Model

Performance sources:

```text
byte-level tokenization
single-pass SSR
zero semantic lookup
canonical reuse
streaming decode
contiguous arenas
hash-consing
```

Complexity:

| Phase | Complexity |
| --- | --- |
| TOKENIZE | O(n) |
| RNORM | O(n) |
| SSR | O(n) |
| CNORM | O(n log n) worst |
| DNA | O(n) |
| EXEC | variable |
| REUSE | O(1) expected |

Forbidden performance killers:

- recursive parsing
- semantic lookup tables
- string-heavy hot paths
- runtime alias resolution
- mutable AST passes
- backtracking parsers

## 20. CLI Surface Model

Target commands:

```text
import
splice
fold
compile
validate
sequence
execute
trace
certify
reuse
```

RNA UX contract:

```text
Detected: gene.rna
Status: RNA contains splice payload fragments

Planned actions:
- splice RNA
- rewrite file in place
- create backup
```

Safety guarantees:

- atomic writes
- explicit confirmation
- rollback support
- backup before mutation

## 21. Current Codebase Alignment

The current codebase is not yet the final rail implementation.

Current codebase classification:

- RNA surface exists as partial success.
- RNORM is mostly real but still needs byte/token-grounded closure.
- SSR exists but must be forced into ephemeral lower-chain form.
- CNORM exists partially but must become canonical topology rather than graph snapshot persistence.
- DNA exists as proto-DNA and must be rebuilt into compact structural encoding.
- CLI exists but must be phase-engine enforced.
- Research, certification, tower, and coverage systems are valuable but must be reattached as derived overlays.
- QC0/QA0/QK0/QM0/JSON/document workflows are extraction sources and deletion targets, not compatibility commitments or final public authority.

Observed codebase facts that affect the rail:

- The workspace still uses `mf-*` crate directories and package identities throughout `Cargo.toml`.
- The public wrapper binary is still `mf`, with `l64-cli` and `l64-admin` shipped beside it.
- Q-surface crates are first-class workspace members: `l64-qc0`, `l64-qa0`, `l64-qk0`, and `l64-qm0`.
- `l64-surfaces` directly imports Q-surface crates and performs Q-surface transcode/export/import work.
- CLI/admin/cert tests still create `.qc0`, `.qa0`, `.qk0`, and `.qm0` fixtures.
- Samples still include `.qc0` bundles.
- Release source snapshots under `release/src` mirror the old `mf-*` names and should not be treated as authoritative source during refactors.
- The rename from `mf` to `l64` is not only cosmetic; it must be staged so dependency names, package names, binary names, docs, release scripts, and tests converge without breaking every edge at once.

Plan-altering conclusion:

- The rail must include a dedicated rename node before Q-surface deletion, because deleting Q crates and renaming the workspace at the same time would make failures harder to localize.
- The phase contract skeleton must exist before lower-chain work, because otherwise token/RNORM/SSR/CNORM/DNA changes will each invent local validation and failure semantics.
- Authority audit must include workspace ownership and dependency fanout, because the Q-surface crates are still actively referenced by CLI, tests, samples, and `l64-surfaces`.
- End-to-end closure must include release packaging and documentation verification, because the rail was created to carry the project to a shippable endpoint.

Adversarial audit result:

- Highest rework risk: performing `mf` -> `l64` rename too late, after new code adds more `mf` references.
- Highest architecture risk: treating Q-surface crates as compatibility instead of deleting them.
- Highest Rust risk: moving logic between crates without first establishing typed phase/error/ID contracts in `core`/command substrate.
- Highest testing risk: workspace tests staying green because `.qc0` fixtures still dominate coverage.
- Highest release risk: stale `release/src` snapshots or old docs being mistaken for current source truth.

## 22. First Compounding Change Chain

This change chain is now applied to the rail:

1. Rename the authoritative rail from `COMPOUNDING_CHANGE_CHAIN.md` to `LINEAR_EXECUTION_RAIL.md`.
2. Define compounding change chains as trajectory-preserving changes to the linear execution rail.
3. Consolidate the substrate ontology into one sequence: RNA, token algebra, RNORM, SSR, KGRAPH, CNORM, DNA, EXEC, LINEAGE, REUSE.
4. Make DNA explicitly reconstructive structural encoding, not graph persistence.
5. Make KGRAPH explicitly ephemeral and temporary.
6. Add `CONST`, `PROJ`, and `ANNOT` as structural opcodes while keeping semantic opcodes forbidden.
7. Make BLAKE3 over canonical binary structure the identity target.
8. Add mandatory/optional file header requirements.
9. Add the phase execution kernel as the implementation anchor.
10. Preserve the old substrate inversion finding as current codebase alignment, not as the main rail text.

## 23. Temporal Implementation Rail

This is the current path-optimized implementation sequence. Execute it in order unless a compounding change chain proves that a missing prerequisite must be inserted.

### Node 00 - Rail Rename And Reference Closure

Purpose:

- make `LINEAR_EXECUTION_RAIL.md` the only authoritative rail file
- make compounding change chains subordinate edits to this rail

Depends on:

- repository documentation baseline

Code targets:

- `README.md`
- `HANDOFF_STATUS.md`
- `EXPORT_STRESS_REMEDIATION_LEDGER.md`
- old `COMPOUNDING_CHANGE_CHAIN.md` path

Actions:

- remove the old rail file
- update references to the new filename
- state that change chains preserve rail trajectory

Invariants:

- no active documentation points to `COMPOUNDING_CHANGE_CHAIN.md` as the current rail
- no separate top-level rail competes with this file

Tests:

- `rg COMPOUNDING_CHANGE_CHAIN`
- `git status --short`

Exit condition:

- docs consistently name `LINEAR_EXECUTION_RAIL.md`

Downstream payoff:

- future planning edits target one stable document

Status:

- complete

### Node 01 - Authority Audit Baseline

Purpose:

- identify every path that can currently create, validate, import, export, or promote artifacts
- classify those paths by authority level before lower-chain changes begin
- establish the current workspace dependency map before renaming or deletion work begins

Depends on:

- Node 00

Code targets:

- current `mf-*` crates until renamed
- future `l64-*` crate and binary names
- root `Cargo.toml`
- `l64-cli`
- `l64-admin`
- `l64-command`
- `l64-locus`
- `l64-qc0`
- `l64-qa0`
- `l64-qk0`
- `l64-qm0`
- `l64-research`
- `l64-cert`
- `l64-runtime`

Step sequence:

1. Run `cargo metadata` or equivalent workspace inspection and record crate membership.
2. Build a dependency fanout map for Q-surface crates, CLI/admin binaries, surfaces, bundle, cert, and tests.
3. Classify each command/import/export/promote path by authority category.
4. Mark generated release snapshots and `target` output as non-source.
5. Add or plan the `authority-audit` command only after the static classification is clear.
6. Add residue searches for Q-surface and `mf` naming.
7. Run targeted checks for the crates touched by the audit.

Actions:

- add an authority classification enum or equivalent static table
- classify paths as `SubstrateAuthority`, `DerivedSemantic`, `ExtractionSource`, or `DeletionTarget`
- expose `l64 authority-audit` or equivalent admin command
- make the audit fail if QC0/QA0/QK0/QM0, JSON, report text, or SSR identities claim substrate authority
- record crate dependency fanout so later rename/deletion order is mechanical

Invariants:

- RNA and DNA are the only target public representation surfaces
- Q-surface paths are not compatibility commitments
- semantic persistence cannot silently become substrate authority
- release snapshots are not edited as authoritative source

Tests:

- `cargo metadata` or equivalent workspace inventory succeeds
- command smoke test for authority audit
- assertion that QC0/QA0/QK0/QM0/JSON/report paths are not classified as substrate authority
- regression test that DNA/lower receipts remain authority-capable

Exit condition:

- developers can see all authority-bearing paths before changing the lower chain

Downstream payoff:

- prevents substrate work from being invalidated by hidden upper-stack authority leakage

### Node 02 - Phase Contract Skeleton And Canonical Identity Foundation

Purpose:

- establish the Rust phase contract before individual phases invent incompatible validation/error/receipt shapes
- make canonical identity precise before token/opcode/DNA work depends on it

Depends on:

- Node 01

Code targets:

- `l64-core`
- `l64-canon`
- `l64-locus`
- `l64-command`

Step sequence:

1. Define closed enums or equivalent stable IDs for `PhaseId`, `ArtifactClass`, `FailureKind`, `ExactnessClass`, and `AuthorityClass`.
2. Define newtypes for `CanonicalId`, `DnaDigest`, `LineageId`, and `LedgerEntryId`.
3. Define minimal `PhaseInput`, `PhaseOutput`, `PhaseReceipt`, and `PhaseFailure` traits or structs.
4. Define canonical identity as BLAKE3 over canonical binary structure.
5. Add collision fallback semantics as structural equality, even if collision tests use an injected/fake hash.
6. Add tests around metadata independence and source formatting independence.
7. Only after these types exist, let later nodes add phase-specific payloads.

Actions:

- add minimal phase contract types without forcing every phase through the engine yet
- verify or add BLAKE3 support for canonical binary structure
- define `CanonicalId` as hash over canonical binary structure, not metadata
- add structural equality fallback for hash collision handling
- isolate debug labels and source text from canonical identity input

Invariants:

- metadata does not affect identity
- source formatting does not affect identity
- canonical binary structure is the only identity input
- later phase receipts share the same phase/failure/authority vocabulary

Tests:

- compile test for shared phase contract types
- same structure with different metadata yields same ID
- different structure yields different ID or falls through to structural comparison on forced collision test
- canonical ID stable across repeated runs

Exit condition:

- all later phases can depend on one identity law and one minimal phase contract vocabulary

Downstream payoff:

- tokenization, RNORM, SSR, CNORM, DNA, execution, reuse, and semantic rekeying do not each invent local contract shapes

### Node 03 - Token Algebra Freeze

Purpose:

- replace scattered/string-centric lexical behavior with one closed token substrate

Depends on:

- Node 02

Code targets:

- `l64-core`
- `l64-cli`
- `l64-locus`
- existing RNA normalization code

Step sequence:

1. Locate all token-like classification currently embedded in RNA, CLI, locus, and tests.
2. Create the closed `TokenClass` and byte classification table in the lowest appropriate crate.
3. Redirect one existing tokenizer path to the shared table.
4. Add fixtures for accepted bytes, rejected bytes, boundaries, whitespace, and newlines.
5. Remove duplicated local token rules only after the shared tests pass.
6. Run crate-local checks before wider workspace tests.

Actions:

- create or consolidate `TokenClass`
- implement one byte classification table
- define invalid byte, control character, whitespace, and newline handling
- define token boundary rules
- add token conformance fixtures

Invariants:

- no dynamic token classes
- no keyword parsing in the substrate tokenizer
- every accepted byte is classified deterministically
- every rejected byte has a deterministic diagnostic

Tests:

- token fixture corpus
- invalid byte tests
- repeated-run determinism tests

Exit condition:

- RNORM can consume token streams without re-tokenizing by private string rules

Downstream payoff:

- parsing, diagnostics, SSR, and RNA UX stop duplicating lexical logic

### Node 04 - RNA Grammar And RNORM Closure

Purpose:

- make normalized RNA an explicit structural sequence rather than a presentation cleanup result

Depends on:

- Node 03

Code targets:

- `l64-core`
- `l64-locus`
- `l64-cli`

Step sequence:

1. Introduce RNA state types or equivalent wrappers without changing behavior.
2. Make RNORM accept the shared `TokenStream`.
3. Move shorthand/splice expansion behind explicit RNORM functions.
4. Emit `RnormReceipt` using shared phase/failure vocabulary.
5. Add idempotence and failure fixtures.
6. Remove old string-only normalization paths after tests prove equivalent or intentionally stricter behavior.

Actions:

- define `RawRna`, `SpliceBearingRna`, `ExpressedRna`, and `NormalizedRna` or equivalent state types
- make RNORM consume `TokenStream`
- expand shorthand into explicit structural form
- remove splice payloads after normalization
- emit `RnormReceipt`
- add deterministic repair-locus diagnostics

Invariants:

- RNORM is O(n)
- RNORM does not canonicalize
- RNORM does not execute
- RNORM does not infer semantics
- normalized output has no unresolved splice fragments

Tests:

- shorthand expansion fixtures
- grouping/separator/arity failure fixtures
- idempotence test for already normalized RNA

Exit condition:

- SSR receives normalized structural tokens only

Downstream payoff:

- SSR can be a mechanical reducer instead of a parser plus repair engine

### Node 05 - Structural Opcode Law Freeze

Purpose:

- define the structural vocabulary before SSR emits graph nodes or DNA encodes them

Depends on:

- Node 04

Code targets:

- `l64-core`
- `l64-locus`
- `l64-runtime`

Step sequence:

1. Inventory existing opcode-like enums, packet tags, and structural node kind matches.
2. Define the shared `Opcode` enum and arity table.
3. Redirect SSR/KGRAPH first, DNA second, execution third.
4. Add forbidden semantic opcode tests.
5. Replace scattered local matches with table lookups where this reduces duplication.
6. Run targeted tests for each consumer before deleting old local definitions.

Actions:

- define one `Opcode` enum
- add `LEAF`, `BIND`, `APPLY`, `GROUP`, `SEQ`, `REF`, `CONST`, `PROJ`, and `ANNOT`
- add structural modifiers `COMMUTE`, `ASSOC`, `INLINE`, and `REDUCE`
- define arity and compatibility tables
- reserve forbidden semantic opcode names in tests to ensure they cannot enter the substrate

Invariants:

- opcodes describe structure only
- semantic operations emerge above the substrate
- arity validation is table-driven

Tests:

- opcode arity corpus
- decode/validation failure tests
- forbidden semantic opcode tests

Exit condition:

- KGRAPH and DNA can share the same structural opcode law

Downstream payoff:

- removes duplicated structural meaning from graph, codec, and executor code

### Node 06 - SSR Ephemerality Refactor

Purpose:

- force SSR into a deterministic pushdown structural machine

Depends on:

- Node 05

Code targets:

- `l64-core`
- `l64-locus`
- any current structural resolution module

Step sequence:

1. Identify every persisted SSR-like ID, graph, receipt, or export path.
2. Add the bounded SSR reducer while leaving old path behind a testable comparison if needed.
3. Emit `SsrReceipt` without authority identity.
4. Switch CNORM input to the new KGRAPH path.
5. Delete or demote SSR persistence/export.
6. Add tests that fail if SSR becomes serializable authority again.

Actions:

- implement SSR as a reducer over normalized token streams
- introduce bounded `SSR_STATE`
- create `SsrReceipt` that records transition facts without preserving authority identity
- remove or demote any persistent SSR graph identity
- ensure source maps are diagnostic-only

Invariants:

- single pass
- no backtracking
- no semantic lookup
- no persistence
- no canonicalization
- no mutation of prior committed state

Tests:

- stack underflow/overflow tests
- reference legality tests
- no SSR serialization path test
- repeated-run deterministic KGRAPH output test

Exit condition:

- SSR produces KGRAPH plus receipt, and only KGRAPH is consumable by CNORM

Downstream payoff:

- closes the largest current architecture inversion point

### Node 07 - KGRAPH Transfer Boundary

Purpose:

- make KGRAPH a temporary transfer object with clear limits

Depends on:

- Node 06

Code targets:

- `l64-core`
- `l64-canon`
- `l64-locus`

Step sequence:

1. Define KGRAPH node/storage shape in the lower crate.
2. Add DAG-by-default validation and explicit cycle representation if needed.
3. Remove semantic payloads from node identity.
4. Make CNORM consume only KGRAPH.
5. Add graph property tests.
6. Remove legacy graph snapshot inputs from CNORM once callers move.

Actions:

- define explicit `KGraph` node shape
- enforce DAG-by-default with explicit cycle representation if cycles are allowed
- prefer contiguous node storage
- remove semantic payloads from node identity
- add debug-only projection if needed

Invariants:

- KGRAPH is not a public identity layer
- KGRAPH does not persist as authority
- KGRAPH contains structure, not semantic prose

Tests:

- graph property tests
- cycle legality tests
- semantic-label exclusion tests

Exit condition:

- CNORM has a single well-scoped input type

Downstream payoff:

- canonicalization stops depending on legacy graph persistence assumptions

### Node 08 - CNORM Canonical Topology

Purpose:

- make canonicalization produce execution topology and stable identity

Depends on:

- Node 07

Code targets:

- `l64-canon`
- `l64-core`
- `l64-locus`

Step sequence:

1. Introduce `CanonicalTopology` beside existing canonical graph code.
2. Implement canonical ordering for the smallest real structure family first.
3. Add idempotence and equivalence fixtures.
4. Compute canonical binary form and canonical ID through Node 02 identity types.
5. Redirect DNA encoding to consume `CanonicalTopology`.
6. Remove presentation-dependent identity inputs.

Actions:

- introduce `CanonicalTopology`
- implement canonical ordering
- flatten associative structures where declared
- order commutative structures where declared
- deduplicate only after canonical ordering
- compute canonical binary form and canonical ID

Invariants:

- `CNORM(CNORM(x)) == CNORM(x)`
- equivalent structures converge
- non-equivalent structures remain separated
- presentation data does not affect identity

Tests:

- equivalence corpus
- non-equivalence corpus
- idempotence tests
- hash stability tests

Exit condition:

- DNA encoder consumes canonical topology, not legacy graph snapshots

Downstream payoff:

- DNA, execution, ledger, and reuse become identity-stable

### Node 09 - DNA Structural Encoding

Purpose:

- replace proto-DNA graph packet behavior with reconstructive structural machine encoding

Depends on:

- Node 08

Code targets:

- `l64-locus`
- `l64-core`

Step sequence:

1. Define DNA header and section structs using fixed-size/varint primitives.
2. Encode a minimal canonical topology without optional strings.
3. Add optional debug/string sections only after required sections validate alone.
4. Add staged decode and validation helpers.
5. Keep proto-DNA import only long enough to migrate tests.
6. Add binary inspection tests proving required sections do not need semantic text.

Actions:

- implement required DNA header fields
- encode required authority sections as opcode/topology data
- encode atoms as varints
- permit string/debug/local-compression sections only as optional non-authority sections
- add staged validator for header, section table, topology digest, and canonicality

Invariants:

- required DNA sections contain no semantic text needed for validation or execution
- DNA is endian-stable
- DNA is streaming-decodable
- DNA reconstructs canonical topology

Tests:

- encode/decode roundtrip
- semantic text absence test for required sections
- truncation/corruption tests
- optional section removal test

Exit condition:

- valid DNA can be validated without RNA, QC0, JSON, or report text

Downstream payoff:

- creates the actual machine authority surface

### Node 10 - DNA Validation Phase

Purpose:

- separate encoding from validation so execution never receives unvalidated DNA

Depends on:

- Node 09

Code targets:

- `l64-locus`
- `l64-runtime`
- `l64-cli`

Step sequence:

1. Add `DNA_VALIDATE` as a distinct phase ID.
2. Implement validator over the new DNA structures.
3. Route CLI validate through validator.
4. Block executor entry points that lack validation receipts.
5. Add invalid corpus tests before execution refactor.

Actions:

- add explicit `DNA_VALIDATE` phase
- emit validation receipts
- classify header failure, section failure, opcode failure, digest failure, and canonicality failure
- make CLI validation use the phase engine path

Invariants:

- execution input is validated DNA only
- validation does not require semantic labels
- validation failure halts propagation

Tests:

- invalid DNA corpus
- compatible unknown section test
- incompatible unknown section test
- CLI `validate` smoke test

Exit condition:

- executor cannot be called through public path with unvalidated DNA

Downstream payoff:

- execution and certification trust one validation boundary

### Node 11 - Execution Over DNA

Purpose:

- move execution from graph/document workflows to structural traversal

Depends on:

- Node 10

Code targets:

- `l64-runtime`
- `l64-cert`
- `l64-core`

Step sequence:

1. Add execution input type that requires validated DNA or canonical topology plus validation receipt.
2. Implement the smallest structural traversal path before optimizing.
3. Split exact witness, numeric evidence, counterexample candidate, replay trace, and residual obligation outputs.
4. Add resource budget and structural bomb checks.
5. Redirect certification to consume execution-native receipts.
6. Remove graph/document execution shortcuts after parity tests pass.

Actions:

- execute over DNA or canonical topology
- keep `ExecutionWitness`, `NumericEvidence`, `CounterexampleCandidate`, `ReplayTrace`, and `ResidualObligation` separate
- define exactness and approximation fields
- add replay trace keyed to DNA/canonical ID
- add resource budget and structural bomb checks

Invariants:

- exact and approximate artifacts do not collapse
- execution does not require reconstructed semantic documents
- replay is deterministic when exactness claims determinism

Tests:

- execution smoke tests
- exactness classification tests
- resource exhaustion tests
- replay determinism tests

Exit condition:

- certification can consume execution-native receipts

Downstream payoff:

- proof and adequacy work can become execution-native

### Node 12 - Phase Engine And Ledger Enforcement

Purpose:

- enforce the linear rail uniformly

Depends on:

- Node 11

Code targets:

- `l64-core`
- `l64-command`
- `l64-cli`
- `l64-admin`

Step sequence:

1. Upgrade Node 02 phase contract skeleton into an actual phase engine.
2. Route one low-risk command through the engine.
3. Add ledger commit and failure-state behavior.
4. Route compile/validate/execute paths through the engine.
5. Add replay/trace command support.
6. Remove direct phase bypasses after tests prove equivalent behavior.

Actions:

- upgrade the early phase contract skeleton into the global execution kernel
- make phase transitions emit exactly one ledger entry
- add failure state with rollback pointer and last valid ledger
- define transaction boundaries and interruption behavior
- expose trace/replay command paths

Invariants:

- no successful phase transition without ledger entry
- no failed phase emits downstream artifact
- ledger is append-only and lineage-preserving

Tests:

- successful phase ledger test
- failed phase halt test
- rollback pointer test
- replay test

Exit condition:

- CLI commands can be routed through one phase engine contract

Downstream payoff:

- later semantic systems inherit closure instead of reimplementing it

### Node 13 - CLI Surface Alignment

Purpose:

- make user-facing commands match the rail

Depends on:

- Node 12

Code targets:

- public command name `l64`
- crate prefix target `l64-*`
- current `l64-cli`
- current `l64-admin`
- current `l64-command`
- `README.md`
- `USAGE_GUIDE.md`
- `LOCUS64_LANGUAGE_SPEC.md`

Step sequence:

1. Add `l64` command surface while `mf` still exists only if tests require a transitional alias.
2. Move docs examples to `l64`.
3. Route `l64` commands through the phase engine.
4. Remove Q-surface command examples before Q crate deletion.
5. Run command smoke tests and stale wording scans.
6. Remove `mf` alias in Node 14 when crate/binary rename completes.

Actions:

- align commands around `import`, `splice`, `fold`, `compile`, `validate`, `sequence`, `execute`, `trace`, `certify`, and `reuse`
- choose `l64` as the public binary name; use uppercase `L64` only for prose/product identity where appropriate
- plan crate rename from `mf-*` to `l64-*` after lower substrate tests protect behavior
- remove, not merely label, obsolete Q-surface commands unless still required as an extraction source during the active migration node
- ensure extension is hint and header is truth
- implement RNA mutation UX with backup and explicit confirmation where applicable

Invariants:

- CLI trace equals phase trace
- user-facing docs do not imply QC0/QA0/QK0/QM0/JSON are public authority or supported ecosystem formats
- mutation is never silent

Tests:

- CLI smoke suite
- help text scan for stale authority wording
- `rg "mf |mf-|qc0|qa0|qk0|qm0"` with documented exceptions only
- RNA backup/rollback test where implemented

Exit condition:

- users can interact with Locus64 through the rail vocabulary

Downstream payoff:

- documentation and CLI stop fighting the architecture

### Node 14 - Codebase Prefix Rename To l64

Purpose:

- rename the codebase identity from `mf`/`math framework` to `l64`/Locus64 without mixing naming churn with substrate deletion

Depends on:

- Node 13

Code targets:

- `Cargo.toml`
- every crate `Cargo.toml`
- crate directories currently named `mf-*`
- root wrapper crate currently named `mf`
- Rust imports using `mf_*`
- command docs and release scripts
- generated release source snapshots only after source-of-truth crates are renamed

Step sequence:

1. Inventory all package names, directory names, binary names, import paths, docs, scripts, and tests containing `mf`, `mf-`, or `mf_`.
2. Freeze source-of-truth scope: exclude `target`, `release/src`, generated zips, and old release payloads from rename edits until final packaging.
3. Add a temporary `l64` wrapper or alias only if needed to keep tests green during the rename.
4. Rename leaf crates first where dependency fanout is smallest.
5. Rename shared crates next and update Rust import paths from `mf_*` to `l64_*`.
6. Rename binary crates and command examples from `mf` to `l64`.
7. Update release scripts and package docs.
8. Remove temporary aliases once all tests pass through `l64`.
9. Run the full workspace tests and a help-text scan for stale public `mf` usage.

Actions:

- preserve behavior while changing names
- prefer mechanical rename patches over semantic edits
- keep one commit or checkpoint per rename band if implementation is split
- treat `mf` mentions as allowed only in historical migration notes until final deletion

Invariants:

- no public command remains `mf` after the rename node exits
- no package name remains `mf-*` after the rename node exits
- any temporary alias has an explicit deletion condition
- Q-surface crates are not deleted in this node unless the rename exposes a trivial isolated removal
- generated release snapshots are regenerated after source rename rather than hand-edited as source truth

Tests:

- `cargo test`
- `cargo build -p l64` or final binary equivalent
- `rg "mf |mf-|mf_"` returns only documented historical notes or no results
- release script dry-run or smoke inspection

Exit condition:

- the active codebase, docs, and release scripts use `l64` naming as the default identity

Downstream payoff:

- Q-surface deletion happens under final project naming, not a discarded `math framework` identity

### Node 15 - Q-Surface Extraction And Deletion

Purpose:

- extract required implementation value from obsolete Q-surface crates, redirect surviving behavior into RNA/DNA substrate modules, and delete the Q-surface crates instead of preserving fake compatibility

Depends on:

- Node 14

Code targets:

- old `l64-qc0` / renamed `l64-qc0` during migration window
- old `l64-qa0` / renamed `l64-qa0` during migration window
- old `l64-qk0` / renamed `l64-qk0` during migration window
- old `l64-qm0` / renamed `l64-qm0` during migration window
- old `l64-surfaces` / renamed `l64-surfaces`
- old `l64-bundle` / renamed `l64-bundle`
- `samples`
- `Cargo.toml`
- `Cargo.lock`
- future or existing RNA/DNA homes:
  - `l64-rna` or current lower-chain module home
  - `l64-dna` or current locus/DNA module home
  - `l64-command`
  - `l64-research`

Step sequence:

1. Inventory Q-surface exports, parsers, renderers, fixtures, tests, commands, and sample files.
2. Classify each item as `Delete`, `MoveToRna`, `MoveToDna`, `MoveToResearchRecord`, `MoveToTestFixture`, or `TemporaryShim`.
3. Move required data structures and logic into the selected RNA/DNA/research home.
4. Redirect commands and tests from Q files to `.rna`, `.dna`, or native lineage records.
5. Delete one Q crate at a time and run targeted tests after each deletion.
6. Remove `l64-surfaces`/`l64-surfaces` only after transcode duties are gone or absorbed into rail-native modules.
7. Remove Q sample files and replace them with rail-native samples.
8. Run full workspace tests and residue searches.

Actions:

- inventory all Q-surface crates by used type, parser, fixture, sample, command, and receipt
- extract reusable logic into RNA/DNA or shared substrate modules only when still required
- redirect commands and tests from `.qc0`, `.qa0`, `.qk0`, and `.qm0` files to `.rna` and `.dna` artifacts
- replace Q-surface samples with rail-native `.gene.rna`, `.locus.rna`, `.genome.rna`, and `.dna` samples
- remove Q-surface crates from the workspace once consumers are redirected
- remove Q-surface exports from user documentation
- preserve no compatibility shim unless an active test proves a still-required internal transition cannot yet be completed
- document any temporary shim with an owner node and deletion condition

Invariants:

- Q-surface artifacts cannot become substrate authority
- public doctrine remains RNA/DNA
- Q-surface support is not treated as a user-facing ecosystem because no real ecosystem exists yet
- extracted code must be smaller and better-rooted than the deleted surface crate
- no Q-surface crate remains in the workspace after this node exits

Tests:

- `rg "l64-qc0|l64-qa0|l64-qk0|l64-qm0|\\.qc0|\\.qa0|\\.qk0|\\.qm0"` returns only allowed historical notes, if any
- workspace membership no longer includes Q-surface crates
- sample certification regression tests pass through RNA/DNA or lineage-native replacements
- promotion rejection test for lineage-free extracted/imported object
- build and test after each removed crate or command group

Exit condition:

- Q-surface crates are gone, useful logic has been absorbed into RNA/DNA substrate homes, and user-facing workflow is RNA/DNA only

Downstream payoff:

- reduces code hoarding, removes false compatibility pressure, and prevents obsolete surface names from shaping future architecture

### Node 16 - Semantic Rekeying And Upper Reattachment

Purpose:

- attach research, certification, adequacy, tower, and coverage to canonical lineage

Depends on:

- Node 15

Code targets:

- `l64-research`
- `l64-cert`
- `l64-selector`
- `l64-atlas`
- `l64-registry`
- `l64-observe`

Step sequence:

1. Inventory semantic records that currently derive from reports, Q bundles, or JSON.
2. Add canonical ID and DNA digest fields where missing.
3. Require lineage for claim, route, adequacy, bridge, operator, proof-shape, and coverage records.
4. Convert former Q semantic payloads into RNA/DNA-backed lineage records or native Rust records.
5. Add rejection tests for lineage-free semantic promotion.
6. Rerun seeded campaigns and imported-claim equivalents through lineage-native paths.

Actions:

- rekey semantic records to canonical ID and DNA digest
- require lineage for claim, route, adequacy, bridge, operator, proof-shape, and coverage records
- make generator/tower growth blocker-driven and lineage-grounded
- preserve challenge/remediation records as derived overlays
- ensure any useful semantic payload formerly expressed in QC0 exists as RNA/DNA-backed lineage objects or native Rust records, not as Q-surface text

Invariants:

- no floating report semantics
- no theorem/campaign object becomes authority without DNA lineage
- upper systems can be replayed or challenged
- no upper system depends on Q-surface crates

Tests:

- seeded campaign regression
- imported claim regression
- lineage-required rejection tests
- coverage reuse tests
- Q-surface dependency absence test

Exit condition:

- upper stack survives as derived capability above the substrate

Downstream payoff:

- the extensive current semantic system becomes an asset rather than substrate debt

### Node 17 - Conformance, Torture, And Release Gates

Purpose:

- turn the rail into a shipping standard

Depends on:

- Node 16

Code targets:

- `l64-testkit`
- `scripts`
- release docs
- CI or local release commands

Step sequence:

1. Build a conformance corpus for token, RNORM, SSR, CNORM, DNA, execution, lineage, and reuse.
2. Add randomized or fuzz-style stress where practical.
3. Update torture tests to exercise the full rail under `l64` naming.
4. Add residue scans for `mf`, Q surfaces, proto-DNA claims, and stale docs.
5. Add release smoke tests for Windows, Linux, compact, perfopt, and source packages.
6. Make failed conformance block release generation.

Actions:

- create conformance corpus for token/RNORM/SSR/CNORM/DNA/execution/reuse
- add fuzzing or randomized stress where practical
- update torture test to exercise the full rail
- add cross-platform determinism checks where available
- make release docs state current protocol version and compatibility status

Invariants:

- release cannot pass with stale docs
- release cannot pass with proto-DNA represented as final DNA
- conformance failures block release

Tests:

- `cargo test`
- rail conformance suite
- torture test
- release package smoke tests

Exit condition:

- project is shippable against this rail

Downstream payoff:

- future work has a regression net and clear resume point

### Node 18 - End-To-End Delivery Closure

Purpose:

- ensure the rail endpoint satisfies the requirements that caused the rail to be created, not merely internal architecture cleanup

Depends on:

- Node 17

Code targets:

- `README.md`
- `USAGE_GUIDE.md`
- `LOCUS64_LANGUAGE_SPEC.md`
- `SEMANTIC_USAGE_GUIDE.md`
- `HANDOFF_STATUS.md`
- `release`
- release scripts
- GitHub repository metadata

Step sequence:

1. Verify the implemented system matches the final rail definition: RNA/DNA public surface, structural authority, canonical identity, lineage-bound execution, lawful reuse.
2. Verify user-facing commands use `l64` and documented examples run.
3. Verify no Q-surface or `mf` naming residue remains outside approved historical notes.
4. Generate or refresh usage documentation for actual interaction language, command syntax, RNA authoring, DNA validation, execution, tracing, certification, and reuse.
5. Run conformance, torture, and release smoke tests.
6. Produce Windows and Linux binary releases for performance and compact profiles.
7. Produce a source release without stale generated cache, target output, or obsolete release payloads.
8. Zip release artifacts and verify archive contents.
9. Commit, tag or document release version, and push to GitHub.
10. Write final handoff with exact commands run, tests passed, known limits, and next rail node if any remains.

Actions:

- close documentation and release packaging as first-class rail outputs
- treat stale docs as release failures
- treat missing source/binary release artifacts as endpoint failure
- keep known limitations explicit rather than hidden behind marketing language

Invariants:

- final endpoint is usable by a new operator without knowing the chat history
- release contents match docs
- GitHub state matches local release state
- all unresolved limitations are documented with next-step location

Tests:

- full conformance suite
- torture test
- release package smoke test
- archive content inspection
- docs command scan
- `git status --short`

Exit condition:

- the project is shipped end-to-end against the rail requirements with reproducible docs, releases, and handoff

Downstream payoff:

- the rail terminates in a product-ready state, not an unfinished implementation plan

## 24. Second Compounding Change Chain

This change chain is now applied to the rail:

1. Reclassify Q-surface crates as extraction-and-deletion targets, not compatibility surfaces.
2. Insert Node 14 as the path-optimized `mf` -> `l64` rename sequence.
3. Replace the former compatibility-demotion phase with Node 15 Q-surface extraction and deletion.
4. Require useful code from `l64-qc0`, `l64-qa0`, `l64-qk0`, and `l64-qm0` to move into RNA/DNA or shared substrate modules.
5. Require commands, samples, and docs to redirect to `.rna` and `.dna` artifacts instead of `.qc0`, `.qa0`, `.qk0`, or `.qm0`.
6. Start the naming migration target from `mf`/`math framework` toward `l64` for command and crate identity.
7. Permit temporary shims only when they have an owner node and explicit deletion condition.
8. Add search/build/test gates proving Q-surface residue is gone rather than merely documented.
9. Add an end-to-end delivery closure node so the rail endpoint includes docs, releases, source archive, GitHub state, and handoff.
10. Add per-node step sequences for new and plan-altering nodes to keep execution temporally honest.

## 25. Third Compounding Change Chain

This change chain optimizes the rail for Rust-specific, high-efficiency execution:

1. Add Rust development optimization rules so implementation proceeds through typed contracts, crate-local tests, table-driven law, and mechanical rename bands.
2. Add an adversarial audit checklist for each node.
3. Record plan-altering codebase facts: Q-surface crates are active dependencies, `mf` naming is pervasive, and release snapshots are not source truth.
4. Move the phase contract skeleton earlier by merging it into Node 02 with canonical identity foundation.
5. Keep the full phase engine later as Node 12, but make it an upgrade of the early skeleton rather than a late invention.
6. Add nested step sequences to lower-chain nodes that previously had only actions.
7. Tighten rename sequencing to exclude generated release snapshots and isolate naming churn from Q-surface deletion.
8. Add local Rust test strategy: check crate, test crate, then run workspace tests at phase exit.

## 26. Definition Of Done

The rail is implemented when:

- required DNA sections contain no semantic text needed for validation or execution
- SSR cannot be serialized as authority
- KGRAPH cannot become a public identity layer
- CNORM identity is independent of presentation strings and source formatting
- DNA validates from structural sections alone
- execution traverses DNA or canonical topology directly
- every public CLI command maps to phase-engine transitions and ledger entries
- semantic systems are lineage-keyed derived overlays
- Q-surface crates are removed and no longer shape the architecture
- public command/crate naming has migrated to `l64` or has a bounded transitional shim with deletion criteria
- compatibility imports are removed unless backed by a concrete active requirement and deletion condition
- conformance, fuzz, torture, replay, migration, and cross-platform determinism tests pass

## 27. Final System Definition

Locus64 is a deterministic structural substrate in which symbolic interaction surfaces are normalized into canonical graph structure, encoded into reconstructive machine form, executed under lineage-preserving authority, and amortized through lawful structural reuse.

Compact form:

```text
syntax is transient
structure is authority
canonical form is identity
reuse is proven rather than assumed
```
