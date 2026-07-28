#!/usr/bin/env bash
set -euo pipefail

ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
cd "$ROOT"

CARGO="${CARGO:-cargo}"
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
EXE_SUFFIX=""
if [[ "${OS:-}" == "Windows_NT" ]]; then EXE_SUFFIX=".exe"; fi
CLI="$TARGET_DIR/debug/l64-cli$EXE_SUFFIX"
WRAPPER="$TARGET_DIR/debug/l64$EXE_SUFFIX"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/l64-native-carrier.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

"$CARGO" build --locked --offline -p l64-cli -p l64

test -x "$CLI"
test -x "$WRAPPER"

"$CLI" run-rna samples/native_triangle.rna > "$WORK/triangle.rna.run.txt"
"$CLI" compile-rna samples/native_triangle.rna --out "$WORK/triangle.dna" > /dev/null
"$CLI" run-dna "$WORK/triangle.dna" > "$WORK/triangle.dna.run.txt"
"$CLI" compile-rna samples/native_equality.rna --out "$WORK/equality.dna" > /dev/null
"$CLI" run-dna "$WORK/equality.dna" > "$WORK/equality.dna.run.txt"

rna_symbol="$(grep '^symbol=' "$WORK/triangle.rna.run.txt")"
dna_symbol="$(grep '^symbol=' "$WORK/triangle.dna.run.txt")"
test "$rna_symbol" = "$dna_symbol"
grep -q '^canonical_roundtrip=true$' "$WORK/triangle.rna.run.txt"
grep -q '^projection_verified=true$' "$WORK/triangle.rna.run.txt"
grep -q '^certification_verdict=CERTIFIED$' "$WORK/triangle.rna.run.txt"
grep -q '^authority_mutation=none$' "$WORK/triangle.rna.run.txt"

"$CLI" compare-dna "$WORK/triangle.dna" "$WORK/equality.dna" > "$WORK/change.txt"
grep -q '^exact_equal=false$' "$WORK/change.txt"

"$CLI" compile-bundle "$WORK/triangle.dna" "$WORK/equality.dna" --out "$WORK/native.l64b" > /dev/null
"$CLI" run-bundle --file "$WORK/native.l64b" > "$WORK/bundle.run.txt"
"$CLI" certify-bundle --file "$WORK/native.l64b" > "$WORK/bundle.certification.txt"
"$CLI" observe-bundle --file "$WORK/native.l64b" > "$WORK/bundle.observation.txt"
grep -q '^members=2$' "$WORK/bundle.run.txt"
grep -q '^composite_execution=none$' "$WORK/bundle.run.txt"
grep -q '^member.0.source=bundle_member$' "$WORK/bundle.run.txt"
grep -q '^member.1.source=bundle_member$' "$WORK/bundle.run.txt"
grep -q '^composite_certificate=none$' "$WORK/bundle.certification.txt"
grep -q '^composite_observation=none$' "$WORK/bundle.observation.txt"

"$CLI" export-genome-release --rna samples/native_triangle.rna --out "$WORK/triangle.release" > /dev/null
test "$(find "$WORK/triangle.release" -maxdepth 1 -type f | wc -l)" -eq 4
grep -q '^projection_verified=true$' "$WORK/triangle.release/release.record"

"$WRAPPER" authority-audit > "$WORK/authority-audit.txt"
grep -q '^execution: direct non-persistent RNA/DNA structural evaluation$' "$WORK/authority-audit.txt"

if "$CLI" legacy >"$WORK/legacy.out" 2>"$WORK/legacy.err"; then
  echo "retired legacy command unexpectedly succeeded" >&2
  exit 1
fi
grep -q 'permanently deleted after historical export' "$WORK/legacy.err"

printf 'native carrier verified: RNA/DNA execution, release, bundle, certification, observation, and change\n'
