#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

fail() {
  printf 'golden portability verification failed: %s\n' "$1" >&2
  exit 1
}

cargo test --locked --offline -q -p l64-cli golden_portability
cargo test --locked --offline -q -p l64 portability_

mapfile -t fixtures < <(find samples/golden -maxdepth 1 -type f -name '*.rna' -printf '%f\n' | sort)
[[ ${#fixtures[@]} -eq 5 ]] || fail "expected five authority fixtures, found ${#fixtures[@]}"
for required in certified_triangle equality_chain open_obligation invalid_child_context matrix_multiply; do
  [[ -f "samples/golden/$required.rna" ]] || fail "missing $required.rna"
  for suffix in run.txt certify.txt dna.hex normalized.rna; do
    [[ -f "samples/golden/expected/$required.$suffix" ]] || fail "missing $required.$suffix"
  done
done

workflow=.github/workflows/native-core.yml
for os in ubuntu-latest macos-latest windows-latest; do
  grep -Fq "$os" "$workflow" || fail "portable CI matrix omits $os"
done
grep -Fq 'cargo test --locked --offline --workspace' "$workflow" \
  || fail 'portable CI does not execute the complete Rust workspace'
grep -Fq "TARGET_DIR=\${CARGO_TARGET_DIR:-\$ROOT/target}" scripts/verify-native-scale.sh \
  || fail 'native scale gate ignores CARGO_TARGET_DIR'
grep -Fq "TARGET_DIR=\${CARGO_TARGET_DIR:-\$ROOT/target}" scripts/verify-documentation-coherence.sh \
  || fail 'documentation gate ignores CARGO_TARGET_DIR'
grep -Fq "\$env:CARGO_TARGET_DIR" scripts/verify-low-memory.ps1 \
  || fail 'PowerShell low-memory gate ignores CARGO_TARGET_DIR'

grep -Fq '## Golden workload and portability contract' LOCUS64.md \
  || fail 'sole project document does not expose the golden portability contract'
grep -Fq 'golden-portability-green' LOCUS64.md \
  || fail 'sole project document omits the golden portability gate'

printf 'GOLDEN_PORTABILITY_GATE v1\n'
printf 'status=pass\n'
printf 'authority_fixtures=%s\n' "${#fixtures[@]}"
printf 'host_execution=%s\n' "$(rustc -vV | awk -F': ' '/^host:/ { print $2 }')"
printf 'canonical_bytes=exact_snapshots\n'
printf 'text_outputs=exact_snapshots\n'
printf 'crlf_lf_equivalence=true\n'
printf 'unicode_space_paths=true\n'
printf 'transport_release_journey=true\n'
printf 'ci_matrix=linux_macos_windows\n'
