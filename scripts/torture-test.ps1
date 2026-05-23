param(
    [string]$WorkspaceRoot = "",
    [string]$OutputRoot = "",
    [int]$OuterRounds = 2,
    [int]$InnerNamespaces = 3
)

$ErrorActionPreference = "Stop"

if ([string]::IsNullOrWhiteSpace($WorkspaceRoot)) {
    $WorkspaceRoot = Split-Path -Parent $PSScriptRoot
}
if ([string]::IsNullOrWhiteSpace($OutputRoot)) {
    $OutputRoot = Join-Path $WorkspaceRoot "release\torture"
}

$campaigns = @(
    "CPG_CHAIN_RULE",
    "CPG_CHAIN_RULE_RECIPE",
    "CPG_CHAIN_RULE_TRANSPORT",
    "CPG_BAYES_BRACE",
    "CPG_EXEC_INFER",
    "CPG_PROB_JUDG",
    "CPG_CERT_PROP",
    "CPG_CH_NORM",
    "CPG_CH_INH"
)

$bundleSamples = @(
    "samples/chain_rule_bundle.qc0",
    "samples/chain_rule_integrated_bundle.qc0",
    "samples/imported_claim_bundle.qc0",
    "samples/imported_claim_stress_gap_bundle.qc0"
)

New-Item -ItemType Directory -Force -Path $OutputRoot | Out-Null
$sampleDir = Join-Path $OutputRoot "samples"
New-Item -ItemType Directory -Force -Path $sampleDir | Out-Null

$summary = [ordered]@{
    outer_rounds = $OuterRounds
    inner_namespaces = $InnerNamespaces
    campaign_runs = 0
    observe_runs = 0
    export_runs = 0
    locus_packet_exports = 0
    locus_packet_imports = 0
    research_govern_runs = 0
    research_derive_runs = 0
    research_status_runs = 0
    research_readiness_runs = 0
    remediation_seed_runs = 0
    runtime_root_runs = 0
    explain_execution_runs = 0
    artifact_resolution_runs = 0
    bundle_runs = 0
    lock_runs = 0
    replay_runs = 0
    surface_runs = 0
    rna_normalize_runs = 0
    rna_compile_runs = 0
    dna_sequence_runs = 0
    roundtrip_runs = 0
    test_runs = 0
    failures = @()
    outcomes = @{}
    reports = @()
}

function Write-Utf8NoBom {
    param(
        [Parameter(Mandatory = $true)]
        [string]$Path,
        [Parameter(Mandatory = $true)]
        [string]$Content
    )

    $directory = Split-Path -Parent $Path
    if (-not [string]::IsNullOrWhiteSpace($directory)) {
        New-Item -ItemType Directory -Force -Path $directory | Out-Null
    }

    $encoding = New-Object System.Text.UTF8Encoding($false)
    [System.IO.File]::WriteAllText($Path, $Content, $encoding)
}

function Invoke-Mf {
    param(
        [string]$Namespace,
        [string[]]$CommandArgs,
        [string]$CapturePath
    )

    $l64 = Join-Path $WorkspaceRoot "target\release\l64.exe"
    if (-not (Test-Path $l64)) {
        throw "Missing binary: $l64"
    }

    $psi = New-Object System.Diagnostics.ProcessStartInfo
    $psi.FileName = $l64
    $psi.Arguments = ($CommandArgs -join ' ')
    $psi.WorkingDirectory = $WorkspaceRoot
    $psi.RedirectStandardOutput = $true
    $psi.RedirectStandardError = $true
    $psi.UseShellExecute = $false
    $psi.Environment["MF_CACHE_NAMESPACE"] = $Namespace

    $process = [System.Diagnostics.Process]::Start($psi)
    $stdout = $process.StandardOutput.ReadToEnd()
    $stderr = $process.StandardError.ReadToEnd()
    $process.WaitForExit()

    if ($CapturePath) {
        $payload = [ordered]@{
            args = $CommandArgs
            namespace = $Namespace
            exit_code = $process.ExitCode
            stdout = $stdout
            stderr = $stderr
        } | ConvertTo-Json -Depth 8
        Set-Content -LiteralPath $CapturePath -Value $payload -Encoding UTF8
    }

    if ($process.ExitCode -ne 0) {
        throw "l64 failed: $($CommandArgs -join ' ')`n$stderr`n$stdout"
    }

    return $stdout
}

