# l64-atlas-export

A narrow atlas export utility for compiling Locus64 atlas cells from the seed registry or an imported DNA bundle overlay.

It does not introduce a second atlas schema. It reuses native `regime`, `bridge`, `atlas`, `claim-packet`, and bundle-overlay authority, then exports route summaries, compiled edges, source cells, and a Graphviz projection.

```text
l64-atlas-export --file framework.dna --overlay-only --dump-edges --dump-cells \
  --out framework.atlas.json --dot-out framework.atlas.dot
```
