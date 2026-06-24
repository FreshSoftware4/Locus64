param(
    [ValidateSet("core", "bundle", "cli", "workspace", "all")]
    [string]$Scope = "workspace",
    [switch]$SkipFmt,
    [switch]$SkipDiff
)

$ErrorActionPreference = "Stop"

$WorkspaceRoot = Split-Path -Parent $PSScriptRoot
Push-Location $WorkspaceRoot

try {
    $env:CARGO_BUILD_JOBS = "1"
    $env:CARGO_INCREMENTAL = "0"
    $env:CARGO_PROFILE_DEV_DEBUG = "0"
    $env:CARGO_PROFILE_TEST_DEBUG = "0"

    if (-not $SkipFmt) {
        cargo fmt --check
    }

    switch ($Scope) {
        "core" {
            cargo test -p l64-core --lib
        }
        "bundle" {
            cargo test -p l64-core --lib
            cargo test -p l64-bundle
        }
        "cli" {
            cargo test -p l64-core --lib
            cargo test -p l64-bundle
            cargo test -p l64-cli
        }
        "workspace" {
            cargo check --workspace
        }
        "all" {
            cargo test -p l64-core --lib
            cargo test -p l64-bundle
            cargo test -p l64-cli
            cargo check --workspace
        }
    }

    if (-not $SkipDiff) {
        git diff --check
    }
} finally {
    Pop-Location
}
