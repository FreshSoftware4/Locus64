# Locus64 Native Language Specification

## Authority surfaces

- `L64R1` is canonical textual RNA.
- `L64D` is canonical binary DNA.
- RNA normalization is deterministic.
- RNA → DNA → RNA → DNA is an exact fixed point.
- DNA decoding revalidates structure, constraints, equality proofs, contexts, operation law, canonical ordering, and exact re-encoding.

## Transport

`L64B` is a versioned ordered frame containing length-prefixed canonical `L64D` members. It has no bundle registry, names, overlays, conflict policy, composite certificate, or composite authority.

## Derived surfaces

Projection, certification, observation, and change outputs are verified read-only derivatives. They may reject or expose invalid authority, but they never become source authority.

## Binary surfaces

- `l64-cli` owns the complete native command grammar and executes every current command directly.
- `l64` adds only `authority-audit`. Every other argument is forwarded unchanged to the sibling `l64-cli`, and the child process status is preserved.
- The wrapper is not a second parser, execution layer, authority path, or compatibility fallback.

## `l64-cli` commands

- `normalize-rna`
- `compile-rna`
- `sequence-dna`
- `inspect-dna`
- `verify-roundtrip`
- `export-genome-release`
- `run-rna`
- `run-dna`
- `compile-bundle`
- `run-bundle`
- `certify-dna`
- `certify-bundle`
- `observe-dna`
- `observe-bundle`
- `compare-dna`
- `compare-bundle`

## `l64`-local command

- `authority-audit`

All historical theorem, campaign, research, registry, producer-host, tower, packet, policy, cache, planning, administration, and `legacy` execution commands are permanently deleted tombstones.

## Process semantics

Certification-bearing commands are successful computations even when their structural verdict is not certified. They emit the complete result and return `0` for `CERTIFIED`, `10` for `OPEN`, `11` for `INCOMPLETE`, and `12` for `INVALID`. Operational failure returns `2`. These codes are process projections and do not alter authority.

RNA syntax failures carry one exact byte span `(line, column, length)`. Diagnostics render that span against the supplied source. `L64B` framing remains unchanged; sequential readers and writers are implementation carriers over the same exact ordered bytes.
