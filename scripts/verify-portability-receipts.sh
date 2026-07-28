#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

fail() {
  printf 'portability receipt verification failed: %s\n' "$1" >&2
  exit 1
}

if [[ ${1:-} == '--self-test' ]]; then
  tmp=$(mktemp -d)
  trap 'rm -rf "$tmp"' EXIT
  for host in ubuntu-latest macos-latest windows-latest; do
    L64_SOURCE_REF=local-pass30 \
      GITHUB_REPOSITORY=local/self-test \
      GITHUB_RUN_ID=self-test \
      GITHUB_RUN_ATTEMPT=1 \
      scripts/write-portability-receipt.sh \
        "$tmp/l64-portability-$host.json" "$host" >/dev/null
  done
  "$0" "$tmp" local-pass30
  exit 0
fi

DIR=${1:?usage: verify-portability-receipts.sh <receipt-directory> <source-ref>}
SOURCE_REF=${2:?usage: verify-portability-receipts.sh <receipt-directory> <source-ref>}
[[ -d "$DIR" ]] || fail "receipt directory does not exist: $DIR"

for host in ubuntu-latest macos-latest windows-latest; do
  receipt="$DIR/l64-portability-$host.json"
  [[ -f "$receipt" ]] || fail "missing receipt for $host"
  grep -Fq '"schema": "L64PORT1"' "$receipt" || fail "$host schema mismatch"
  grep -Fq "\"source_ref\": \"$SOURCE_REF\"" "$receipt" || fail "$host source mismatch"
  grep -Fq "\"host_label\": \"$host\"" "$receipt" || fail "$host label mismatch"
  grep -Fq '"workspace_packages": 11' "$receipt" || fail "$host package count mismatch"
  grep -Fq '"external_dependencies": 0' "$receipt" || fail "$host dependency boundary mismatch"
  grep -Fq '"test_command": "cargo test --locked --offline --workspace"' "$receipt" \
    || fail "$host command mismatch"
  grep -Fq '"result": "passed"' "$receipt" || fail "$host did not pass"
done

receipt_count=$(find "$DIR" -maxdepth 1 -type f -name 'l64-portability-*.json' | wc -l | tr -d ' ')
[[ "$receipt_count" -eq 3 ]] || fail "expected exactly three portability receipts, found $receipt_count"
printf 'portability receipts verified: source=%s hosts=ubuntu-latest,macos-latest,windows-latest\n' "$SOURCE_REF"
