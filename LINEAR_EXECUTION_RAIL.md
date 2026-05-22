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

## 23. Path-Optimized Implementation Order

The implementation order is:

1. Rename references and docs to `LINEAR_EXECUTION_RAIL.md`.
2. Add or verify BLAKE3 canonical identity support.
3. Freeze token classes and byte classification in a single Rust module.
4. Make RNORM consume token streams rather than independent string rules.
5. Refactor SSR so no persistent SSR identity can leak.
6. Introduce `KGraph` as explicit ephemeral transfer object.
7. Introduce `CanonicalTopology` as CNORM output.
8. Rebuild DNA required sections around structural opcode encoding.
9. Add DNA validation as its own phase before execution.
10. Route CLI commands through the phase execution kernel.
11. Rekey research/cert/tower objects to canonical DNA lineage.
12. Demote QC0/JSON/report paths to compatibility projection adapters.

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
