# R08 SDK prerequisites

R08 certification requires the same cross-platform toolchain exercised by CI:

- Rust 1.98, selected by the repository toolchain file.
- .NET 10 SDK, used by the SDK-style `.rsproj` lifecycle.

The supported R08 lifecycle is exercised through the real .NET CLI and FerrumWeave SDK boundary:

```text
dotnet restore
dotnet build
dotnet run
dotnet test
dotnet clean
```

`dotnet test` currently fails intentionally with an explicit FerrumWeave SDK diagnostic because R08 does not yet provide a Rust test adapter. A silent zero-test success is not treated as supported behavior.
