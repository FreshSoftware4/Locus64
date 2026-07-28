#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"
TARGET_DIR=${CARGO_TARGET_DIR:-$ROOT/target}

fail() {
  printf 'native scale verification failed: %s\n' "$1" >&2
  exit 1
}

# Structural guards prevent reintroducing the measured whole-state and whole-graph hot paths.
grep -Fq 'Graph::new_bulk()' l64-native/src/rna/compile.rs \
  || fail 'RNA compilation no longer uses bulk graph construction'
[[ $(grep -R -F 'crate::symbol::state_symbol(self)' l64-native/src/graph | wc -l) -eq 1 ]] \
  || fail 'graph mutation paths recompute whole-state symbols outside the single mutation boundary'
grep -Fq 'primary_route' l64-native/src/graph/derived.rs \
  || fail 'per-node primary-route index is absent'
for projection in atlas.rs certification.rs report.rs research.rs replay.rs set.rs; do
  if grep -Fq 'graph.closure_state(' "l64-projection/src/$projection"; then
    fail "$projection reintroduced per-node closure recomputation"
  fi
done
if grep -Fq 'projection.verify(graph)' l64-execution/src/lib.rs; then
  fail 'fresh execution projection is immediately rederived'
fi
if grep -Fq 'view.verify(graph)' l64-certification/src/lib.rs; then
  fail 'fresh certification projection is immediately rederived'
fi

tmp=$(mktemp -d)
trap 'rm -rf "$tmp"' EXIT
linear="$tmp/linear-8192.rna"
ops="$tmp/ops-4096.rna"
{
  printf 'L64R1 0x5343414c454c494e\n'
  for ((i = 1; i <= 8192; i++)); do
    printf 'a %d 0x%x\n' "$i" "$i"
  done
} > "$linear"
{
  printf 'L64R1 0x5343414c454f5053\n'
  printf 'a 1 0x41\na 2 0x42\na 3 0x43\n'
  slot=4
  for ((i = 0; i < 4096; i++)); do
    f1=$slot; f2=$((slot + 1)); f3=$((slot + 2)); v1=$((slot + 3)); v2=$((slot + 4)); c=$((slot + 5))
    printf 'f %d 1 2\nf %d 2 3\nf %d 1 3\nv %d %d\nv %d %d\nc %d %d %d %d\n' \
      "$f1" "$f2" "$f3" "$v1" "$f1" "$v2" "$f2" "$c" "$v1" "$v2" "$f3"
    slot=$((slot + 6))
  done
} > "$ops"

cargo build --locked --offline --release -p l64-cli >/dev/null
bin="$TARGET_DIR/release/l64-cli"
timeout 20s "$bin" compile-rna "$linear" --out "$tmp/linear.dna" >/dev/null
timeout 30s "$bin" run-rna "$ops" > "$tmp/ops.run"
grep -Fq 'canonical_roundtrip=true' "$tmp/ops.run" \
  || fail 'operation-heavy execution lost the exact roundtrip'
grep -Fq 'projection_verified=true' "$tmp/ops.run" \
  || fail 'operation-heavy execution lost projection construction'

printf 'native scale gate passed: bulk compilation, indexed routes, shared closure analysis, and bounded operation execution\n'
