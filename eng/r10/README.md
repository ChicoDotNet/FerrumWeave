# R10 engineering certifiers

R10 uses executable scripts here as cross-platform acceptance boundaries for installed-package developer experience.

- `Test-FerrumWeaveSdkDistribution.ps1` — `FW-R10-DIST-001`: pack/install/scaffold/build outside the repository and preserve Rust-source-causal managed observables.
- `Test-FerrumWeaveClrEntryPoint.ps1` — `FW-R10-CLR-ENTRY-001`: certify ordinary Rust `fn main()` as a static `<RootNamespace>.Program.Main` CLI entry point, prove the adapter calls the projected Rust-main helper, execute it through `dotnet run`, and falsify source causality by removing Rust `main`.
- `Test-FerrumWeaveConsoleRun.ps1` — `FW-R10-DX-003`: require the installed console's meaningful Rust-source-causal direct-run observable. This remains a stronger contract than entry-point existence alone.
