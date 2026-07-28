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

is_historical() {
  local file=$1
  head -n 16 "$file" | grep -Eq '^document_status: historical$|^field=key=document_status;value=historical$'
}

mapfile -t markdown_docs < <(find . -type f -name '*.md' -not -path './target/*' -not -path './.git/*' | sort)
current_docs=()
for file in "${markdown_docs[@]}"; do
  if ! is_historical "$file"; then
    current_docs+=("$file")
  fi
done

[[ ${#current_docs[@]} -gt 0 ]] || fail 'no current-facing Markdown documents were found'

package_count=$(awk '
  /^members = \[/ { in_members=1; next }
  in_members && /^\]/ { in_members=0 }
  in_members && /^[[:space:]]*"[^"]+",?[[:space:]]*$/ { count++ }
  END { print count+0 }
' Cargo.toml)
[[ "$package_count" -eq 11 ]] || fail "Cargo workspace package count is $package_count, expected 11"

grep -Fq 'workspace_packages: 11' LOCUS64_NATIVE_CONSTITUTION.md \
  || fail 'current constitution does not bind the eleven-package workspace'
grep -Fq 'eleven packages' LOCUS64_STACK.md \
  || fail 'stack document does not state the Cargo-derived package count'
grep -Fq 'eleven dependency-free native packages' HANDOFF_STATUS.md \
  || fail 'handoff does not state the Cargo-derived package count'
grep -Fq 'field=key=live_workspace_packages;value=11' LOCUS64_EXECUTION_COHERENCE_RAIL.athens \
  || fail 'current execution rail package count disagrees with Cargo'

for stale in \
  'l64-cli legacy ...' \
  'available only through `l64-cli legacy' \
  'replacement and quarantine of the legacy runtime' \
  'transitional legacy workspace' \
  'field=key=ambient_legacy_authority;value=explicit_only'; do
  if grep -Fq "$stale" "${current_docs[@]}" LOCUS64_EXECUTION_COHERENCE_RAIL.athens; then
    fail "stale live claim remains: $stale"
  fi
done

grep -Fq 'field=key=ambient_legacy_authority;value=deleted_tombstones_only' LOCUS64_EXECUTION_COHERENCE_RAIL.athens \
  || fail 'legacy authority field is not deleted_tombstones_only'

[[ -f LOCUS64_GOLDEN_PORTABILITY_CONTRACT.md ]] \
  || fail 'golden portability contract is missing'
[[ -x scripts/verify-golden-portability.sh ]] \
  || fail 'golden portability gate is missing or not executable'
[[ -x scripts/write-portability-receipt.sh ]] \
  || fail 'portability receipt writer is missing or not executable'
[[ -x scripts/verify-portability-receipts.sh ]] \
  || fail 'portability receipt verifier is missing or not executable'
[[ ! -e scripts/torture-test.ps1 ]] \
  || fail 'obsolete legacy torture harness returned to the live source'
grep -Fq 'The complete Pass 29 source archive is the restore boundary for Pass 30.' SOURCE_REFERENCE_MANIFEST.md \
  || fail 'source manifest does not name the current restore boundary'
grep -Fq '## Current boundary through Pass 30' LOCUS64_RESEARCH_ADMIN_CLASSIFICATION.md \
  || fail 'research and administration classification is stale'
grep -Fq 'receipt_schema: L64PORT1' LOCUS64_PORTABILITY_RECEIPT_CONTRACT.md \
  || fail 'portability receipt contract is missing or malformed'
for workflow_contact in \
  'actions/upload-artifact@v4' \
  'actions/download-artifact@v4' \
  'locus64-portability-receipts' \
  'scripts/write-portability-receipt.sh' \
  'scripts/verify-portability-receipts.sh'; do
  grep -Fq "$workflow_contact" .github/workflows/native-core.yml \
    || fail "workflow omits portability receipt contact: $workflow_contact"
done
scripts/verify-portability-receipts.sh --self-test >/dev/null

# Derive retired command names from the executable tombstone list. Every live
# script except the two gates that verify the tombstones must remain free of
# those command contacts.
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
grep -Fq 'golden-portability-green' L64_APPROVAL_GATES.md \
  || fail 'live approval gates omit golden portability'
grep -Fq 'residue-remote-proof-green' L64_APPROVAL_GATES.md \
  || fail 'live approval gates omit residue and remote-proof closure'
for os in ubuntu-latest macos-latest windows-latest; do
  grep -Fq "$os" .github/workflows/native-core.yml \
    || fail "portable Rust matrix omits $os"
done

if grep -REn --include='*.sh' --include='*.ps1' \
  'cargo[[:space:]]+(test|check|run|build)[^\n]*-p[[:space:]]+l64-(core|locus|research|kernel|command|registry|selector|runtime|canon|atlas|cert|testkit|bundle|policy|admin|surfaces|qc0|qa0|qm0)' \
  scripts; then
  fail 'a current script invokes a deleted package'
fi

# Deleted crate names are allowed in current prose only where the line explicitly
# declares deletion, history, tombstone status, absence, or supersession.
deleted='(^|[^A-Za-z0-9-])l64-(core|locus|research|kernel|command|registry|selector|runtime|canon|atlas|cert|testkit|bundle|policy|admin|surfaces|qc0|qa0|qm0)([^A-Za-z0-9-]|$)'
for file in "${current_docs[@]}"; do
  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    if ! printf '%s\n' "$line" | grep -Eiq 'deleted|historical|tombstone|removed|superseded|absent|no live|formerly|prior|old'; then
      fail "deleted crate appears as a live contact in $file: $line"
    fi
  done < <(grep -En "$deleted" "$file" || true)
done

for historical in \
  LOCUS64_HISTORICAL_LAW_LEDGER.md \
  L64_HISTORICAL_APPROVAL_LEDGER.md \
  LINEAR_EXECUTION_RAIL.md \
  LOCUS64_ARCHITECTURAL_CONSTITUTION_V1.md; do
  [[ -f "$historical" ]] || fail "historical surface missing: $historical"
  is_historical "$historical" || fail "historical surface lacks an early historical marker: $historical"
done

for current in LOCUS64_NATIVE_CONSTITUTION.md L64_APPROVAL_GATES.md LOCUS64_LANGUAGE_SPEC.md LOCUS64_STACK.md HANDOFF_STATUS.md; do
  [[ -f "$current" ]] || fail "current source missing: $current"
  if is_historical "$current"; then
    fail "current source is marked historical: $current"
  fi
done

# Every live script named by current-facing Markdown must exist. A current
# document may name a removed script only on a line that explicitly marks the
# contact as removed, deleted, historical, forensic, or absent.
for file in "${current_docs[@]}"; do
  while IFS= read -r line; do
    [[ -z "$line" ]] && continue
    while IFS= read -r script; do
      [[ -z "$script" ]] && continue
      if [[ ! -f "$script" ]] && ! printf '%s
' "$line" | grep -Eiq 'removed|deleted|historical|forensic|absent|obsolete'; then
        fail "current documentation references missing live script in $file: $script"
      fi
    done < <(printf '%s
' "$line" | grep -Eo 'scripts/[A-Za-z0-9._-]+\.(sh|ps1)' || true)
  done < "$file"
done

cargo build --locked --offline -p l64-cli -p l64 >/dev/null
cli_help=$("$CLI" --help)
wrapper_audit=$("$WRAPPER" authority-audit)

grep -Fq 'Locus64 authority audit' <<<"$wrapper_audit" \
  || fail 'l64 authority-audit did not execute through the wrapper'
grep -Fq 'The wrapper is not a second parser' LOCUS64_LANGUAGE_SPEC.md \
  || fail 'binary responsibility split is not explicit in the language specification'

mapfile -t help_commands < <(printf '%s\n' "$cli_help" | sed -n -E 's/^  ([a-z][a-z0-9-]+)[[:space:]]{2,}.*/\1/p' | sort -u)
mapfile -t spec_commands < <(awk '
  /^## `l64-cli` commands$/ { in_commands=1; next }
  in_commands && /^## / { in_commands=0 }
  in_commands && /^- `[a-z][a-z0-9-]+`$/ {
    line=$0
    sub(/^- `/, "", line)
    sub(/`$/, "", line)
    print line
  }
' LOCUS64_LANGUAGE_SPEC.md | sort -u)

[[ ${#help_commands[@]} -gt 0 ]] || fail 'could not derive commands from live CLI help'
[[ ${#spec_commands[@]} -gt 0 ]] || fail 'could not derive commands from language specification'

if [[ "$(printf '%s\n' "${help_commands[@]}")" != "$(printf '%s\n' "${spec_commands[@]}")" ]]; then
  printf 'live help commands:\n%s\n' "$(printf '%s\n' "${help_commands[@]}")" >&2
  printf 'spec commands:\n%s\n' "$(printf '%s\n' "${spec_commands[@]}")" >&2
  fail 'language specification command set differs from live CLI help'
fi

# shellcheck disable=SC2016
authority_audit_spec='- `authority-audit`'
grep -Fq -- "$authority_audit_spec" LOCUS64_LANGUAGE_SPEC.md \
  || fail 'wrapper-local authority-audit is absent from the language specification'
if "$CLI" help authority-audit >/tmp/l64-doc-gate.out 2>/tmp/l64-doc-gate.err; then
  fail 'l64-cli unexpectedly accepted wrapper-local authority-audit'
fi
# shellcheck disable=SC2016
wrapper_boundary='belongs to the `l64` wrapper'
grep -Fq "$wrapper_boundary" /tmp/l64-doc-gate.err \
  || fail 'l64-cli does not reject authority-audit at the correct binary boundary'
rm -f /tmp/l64-doc-gate.out /tmp/l64-doc-gate.err

printf 'documentation coherence gate passed: %s current docs, %s packages, %s live CLI commands, explicit historical boundaries\n' \
  "${#current_docs[@]}" "$package_count" "${#help_commands[@]}"
