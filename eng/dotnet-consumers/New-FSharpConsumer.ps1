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
. (Join-Path $PSScriptRoot "ConsumerProject.ps1")

$projectNames = @{
    xunit  = "FerrumWeave.Interop.FSharp.Xunit"
    mstest = "FerrumWeave.Interop.FSharp.MSTest"
    nunit  = "FerrumWeave.Interop.FSharp.NUnit"
}

$projectPath = New-FerrumWeaveInteropProject `
    -Framework $Framework `
    -Language "F#" `
    -ProjectName $projectNames[$Framework] `
    -ProjectExtension "fsproj" `
    -OutputRoot $OutputRoot `
    -RepoRoot $RepoRoot

$projectDirectory = Split-Path -Parent $projectPath
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

Write-Utf8NoBom `
    -Path $testFiles[0].FullName `
    -Content $testSource

Write-Output $projectPath
