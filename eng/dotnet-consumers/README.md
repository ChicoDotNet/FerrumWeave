# .NET consumer interoperability certification

This lane certifies that ordinary .NET test projects can consume the managed surface emitted from FerrumWeave Rust. It does **not** duplicate FerrumWeave's Rust behavioral suite.

The canonical observable is intentionally small: an external test project references `tests/fixtures/r09/RiskEngine/RiskEngine.rsproj`, builds it through the FerrumWeave SDK/codegen path, calls `FerrumWeave.RustApi.Answer()`, and requires the result `42`.

This is an R09 interoperability follow-up, not evidence for the separate R10 installed-package boundary. The current fixture imports the repository SDK explicitly, so it must not be used to claim `FW-R10-DIST-001` or `FW-R10-DX-003` GREEN.

## Generation policy

Consumer projects are generated during certification with official .NET SDK templates instead of hand-maintaining copies of their package graphs:

```console
dotnet new xunit  -lang C# -f net10.0
dotnet new mstest -lang C# -f net10.0
dotnet new nunit  -lang C# -f net10.0

dotnet new xunit  -lang F# -f net10.0
dotnet new mstest -lang F# -f net10.0
dotnet new nunit  -lang F# -f net10.0

dotnet new xunit  -lang VB -f net10.0
dotnet new mstest -lang VB -f net10.0
dotnet new nunit  -lang VB -f net10.0
```

The harness has two deliberate abstraction layers:

1. `ConsumerProject.ps1` owns only the common scaffolding seam: `dotnet new`, locating the generated project, and adding the canonical `RiskEngine.rsproj` reference.
2. `Invoke-DotNetConsumer.ps1` owns only the common certification phases: Generate, Restore, Build, and Test across the 3 x 3 x 2 matrix.

Language-specific generators remain separate because their source contracts are genuinely different. C# and Visual Basic replace starter sources through the SDK's implicit compile glob. F# overwrites the template test source **in place** so the explicit compile ordering emitted by the official `.fsproj` remains intact. Framework-specific assertion syntax also stays in each language generator.

No manual DLL copying, generated managed substitute, special assembly loading, or framework-specific FerrumWeave shim is allowed.

## Local certification

Prerequisites match repository CI: .NET SDK `10.0.400`, the repository Rust toolchain, and the pinned backend components for `nightly-2025-10-14`.

```powershell
rustup component add --toolchain nightly-2025-10-14 rustc-dev rust-src llvm-tools-preview

$root = Join-Path $env:TEMP "ferrumweave-consumer"
$languages = @("C#", "F#", "VB.NET")
$frameworks = @("xunit", "mstest", "nunit")
$phases = @("Generate", "Restore", "Build", "Test")

foreach ($language in $languages) {
    foreach ($framework in $frameworks) {
        foreach ($phase in $phases) {
            ./eng/dotnet-consumers/Invoke-DotNetConsumer.ps1 `
                -Language $language `
                -Framework $framework `
                -Phase $phase `
                -OutputRoot $root `
                -RepoRoot $PWD
        }
    }
}
```

The dedicated `.NET consumer interoperability` workflow executes all three language rows across xUnit, MSTest, and NUnit on Ubuntu and Windows: 18 independent certification cells.
