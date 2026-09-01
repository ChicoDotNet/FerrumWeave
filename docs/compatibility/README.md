# Compatibility status

This document records **implemented and certified capability**, not roadmap intent.

## R00 — Native Rust bootstrap

| Capability | Linux | Windows | Status |
| --- | --- | --- | --- |
| Native `Hello FerrumWeave` bootstrap | ✅ | ✅ | Implemented |
| Rust formatting / Clippy / tests | ✅ | ✅ | Implemented |
| Code-coverage gate | ✅ | CI-hosted | Implemented |
| Functional-contract gate | ✅ | exercised by test matrix | Implemented |

R00 does not claim managed CLR code generation.

## R01 — CLR artifact probe

**Status: Certified in CI — 5/5 functional contracts, 96.21% Rust line coverage.**

R01's compatibility contract is intentionally small:

| Capability | Linux | Windows | Certified R01 proof |
| --- | --- | --- | --- |
| Emit one deterministic PE/CLI assembly | ✅ | ✅ | Same Rust emitter and same artifact format |
| CLR recognizes managed metadata | ✅ | ✅ | CLI header + metadata root + MethodDef entry point |
| Execute probe with CoreCLR | ✅ | ✅ | `dotnet FerrumWeave.Probe.dll` |
| Output | ✅ | ✅ | `Hello FerrumWeave` |
| Platform-specific native executable required | No | No | IL-only managed artifact |
| Native/PInvoke implementation | No | No | Explicitly excluded |

The CI runtime baseline for the R01 certification is .NET 10 LTS, currently installed from SDK `10.0.400` (runtime 10.0.11).

## Not implemented by R01

R01 does **not** mean that Rust source compiles to CIL yet. In particular, these remain future work:

- `rustc` / MIR integration;
- Rust semantic code generation;
- Rust ↔ CTS type mappings;
- consuming .NET APIs from Rust;
- consuming Rust APIs from C#/VB/F#;
- `.rsproj` / MSBuild / `dotnet new rust`;
- NuGet and ProjectReference integration.

Those claims advance only when their roadmap milestones meet their Definition of Done.
