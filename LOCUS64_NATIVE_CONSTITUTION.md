---
document_status: current
constitution_scope: live dependency-free native workspace
workspace_packages: 11
external_dependencies: 0
ambient_legacy_authority: deleted_tombstones_only
---

# Locus64 Native Constitution

This is the current architecture law for the live Locus64 workspace. It is derived from current code, wire formats, product gates, and direct command behavior. Historical laws remain recoverable in [`LOCUS64_HISTORICAL_LAW_LEDGER.md`](LOCUS64_HISTORICAL_LAW_LEDGER.md), but they cannot authorize current behavior.

## 1. Authority

1. Canonical `L64R1` RNA and canonical `L64D` DNA are the only source-authority surfaces.
2. Positive authority equality is exact equality of the domain-qualified canonical representation. A compact symbolic seal may prove fast inequality, but a matching seal never proves equality.
3. Canonical decoding must revalidate structure, contexts, constraints, equality evidence, operation law, ordering, framing, and exact re-encoding before authority is accepted.
4. Admission is transactional: a failed transition does not mutate authority, context, route indexes, or journal state.
5. Names, projections, reports, process codes, CLI formatting, caches, package topology, and historical ledgers are not authority.

Evidence: `l64-symbolic`, `l64-native`, `scripts/verify-native-carrier.sh`, full locked-offline workspace tests.

## 2. Native structure and proof

1. Routes, nodes, ports, contexts, constraints, judgments, witnesses, obligations, and the transition journal share one native graph authority.
2. Dimensions and guarded operations are checked at admission. Proven guards receive native evidence; refuted guards reject; unresolved guards create native obligations.
3. Equality is proof-producing and context-scoped. Reflexivity, exact identity, symmetry, transitivity, and congruence are re-executed during decode.
4. Equivalence classes, representative caches, reverse indexes, route indexes, and closure memoization are derived accelerators. They do not serialize into RNA or DNA and do not participate in authority identity.
5. Closure is context-relative and may be `Closed`, `Open`, or `Invalid`; assumption changes create child-context refinements rather than rewriting prior authority.

Evidence: `l64-native/tests/constraints.rs`, `l64-native/tests/equality.rs`, `l64-native/tests/closure.rs`, full locked-offline workspace tests.

## 3. Transport

1. `L64B` is an ordered frame of exact canonical `L64D` members.
2. A bundle preserves order and member boundaries but creates no composite authority, certificate, observation, change verdict, registry, name resolution, overlay, or conflict policy.
3. Bundle readers and writers process members sequentially so transport scale does not require retaining the complete bundle and all decoded graphs simultaneously.

Evidence: `l64-transport`, `scripts/verify-native-carrier.sh`, `scripts/verify-process-contract.sh`.

## 4. Execution and derivation

1. `l64-execution` evaluates canonical RNA or DNA directly and non-persistently.
2. Execution does not mutate authority, invent runtime authority, or persist scheduler state.
3. Projection, certification, observation, and change are verified read-only derivatives over exact authority.
4. Fresh in-process projections may be trusted by construction only inside the deriving operation. Retained or externally supplied projections require explicit rederive-and-compare verification.
5. Certification scope is structural and context-local. It cannot silently broaden into theorem truth, campaign truth, policy truth, or external truth.

Evidence: `l64-projection`, `l64-execution`, `l64-certification`, `l64-observation`, `l64-change`, current native carrier gates.

## 5. Release

1. A native release contains exactly four direct contacts: canonical RNA, canonical DNA, one verified projection, and one small release record binding them.
2. Release rendering does not create a second authority schema.
3. File-producing commands stage output atomically and refuse to overwrite an existing destination.

Evidence: `l64-release`, `scripts/verify-native-carrier.sh`, `scripts/verify-cli-hardening.sh`.

## 6. Command surfaces

1. `l64-cli` owns the complete native command grammar and executes every current authoring, execution, derivation, transport, and release command directly.
2. `l64` is a small sibling-binary wrapper. It adds only `authority-audit`; every other argument vector is forwarded unchanged to the colocated `l64-cli`, and the exact child exit code is preserved.
3. The wrapper is not a second dispatcher, parser, authority layer, fallback membrane, or compatibility runtime.
4. Wrong carrier formats fail at the requested command contact. Dead commands remain permanent tombstones and cannot route to executable compatibility behavior.
5. Process results are projections over completed computation: `0` success/certified, `10` open, `11` incomplete, `12` invalid, and `2` usage/input/filesystem/processing failure.

