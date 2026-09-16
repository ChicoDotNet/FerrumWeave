[CmdletBinding()]
param(
    [Parameter(Mandatory = $true)]
    [string] $RepoRoot,

    [Parameter(Mandatory = $true)]
    [string] $WorkRoot
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

$packageId = "FerrumWeave.Sdk"
$packageVersion = "0.1.0-alpha.1"
$sdkProject = Join-Path $RepoRoot "sdk/FerrumWeave.Sdk/FerrumWeave.Sdk.csproj"
$feedRoot = Join-Path $WorkRoot "feed"
$externalRoot = Join-Path $WorkRoot "external"
$dotnetHome = Join-Path $WorkRoot "dotnet-home"
$packagesRoot = Join-Path $WorkRoot "packages"
$forbiddenRepoRoot = Join-Path $WorkRoot "must-not-use-repository"

Remove-Item -LiteralPath $WorkRoot -Recurse -Force -ErrorAction SilentlyContinue
New-Item -ItemType Directory -Force -Path $feedRoot, $externalRoot, $dotnetHome, $packagesRoot | Out-Null

$env:DOTNET_CLI_HOME = $dotnetHome
$env:NUGET_PACKAGES = $packagesRoot
$env:MSBuildSDKsPath = $null
$env:FerrumWeaveRepoRoot = $forbiddenRepoRoot

function Invoke-DotNet {
    param(
        [Parameter(Mandatory = $true)]
        [string[]] $Arguments,

        [string] $WorkingDirectory = $RepoRoot
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

function Assert-ZipEntry {
    param(
        [Parameter(Mandatory = $true)]
        [System.IO.Compression.ZipArchive] $Archive,

        [Parameter(Mandatory = $true)]
        [string] $EntryName
    )

    if ($null -eq $Archive.GetEntry($EntryName)) {
        throw "Packed SDK is missing required entry '$EntryName'."
    }
}

function Invoke-CoreClrProbe {
    param(
        [Parameter(Mandatory = $true)]
        [string] $ProbeProject,

        [Parameter(Mandatory = $true)]
        [string] $AssemblyPath,

        [Parameter(Mandatory = $true)]
        [int] $ExpectedValue
    )

    Invoke-DotNet -Arguments @(
        "run",
        "--project", $ProbeProject,
        "--no-restore",
        "--",
        $AssemblyPath,
        $ExpectedValue.ToString([System.Globalization.CultureInfo]::InvariantCulture)
    ) -WorkingDirectory (Split-Path -Parent $ProbeProject)
}

Write-Host "[FW-R10-DIST-001] Pack $packageId@$packageVersion"
Invoke-DotNet -Arguments @(
    "pack", $sdkProject,
    "-c", "Release",
    "-o", $feedRoot,
    "--nologo"
)

$nupkg = Join-Path $feedRoot "$packageId.$packageVersion.nupkg"
if (-not (Test-Path -LiteralPath $nupkg)) {
    throw "Expected package was not produced: $nupkg"
}

$archive = [System.IO.Compression.ZipFile]::OpenRead($nupkg)
try {
    Assert-ZipEntry -Archive $archive -EntryName "Sdk/Sdk.props"
    Assert-ZipEntry -Archive $archive -EntryName "Sdk/Sdk.targets"
    Assert-ZipEntry -Archive $archive -EntryName "content/console/.template.config/template.json"
    Assert-ZipEntry -Archive $archive -EntryName "content/console/HelloFerrum.rsproj"
    Assert-ZipEntry -Archive $archive -EntryName "content/console/src/main.rs"
    Assert-ZipEntry -Archive $archive -EntryName "tools/compiler/codegen-backend/Cargo.toml"
    Assert-ZipEntry -Archive $archive -EntryName "tools/compiler/cil/Cargo.toml"
    Assert-ZipEntry -Archive $archive -EntryName "tools/projection/types/Cargo.toml"
}
finally {
    $archive.Dispose()
}

$escapedFeedRoot = [System.Security.SecurityElement]::Escape($feedRoot)
$nugetConfig = Join-Path $externalRoot "NuGet.Config"
Write-Utf8NoBom -Path $nugetConfig -Content @"
<?xml version="1.0" encoding="utf-8"?>
<configuration>
  <packageSources>
    <clear />
    <add key="ferrumweave-isolated" value="$escapedFeedRoot" />
  </packageSources>
</configuration>
"@

Write-Host "[FW-R10-DIST-001] Install package by ID/version from isolated feed"
Invoke-DotNet -Arguments @(
    "new", "install", "$packageId@$packageVersion",
    "--add-source", $feedRoot,
    "--force"
) -WorkingDirectory $externalRoot

Write-Host "[FW-R10-DIST-001] Scaffold standard console family with -lang Rust outside repository"
Invoke-DotNet -Arguments @(
    "new", "console",
    "-lang", "Rust",
    "-n", "HelloFerrum"
) -WorkingDirectory $externalRoot

$projectRoot = Join-Path $externalRoot "HelloFerrum"
$projectPath = Join-Path $projectRoot "HelloFerrum.rsproj"
$rustSource = Join-Path $projectRoot "src/main.rs"
if (-not (Test-Path -LiteralPath $projectPath)) {
    throw "Rust console template did not produce $projectPath"
}
if (-not (Test-Path -LiteralPath $rustSource)) {
    throw "Rust console template did not produce $rustSource"
}

$projectText = Get-Content -LiteralPath $projectPath -Raw
if ($projectText -notmatch '<Project\s+Sdk="FerrumWeave\.Sdk"') {
    throw "Generated .rsproj does not use FerrumWeave.Sdk."
}
if ($projectText.Contains($RepoRoot, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "Generated project leaked a repository-relative path."
}

# FW-R10-DIST-001 requires explicit Rust-source causality 137 -> 211.
Write-Utf8NoBom -Path $rustSource -Content @'
#[no_mangle]
pub extern "C" fn answer() -> i32 {
    137
}
'@

Write-Host "[FW-R10-DIST-001] Restore/build first Rust observable with repository fallback poisoned"
Invoke-DotNet -Arguments @(
    "restore", $projectPath,
    "--configfile", $nugetConfig,
    "--packages", $packagesRoot,
    "--nologo"
) -WorkingDirectory $projectRoot
Invoke-DotNet -Arguments @(
    "build", $projectPath,
    "--no-restore",
    "--nologo"
) -WorkingDirectory $projectRoot

$assemblyPath = Join-Path $projectRoot "bin/Debug/net10.0/HelloFerrum.dll"
if (-not (Test-Path -LiteralPath $assemblyPath)) {
    throw "External Rust build did not produce managed assembly $assemblyPath"
}
$firstHash = (Get-FileHash -LiteralPath $assemblyPath -Algorithm SHA256).Hash

Write-Host "[FW-R10-DIST-001] Build independent CoreCLR reflection probe"
$probeRoot = Join-Path $externalRoot "CoreClrProbe"
Invoke-DotNet -Arguments @(
    "new", "console",
    "-lang", "C#",
    "-n", "CoreClrProbe",
    "-f", "net10.0",
    "--no-restore"
) -WorkingDirectory $externalRoot

$probeProject = Join-Path $probeRoot "CoreClrProbe.csproj"
$probeSource = Join-Path $probeRoot "Program.cs"
Write-Utf8NoBom -Path $probeSource -Content @'
using System.Reflection;

if (args.Length != 2)
{
    throw new InvalidOperationException("Expected managed assembly path and expected integer value.");
}

var assembly = Assembly.LoadFrom(Path.GetFullPath(args[0]));
var rustApi = assembly.GetType("FerrumWeave.RustApi", throwOnError: true)!;
var answer = rustApi.GetMethod("Answer", BindingFlags.Public | BindingFlags.Static)
    ?? throw new MissingMethodException("FerrumWeave.RustApi.Answer");
var actual = Convert.ToInt32(answer.Invoke(null, null));
var expected = int.Parse(args[1], System.Globalization.CultureInfo.InvariantCulture);
if (actual != expected)
{
    throw new InvalidOperationException($"Expected {expected}, observed {actual}.");
}
Console.WriteLine(actual);
'@

Invoke-DotNet -Arguments @(
    "restore", $probeProject,
    "--configfile", $nugetConfig,
    "--packages", $packagesRoot,
    "--nologo"
) -WorkingDirectory $probeRoot
Invoke-DotNet -Arguments @(
    "build", $probeProject,
    "--no-restore",
    "--nologo"
) -WorkingDirectory $probeRoot
Invoke-CoreClrProbe -ProbeProject $probeProject -AssemblyPath $assemblyPath -ExpectedValue 137

Write-Host "[FW-R10-DIST-001] Mutate only Rust source 137 -> 211 and rebuild"
Write-Utf8NoBom -Path $rustSource -Content @'
#[no_mangle]
pub extern "C" fn answer() -> i32 {
    211
}
'@

Invoke-DotNet -Arguments @(
    "build", $projectPath,
    "--no-restore",
    "--nologo"
) -WorkingDirectory $projectRoot

$secondHash = (Get-FileHash -LiteralPath $assemblyPath -Algorithm SHA256).Hash
if ($firstHash -eq $secondHash) {
    throw "Rust-only mutation 137 -> 211 did not change the managed assembly hash."
}
Invoke-CoreClrProbe -ProbeProject $probeProject -AssemblyPath $assemblyPath -ExpectedValue 211

if ($env:MSBuildSDKsPath) {
    throw "FW-R10-DIST-001 must not depend on MSBuildSDKsPath."
}
if (Test-Path -LiteralPath $forbiddenRepoRoot) {
    throw "Sentinel repository fallback path unexpectedly exists."
}

Write-Host "[FW-R10-DIST-001] PASS: installed package built externally and Rust-only mutation changed managed artifact + CoreCLR observable."
