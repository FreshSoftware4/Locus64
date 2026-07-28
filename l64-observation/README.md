# l64-observation

`l64-observation` is the read-only current-path observation carrier for exact native authority.

It binds three already-existing contacts for every native context:

- direct structural certification from `l64-certification`,
- verified replay projection from `l64-projection`,
- verified closure/opcode report projection from `l64-projection`.

The carrier does not persist reports, create campaigns, resolve policies, manage caches, or issue a second authority. A native bundle remains an ordered transport of independently observed `L64D` members and receives no composite authority, verdict, or observation.