Evidence: `l64-cli`, `l64`, `LOCUS64_LANGUAGE_SPEC.md`, `scripts/verify-cli-hardening.sh`, `scripts/verify-process-contract.sh`.

## 7. Historical boundary

1. The legacy theorem, campaign, research, registry, tower, packet, policy, cache, planning, administration, and compatibility execution island was externally recovered and deleted.
2. No live compatibility dispatcher remains. The token `legacy` is a rejection tombstone only.
3. Historical documents may preserve deleted models when explicitly marked `document_status: historical`; they cannot be used as current source-of-truth without current-code verification.
4. Reintroducing a deleted authority contact requires a new explicit authority law and proof. Historical familiarity is not evidence.

Evidence: `scripts/verify-legacy-authority-island-deletion.sh`, workspace architecture tests, `changelog.log`.

## 8. Scale and performance

1. Performance changes require a measured structural cause.
2. Optimization may remove repeated derivation, allocation, scanning, or transient journaling only when exact authority, wire bytes, member order, verdict law, stale/forged projection rejection, and public mutation semantics remain unchanged.
3. Canonical source compilation may bulk-construct and derive the final state once; public graph mutation remains fully tracked.
4. No generic digest service, schema router, cache bureaucracy, or alternate graph may be introduced to conceal local work.
5. The live workspace contains eleven packages, zero external Rust dependencies, and must remain buildable and testable with locked offline Cargo.

Evidence: `scripts/verify-native-scale.sh`, native scale and fixed-point tests, full workspace tests and Clippy.

## 9. Documentation law

1. Current-facing documentation must reconstruct the live package graph, command surface, authority boundary, process law, and deletion state.
2. Historical material must be visibly historical before its first substantive claim.
3. Current scripts may not invoke deleted crates or removed commands.
4. Every command listed in the native language specification must appear in live CLI help, except the wrapper-local `authority-audit`, which must execute through `l64` only.
5. Documentation validation must remain a direct contact check over files, Cargo membership, and live help. It may not become a document registry or a parallel architecture database.

Evidence: `scripts/verify-documentation-coherence.sh`, current-document contact checks, live CLI help.

## 10. Golden workload and portability law

1. Representative workload snapshots are executable compatibility evidence, not authority.
2. Canonical RNA, canonical DNA bytes, stable derivative text, verdict status, ordered transport behavior, and native release contents must remain identical across supported hosts unless their governing law is deliberately changed.
3. LF and CRLF textual inputs must normalize to the same LF canonical RNA and exact DNA.
4. Valid paths containing spaces and Unicode must pass through native path contacts without lossy string conversion.
5. The dependency-free Rust workspace must execute its complete tests on Linux, macOS, and Windows. Host-specific shell orchestration cannot become a runtime dependency.
6. Each successful host execution must emit one machine-readable receipt bound to the exact source commit; the three-host set must be verified as one source before portability is called closed.
7. Portability receipts are external execution evidence only. They cannot enter native authority, transport, runtime persistence, certification, observation, or change.
8. A golden snapshot may block accidental drift but cannot promote a new authority law by itself.

Evidence: `LOCUS64_GOLDEN_PORTABILITY_CONTRACT.md`, `LOCUS64_PORTABILITY_RECEIPT_CONTRACT.md`, `l64-cli/src/golden_tests.rs`, `scripts/verify-golden-portability.sh`, `scripts/verify-portability-receipts.sh`, host-matrix CI artifacts.

## 11. Promotion law

A new architectural law becomes current only when:

1. the operational contact it governs is named;
2. executable evidence is identified;
3. the change preserves or explicitly replaces every dependent law;
4. the relevant direct gate passes;
5. full locked-offline workspace tests and Clippy pass;
6. current-facing documentation and the historical ledger are updated in the same change.

Documentation alone cannot promote behavior. Passing code alone cannot leave contradictory current documentation behind.
