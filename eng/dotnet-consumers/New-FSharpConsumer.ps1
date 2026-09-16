[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateSet("xunit", "mstest", "nunit")]
    [string] $Framework,

    [Parameter(Mandatory = $true)]
    [string] $OutputRoot,

    [Parameter(Mandatory = $true)]
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$projectNames = @{
    xunit  = "FerrumWeave.Interop.FSharp.Xunit"
    mstest = "FerrumWeave.Interop.FSharp.MSTest"
    nunit  = "FerrumWeave.Interop.FSharp.NUnit"
}

$projectName = $projectNames[$Framework]
$projectDirectory = Join-Path $OutputRoot $projectName
$riskEngineProject = Join-Path $RepoRoot "tests/fixtures/r09/RiskEngine/RiskEngine.rsproj"

if (-not (Test-Path -LiteralPath $riskEngineProject)) {
    throw "Canonical R09 RiskEngine project was not found at $riskEngineProject"
}

if (Test-Path -LiteralPath $projectDirectory) {
    Remove-Item -LiteralPath $projectDirectory -Recurse -Force
}

New-Item -ItemType Directory -Path $OutputRoot -Force | Out-Null

& dotnet new $Framework -lang "F#" -f net10.0 -n $projectName -o $projectDirectory --no-restore
if ($LASTEXITCODE -ne 0) {
    throw "dotnet new $Framework -lang F# failed with exit code $LASTEXITCODE"
}

$projects = @(Get-ChildItem -LiteralPath $projectDirectory -Filter "*.fsproj" -File)
if ($projects.Count -ne 1) {
    throw "Expected exactly one generated F# project, found $($projects.Count) in $projectDirectory"
}

$project = $projects[0]
& dotnet add $project.FullName reference $riskEngineProject
if ($LASTEXITCODE -ne 0) {
    throw "dotnet add reference failed with exit code $LASTEXITCODE"
}

$testMarker = switch ($Framework) {
    "xunit"  { "[<Fact>]" }
    "mstest" { "[<TestMethod>]" }
    "nunit"  { "[<Test>]" }
}

$testFiles = @(
    Get-ChildItem -LiteralPath $projectDirectory -Filter "*.fs" -File |
        Where-Object {
            (Get-Content -LiteralPath $_.FullName -Raw).Contains($testMarker)
        }
)

if ($testFiles.Count -ne 1) {
    $allSources = @(
        Get-ChildItem -LiteralPath $projectDirectory -Filter "*.fs" -File |
            ForEach-Object { $_.Name }
    ) -join ", "
    throw "Expected exactly one F# template test source containing $testMarker, found $($testFiles.Count). Sources: $allSources"
}

$testSource = switch ($Framework) {
    "xunit" {
@'
module FerrumWeave.Interop.FSharp.Tests.RustApiInteropTests

open Xunit

[<Fact>]
let ``Answer crosses the FerrumWeave boundary`` () =
    Assert.Equal(42, FerrumWeave.RustApi.Answer())
'@
    }
    "mstest" {
@'
namespace FerrumWeave.Interop.FSharp.Tests

open Microsoft.VisualStudio.TestTools.UnitTesting

[<TestClass>]
type RustApiInteropTests() =
    [<TestMethod>]
    member _.``Answer crosses the FerrumWeave boundary``() =
        Assert.AreEqual(42, FerrumWeave.RustApi.Answer())
'@
    }
    "nunit" {
@'
namespace FerrumWeave.Interop.FSharp.Tests

open NUnit.Framework

[<TestFixture>]
type RustApiInteropTests() =
    [<Test>]
    member _.``Answer crosses the FerrumWeave boundary``() =
        Assert.That(FerrumWeave.RustApi.Answer(), Is.EqualTo(42))
'@
    }
}

[System.IO.File]::WriteAllText(
    $testFiles[0].FullName,
    $testSource,
    [System.Text.UTF8Encoding]::new($false)
)

Write-Output $project.FullName