Push-Location $WorkspaceRoot
try {
    cargo build --release -p l64 -p l64-cli -p l64-admin | Out-Host
    cargo test -q | Out-Host
    $summary.test_runs++

    for ($outer = 1; $outer -le $OuterRounds; $outer++) {
        for ($inner = 1; $inner -le $InnerNamespaces; $inner++) {
            $ns = "torture_r${outer}_n${inner}"
            $nsDir = Join-Path $OutputRoot $ns
            New-Item -ItemType Directory -Force -Path $nsDir | Out-Null

            Invoke-Mf -Namespace $ns -CommandArgs @("clear-cache", "--scope", "all") -CapturePath (Join-Path $nsDir "clear-cache.json") | Out-Null
            Invoke-Mf -Namespace $ns -CommandArgs @("surface-capabilities") -CapturePath (Join-Path $nsDir "surface-capabilities.json") | Out-Null
            Invoke-Mf -Namespace $ns -CommandArgs @("dump-runtime-roots") -CapturePath (Join-Path $nsDir "dump-runtime-roots.json") | Out-Null
            $summary.surface_runs++
            $summary.runtime_root_runs++

            Invoke-Mf -Namespace $ns -CommandArgs @("research-seed-export-remediation", "--persist") -CapturePath (Join-Path $nsDir "research-seed-export-remediation.json") | Out-Null
            Invoke-Mf -Namespace $ns -CommandArgs @("research-remediation-summary") -CapturePath (Join-Path $nsDir "research-remediation-summary.json") | Out-Null
            $summary.remediation_seed_runs++

            foreach ($bundle in $bundleSamples) {
                $bundleName = [IO.Path]::GetFileNameWithoutExtension($bundle)
                $bundleOut = Join-Path $nsDir "bundle-$bundleName.json"
                Invoke-Mf -Namespace $ns -CommandArgs @("certify-bundle", "--file", $bundle, "--conflict-policy", "exact-match") -CapturePath $bundleOut | Out-Null
                $summary.bundle_runs++
            }

            Invoke-Mf -Namespace $ns -CommandArgs @("roundtrip-check", "samples/chain_rule_bundle.qc0") -CapturePath (Join-Path $nsDir "roundtrip-chain-rule.json") | Out-Null
            Invoke-Mf -Namespace $ns -CommandArgs @("roundtrip-check", "samples/imported_claim_bundle.qc0") -CapturePath (Join-Path $nsDir "roundtrip-imported-claim.json") | Out-Null
            $summary.roundtrip_runs += 2

            $rnaPath = Join-Path $nsDir "lower-chain.gene.rna"
            $dnaPath = Join-Path $nsDir "lower-chain.gene.dna"
            Write-Utf8NoBom -Path $rnaPath -Content "i   :=   s   ||   k"
            Invoke-Mf -Namespace $ns -CommandArgs @("normalize-rna", $rnaPath) -CapturePath (Join-Path $nsDir "normalize-rna.json") | Out-Null
            Invoke-Mf -Namespace $ns -CommandArgs @("compile-rna", $rnaPath, "--out", $dnaPath, "--artifact-class", "gene", "--persist-lineage") -CapturePath (Join-Path $nsDir "compile-rna.json") | Out-Null
            Invoke-Mf -Namespace $ns -CommandArgs @("sequence-dna", $dnaPath) -CapturePath (Join-Path $nsDir "sequence-dna.json") | Out-Null
            $summary.rna_normalize_runs++
            $summary.rna_compile_runs++
            $summary.dna_sequence_runs++

            foreach ($campaign in $campaigns) {
                $stdout = Invoke-Mf -Namespace $ns -CommandArgs @("certify-derived", "--campaign", $campaign) -CapturePath (Join-Path $nsDir "$campaign-certify.json")
                $summary.campaign_runs++

                $reportId = switch ($campaign) {
                    "CPG_CHAIN_RULE" { "REPORT_THS_CHAIN_RULE_CPG_CHAIN_RULE" }
                    "CPG_CHAIN_RULE_RECIPE" { "REPORT_THS_CHAIN_RULE_RECIPE_CPG_CHAIN_RULE_RECIPE" }
                    "CPG_CHAIN_RULE_TRANSPORT" { "REPORT_THS_CHAIN_RULE_TRANSPORT_CPG_CHAIN_RULE_TRANSPORT" }
                    "CPG_BAYES_BRACE" { "REPORT_THS_BAYES_BRACE_CPG_BAYES_BRACE" }
                    "CPG_EXEC_INFER" { "REPORT_THS_EXEC_INFER_CPG_EXEC_INFER" }
                    "CPG_PROB_JUDG" { "REPORT_THS_PROB_JUDG_CPG_PROB_JUDG" }
                    "CPG_CERT_PROP" { "REPORT_THS_CERT_PROP_CPG_CERT_PROP" }
                    "CPG_CH_NORM" { "REPORT_THS_CH_NORM_CPG_CH_NORM" }
                    "CPG_CH_INH" { "REPORT_THS_CH_INH_CPG_CH_INH" }
                    default { throw "Unknown report for $campaign" }
                }

                if (-not $summary.outcomes.Contains($campaign)) { $summary.outcomes[$campaign] = @() }
                try {
                    $parsed = $stdout | ConvertFrom-Json
                    if ($parsed.verdict) {
                        $summary.outcomes[$campaign] += [string]$parsed.verdict
                    }
                } catch {
                    $summary.outcomes[$campaign] += "unparsed"
                }

                Invoke-Mf -Namespace $ns -CommandArgs @("observe-run", "--report", $reportId) -CapturePath (Join-Path $nsDir "$campaign-observe.json") | Out-Null
                $summary.observe_runs++

                Invoke-Mf -Namespace $ns -CommandArgs @("explain-execution", $reportId) -CapturePath (Join-Path $nsDir "$campaign-explain-execution.json") | Out-Null
                Invoke-Mf -Namespace $ns -CommandArgs @("resolve-artifact-path", $reportId) -CapturePath (Join-Path $nsDir "$campaign-resolve-artifact.json") | Out-Null
                $summary.explain_execution_runs++
                $summary.artifact_resolution_runs++

                $exportPath = Join-Path $sampleDir "$ns-$campaign.qc0"
                $export = Invoke-Mf -Namespace $ns -CommandArgs @("export-report", "--id", $reportId, "--to", "qc0") -CapturePath (Join-Path $nsDir "$campaign-export.json")
                Write-Utf8NoBom -Path $exportPath -Content $export
                $summary.export_runs++
                $summary.reports += $reportId

                $validationPath = Join-Path $sampleDir "$ns-$campaign-validation.qc0"
                $validation = Invoke-Mf -Namespace $ns -CommandArgs @("export-validation-bundle", "--id", $reportId, "--to", "qc0") -CapturePath (Join-Path $nsDir "$campaign-export-validation.json")
                Write-Utf8NoBom -Path $validationPath -Content $validation
                Invoke-Mf -Namespace $ns -CommandArgs @("validate", $validationPath, "--as", "qc0") -CapturePath (Join-Path $nsDir "$campaign-validate-validation-bundle.json") | Out-Null

                $packetPath = Join-Path $nsDir "$campaign.locus"
                Invoke-Mf -Namespace $ns -CommandArgs @("export-locus-packet", "--report-id", $reportId, "--out", $packetPath) -CapturePath (Join-Path $nsDir "$campaign-export-locus.json") | Out-Null
                Invoke-Mf -Namespace $ns -CommandArgs @("import-locus-packet", $packetPath) -CapturePath (Join-Path $nsDir "$campaign-import-locus.json") | Out-Null
                $summary.locus_packet_exports++
                $summary.locus_packet_imports++

                Invoke-Mf -Namespace $ns -CommandArgs @("research-govern-report", "--report-id", $reportId, "--persist") -CapturePath (Join-Path $nsDir "$campaign-research-govern.json") | Out-Null
                Invoke-Mf -Namespace $ns -CommandArgs @("research-derive-from-report", "--report-id", $reportId, "--persist") -CapturePath (Join-Path $nsDir "$campaign-research-derive.json") | Out-Null
                Invoke-Mf -Namespace $ns -CommandArgs @("research-promotion-readiness", $reportId) -CapturePath (Join-Path $nsDir "$campaign-research-readiness.json") | Out-Null
                $summary.research_govern_runs++
                $summary.research_derive_runs++
                $summary.research_readiness_runs++
            }

            Invoke-Mf -Namespace $ns -CommandArgs @("research-status") -CapturePath (Join-Path $nsDir "research-status.json") | Out-Null
            $summary.research_status_runs++

            $lockStdout = Invoke-Mf -Namespace $ns -CommandArgs @("lock-bundle", "samples/chain_rule_integrated_bundle.qc0", "--optimizer-policy", "conservative", "--conflict-policy", "exact-match") -CapturePath (Join-Path $nsDir "lock-bundle.json")
            $summary.lock_runs++
            try {
                $lockParsed = $lockStdout | ConvertFrom-Json
                if ($lockParsed.id) {
                    $blockId = [string]$lockParsed.id
                } elseif ($lockParsed.lock -and $lockParsed.lock.id) {
                    $blockId = [string]$lockParsed.lock.id
                } else {
                    $blockId = ""
                }
            } catch {
                $blockId = ""
            }
            if (-not [string]::IsNullOrWhiteSpace($blockId)) {
                Invoke-Mf -Namespace $ns -CommandArgs @("replay-with-lock", $blockId, "--parallel-obligations", "--max-obligation-workers", "3") -CapturePath (Join-Path $nsDir "replay-with-lock.json") | Out-Null
                $summary.replay_runs++
            } else {
                throw "Could not extract block_id from lock output"
            }
        }
    }
}
catch {
    $summary.failures += $_.Exception.Message
    throw
}
finally {
    $summaryPath = Join-Path $OutputRoot "summary.json"
    ($summary | ConvertTo-Json -Depth 8) | Set-Content -LiteralPath $summaryPath -Encoding UTF8
    Pop-Location
}
