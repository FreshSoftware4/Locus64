# l64-projection

`l64-projection` derives read-only upper views from `l64-native::Graph` authority.

It provides five coordinated views over one native state and context:

- atlas candidates from executable operation and equality nodes;
- certification burdens from context-relative closure and native evidence;
- replay steps from the native journal;
- deterministic aggregate reporting;
- ranked research candidates from open and invalid reverse-reachable structure.

Every projection carries its native composed state symbol, context, structural counts, journal length, and projection version. Verification rebuilds the complete view from the graph and requires exact equality. The crate has no storage, registry, cache, import, promotion, serialization, independent hashing, or alternate graph dependency. It reuses the symbolic identity carried by native authority.

Projection records are expendable. Native authority remains canonical.
