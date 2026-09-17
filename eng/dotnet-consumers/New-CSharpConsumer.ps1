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
    xunit  = "FerrumWeave.Interop.CSharp.Xunit"
    mstest = "FerrumWeave.Interop.CSharp.MSTest"
    nunit  = "FerrumWeave.Interop.CSharp.NUnit"
}

$projectPath = New-FerrumWeaveInteropProject `
    -Framework $Framework `
    -Language "C#" `
    -ProjectName $projectNames[$Framework] `
    -ProjectExtension "csproj" `
    -OutputRoot $OutputRoot `
    -RepoRoot $RepoRoot

$projectDirectory = Split-Path -Parent $projectPath
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

Write-Utf8NoBom `
    -Path (Join-Path $projectDirectory "RustApiInteropTests.cs") `
    -Content $testSource

Write-Output $projectPath
