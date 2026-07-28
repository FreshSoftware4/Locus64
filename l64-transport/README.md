# l64-transport

`l64-transport` is the dependency-free native bundle carrier.

A native bundle is an ordered `L64B` frame containing one or more canonical current-authority `L64D` members. The transport layer adds only framing:

- a 16-byte versioned header;
- a member count;
- exact length framing for each DNA member.

It does not add names, hashes, manifests, registries, overlays, conflict policies, receipts, caches, or composite authority. Member order is transport sequence only. Every member is decoded through `l64-native`, re-encoded to prove canonical DNA, and projected through `l64-projection` during execution.

The public contacts are:

- `bundle_bytes` — validate and frame canonical DNA members;
- `decode_bundle` — exact framing and member-authority validation;
- `execute_bundle` — verify one native projection per member in transport order.
