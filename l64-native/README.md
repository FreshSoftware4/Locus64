# l64-native

The first additive native execution slice for Locus64.

This crate deliberately does less:

- one node algebra represents types, values, and operations;
- composed numeric routes replace native names;
- ordered ports carry incidence;
- persistent context deltas carry scope;
- one transition journal records committed changes;
- one canonical byte encoder determines a provisional dependency-free state stamp;
- one transaction path validates before mutation.

The first workloads are intentionally narrow:

1. typed function composition must commit;
2. incompatible matrix multiplication must return a structured obstruction and leave state unchanged.

The crate has no dependency on the legacy registry, bundle, report, or certification object families. It is additive and cannot yet replace those systems.

The first-pass stamp is deterministic but not cryptographic. DNA v2 must bind the same canonical bytes to the repository's domain-separated cryptographic commitment implementation.
