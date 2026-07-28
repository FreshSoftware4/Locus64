#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"

OUT=${1:?usage: write-portability-receipt.sh <output.json> <host-label>}
HOST_LABEL=${2:?usage: write-portability-receipt.sh <output.json> <host-label>}
SOURCE_REF=${L64_SOURCE_REF:-${GITHUB_SHA:-}}

case "$HOST_LABEL" in
  ubuntu-latest|macos-latest|windows-latest) ;;
  *) printf 'unsupported portability host label: %s\n' "$HOST_LABEL" >&2; exit 1 ;;
esac

[[ -n "$SOURCE_REF" ]] || { printf 'L64_SOURCE_REF or GITHUB_SHA is required\n' >&2; exit 1; }

json_escape() {
  local value=$1
  value=${value//\\/\\\\}
  value=${value//\"/\\\"}
  value=${value//$'\n'/\\n}
  value=${value//$'\r'/\\r}
  value=${value//$'\t'/\\t}
  printf '%s' "$value"
}

package_count=$(awk '
  /^members = \[/ { in_members=1; next }
  in_members && /^\]/ { in_members=0 }
  in_members && /^[[:space:]]*"[^"]+",?[[:space:]]*$/ { count++ }
  END { print count+0 }
' Cargo.toml)
external_dependencies=$(grep -c '^source = ' Cargo.lock || true)
[[ "$package_count" -eq 11 ]] || { printf 'unexpected package count: %s\n' "$package_count" >&2; exit 1; }
[[ "$external_dependencies" -eq 0 ]] || { printf 'external dependencies present: %s\n' "$external_dependencies" >&2; exit 1; }

rustc_version=$(rustc --version)
cargo_version=$(cargo --version)
rust_host=$(rustc -vV | sed -n 's/^host: //p')
[[ -n "$rust_host" ]] || { printf 'could not determine rust host\n' >&2; exit 1; }

repository=${GITHUB_REPOSITORY:-local}
workflow_run_id=${GITHUB_RUN_ID:-local}
workflow_run_attempt=${GITHUB_RUN_ATTEMPT:-1}
created_utc=$(date -u +'%Y-%m-%dT%H:%M:%SZ')
mkdir -p "$(dirname "$OUT")"
tmp="${OUT}.tmp.$$"

cat > "$tmp" <<EOF
{
  "schema": "L64PORT1",
  "source_ref": "$(json_escape "$SOURCE_REF")",
  "repository": "$(json_escape "$repository")",
  "workflow_run_id": "$(json_escape "$workflow_run_id")",
  "workflow_run_attempt": "$(json_escape "$workflow_run_attempt")",
  "host_label": "$(json_escape "$HOST_LABEL")",
  "rust_host": "$(json_escape "$rust_host")",
  "rustc": "$(json_escape "$rustc_version")",
  "cargo": "$(json_escape "$cargo_version")",
  "workspace_packages": $package_count,
  "external_dependencies": $external_dependencies,
  "test_command": "cargo test --locked --offline --workspace",
  "result": "passed",
  "created_utc": "$(json_escape "$created_utc")"
}
EOF
mv "$tmp" "$OUT"
printf 'wrote portability receipt: %s\n' "$OUT"
