[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [ValidateSet("C#", "F#", "VB.NET")]
    [string] $Language,

    [Parameter(Mandatory = $true)]
    [ValidateSet("xunit", "mstest", "nunit")]
    [string] $Framework,

    [Parameter(Mandatory = $true)]
    [ValidateSet("Generate", "Restore", "Build", "Test")]
    [string] $Phase,

    [Parameter(Mandatory = $true)]
    [string] $OutputRoot,

    [Parameter(Mandatory = $true)]
    [string] $RepoRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$consumer = switch ($Language) {
    "C#" {
        [pscustomobject]@{
            Generator = "New-CSharpConsumer.ps1"
            ProjectExtension = "csproj"
        }
    }
    "F#" {
        [pscustomobject]@{
            Generator = "New-FSharpConsumer.ps1"
            ProjectExtension = "fsproj"
        }
    }
    "VB.NET" {
        [pscustomobject]@{
            Generator = "New-VisualBasicConsumer.ps1"
            ProjectExtension = "vbproj"
        }
    }
}

if ($Phase -eq "Generate") {
    $generator = Join-Path $PSScriptRoot $consumer.Generator
    & $generator `
        -Framework $Framework `
        -OutputRoot $OutputRoot `
        -RepoRoot $RepoRoot | Out-Host
    return
}

$projects = @(
    Get-ChildItem -LiteralPath $OutputRoot -Filter "*.$($consumer.ProjectExtension)" -File -Recurse
)

if ($projects.Count -ne 1) {
    throw "Expected exactly one generated $Language consumer project, found $($projects.Count) in $OutputRoot"
}

$project = $projects[0].FullName
$arguments = switch ($Phase) {
    "Restore" { @("restore", $project) }
    "Build" { @("build", $project, "--no-restore", "--configuration", "Release", "--nologo") }
    "Test" { @("test", $project, "--no-build", "--configuration", "Release", "--nologo") }
}

& dotnet @arguments
if ($LASTEXITCODE -ne 0) {
    throw "dotnet $($Phase.ToLowerInvariant()) failed for $Language/$Framework with exit code $LASTEXITCODE"
}
