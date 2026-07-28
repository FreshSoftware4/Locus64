param(
    [ValidateSet("transport", "execution", "cli", "workspace", "scale", "all")]
    [string]$Scope = "workspace",
    [switch]$SkipFmt,
    [switch]$SkipDiff
)

$ErrorActionPreference = "Stop"

$WorkspaceRoot = Split-Path -Parent $PSScriptRoot
Push-Location $WorkspaceRoot

function Invoke-Cargo([string[]]$Arguments) {
    & cargo @Arguments
    if ($LASTEXITCODE -ne 0) {
        throw "cargo $($Arguments -join ' ') failed with exit code $LASTEXITCODE"
    }
}

function Invoke-ScaleSmoke {
    $TempRoot = Join-Path ([System.IO.Path]::GetTempPath()) ("l64-low-memory-" + [Guid]::NewGuid().ToString("N"))
    New-Item -ItemType Directory -Path $TempRoot | Out-Null
    try {
        $Rna = Join-Path $TempRoot "linear-8192.rna"
        $Dna = Join-Path $TempRoot "linear-8192.dna"
        $Lines = [System.Collections.Generic.List[string]]::new(8193)
        $Lines.Add("L64R1 0x4c4f574d454d4f52")
        for ($i = 1; $i -le 8192; $i++) {
            $Lines.Add(("a {0} 0x{0:x}" -f $i))
        }
        [System.IO.File]::WriteAllLines($Rna, $Lines)

        Invoke-Cargo @("build", "--locked", "--offline", "--release", "-p", "l64-cli")
        $TargetRoot = if ($env:CARGO_TARGET_DIR) {
            if ([System.IO.Path]::IsPathRooted($env:CARGO_TARGET_DIR)) {
                $env:CARGO_TARGET_DIR
            } else {
                Join-Path $WorkspaceRoot $env:CARGO_TARGET_DIR
            }
        } else {
            Join-Path $WorkspaceRoot "target"
        }
        $Exe = Join-Path $TargetRoot "release/l64-cli.exe"
        if (-not (Test-Path $Exe)) {
            $Exe = Join-Path $TargetRoot "release/l64-cli"
        }
        & $Exe compile-rna $Rna --out $Dna | Out-Null
        if ($LASTEXITCODE -ne 0) { throw "scale compile failed" }
        $Run = & $Exe run-dna $Dna
        if ($LASTEXITCODE -ne 0) { throw "scale execution failed" }
        if ($Run -notcontains "canonical_roundtrip=true") { throw "scale smoke lost canonical roundtrip" }
        if ($Run -notcontains "projection_verified=true") { throw "scale smoke lost verified projection" }
    } finally {
        Remove-Item -Recurse -Force $TempRoot -ErrorAction SilentlyContinue
    }
}

try {
    $env:CARGO_BUILD_JOBS = "1"
    $env:CARGO_INCREMENTAL = "0"
    $env:CARGO_PROFILE_DEV_DEBUG = "0"
    $env:CARGO_PROFILE_TEST_DEBUG = "0"

    if (-not $SkipFmt) {
        Invoke-Cargo @("fmt", "--all", "--", "--check")
    }

    switch ($Scope) {
        "transport" { Invoke-Cargo @("test", "--locked", "--offline", "-p", "l64-transport") }
        "execution" { Invoke-Cargo @("test", "--locked", "--offline", "-p", "l64-execution") }
        "cli" { Invoke-Cargo @("test", "--locked", "--offline", "-p", "l64-cli", "-p", "l64") }
        "workspace" { Invoke-Cargo @("check", "--locked", "--offline", "--workspace", "--all-targets") }
        "scale" { Invoke-ScaleSmoke }
        "all" {
            Invoke-Cargo @("test", "--locked", "--offline", "-p", "l64-transport", "-p", "l64-execution", "-p", "l64-cli", "-p", "l64")
            Invoke-Cargo @("check", "--locked", "--offline", "--workspace", "--all-targets")
            Invoke-ScaleSmoke
        }
    }

    if (-not $SkipDiff -and (Test-Path ".git")) {
        & git diff --check
        if ($LASTEXITCODE -ne 0) { throw "git diff --check failed" }
    }
} finally {
    Pop-Location
}
