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
    xunit  = "FerrumWeave.Interop.VisualBasic.Xunit"
    mstest = "FerrumWeave.Interop.VisualBasic.MSTest"
    nunit  = "FerrumWeave.Interop.VisualBasic.NUnit"
}

$projectPath = New-FerrumWeaveInteropProject `
    -Framework $Framework `
    -Language "VB" `
    -ProjectName $projectNames[$Framework] `
    -ProjectExtension "vbproj" `
    -OutputRoot $OutputRoot `
    -RepoRoot $RepoRoot

$projectDirectory = Split-Path -Parent $projectPath
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

Write-Utf8NoBom `
    -Path (Join-Path $projectDirectory "RustApiInteropTests.vb") `
    -Content $testSource

Write-Output $projectPath
