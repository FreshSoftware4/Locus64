# Symbolic commitment law

## 1. Separation of authority and acceleration

For domain `D` and canonical structure `x`, exact identity is the self-delimiting term

```text
I(D, x) = L64S1 ‖ |D| ‖ |x| ‖ D ‖ x
```

where lengths are fixed-width little-endian integers. Equality of `I(D, x)` is ordinary byte equality, so no collision assumption is involved.

The compact seal

```text
S(D, x) = ⟦D ∷ Σ(x) ⊗ Π(x) ⊗ Δ(x) ⊗ Ω(x)⟧
```

is a 256-bit projection of the exact term. `S(a) ≠ S(b)` proves inequality. `S(a) = S(b)` is only permission to continue to exact comparison.

## 2. Composition grammar

- `∷` qualifies a term by domain.
- `⊗` composes an ordered sequence; permutation changes the result.
- `⊕` composes a canonical set after sorting; permutation does not change the result.
- `↦` binds a source, relation, and result as a derivation.
- `⟦…⟧` delimits a symbolic commitment expression.

Every field is length-delimited and marked by a distinct boundary token. Domain, label, value, order, segment count, and total absorbed length therefore occupy explicit positions in the input algebra.

## 3. Coordinate roles

All arithmetic is performed modulo the prime `2^64 - 59`.

### `Σ` — aggregate content mass

A weighted modular sum over tokens and position. It intentionally changes more locally than the other coordinates and supports coarse change diagnosis.

### `Π` — ordered composition

A Horner-style polynomial recurrence. It distinguishes sequence order and broadly diffuses changes.

### `Δ` — adjacent transition structure

A recurrence over squared token-to-token differences. It distinguishes changes in local transitions even when aggregate content is similar.

### `Ω` — nonlinear boundary closure

A quadratic recurrence influenced by token, position, and segment boundaries. It supplies broad diffusion and boundary sensitivity.

The coordinates are deliberately nonredundant. A seal is interpreted as their conjunction, not as four interchangeable random words.

## 4. Native state decomposition

The native graph derives independent seals for:

```text
nodes ⊗ ports ⊗ contexts ⊗ routes
```

The root state symbol composes these section seals with codec version and section counts. This permits fast rejection and localizes which structural family changed without introducing a second graph or stored index.

## 5. Verification law

1. Reject malformed framing or a mismatched compact seal.
2. Decode the canonical payload under native structural law.
3. Re-execute validation rules.
4. Re-encode canonically and require the exact fixed point.
5. Use exact identity for positive authority equality.

No compact symbolic seal, including a matching seal, may bypass steps 2–5.

## 6. Security and trust boundary

The seal is not claimed as a cryptographic hash, signature, MAC, or adversarial authenticity mechanism. An unkeyed digest does not establish who produced an artifact. Authenticity belongs to signed transport or another explicit trust carrier. The symbolic system replaces BLAKE3 inside native state identity by removing probabilistic equality from the authority decision, not by making an unsupported cryptographic claim.
