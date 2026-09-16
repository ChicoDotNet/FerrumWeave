# .NET consumer interoperability certification

This lane certifies that ordinary .NET test projects can consume the managed surface emitted from FerrumWeave Rust. It does **not** duplicate FerrumWeave's Rust behavioral suite.

The canonical observable is intentionally small: an external test project references `tests/fixtures/r09/RiskEngine/RiskEngine.rsproj`, builds it through the FerrumWeave SDK/codegen path, calls `FerrumWeave.RustApi.Answer()`, and requires the result `42`.

This is an R09 interoperability follow-up, not evidence for the separate R10 installed-package boundary. The current fixture imports the repository SDK explicitly, so it must not be used to claim `FW-R10-DIST-001` or `FW-R10-DX-003` GREEN.

## Generation policy

Consumer projects are generated during certification with the official .NET SDK templates instead of hand-maintaining copies of their package graphs.

The certified C# row uses:

```console
dotnet new xunit  -lang C# -f net10.0
dotnet new mstest -lang C# -f net10.0
dotnet new nunit  -lang C# -f net10.0
```

The F# row uses the same official framework templates with the language changed at generation time:

```console
dotnet new xunit  -lang F# -f net10.0
dotnet new mstest -lang F# -f net10.0
dotnet new nunit  -lang F# -f net10.0
```

The Visual Basic row uses the same official framework templates and their `VB` language selector:

```console
dotnet new xunit  -lang VB -f net10.0
dotnet new mstest -lang VB -f net10.0
dotnet new nunit  -lang VB -f net10.0
```

Each generator performs only normal consumer customization:

1. adds a `ProjectReference` to the canonical `RiskEngine.rsproj`;
2. replaces the template starter test with one framework- and language-idiomatic assertion;
3. leaves restore, build, and test as ordinary `dotnet` commands.

For F#, the generated test source is overwritten **in place** instead of deleting and recreating `.fs` files. This preserves the explicit compile ordering emitted by the official F# project template. C# and Visual Basic use the SDK's implicit compile glob, so their starter sources can be replaced directly.

No manual DLL copying, generated managed substitute, special assembly loading, or framework-specific FerrumWeave shim is allowed.

## Local certification

Prerequisites match repository CI: .NET SDK `10.0.400`, the repository Rust toolchain, and the pinned backend components for `nightly-2025-10-14`.

```powershell
rustup component add --toolchain nightly-2025-10-14 rustc-dev rust-src llvm-tools-preview

$root = Join-Path $env:TEMP "ferrumweave-consumer"

foreach ($framework in @("xunit", "mstest", "nunit")) {
    $project = ./eng/dotnet-consumers/New-CSharpConsumer.ps1 `
        -Framework $framework `
        -OutputRoot $root `
        -RepoRoot $PWD

    dotnet restore $project
    dotnet build $project --no-restore --configuration Release --nologo
    dotnet test $project --no-build --configuration Release --nologo
}

foreach ($framework in @("xunit", "mstest", "nunit")) {
    $project = ./eng/dotnet-consumers/New-FSharpConsumer.ps1 `
        -Framework $framework `
        -OutputRoot $root `
        -RepoRoot $PWD

    dotnet restore $project
    dotnet build $project --no-restore --configuration Release --nologo
    dotnet test $project --no-build --configuration Release --nologo
}

foreach ($framework in @("xunit", "mstest", "nunit")) {
    $project = ./eng/dotnet-consumers/New-VisualBasicConsumer.ps1 `
        -Framework $framework `
        -OutputRoot $root `
        -RepoRoot $PWD

    dotnet restore $project
    dotnet build $project --no-restore --configuration Release --nologo
    dotnet test $project --no-build --configuration Release --nologo
}
```

The dedicated `.NET consumer interoperability` workflow executes all three language rows across xUnit, MSTest, and NUnit on Ubuntu and Windows.
