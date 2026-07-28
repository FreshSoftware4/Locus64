---
document_status: current
contract_scope: golden workload and host portability
supported_hosts: Linux, macOS, Windows
---

# Locus64 Golden Workload and Portability Contract

This contract fixes the smallest representative workload set that can detect drift across authority, execution, verdict, transport, release, text, path, and host boundaries. It does not create a second specification or promote snapshots into authority. Canonical RNA and DNA remain the only source-authority surfaces; the snapshots are executable comparison evidence.

## Golden authority workloads

| Workload | Native contact exercised | Required verdict | Distinction protected |
|---|---|---:|---|
| `certified_triangle.rna` | typed function composition with native witness | `CERTIFIED` / `0` | proof-producing operation and exact fixed point |
| `equality_chain.rna` | reflexive, symmetric, and transitive equality evidence | `CERTIFIED` / `0` | equality replay and canonical ordering |
| `open_obligation.rna` | unresolved guarded square root | `OPEN` / `10` | open burden remains visible and non-fatal |
| `invalid_child_context.rna` | child context with refuted non-negative guard | `INVALID` / `12` | context-local invalidity propagates without rewriting the root |
| `matrix_multiply.rna` | valid matrix shape admission | `CERTIFIED` / `0` | typed shape law and witness production |

For every workload, the repository retains exact snapshots of:

- canonical normalized RNA;
- canonical `L64D` bytes rendered as lowercase hexadecimal;
- direct RNA execution text;
- direct DNA certification text.

The product-journey workload additionally fixes:

- exact DNA comparison output;
- ordered mixed-verdict bundle execution, certification, and observation;
- the four-file native release contents and exact release record.

## Portability law

1. The same canonical RNA yields the same canonical DNA bytes on every supported host.
2. LF and CRLF RNA normalize to the same LF canonical RNA and the same DNA bytes.
3. Stable text outputs use `\n` and must match the golden snapshots byte-for-byte.
4. Paths containing spaces and valid Unicode must work through direct `Path`/`OsString` contacts without text round-tripping.
5. Wire integers, lengths, ordering, and symbolic coordinates are explicitly encoded; host pointer width and host endianness cannot enter authority identity.
6. `l64` selects the platform-appropriate sibling `l64-cli` executable without changing the forwarded argument vector or exit code.
7. The complete dependency-free Rust workspace must compile and execute its tests on Linux, macOS, and Windows in CI.
8. Each successful host must emit one `L64PORT1` receipt bound to the exact tested commit. Portability closes only when the combined verifier accepts the exact three-host set.
9. Portability receipts remain external execution evidence and cannot become runtime or authority state.
10. Unix shell gates remain release-orchestration contacts, not runtime requirements. Product behavior is proven by host-neutral Rust tests beneath them.
11. Build-and-execute gates on both Unix and Windows must honor `CARGO_TARGET_DIR`; forge and execution cannot silently resolve different binary locations.

## What the snapshots may and may not do

A snapshot may reject an unintended behavioral change. It cannot prove that the behavior is mathematically sufficient, cryptographically secure, or semantically complete. A deliberate snapshot change requires the same named operational contact, law change, current documentation update, and full workspace proof as any other promoted behavior.

## Direct evidence

- `samples/golden/*.rna`
- `samples/golden/expected/*`
- `l64-cli/src/golden_tests.rs`
- `l64/src/main.rs` portability test
- `scripts/verify-golden-portability.sh`
- `LOCUS64_PORTABILITY_RECEIPT_CONTRACT.md`
- `scripts/write-portability-receipt.sh`
- `scripts/verify-portability-receipts.sh`
- `.github/workflows/native-core.yml` host matrix and retained receipt artifacts
