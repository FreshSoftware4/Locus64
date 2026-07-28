#!/usr/bin/env bash
set -euo pipefail

ROOT=$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)
cd "$ROOT"
TARGET_DIR=${CARGO_TARGET_DIR:-$ROOT/target}
CLI="$TARGET_DIR/debug/l64-cli"
WRAPPER="$TARGET_DIR/debug/l64"

fail() {
  printf 'documentation coherence verification failed: %s\n' "$1" >&2
  exit 1
}

[[ -f LOCUS64.md ]] || fail 'sole project document LOCUS64.md is missing'
[[ -f changelog.log ]] || fail 'consolidated project changelog is missing'
head -n 16 changelog.log | grep -Fq 'document_status: historical' \
  || fail 'consolidated changelog is not marked historical'

mapfile -t markdown_docs < <(
  find . -type f -name '*.md' \
    -not -path './target/*' \
    -not -path './release/*' \
    -not -path './.git/*' |
    sort
)
[[ ${#markdown_docs[@]} -eq 1 ]] \
  || fail "expected exactly one project Markdown file, found ${#markdown_docs[@]}"
[[ "${markdown_docs[0]}" == './LOCUS64.md' ]] \
  || fail "unexpected sole Markdown document: ${markdown_docs[0]}"

if find . -type f -name '*.athens' -not -path './target/*' -not -path './.git/*' | grep -q .; then
  fail 'completed Athens rails returned instead of remaining consolidated in changelog.log'
fi

package_count=$(awk '
  /^members = \[/ { in_members=1; next }
  in_members && /^\]/ { in_members=0 }
  in_members && /^[[:space:]]*"[^"]+",?[[:space:]]*$/ { count++ }
  END { print count+0 }
' Cargo.toml)
[[ "$package_count" -eq 11 ]] || fail "Cargo workspace package count is $package_count, expected 11"

for required in \
  'document_role: sole_markdown_project_authority' \
  'workspace_packages: 11' \
  'external_dependencies: 0' \
  'ambient_legacy_authority: deleted_tombstones_only' \
  'receipt_schema: L64PORT1' \
  'eleven dependency-free native packages' \
  'The wrapper is not a second parser' \
  'golden-portability-green' \
  'residue-remote-proof-green' \
  '## Documentation and promotion law' \
  '## Release record: v0.1.3'; do
  grep -Fq "$required" LOCUS64.md || fail "sole project document omits required contract: $required"
done

for stale in \
  'l64-cli legacy ...' \
  'available only through `l64-cli legacy' \
  'replacement and quarantine of the legacy runtime' \
  'transitional legacy workspace' \
  'field=key=ambient_legacy_authority;value=explicit_only'; do
  if grep -Fq "$stale" LOCUS64.md; then
    fail "stale live claim remains: $stale"
  fi
done

for workflow_contact in \
  'actions/upload-artifact@v7' \
  'actions/download-artifact@v8' \
  'locus64-portability-receipts' \
  'scripts/write-portability-receipt.sh' \
  'scripts/verify-portability-receipts.sh'; do
  grep -Fq "$workflow_contact" .github/workflows/native-core.yml \
    || fail "workflow omits portability receipt contact: $workflow_contact"
done
scripts/verify-portability-receipts.sh --self-test >/dev/null

mapfile -t retired_commands < <(awk '
  /const RETIRED_LEGACY_COMMANDS:/ { in_list=1; next }
  in_list && /^];/ { in_list=0 }
  in_list && /^[[:space:]]*"[^"]+",/ {
    line=$0
    sub(/^[[:space:]]*"/, "", line)
    sub(/",.*$/, "", line)
    print line
  }
' l64-cli/src/native_membrane.rs)
[[ ${#retired_commands[@]} -gt 0 ]] || fail 'could not derive retired command vocabulary'
for script in scripts/*; do
  [[ -f "$script" ]] || continue
  case "$(basename "$script")" in
    verify-legacy-authority-island-deletion.sh|verify-cli-hardening.sh|verify-documentation-coherence.sh) continue ;;
  esac
  for command in "${retired_commands[@]}"; do
    if grep -Eq "[\"']${command}[\"']" "$script"; then
      fail "current script invokes or advertises retired command $command: $script"
    fi
  done
done

for os in ubuntu-latest macos-latest windows-latest; do
  grep -Fq "$os" .github/workflows/native-core.yml \
    || fail "portable Rust matrix omits $os"
done

if grep -REn --include='*.sh' --include='*.ps1' \
  'cargo[[:space:]]+(test|check|run|build)[^\n]*-p[[:space:]]+l64-(core|locus|research|kernel|command|registry|selector|runtime|canon|atlas|cert|testkit|bundle|policy|admin|surfaces|qc0|qa0|qm0)' \
  scripts; then
  fail 'a current script invokes a deleted package'
fi

while IFS= read -r line; do
  [[ -z "$line" ]] && continue
  while IFS= read -r script; do
    [[ -z "$script" ]] && continue
    [[ -f "$script" ]] || fail "LOCUS64.md references missing live script: $script"
  done < <(printf '%s\n' "$line" | grep -Eo 'scripts/[A-Za-z0-9._-]+\.(sh|ps1)' || true)
done < LOCUS64.md

cargo build --locked --offline -p l64-cli -p l64 >/dev/null
cli_help=$("$CLI" --help)
wrapper_audit=$("$WRAPPER" authority-audit)
grep -Fq 'Locus64 authority audit' <<<"$wrapper_audit" \
  || fail 'l64 authority-audit did not execute through the wrapper'

mapfile -t help_commands < <(
  printf '%s\n' "$cli_help" |
    sed -n -E 's/^  ([a-z][a-z0-9-]+)[[:space:]]{2,}.*/\1/p' |
    sort -u
)
mapfile -t spec_commands < <(awk '
  /^### `l64-cli` commands$/ { in_commands=1; next }
  in_commands && /^### / { in_commands=0 }
  in_commands && /^- `[a-z][a-z0-9-]+`$/ {
    line=$0
    sub(/^- `/, "", line)
    sub(/`$/, "", line)
    print line
  }
' LOCUS64.md | sort -u)

[[ ${#help_commands[@]} -gt 0 ]] || fail 'could not derive commands from live CLI help'
[[ ${#spec_commands[@]} -gt 0 ]] || fail 'could not derive commands from LOCUS64.md'
if [[ "$(printf '%s\n' "${help_commands[@]}")" != "$(printf '%s\n' "${spec_commands[@]}")" ]]; then
  printf 'live help commands:\n%s\n' "$(printf '%s\n' "${help_commands[@]}")" >&2
  printf 'documented commands:\n%s\n' "$(printf '%s\n' "${spec_commands[@]}")" >&2
  fail 'documented command set differs from live CLI help'
fi

grep -Fq -- '- `authority-audit`' LOCUS64.md \
  || fail 'wrapper-local authority-audit is absent from LOCUS64.md'
if "$CLI" help authority-audit >/tmp/l64-doc-gate.out 2>/tmp/l64-doc-gate.err; then
  fail 'l64-cli unexpectedly accepted wrapper-local authority-audit'
fi
grep -Fq 'belongs to the `l64` wrapper' /tmp/l64-doc-gate.err \
  || fail 'l64-cli does not reject authority-audit at the correct binary boundary'
rm -f /tmp/l64-doc-gate.out /tmp/l64-doc-gate.err

printf 'documentation coherence gate passed: one Markdown authority, %s packages, %s live CLI commands\n' \
  "$package_count" "${#help_commands[@]}"
