# l64-native

A deliberately small, additive execution spine for Locus64.

This crate is not a projection of the legacy `QaEntry` / `RegistryBundle` ontology. It carries:

- composed numeric routes;
- one node algebra for types, values, constraints, operations, judgments, witnesses, and obligations;
- ordered compact ports;
- persistent context deltas with contradiction checks;
- one transition journal;
- one canonical structural codec;
- one transactional admission path.

The first workload boundary proves that lawful typed function composition commits while incompatible matrix multiplication is rejected without state mutation.

The second boundary adds a bounded native decoder. Canonical bytes must decode, satisfy structural laws, and re-encode byte-for-byte. The decoder rejects malformed node order, invalid port laws, bad contexts, invalid route coverage, unknown opcodes, structural overrun, trailing bytes, and non-canonical ordering.

The third boundary makes primitive execution proof-carrying without adding a receipt schema. Every admitted operation receives a judgment type and kernel witness at routes deterministically composed from the operation route. Generic value insertion cannot construct a witness for a kernel-only judgment.

The fourth boundary adds a native DNA frame with a fixed 44-byte binary header, bounded canonical payload, embedded domain-separated BLAKE3 commitment, and exact DNA decode/re-encode fixed point. The frame contains no string metadata or legacy record payload.

The fifth boundary adds a compact authored RNA ingress. `L64R1` uses one declared domain and strictly increasing numeric local slots; routes are composed as `(domain, slot)`. Its byte-oriented instructions lower directly through the existing graph and transaction APIs. Sequencing omits intrinsic evidence nodes because their routes and structure are deterministically derived.

The sixth boundary adds the first native constraint core without creating a parallel schema system:

- seven-axis signed dimensions packed into one 64-bit value;
- quantity types over existing carrier types;
- dimension-checked addition, multiplication, division, and square root;
- compact constraint nodes and persistent context deltas;
- contradiction rejection before context or journal mutation;
- a guarded square-root operation that yields a kernel witness when proven, rejects when refuted, and yields a native obligation when unresolved;
- decoder-side re-execution of operation and evidence law, preventing structurally plausible forged authority;
- canonical `L64R1 → L64D → L64R1 → L64D` fixed points for both discharged and unresolved guards.

`LOCUS64_CONSTRAINT_CORE_CHANGE_CHAIN.athens` is the promotion boundary for this sixth stage: dimension algebra, context consistency, guarded obligation, native surface, then full closure. The parent execution rail cannot advance until that chain closes on repository evidence.

The larger implementation files are factored only at existing item boundaries into construction, typing, transaction, validation, codec, and RNA concerns. This changes review locality without introducing another authority layer or altering canonical bytes.



The seventh boundary adds proof-producing congruence without promoting a union-find table into authority:

- equality is a native type judgment with a deterministically attached equality witness;
- primitive rules cover reflexivity, exact structural identity, symmetry, and transitivity;
- congruence lifts checked equalities through type constructors and executable operations;
- every premise points to an earlier equality judgment with its own checked witness;
- proof paths are returned deterministically and remain context-scoped;
- canonical representatives are selected by minimum composed route over the validated equality component;
- no equivalence class, representative cache, or merge table is persisted;
- decoder-side rule re-execution rejects forged rules, missing provenance, scope escape, and mismatched congruence premises;
- equality-bearing `L64R1` reaches the exact `L64D → L64R1 → L64D` fixed point.

`LOCUS64_PROOF_CONGRUENCE_CHANGE_CHAIN.athens` is complete on repository evidence. The parent execution rail has advanced to incremental dependency closure.

State identity is the domain-separated BLAKE3 commitment of canonical native bytes. No native name, claim identifier, theorem identifier, campaign identifier, JSON field name, or generic serialization schema participates.

The existing `l64-cli` command names now route `L64R1` and `L64D` directly through this native path. Legacy RNA/DNA behavior is classified as compatibility/forensic ingress and is available explicitly through `l64-cli legacy ...`; ambient fallback remains temporarily available with a mandatory deprecation warning.

This remains additive. It does not yet implement incremental dependency closure, native upper-stack projections, or replacement of the legacy runtime, registry, certification, and old packet implementation internally.

The eighth boundary adds incremental closure without turning invalidation into a second authority database:

- reverse dependencies and context-local node lists are derived from canonical type/port incidence and rebuilt after decode;
- assumption change is represented by an immutable direct child-context refinement, preserving prior authority in its original scope;
- guarded operations, their judgments, evidence, downstream operations, and equality proofs receive context-relative `Closed`, `Open`, or `Invalid` closure states;
- closure transitions identify the exact reverse-reachable subgraph whose state changed and carry the constraint binding that caused the transition;
- independent structure remains outside the affected set;
- local and global closure are distinguishable;
- closure queries do not alter canonical bytes, commitments, routes, contexts, or journal history.

The derived reverse index is an in-memory accelerator only. It is excluded from RNA, DNA, state commitments, and authority identity.
