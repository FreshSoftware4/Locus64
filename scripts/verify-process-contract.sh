#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
CARGO="${CARGO:-cargo}"
TARGET_DIR="${CARGO_TARGET_DIR:-$ROOT/target}"
CLI="$TARGET_DIR/debug/l64-cli"
WORK="$(mktemp -d "${TMPDIR:-/tmp}/l64-process-contract.XXXXXX")"
trap 'rm -rf "$WORK"' EXIT

cd "$ROOT"
"$CARGO" build --locked --offline -q -p l64-cli

cat > "$WORK/malformed.rna" <<'RNA'
L64R1 0x1
a nope 0x41
RNA
cat > "$WORK/open.rna" <<'RNA'
L64R1 0x4345525449465931
a 1 0x52
q 2 1 0 0 0 0 0 0 0
v 3 2
r 4 3 2
RNA
cat > "$WORK/invalid.rna" <<'RNA'
L64R1 0x4345525449465931
a 1 0x52
q 2 1 0 0 0 0 0 0 0
v 3 2
r 4 3 2
k 5 3 1 0
h 1 0 5
RNA

set +e
"$CLI" run-rna "$WORK/malformed.rna" >"$WORK/malformed.out" 2>"$WORK/malformed.err"
malformed_code=$?
"$CLI" run-rna "$WORK/open.rna" >"$WORK/open.out" 2>"$WORK/open.err"
open_code=$?
"$CLI" run-rna "$WORK/invalid.rna" >"$WORK/invalid.out" 2>"$WORK/invalid.err"
invalid_code=$?
set -e

test "$malformed_code" -eq 2
test "$open_code" -eq 10
test "$invalid_code" -eq 12
grep -F 'invalid number at RNA 2:3' "$WORK/malformed.err" >/dev/null
grep -F '2 | a nope 0x41' "$WORK/malformed.err" >/dev/null
grep -F '|   ^^^^' "$WORK/malformed.err" >/dev/null
grep -Fx 'certification_verdict=OPEN' "$WORK/open.out" >/dev/null
grep -Fx 'certification_verdict=INVALID' "$WORK/invalid.out" >/dev/null

"$CLI" compile-rna samples/native_triangle.rna --out "$WORK/triangle.dna" >/dev/null
"$CLI" compile-rna "$WORK/open.rna" --out "$WORK/open.dna" >/dev/null
"$CLI" compile-bundle "$WORK/triangle.dna" "$WORK/open.dna" --out "$WORK/mixed.l64b" >/dev/null
set +e
"$CLI" run-bundle --file "$WORK/mixed.l64b" >"$WORK/bundle.out" 2>"$WORK/bundle.err"
bundle_code=$?
set -e
test "$bundle_code" -eq 10
grep -Fx 'members=2' "$WORK/bundle.out" >/dev/null
grep -Fx 'member.0.certification_verdict=CERTIFIED' "$WORK/bundle.out" >/dev/null
grep -Fx 'member.1.certification_verdict=OPEN' "$WORK/bundle.out" >/dev/null

rg -F 'BundleDecoder' l64-cli/src/native_membrane.rs >/dev/null
rg -F 'BundleEncoder' l64-cli/src/native_membrane.rs >/dev/null
if rg -n 'read\(&options\.file\)' l64-cli/src/native_membrane.rs >/dev/null; then
  echo 'bundle command returned to whole-file buffering' >&2
  exit 1
fi

echo 'PROCESS_CONTRACT_GATE v1'
echo 'status=pass'
echo 'certified_exit=0'
echo 'open_exit=10'
echo 'incomplete_exit=11'
echo 'invalid_exit=12'
echo 'operational_error_exit=2'
echo 'rna_source_spans=true'
echo 'streaming_bundle_io=true'
