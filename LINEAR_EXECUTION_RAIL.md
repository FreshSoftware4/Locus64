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
  actions:
  invariants:
  tests:
  exit_condition:
  downstream_payoff:
```

If a node cannot be written in that form, it is still a concept, not executable rail material.

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
- QC0/JSON/document workflows are compatibility/import/export projections, not final public authority.

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

Depends on:

- Node 00

Code targets:

- `mf-cli`
- `mf-admin`
- `mf-command`
- `mf-locus`
- `mf-qc0`
- `mf-research`
- `mf-cert`
- `mf-runtime`

Actions:

- add an authority classification enum or equivalent static table
- classify paths as `SubstrateAuthority`, `DerivedSemantic`, `CompatibilityProjection`, or `LegacyRisk`
- expose `mf authority-audit` or equivalent admin command
- make the audit fail if QC0, JSON, report text, or SSR identities claim substrate authority

Invariants:

- RNA and DNA are the only target public representation surfaces
- compatibility paths are explicitly compatibility
- semantic persistence cannot silently become substrate authority

Tests:

- command smoke test for authority audit
- assertion that QC0/JSON/report paths are not classified as substrate authority
- regression test that DNA/lower receipts remain authority-capable

Exit condition:

- developers can see all authority-bearing paths before changing the lower chain

Downstream payoff:

- prevents substrate work from being invalidated by hidden upper-stack authority leakage

### Node 02 - Canonical Identity Foundation

Purpose:

- make canonical identity precise before token/opcode/DNA work depends on it

Depends on:

- Node 01

Code targets:

- `mf-core`
- `mf-canon`
- `mf-locus`

Actions:

- verify or add BLAKE3 support for canonical binary structure
- define `CanonicalId` as hash over canonical binary structure, not metadata
- add structural equality fallback for hash collision handling
- isolate debug labels and source text from canonical identity input

Invariants:

- metadata does not affect identity
- source formatting does not affect identity
- canonical binary structure is the only identity input

Tests:

- same structure with different metadata yields same ID
- different structure yields different ID or falls through to structural comparison on forced collision test
- canonical ID stable across repeated runs

Exit condition:

- all later phases can depend on one identity law

Downstream payoff:

- reuse, ledger, DNA integrity, and semantic rekeying share one root

### Node 03 - Token Algebra Freeze

Purpose:

- replace scattered/string-centric lexical behavior with one closed token substrate

Depends on:

- Node 02

Code targets:

- `mf-core`
- `mf-cli`
- `mf-locus`
- existing RNA normalization code

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

- `mf-core`
- `mf-locus`
- `mf-cli`

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

- `mf-core`
- `mf-locus`
- `mf-runtime`

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

- `mf-core`
- `mf-locus`
- any current structural resolution module

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

- `mf-core`
- `mf-canon`
- `mf-locus`

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

- `mf-canon`
- `mf-core`
- `mf-locus`

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

- `mf-locus`
- `mf-core`

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

- `mf-locus`
- `mf-runtime`
- `mf-cli`

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

- `mf-runtime`
- `mf-cert`
- `mf-core`

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

- `mf-core`
- `mf-command`
- `mf-cli`
- `mf-admin`

Actions:

- introduce global phase execution kernel if not already sufficient
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

- `mf-cli`
- `mf-admin`
- `mf-command`
- `README.md`
- `USAGE_GUIDE.md`
- `LOCUS64_LANGUAGE_SPEC.md`

Actions:

- align commands around `import`, `splice`, `fold`, `compile`, `validate`, `sequence`, `execute`, `trace`, `certify`, and `reuse`
- label legacy commands or compatibility workflows clearly
- ensure extension is hint and header is truth
- implement RNA mutation UX with backup and explicit confirmation where applicable

Invariants:

- CLI trace equals phase trace
- user-facing docs do not imply QC0/JSON are public authority
- mutation is never silent

Tests:

- CLI smoke suite
- help text scan for stale authority wording
- RNA backup/rollback test where implemented

Exit condition:

- users can interact with Locus64 through the rail vocabulary

Downstream payoff:

- documentation and CLI stop fighting the architecture

### Node 14 - Compatibility Demotion

Purpose:

- preserve useful legacy workflows without allowing them to define authority

Depends on:

- Node 13

Code targets:

- `mf-qc0`
- `mf-qa0`
- `mf-qk0`
- `mf-qm0`
- `mf-surfaces`
- `mf-bundle`
- `samples`

Actions:

- mark QC0/JSON/report paths as compatibility import/export projections
- require recompilation or validated lineage before promotion
- add compatibility receipts with origin and projection status
- update sample docs to distinguish compatibility from target public surface

Invariants:

- compatibility artifacts cannot silently become substrate authority
- public doctrine remains RNA/DNA
- legacy import remains useful but subordinate

Tests:

- compatibility import tests
- promotion rejection test for lineage-free legacy object
- sample certification regression tests

Exit condition:

- legacy paths are safe adapters rather than competing languages

Downstream payoff:

- current sample and certification value is retained without preserving the inversion

### Node 15 - Semantic Rekeying And Upper Reattachment

Purpose:

- attach research, certification, adequacy, tower, and coverage to canonical lineage

Depends on:

- Node 14

Code targets:

- `mf-research`
- `mf-cert`
- `mf-selector`
- `mf-atlas`
- `mf-registry`
- `mf-observe`

Actions:

- rekey semantic records to canonical ID and DNA digest
- require lineage for claim, route, adequacy, bridge, operator, proof-shape, and coverage records
- make generator/tower growth blocker-driven and lineage-grounded
- preserve challenge/remediation records as derived overlays

Invariants:

- no floating report semantics
- no theorem/campaign object becomes authority without DNA lineage
- upper systems can be replayed or challenged

Tests:

- seeded campaign regression
- imported claim regression
- lineage-required rejection tests
- coverage reuse tests

Exit condition:

- upper stack survives as derived capability above the substrate

Downstream payoff:

- the extensive current semantic system becomes an asset rather than substrate debt

### Node 16 - Conformance, Torture, And Release Gates

Purpose:

- turn the rail into a shipping standard

Depends on:

- Node 15

Code targets:

- `mf-testkit`
- `scripts`
- release docs
- CI or local release commands

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

## 24. Definition Of Done

The rail is implemented when:

- required DNA sections contain no semantic text needed for validation or execution
- SSR cannot be serialized as authority
- KGRAPH cannot become a public identity layer
- CNORM identity is independent of presentation strings and source formatting
- DNA validates from structural sections alone
- execution traverses DNA or canonical topology directly
- every public CLI command maps to phase-engine transitions and ledger entries
- semantic systems are lineage-keyed derived overlays
- compatibility imports are clearly marked and cannot become authority without recompilation
- conformance, fuzz, torture, replay, migration, and cross-platform determinism tests pass

## 25. Final System Definition

Locus64 is a deterministic structural substrate in which symbolic interaction surfaces are normalized into canonical graph structure, encoded into reconstructive machine form, executed under lineage-preserving authority, and amortized through lawful structural reuse.

Compact form:

```text
syntax is transient
structure is authority
canonical form is identity
reuse is proven rather than assumed
```
