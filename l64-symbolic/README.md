# l64-symbolic

`l64-symbolic` separates **exact symbolic identity** from **compact symbolic sealing**.

- Exact identity is the domain plus canonical bytes. Equality is therefore injective by representation, not probabilistic.
- A `SymbolicSeal` is a fixed-width, composable projection used for fast inequality, frame corruption checks, indexing, and developer-facing state references. It is never sufficient authority for equality.

## Algebra

A composer absorbs an explicitly tagged token stream under the prime modulus `2^64 - 59` and exposes four independently purposed coordinates:

- `Σ` — weighted aggregate mass;
- `Π` — ordered Horner composition;
- `Δ` — adjacency/change structure;
- `Ω` — nonlinear boundary closure.

Ordered composition uses `⊗`; commutative composition uses `⊕` after canonical sorting; derivation uses `↦`; domain qualification uses `∷`; `⟦…⟧` marks the symbolic boundary.

The pretty form is intentionally readable:

```text
⟦l64.native.state.v2 ∷ Σ… ⊗ Π… ⊗ Δ… ⊗ Ω…⟧
```

The stable ASCII form is parser-safe:

```text
s1:<sigma>.<pi>.<delta>.<omega>
```

## Security boundary

The compact seal is not presented as a new cryptographic primitive. Exact canonical bytes remain authority. A seal may reject inequality early, but positive equality requires exact canonical reconstruction or byte equality. This gives Locus64 stronger system-level discipline than substituting an unaudited home-grown digest for BLAKE3.
