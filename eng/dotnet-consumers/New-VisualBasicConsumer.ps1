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
    xunit  = "FerrumWeave.Interop.VisualBasic.Xunit"
    mstest = "FerrumWeave.Interop.VisualBasic.MSTest"
    nunit  = "FerrumWeave.Interop.VisualBasic.NUnit"
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

& dotnet new $Framework -lang "VB" -f net10.0 -n $projectName -o $projectDirectory --no-restore
if ($LASTEXITCODE -ne 0) {
    throw "dotnet new $Framework -lang VB failed with exit code $LASTEXITCODE"
}

$projects = @(Get-ChildItem -LiteralPath $projectDirectory -Filter "*.vbproj" -File)
if ($projects.Count -ne 1) {
    throw "Expected exactly one generated Visual Basic project, found $($projects.Count) in $projectDirectory"
}

$project = $projects[0]
& dotnet add $project.FullName reference $riskEngineProject
if ($LASTEXITCODE -ne 0) {
    throw "dotnet add reference failed with exit code $LASTEXITCODE"
}

Get-ChildItem -LiteralPath $projectDirectory -Filter "*.vb" -File | Remove-Item -Force

$testSource = switch ($Framework) {
    "xunit" {
@'
Imports Xunit

Public Class RustApiInteropTests
    <Fact>
    Public Sub Answer_crosses_the_FerrumWeave_boundary()
        Assert.Equal(42, Global.FerrumWeave.RustApi.Answer())
    End Sub
End Class
'@
    }
    "mstest" {
@'
Imports Microsoft.VisualStudio.TestTools.UnitTesting

<TestClass>
Public Class RustApiInteropTests
    <TestMethod>
    Public Sub Answer_crosses_the_FerrumWeave_boundary()
        Assert.AreEqual(42, Global.FerrumWeave.RustApi.Answer())
    End Sub
End Class
'@
    }
    "nunit" {
@'
Imports NUnit.Framework

<TestFixture>
Public Class RustApiInteropTests
    <Test>
    Public Sub Answer_crosses_the_FerrumWeave_boundary()
        Assert.That(Global.FerrumWeave.RustApi.Answer(), [Is].EqualTo(42))
    End Sub
End Class
'@
    }
}

$testPath = Join-Path $projectDirectory "RustApiInteropTests.vb"
[System.IO.File]::WriteAllText(
    $testPath,
    $testSource,
    [System.Text.UTF8Encoding]::new($false)
)

Write-Output $project.FullName
