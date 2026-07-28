#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO="${CARGO:-cargo}"
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
CLI="$TARGET_DIR/debug/l64-cli"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

cd "$ROOT"
"$CARGO" build --locked --offline -q -p l64-cli

VERSION=$(awk -F '"' '/^version = "/ { print $2; exit }' Cargo.toml)
"$CLI" --version | grep -Fx "l64-cli $VERSION" >/dev/null
"$CLI" help run-rna | grep -F "usage: l64-cli run-rna <file.rna>" >/dev/null

printf 'not native\n' > "$WORK/not-rna.txt"
if "$CLI" run-rna "$WORK/not-rna.txt" >"$WORK/out" 2>"$WORK/error"; then
  echo "run-rna accepted a non-L64R1 input" >&2
  exit 1
fi
grep -F 'is not current L64R1 RNA' "$WORK/error" >/dev/null
if grep -F 'legacy theorem' "$WORK/error" >/dev/null; then
  echo "wrong-format error fell back to deleted legacy messaging" >&2
  exit 1
fi

printf 'L64R1 0x1\na X 0x41\n' > "$WORK/bad.rna"
if "$CLI" run-rna "$WORK/bad.rna" >"$WORK/out" 2>"$WORK/error"; then
  echo "run-rna accepted malformed RNA" >&2
  exit 1
fi
grep -F 'invalid number at RNA 2:3' "$WORK/error" >/dev/null
if grep -F 'InvalidNumber {' "$WORK/error" >/dev/null; then
  echo "malformed RNA leaked Rust debug syntax" >&2
  exit 1
fi

"$CLI" compile-rna samples/native_triangle.rna --out "$WORK/triangle.dna" >/dev/null
cp "$WORK/triangle.dna" "$WORK/original.dna"
if "$CLI" compile-rna samples/native_equality.rna --out "$WORK/triangle.dna" >"$WORK/out" 2>"$WORK/error"; then
  echo "compile-rna overwrote an existing output" >&2
  exit 1
fi
grep -F 'refusing to overwrite existing output' "$WORK/error" >/dev/null
cmp "$WORK/triangle.dna" "$WORK/original.dna"
if find "$WORK" -maxdepth 1 -name '.*.l64-stage-*' -print -quit | grep . >/dev/null; then
  echo "atomic output staging residue remained" >&2
  exit 1
fi

if "$CLI" compile-rna samples/native_triangle.rna --artifact-class gene --out "$WORK/class.dna" >"$WORK/out" 2>"$WORK/error"; then
  echo "deleted artifact-class option was accepted" >&2
  exit 1
fi
grep -F -- '--artifact-class was permanently deleted' "$WORK/error" >/dev/null

if grep -REn --include='*.rs' 'Result<bool, String>|Ok\(false\)' l64-cli/src >/dev/null \
  || grep -REn --include='*.rs' '\{error:\?\}' l64-cli/src l64-*/src >/dev/null; then
  echo "obsolete fallback or debug diagnostic returned" >&2
  exit 1
fi

echo "CLI_HARDENING_GATE v1"
echo "status=pass"
echo "direct_dispatch=true"
echo "stable_diagnostics=true"
echo "atomic_no_overwrite=true"
echo "command_help=true"
