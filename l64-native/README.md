# l64-native

`l64-native` owns canonical `L64R1` RNA, canonical `L64D` DNA, the native graph, checked admission, proof-producing equality, context-relative closure, exact authority identity, and the canonical codec.

## Authority boundary

- Positive identity is exact equality of the domain-qualified canonical representation.
- The fixed-width symbolic seal is a non-authoritative fast inequality and structural-diagnosis carrier.
- RNA and DNA decoding re-execute structural, constraint, equality, context, operation, and ordering law before acceptance.
- Failed admission is atomic: authority, contexts, indexes, and journal state remain unchanged.
- Derived indexes and closure analyses are in-memory accelerators only. They do not serialize and do not participate in authority identity.

## Native graph

One graph carries routes, nodes, compact ports, persistent contexts, constraints, judgments, witnesses, obligations, and the transition journal. The kernel admits typed operations and deterministic evidence routes; generic value insertion cannot fabricate kernel evidence.

The constraint core provides packed seven-axis dimensions, dimension-checked quantity operations, contradiction rejection, guarded square root, and native unresolved obligations. The equality core provides reflexivity, exact identity, symmetry, transitivity, and congruence with checked premise provenance. Incremental closure derives `Closed`, `Open`, or `Invalid` state per context without persisting a second authority database.

## Canonical carriers

- `L64R1`: compact canonical authored RNA with one domain and strictly increasing local slots.
- `L64D`: bounded binary DNA with a fixed 44-byte header, exact canonical payload, and symbolic state seal.
- `L64S1`: exact self-delimiting identity envelope provided by `l64-symbolic`.

The exact fixed point is `L64R1 → L64D → L64R1 → L64D`.

## Construction and scale

Canonical source compilation uses private bulk construction, suppresses transient construction journaling, and derives the final state symbol once. Public graph mutation remains fully journaled. Primary routes, reverse dependencies, context-local nodes, and closure memoization are derived from canonical authority and rebuilt after decode.

Measured scale law is enforced by `scripts/verify-native-scale.sh`; no optimization may alter canonical bytes, public mutation semantics, verdict law, or retained/external projection verification.

## Relationships

- `l64-projection` derives verified read-only views from `l64-native`.
- `l64-execution`, `l64-certification`, `l64-observation`, and `l64-change` consume exact native authority without becoming authority.
- `l64-transport` frames ordered DNA members without composite authority.
- `l64-cli` and the `l64` wrapper expose the product boundary.

The historical compatibility authority island was exported and deleted. No live legacy dispatcher, registry, packet world, cache, or fallback remains.

## Direct verification

```bash
cargo test --locked --offline -p l64-native
./scripts/verify-native-carrier.sh
./scripts/verify-native-scale.sh
```

Current architectural law: [`../LOCUS64_NATIVE_CONSTITUTION.md`](../LOCUS64_NATIVE_CONSTITUTION.md). Historical correction chain: [`../LOCUS64_HISTORICAL_LAW_LEDGER.md`](../LOCUS64_HISTORICAL_LAW_LEDGER.md).
