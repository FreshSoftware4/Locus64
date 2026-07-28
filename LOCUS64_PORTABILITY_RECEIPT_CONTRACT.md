---
document_status: current
contract_scope: source-bound remote portability evidence
receipt_schema: L64PORT1
---

# Locus64 Portability Receipt Contract

The host matrix proves portability only when each runner emits a machine-readable receipt bound to the exact source commit it tested. Workflow configuration and transient logs are not substitutes for retained evidence.

## Receipt law

Each successful host produces exactly one `L64PORT1` JSON receipt containing:

- the exact Git commit in `source_ref`;
- repository, workflow-run, and attempt coordinates;
- the declared GitHub runner label;
- the actual Rust host triple, `rustc`, and Cargo versions;
- the live Cargo-derived package count;
- the external-dependency count;
- the exact test command and a `passed` result.

The supported set is exactly:

- `ubuntu-latest`;
- `macos-latest`;
- `windows-latest`.

A combined verifier accepts the set only when all three receipts exist, bind the same source, retain eleven workspace packages, retain zero external Rust dependencies, and report the complete locked-offline workspace test as passed.

## Authority boundary

A portability receipt is execution evidence. It is not mathematical authority, canonical RNA, canonical DNA, a transport member, a runtime registry, or a substitute for exact product outputs. It may prove that a named source executed on a named host; it cannot promote behavior or change authority law.

## Direct contacts

- `scripts/write-portability-receipt.sh` writes one receipt after a successful host test.
- `scripts/verify-portability-receipts.sh` verifies the exact three-host set and provides a local self-test.
- `.github/workflows/native-core.yml` uploads each host receipt, combines the three artifacts, verifies the common source, and uploads the accepted set.
