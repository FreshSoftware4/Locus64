# Usage Guide

## Binary choice

Use `l64-cli` for the complete command grammar. Use `l64` as the installed convenience entrypoint when both sibling binaries are packaged together. `l64` forwards every ordinary command unchanged to `l64-cli` and preserves its exit code; it adds only one local diagnostic command:

```bash
l64 authority-audit
```

The wrapper does not parse or reinterpret authority and is not a second execution layer.

## Discovery

```bash
cargo run -p l64-cli -- --help
cargo run -p l64-cli -- help compile-rna
cargo run -p l64-cli -- --version
cargo run -p l64 -- authority-audit
```

## Authoring, execution, derivation, and transport

```bash
cargo run -p l64-cli -- normalize-rna samples/native_triangle.rna
cargo run -p l64-cli -- run-rna samples/native_triangle.rna
cargo run -p l64-cli -- compile-rna samples/native_triangle.rna --out triangle.dna
cargo run -p l64-cli -- run-dna triangle.dna
cargo run -p l64-cli -- sequence-dna triangle.dna
cargo run -p l64-cli -- inspect-dna triangle.dna
cargo run -p l64-cli -- certify-dna triangle.dna
cargo run -p l64-cli -- observe-dna triangle.dna
cargo run -p l64-cli -- compare-dna before.dna after.dna
cargo run -p l64-cli -- compile-bundle first.dna second.dna --out work.l64b
cargo run -p l64-cli -- run-bundle --file work.l64b
```

Former `legacy`, theorem, campaign, research, registry, and tower commands are permanent deletion tombstones.

End-to-end offline demo:

```bash
./scripts/run-native-demo.sh /tmp/l64-demo
```

`compile-rna`, `compile-bundle`, and `export-genome-release` refuse existing outputs. Choose a new path or remove the old output deliberately; there is no implicit force mode.

Wrong carrier formats fail at the requested command contact, and malformed RNA/DNA diagnostics use stable human-readable messages rather than Rust debug syntax.

## Process outcomes

Verdict-bearing commands emit their normal text before returning:

- `0`: command succeeded; certification is `CERTIFIED` when applicable
- `10`: `OPEN` burdens remain
- `11`: certification is `INCOMPLETE` because evidence is missing
- `12`: certification is `INVALID`
- `2`: usage, input, filesystem, or processing failure

Malformed RNA diagnostics identify the exact token span. `L64B` commands decode, execute, certify, observe, and compare members sequentially; bundle creation writes members sequentially into the atomic staging output.

## Golden portability verification

The repository carries five exact authority workloads plus a complete compare, bundle, and release journey. Run the host-neutral snapshot tests through the direct gate:

```bash
./scripts/verify-golden-portability.sh
```

The same Rust workspace tests run in CI on Linux, macOS, and Windows. LF and CRLF sources must compile to identical authority bytes, and valid paths containing spaces or Unicode must not change command behavior. See [`LOCUS64_GOLDEN_PORTABILITY_CONTRACT.md`](LOCUS64_GOLDEN_PORTABILITY_CONTRACT.md).
