#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="${1:-$ROOT/l64-native-demo-output}"
CARGO="${CARGO:-cargo}"
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
EXE_SUFFIX=""
if [[ "${OS:-}" == "Windows_NT" ]]; then EXE_SUFFIX=".exe"; fi
CLI="$TARGET_DIR/release/l64-cli$EXE_SUFFIX"
WRAPPER="$TARGET_DIR/release/l64$EXE_SUFFIX"

if [[ -e "$OUT" ]]; then
  echo "demo output already exists: $OUT" >&2
  exit 2
fi
mkdir -p "$OUT"

cd "$ROOT"
"$CARGO" build --release --locked --offline -p l64-cli -p l64

"$WRAPPER" authority-audit > "$OUT/authority-audit.txt"
"$CLI" run-rna samples/native_triangle.rna > "$OUT/triangle.rna.run.txt"
"$CLI" compile-rna samples/native_triangle.rna --out "$OUT/triangle.dna" > /dev/null
"$CLI" run-dna "$OUT/triangle.dna" > "$OUT/triangle.dna.run.txt"
"$CLI" certify-dna "$OUT/triangle.dna" > "$OUT/triangle.certification.txt"
"$CLI" observe-dna "$OUT/triangle.dna" > "$OUT/triangle.observation.txt"
"$CLI" inspect-dna "$OUT/triangle.dna" > "$OUT/triangle.projection.txt"

"$CLI" run-rna samples/native_equality.rna > "$OUT/equality.rna.run.txt"
"$CLI" compile-rna samples/native_equality.rna --out "$OUT/equality.dna" > /dev/null
"$CLI" run-dna "$OUT/equality.dna" > "$OUT/equality.dna.run.txt"
"$CLI" compare-dna "$OUT/triangle.dna" "$OUT/equality.dna" > "$OUT/triangle-to-equality.change.txt"

"$CLI" compile-bundle "$OUT/triangle.dna" "$OUT/equality.dna" --out "$OUT/native-pair.l64b" > /dev/null
"$CLI" run-bundle --file "$OUT/native-pair.l64b" > "$OUT/native-pair.run.txt"
"$CLI" certify-bundle --file "$OUT/native-pair.l64b" > "$OUT/native-pair.certification.txt"
"$CLI" observe-bundle --file "$OUT/native-pair.l64b" > "$OUT/native-pair.observation.txt"

"$CLI" export-genome-release --rna samples/native_triangle.rna --out "$OUT/triangle.release" > /dev/null
"$CLI" export-genome-release --rna samples/native_equality.rna --out "$OUT/equality.release" > /dev/null

cat > "$OUT/demo.summary.txt" <<SUMMARY
L64 NATIVE DEMO v1
authority=triangle.dna,equality.dna
transport=native-pair.l64b
derived_outputs=run,certification,observation,projection,change
release_roots=triangle.release,equality.release
derived_authority=non_authoritative
persistence=explicit_demo_output_only
SUMMARY

printf '\n=== triangle RNA execution ===\n'
cat "$OUT/triangle.rna.run.txt"
printf '\n=== bundle execution ===\n'
cat "$OUT/native-pair.run.txt"
printf '\nDemo output: %s\n' "$OUT"
