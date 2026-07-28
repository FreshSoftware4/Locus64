[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateSet("compact", "perfopt")]
    [string]$Profile,

    [Parameter(Mandatory = $true)]
    [ValidateSet("windows-x86_64", "linux-x86_64")]
    [string]$Platform,

    [Parameter(Mandatory = $true)]
    [string]$Version,

    [string]$OutputDirectory = "release/packages"
)

$ErrorActionPreference = "Stop"
$repository = (Resolve-Path (Join-Path $PSScriptRoot "..")).Path
$outputRoot = [System.IO.Path]::GetFullPath((Join-Path $repository $OutputDirectory))
$releaseLabel = if ($Version.StartsWith("v")) { $Version } else { "v$Version" }
$archiveBase = "locus64-$releaseLabel-$Platform-$Profile"
$stageRoot = Join-Path $outputRoot "staging"
$stage = Join-Path $stageRoot $archiveBase
$targetRoot = Join-Path $repository "target"

if (-not $outputRoot.StartsWith($repository, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "release output must remain inside the repository"
}

$hostPlatform = if ($IsWindows -or $env:OS -eq "Windows_NT") {
    "windows-x86_64"
} else {
    "linux-x86_64"
}
if ($Platform -ne $hostPlatform) {
    throw "native package platform '$Platform' does not match host '$hostPlatform'"
}

Push-Location $repository
try {
    cargo build --locked --offline --profile $Profile -p l64-cli -p l64
    if ($LASTEXITCODE -ne 0) {
        throw "cargo build failed"
    }

    $binaryDirectory = Join-Path $targetRoot $Profile
    $extension = if ($hostPlatform -eq "windows-x86_64") { ".exe" } else { "" }
    $cli = Join-Path $binaryDirectory "l64-cli$extension"
    $wrapper = Join-Path $binaryDirectory "l64$extension"
    foreach ($binary in @($cli, $wrapper)) {
        if (-not (Test-Path -LiteralPath $binary -PathType Leaf)) {
            throw "missing release binary '$binary'"
        }
    }

    & $cli --version | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "l64-cli release self-check failed"
    }
    & $wrapper authority-audit | Out-Null
    if ($LASTEXITCODE -ne 0) {
        throw "l64 wrapper release self-check failed"
    }

    New-Item -ItemType Directory -Force -Path $outputRoot, $stageRoot | Out-Null
    if (Test-Path -LiteralPath $stage) {
        $resolvedStage = [System.IO.Path]::GetFullPath($stage)
        if (-not $resolvedStage.StartsWith($stageRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
            throw "refusing to clean stage outside release staging root"
        }
        Remove-Item -LiteralPath $resolvedStage -Recurse -Force
    }
    New-Item -ItemType Directory -Path $stage | Out-Null

    Copy-Item -LiteralPath $cli, $wrapper -Destination $stage
    foreach ($document in @(
        "LOCUS64.md",
        "LICENSE"
    )) {
        Copy-Item -LiteralPath (Join-Path $repository $document) -Destination $stage
    }

    $commit = (git rev-parse HEAD).Trim()
    $rustVersion = (rustc --version).Trim()
    @(
        "distribution_metadata=non_authoritative",
        "version=$releaseLabel",
        "profile=$Profile",
        "platform=$Platform",
        "source_commit=$commit",
        "rustc=$rustVersion"
    ) | Set-Content -LiteralPath (Join-Path $stage "BUILD.txt") -Encoding utf8NoBOM

    if ($hostPlatform -eq "windows-x86_64") {
        $archive = Join-Path $outputRoot "$archiveBase.zip"
        if (Test-Path -LiteralPath $archive) {
            Remove-Item -LiteralPath $archive -Force
        }
        Compress-Archive -LiteralPath $stage -DestinationPath $archive -CompressionLevel Optimal
    } else {
        & chmod +x (Join-Path $stage "l64") (Join-Path $stage "l64-cli")
        if ($LASTEXITCODE -ne 0) {
            throw "failed to set executable permissions"
        }
        $archive = Join-Path $outputRoot "$archiveBase.tar.gz"
        if (Test-Path -LiteralPath $archive) {
            Remove-Item -LiteralPath $archive -Force
        }
        & tar -C $stageRoot -czf $archive $archiveBase
        if ($LASTEXITCODE -ne 0) {
            throw "tar packaging failed"
        }
    }

    if (-not (Test-Path -LiteralPath $archive -PathType Leaf)) {
        throw "release archive was not created"
    }
    Write-Output $archive
} finally {
    Pop-Location
}
