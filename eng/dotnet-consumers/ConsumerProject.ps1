Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function New-FerrumWeaveInteropProject {
    [CmdletBinding()]
    param(
        [Parameter(Mandatory = $true)]
        [ValidateSet("xunit", "mstest", "nunit")]
        [string] $Framework,

        [Parameter(Mandatory = $true)]
        [string] $Language,

        [Parameter(Mandatory = $true)]
        [string] $ProjectName,

        [Parameter(Mandatory = $true)]
        [ValidateSet("csproj", "fsproj", "vbproj")]
        [string] $ProjectExtension,

        [Parameter(Mandatory = $true)]
        [string] $OutputRoot,

        [Parameter(Mandatory = $true)]
        [string] $RepoRoot
    )

    $projectDirectory = Join-Path $OutputRoot $ProjectName
    $riskEngineProject = Join-Path $RepoRoot "tests/fixtures/r09/RiskEngine/RiskEngine.rsproj"

    if (-not (Test-Path -LiteralPath $riskEngineProject)) {
        throw "Canonical R09 RiskEngine project was not found at $riskEngineProject"
    }

    if (Test-Path -LiteralPath $projectDirectory) {
        Remove-Item -LiteralPath $projectDirectory -Recurse -Force
    }

    New-Item -ItemType Directory -Path $OutputRoot -Force | Out-Null

    & dotnet new $Framework -lang $Language -f net10.0 -n $ProjectName -o $projectDirectory --no-restore | Out-Host
    if ($LASTEXITCODE -ne 0) {
        throw "dotnet new $Framework -lang $Language failed with exit code $LASTEXITCODE"
    }

    $projects = @(Get-ChildItem -LiteralPath $projectDirectory -Filter "*.$ProjectExtension" -File)
    if ($projects.Count -ne 1) {
        throw "Expected exactly one generated $Language project, found $($projects.Count) in $projectDirectory"
    }

    $project = $projects[0]
    & dotnet add $project.FullName reference $riskEngineProject | Out-Host
    if ($LASTEXITCODE -ne 0) {
        throw "dotnet add reference failed with exit code $LASTEXITCODE"
    }

    return $project.FullName
}

function Write-Utf8NoBom {
    [CmdletBinding()]
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
