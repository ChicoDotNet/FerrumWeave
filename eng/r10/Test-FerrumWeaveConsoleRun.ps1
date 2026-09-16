[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string] $RepoRoot,

    [Parameter(Mandatory = $true)]
    [string] $WorkRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$distributionWorkRoot = Join-Path $WorkRoot "distribution"
$distributionVerifier = Join-Path $RepoRoot "eng/r10/Test-FerrumWeaveSdkDistribution.ps1"

# Replay the already-certified distributable boundary first. This intentionally
# leaves the external project at the 211 mutation, proving the next assertion
# observes Rust source through the installed FerrumWeave product path.
& pwsh -NoLogo -NoProfile -File $distributionVerifier -RepoRoot $RepoRoot -WorkRoot $distributionWorkRoot
if ($LASTEXITCODE -ne 0) {
    throw "FW-R10-DX-003 prerequisite FW-R10-DIST-001 replay failed with exit code $LASTEXITCODE."
}

$projectRoot = Join-Path $distributionWorkRoot "external/HelloFerrum"
$projectPath = Join-Path $projectRoot "HelloFerrum.rsproj"
if (-not (Test-Path -LiteralPath $projectPath)) {
    throw "FW-R10-DX-003 prerequisite project was not generated: $projectPath"
}

Write-Host "[FW-R10-DX-003] RED boundary: generated installed console must run directly"
Push-Location $projectRoot
try {
    $output = & dotnet run --project $projectPath --no-restore 2>&1
    $exitCode = $LASTEXITCODE
}
finally {
    Pop-Location
}

$outputText = ($output | Out-String).Trim()
if ($exitCode -ne 0) {
    throw "FW-R10-DX-003 RED: dotnet run failed with exit code $exitCode.`n$outputText"
}

if ($outputText -notmatch '(?m)^211\s*$') {
    throw "FW-R10-DX-003 RED: dotnet run did not expose the Rust-source-causal observable 211. Output:`n$outputText"
}

Write-Host "[FW-R10-DX-003] PASS: installed Rust console ran directly and exposed observable 211."
