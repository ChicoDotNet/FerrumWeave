[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string] $RepoRoot,

    [Parameter(Mandatory = $true)]
    [string] $WorkRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$distributionRoot = Join-Path $WorkRoot "distribution"
$distributionVerifier = Join-Path $RepoRoot "eng/r10/Test-FerrumWeaveSdkDistribution.ps1"

& pwsh -NoLogo -NoProfile -File $distributionVerifier -RepoRoot $RepoRoot -WorkRoot $distributionRoot
if ($LASTEXITCODE -ne 0) {
    throw "FW-R10-CLR-ENTRY-001 prerequisite FW-R10-DIST-001 replay failed with exit code $LASTEXITCODE."
}

$externalRoot = Join-Path $distributionRoot "external"
$projectRoot = Join-Path $externalRoot "HelloFerrum"
$projectPath = Join-Path $projectRoot "HelloFerrum.rsproj"
$rustSource = Join-Path $projectRoot "src/main.rs"
$assemblyPath = Join-Path $projectRoot "bin/Debug/net10.0/HelloFerrum.dll"
$nugetConfig = Join-Path $externalRoot "NuGet.Config"
$packagesRoot = Join-Path $distributionRoot "packages"

function Invoke-DotNet {
    param(
        [Parameter(Mandatory = $true)]
        [string[]] $Arguments,

        [Parameter(Mandatory = $true)]
        [string] $WorkingDirectory
    )

    Push-Location $WorkingDirectory
    try {
        & dotnet @Arguments
        if ($LASTEXITCODE -ne 0) {
            throw "dotnet $($Arguments -join ' ') failed with exit code $LASTEXITCODE."
        }
    }
    finally {
        Pop-Location
    }
}

function Write-Utf8NoBom {
    param(
        [Parameter(Mandatory = $true)]
        [string] $Path,

        [Parameter(Mandatory = $true)]
        [string] $Content
    )

    [System.IO.File]::WriteAllText(
        $Path,
        $Content,
        [System.Text.UTF8Encoding]::new($false)
    )
}

Write-Host "[FW-R10-CLR-ENTRY-001] Inspect managed entry identity from an independent .NET consumer"
$probeRoot = Join-Path $externalRoot "EntryPointProbe"
Invoke-DotNet -Arguments @(
    "new", "console",
    "-lang", "C#",
    "-n", "EntryPointProbe",
    "-f", "net10.0",
    "--no-restore"
) -WorkingDirectory $externalRoot

$probeProject = Join-Path $probeRoot "EntryPointProbe.csproj"
$probeSource = Join-Path $probeRoot "Program.cs"
Write-Utf8NoBom -Path $probeSource -Content @'
using System.Reflection;

if (args.Length != 1)
{
    throw new InvalidOperationException("Expected managed assembly path.");
}

var assembly = Assembly.LoadFrom(Path.GetFullPath(args[0]));
var entry = assembly.EntryPoint ?? throw new MissingMethodException("Assembly.EntryPoint");
if (entry.Name != "Main")
{
    throw new InvalidOperationException($"Expected entry method Main, observed {entry.Name}.");
}
if (!entry.IsStatic)
{
    throw new InvalidOperationException("Managed entry point must be static.");
}
if (entry.DeclaringType?.FullName != "HelloFerrum.Program")
{
    throw new InvalidOperationException(
        $"Expected HelloFerrum.Program entry type, observed {entry.DeclaringType?.FullName ?? "<null>"}."
    );
}
if (entry.ReturnType != typeof(void) || entry.GetParameters().Length != 0)
{
    throw new InvalidOperationException($"Unexpected initial entry signature: {entry}.");
}

var helper = entry.DeclaringType.GetMethod(
    "__FerrumWeaveRustMain",
    BindingFlags.NonPublic | BindingFlags.Static
) ?? throw new MissingMethodException("HelloFerrum.Program.__FerrumWeaveRustMain");

var il = entry.GetMethodBody()?.GetILAsByteArray()
    ?? throw new InvalidOperationException("Program.Main has no IL body.");
if (il.Length != 6 || il[0] != 0x28 || il[^1] != 0x2A)
{
    throw new InvalidOperationException(
        $"Program.Main must be the compiler-owned call adapter; IL={Convert.ToHexString(il)}"
    );
}
var targetToken = BitConverter.ToInt32(il, 1);
var called = entry.Module.ResolveMethod(targetToken);
if (called?.MetadataToken != helper.MetadataToken)
{
    throw new InvalidOperationException(
        $"Program.Main does not call the projected Rust-main helper; token=0x{targetToken:X8}."
    );
}

Console.WriteLine($"{entry.DeclaringType.FullName}.{entry.Name}->{helper.Name}");
'@

Invoke-DotNet -Arguments @(
    "restore", $probeProject,
    "--configfile", $nugetConfig,
    "--packages", $packagesRoot,
    "--nologo"
) -WorkingDirectory $probeRoot
Invoke-DotNet -Arguments @(
    "run", "--project", $probeProject,
    "--no-restore",
    "--",
    $assemblyPath
) -WorkingDirectory $probeRoot

Write-Host "[FW-R10-CLR-ENTRY-001] Execute installed Rust console through normal dotnet run"
Push-Location $projectRoot
try {
    $runOutput = & dotnet run --project $projectPath --no-restore 2>&1
    $runExitCode = $LASTEXITCODE
}
finally {
    Pop-Location
}
if ($runExitCode -ne 0) {
    throw "FW-R10-CLR-ENTRY-001 dotnet run failed with exit code $runExitCode.`n$($runOutput | Out-String)"
}

Write-Host "[FW-R10-CLR-ENTRY-001] Falsify source causality by removing Rust main"
Write-Utf8NoBom -Path $rustSource -Content @'
#[no_mangle]
pub extern "C" fn answer() -> i32 {
    211
}
'@

Push-Location $projectRoot
try {
    $negativeOutput = & dotnet build $projectPath --no-restore --nologo 2>&1
    $negativeExitCode = $LASTEXITCODE
}
finally {
    Pop-Location
}
if ($negativeExitCode -eq 0) {
    throw "FW-R10-CLR-ENTRY-001 falsifier failed: executable build succeeded after Rust main was removed."
}
$negativeText = ($negativeOutput | Out-String)
if ($negativeText -notmatch 'E0601|main function not found') {
    throw "FW-R10-CLR-ENTRY-001 falsifier failed for an unexpected reason.`n$negativeText"
}

Write-Host "[FW-R10-CLR-ENTRY-001] PASS: Rust main projected to static HelloFerrum.Program.Main, the CLI entry point executed on CoreCLR, and removing Rust main failed closed."
