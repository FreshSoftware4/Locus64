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

The fourth boundary adds a native DNA frame with a fixed 44-byte binary header, bounded canonical payload, compact state field, and exact DNA decode/re-encode fixed point. The original frame used BLAKE3; the tenth boundary replaces that field with a composed symbolic seal while preserving the header width. The frame contains no string metadata or legacy record payload.

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

Native authority identity is exact: the domain-qualified canonical byte sequence itself. A composed symbolic seal provides a fixed-width state reference and fast inequality check, but seal equality never substitutes for exact canonical comparison. No native name, claim identifier, theorem identifier, campaign identifier, JSON field name, or generic serialization schema participates.

The existing `l64-cli` command names now route `L64R1` and `L64D` directly through this native path. Legacy RNA/DNA behavior is classified as compatibility/forensic ingress and is available explicitly through `l64-cli legacy ...`; ambient fallback remains temporarily available with a mandatory deprecation warning.

This remains additive. Native upper projections now exist as read-only derivations in `l64-projection`; replacement and quarantine of the legacy runtime, registry, certification, and old packet implementation remain incomplete.

The eighth boundary adds incremental closure without turning invalidation into a second authority database:

- reverse dependencies and context-local node lists are derived from canonical type/port incidence and rebuilt after decode;
- assumption change is represented by an immutable direct child-context refinement, preserving prior authority in its original scope;
- guarded operations, their judgments, evidence, downstream operations, and equality proofs receive context-relative `Closed`, `Open`, or `Invalid` closure states;
- closure transitions identify the exact reverse-reachable subgraph whose state changed and carry the constraint binding that caused the transition;
- independent structure remains outside the affected set;
- local and global closure are distinguishable;
- closure queries do not alter canonical bytes, state symbols, routes, contexts, or journal history.

The derived reverse index is an in-memory accelerator only. It is excluded from RNA, DNA, state symbols, and authority identity.

The ninth boundary derives the first native upper views without turning any view into authority:

- `l64-projection` depends only on `l64-native`;
- atlas candidates are reconstructed from executable operations, equalities, ports, routes, closure, and native evidence;
- certification burdens distinguish discharged, open, invalid, and missing-evidence states without rewriting obligation nodes;
- replay is a deterministic view of the native journal and rejects a canonical decode that lacks that runtime history;
- reporting counts visible native structure and exposes obligation and invalid routes;
- research ranking follows open, invalid, and high-impact reverse-reachable structure;
- every view binds to the native state symbol, context, structural counts, journal length, and projection version;
- verification rebuilds the complete view and requires exact equality;
- the projection crate contains no storage, registry, cache, alternate graph, import, promotion, serialization, or hash authority.

`LOCUS64_NATIVE_UPPER_PROJECTION_CHANGE_CHAIN.athens` is complete on repository evidence. The parent execution rail has advanced to legacy authority quarantine.

The tenth boundary removes BLAKE3 from the native execution and projection spine through a purpose-built symbolic identity system:

- `l64-symbolic` separates exact identity from compact sealing;
- exact identity is a self-delimiting `L64S1` envelope containing domain and canonical bytes, so authoritative equality is collision-free by representation;
- compact `SymbolicSeal` values expose `Σ`, `Π`, `Δ`, and `Ω` coordinates for aggregate content, ordered composition, adjacent transitions, and nonlinear boundary closure;
- ordered (`⊗`), commutative (`⊕`), derivational (`↦`), and domain-qualified (`∷`) composition are explicit operations rather than hidden byte concatenation;
- `StateSymbol` independently composes nodes, ports, contexts, and routes, allowing a changed state to identify the structural section that moved;
- journal events and projections carry symbolic seals, while `Graph::exact_state_identity()` remains the positive equality boundary;
- DNA v2 retains the 44-byte frame but stores the symbolic state seal; v1 is rejected instead of ambiguously reinterpreted;
- seal equality is only a fast-match signal. Canonical decoding, validation, and exact re-encoding remain mandatory before authority is accepted;
- the native and projection crates contain no BLAKE3 dependency.

Legacy `l64-core` still uses BLAKE3-backed string digests across its pre-native record, cache, receipt, and packet surfaces. That dependency is now confined to the legacy-authority-quarantine boundary; removing it requires retiring or exactly translating those roles, not substituting another opaque digest.
