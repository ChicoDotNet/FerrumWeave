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
    xunit  = "FerrumWeave.Interop.CSharp.Xunit"
    mstest = "FerrumWeave.Interop.CSharp.MSTest"
    nunit  = "FerrumWeave.Interop.CSharp.NUnit"
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

& dotnet new $Framework -lang "C#" -f net10.0 -n $projectName -o $projectDirectory --no-restore
if ($LASTEXITCODE -ne 0) {
    throw "dotnet new $Framework -lang C# failed with exit code $LASTEXITCODE"
}

$projects = @(Get-ChildItem -LiteralPath $projectDirectory -Filter "*.csproj" -File)
if ($projects.Count -ne 1) {
    throw "Expected exactly one generated C# project, found $($projects.Count) in $projectDirectory"
}

$project = $projects[0]
& dotnet add $project.FullName reference $riskEngineProject
if ($LASTEXITCODE -ne 0) {
    throw "dotnet add reference failed with exit code $LASTEXITCODE"
}

Get-ChildItem -LiteralPath $projectDirectory -Filter "*.cs" -File | Remove-Item -Force

$testSource = switch ($Framework) {
    "xunit" {
@'
using Xunit;

namespace FerrumWeave.Interop.CSharp.Tests;

public sealed class RustApiInteropTests
{
    [Fact]
    public void Answer_crosses_the_FerrumWeave_boundary()
    {
        Assert.Equal(42, global::FerrumWeave.RustApi.Answer());
    }
}
'@
    }
    "mstest" {
@'
using Microsoft.VisualStudio.TestTools.UnitTesting;

namespace FerrumWeave.Interop.CSharp.Tests;

[TestClass]
public sealed class RustApiInteropTests
{
    [TestMethod]
    public void Answer_crosses_the_FerrumWeave_boundary()
    {
        Assert.AreEqual(42, global::FerrumWeave.RustApi.Answer());
    }
}
'@
    }
    "nunit" {
@'
using NUnit.Framework;

namespace FerrumWeave.Interop.CSharp.Tests;

[TestFixture]
public sealed class RustApiInteropTests
{
    [Test]
    public void Answer_crosses_the_FerrumWeave_boundary()
    {
        Assert.That(global::FerrumWeave.RustApi.Answer(), Is.EqualTo(42));
    }
}
'@
    }
}

$testPath = Join-Path $projectDirectory "RustApiInteropTests.cs"
[System.IO.File]::WriteAllText(
    $testPath,
    $testSource,
    [System.Text.UTF8Encoding]::new($false)
)

Write-Output $project.FullName
