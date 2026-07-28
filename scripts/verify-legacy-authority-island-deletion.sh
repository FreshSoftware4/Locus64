#!/usr/bin/env bash
set -euo pipefail
ROOT="${1:-$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)}"
cd "$ROOT"

expected=(l64 l64-certification l64-change l64-cli l64-execution l64-native l64-observation l64-projection l64-release l64-symbolic l64-transport)
for deleted in l64-core l64-locus l64-research l64-kernel l64-command l64-registry l64-selector l64-runtime l64-canon l64-atlas l64-cert l64-testkit; do
  test ! -e "$deleted" || { echo "legacy crate still live: $deleted" >&2; exit 1; }
done

grep -q '"l64-cli"' Cargo.toml
test "$(grep -c '^name = ' Cargo.lock)" -eq 11
for package in "${expected[@]}"; do grep -q "name = \"$package\"" Cargo.lock; done
if grep -q '^\[workspace.dependencies\]' Cargo.toml; then echo 'workspace dependencies table returned' >&2; exit 1; fi
if grep -R -nE '^[[:space:]]*(anyhow|clap|serde|serde_json|bincode|hex|thiserror|assert_cmd|predicates)[[:space:]]*=' -- */Cargo.toml Cargo.toml; then echo 'external dependency returned' >&2; exit 1; fi
if grep -R -nE 'path[[:space:]]*=[[:space:]]*"\.\./l64-(core|locus|research|kernel|command|registry|selector|runtime|canon|atlas|cert|testkit)"' -- */Cargo.toml; then echo 'legacy dependency returned' >&2; exit 1; fi

test ! -e l64-cli/src/main.rs
grep -q 'permanently deleted after historical export' l64-cli/src/native_membrane.rs
grep -q '"run-theorem"' l64-cli/src/native_membrane.rs
grep -q '"research-status"' l64-cli/src/native_membrane.rs
grep -q '"tower-step"' l64-cli/src/native_membrane.rs
if grep -q 'mod legacy' l64-cli/src/native_main.rs; then echo 'legacy dispatcher returned' >&2; exit 1; fi

cargo metadata --locked --offline --no-deps --format-version 1 >/dev/null
printf 'legacy authority island deletion verified: 11 dependency-free native packages\n'
