# l64-native

A deliberately small, additive execution spine for Locus64.

This crate is not a projection of the legacy `QaEntry` / `RegistryBundle` ontology. It carries:

- composed numeric routes;
- one node algebra for types, values, and operations;
- ordered compact ports;
- persistent context deltas;
- one transition journal;
- one canonical structural codec;
- one transactional admission path.

The first workload boundary proves that lawful typed function composition commits while incompatible matrix multiplication is rejected without state mutation.

The second boundary adds a bounded native decoder. Canonical bytes must decode, satisfy structural laws, and re-encode byte-for-byte. The decoder rejects malformed node order, invalid port laws, bad contexts, invalid route coverage, unknown opcodes, structural overrun, trailing bytes, and non-canonical ordering.

State identity is the domain-separated BLAKE3 commitment of canonical native bytes. No native name, claim identifier, theorem identifier, campaign identifier, JSON field name, or generic serialization schema participates.

This remains additive. It does not yet replace the legacy runtime or DNA packet path.
