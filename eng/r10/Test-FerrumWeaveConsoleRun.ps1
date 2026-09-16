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

# Replay the already-certified distributable boundary first. The verifier leaves
# the external project at the Rust-only 211 mutation and, for an executable
# console, preserves ordinary `fn main` as the causal process-entry source.
& pwsh -NoLogo -NoProfile -File $distributionVerifier -RepoRoot $RepoRoot -WorkRoot $distributionWorkRoot
if ($LASTEXITCODE -ne 0) {
    throw "FW-R10-DX-003 prerequisite FW-R10-DIST-001 replay failed with exit code $LASTEXITCODE."
}

# Re-enter the same isolated template/package environment created by the child
# verifier so coexistence assertions observe the installed FerrumWeave package.
$env:DOTNET_CLI_HOME = Join-Path $distributionWorkRoot "dotnet-home"
$env:NUGET_PACKAGES = Join-Path $distributionWorkRoot "packages"
$env:MSBuildSDKsPath = $null

$projectRoot = Join-Path $distributionWorkRoot "external/HelloFerrum"
$projectPath = Join-Path $projectRoot "HelloFerrum.rsproj"
$rustSource = Join-Path $projectRoot "src/main.rs"
$assemblyPath = Join-Path $projectRoot "bin/Debug/net10.0/HelloFerrum.dll"
if (-not (Test-Path -LiteralPath $projectPath)) {
    throw "FW-R10-DX-003 prerequisite project was not generated: $projectPath"
}
if (-not (Test-Path -LiteralPath $rustSource)) {
    throw "FW-R10-DX-003 prerequisite Rust source was not generated: $rustSource"
}
if (-not (Test-Path -LiteralPath $assemblyPath)) {
    throw "FW-R10-DX-003 prerequisite managed assembly was not built: $assemblyPath"
}

$sourceText = Get-Content -LiteralPath $rustSource -Raw
if ($sourceText -notmatch '(?m)^fn main\(\)\s*\{') {
    throw "FW-R10-CLR-ENTRY-001 requires ordinary Rust fn main in the generated console source."
}
if ($sourceText -notmatch 'std::process::exit\(answer\(\)\)') {
    throw "FW-R10-CLR-ENTRY-001 verifier expected main to route the Rust answer through process::exit."
}

Write-Host "[FW-R10-DX-003] Verify built-in console language variants still coexist"
$coexistRoot = Join-Path $distributionWorkRoot "coexistence"
New-Item -ItemType Directory -Force -Path $coexistRoot | Out-Null
foreach ($variant in @(
    @{ Language = "C#"; Name = "CoexistCSharp"; Extension = ".csproj" },
    @{ Language = "VB"; Name = "CoexistVisualBasic"; Extension = ".vbproj" },
    @{ Language = "F#"; Name = "CoexistFSharp"; Extension = ".fsproj" }
)) {
    & dotnet new console -lang $variant.Language -n $variant.Name -o (Join-Path $coexistRoot $variant.Name) --no-restore
    if ($LASTEXITCODE -ne 0) {
        throw "Built-in console language '$($variant.Language)' stopped scaffolding after FerrumWeave installation."
    }
    $expectedProject = Join-Path (Join-Path $coexistRoot $variant.Name) "$($variant.Name)$($variant.Extension)"
    if (-not (Test-Path -LiteralPath $expectedProject)) {
        throw "Built-in console language '$($variant.Language)' did not produce $expectedProject"
    }
}

Write-Host "[FW-R10-CLR-ENTRY-001] Verify managed Program.Main identity and Rust-derived result"
$probeRoot = Join-Path $distributionWorkRoot "external/CoreClrProbe"
$probeProject = Join-Path $probeRoot "CoreClrProbe.csproj"
$probeSource = Join-Path $probeRoot "Program.cs"
if (-not (Test-Path -LiteralPath $probeProject)) {
    throw "Distribution replay did not leave the independent CoreCLR probe project."
}

[System.IO.File]::WriteAllText(
    $probeSource,
    @'
using System.Reflection;

if (args.Length != 1)
{
    throw new InvalidOperationException("Expected the managed Rust assembly path.");
}

var assembly = Assembly.LoadFrom(Path.GetFullPath(args[0]));
var entry = assembly.EntryPoint ?? throw new MissingMethodException("Managed CLI entry point");
if (entry.DeclaringType?.FullName != "HelloFerrum.Program")
{
    throw new InvalidOperationException($"Expected HelloFerrum.Program, observed {entry.DeclaringType?.FullName ?? "<null>"}.");
}
if (entry.Name != "Main" || !entry.IsStatic)
{
    throw new InvalidOperationException("Expected a static Program.Main entry method.");
}
if (entry.ReturnType != typeof(int) || entry.GetParameters().Length != 0)
{
    throw new InvalidOperationException($"Expected static int Main(), observed {entry}.");
}
var observed = Convert.ToInt32(entry.Invoke(null, null));
if (observed != 211)
{
    throw new InvalidOperationException($"Expected Rust-derived entry result 211, observed {observed}.");
}
Console.WriteLine($"{entry.DeclaringType!.FullName}.{entry.Name}={observed}");
'@,
    [System.Text.UTF8Encoding]::new($false)
)

Push-Location $probeRoot
try {
    & dotnet build $probeProject --no-restore --nologo
    if ($LASTEXITCODE -ne 0) {
        throw "FW-R10-CLR-ENTRY-001 reflection probe failed to rebuild."
    }
    & dotnet run --project $probeProject --no-build --no-restore -- $assemblyPath
    if ($LASTEXITCODE -ne 0) {
        throw "FW-R10-CLR-ENTRY-001 reflection probe failed with exit code $LASTEXITCODE."
    }
}
finally {
    Pop-Location
}

Write-Host "[FW-R10-DX-003] Execute installed Rust console through dotnet run"
Push-Location $projectRoot
try {
    $output = & dotnet run --project $projectPath --no-restore 2>&1
    $exitCode = $LASTEXITCODE
}
finally {
    Pop-Location
}

$outputText = ($output | Out-String).Trim()
if ($exitCode -ne 211) {
    throw "FW-R10-DX-003 RED: expected Rust-source-causal process exit 211, observed $exitCode.`n$outputText"
}

Write-Host "[FW-R10-DX-003] PASS: installed Rust console executed Program.Main and propagated Rust-source-causal exit code 211."
