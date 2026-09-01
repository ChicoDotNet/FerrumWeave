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

## R02 — Rust → CLR vertical slice

**Status: Certified in CI — 5/5 functional contracts across Linux and Windows.**

R02 proves the first real Rust-language path into the CLR:

| Capability | Linux | Windows | Certified R02 proof |
| --- | --- | --- | --- |
| Real `.rs` source enters `rustc` | ✅ | ✅ | Direct `rustc` invocation with the pinned CLR codegen backend |
| Rust frontend / MIR path participates | ✅ | ✅ | `rustc_codegen_clr` is loaded as the codegen backend |
| Borrow checker remains authoritative | ✅ | ✅ | Deliberately invalid source is rejected with `E0502` |
| Managed PE/CLI output | ✅ | ✅ | PE + CLI header + CLR metadata + MethodDef entry point + ILONLY |
| Execute generated program with CoreCLR | ✅ | ✅ | `dotnet FerrumWeave.R02.exe` |
| Output | ✅ | ✅ | `Hello FerrumWeave` |
| Substitute C#/VB/F#/C/C++ implementation | No | No | Fixture and verifier explicitly reject source-language substitution |
| Native Rust launcher as implementation | No | No | Managed artifact is executed directly by CoreCLR |

The R02 compatibility lane pins `FractalFir/rustc_codegen_clr@a9aa553b136fce00eceb41fba30758830500a63f` and `nightly-2025-10-14`. FerrumWeave's normal repository toolchain remains stable; nightly compiler internals are isolated to this compatibility lane.

R02 is deliberately a **vertical slice**, not broad Rust support. The certified positive program is intentionally tiny so the milestone proves the compiler/runtime path without pretending R03 semantics already exist.

## Not implemented by R02

R02 does **not** yet provide a general Rust-on-.NET language implementation. In particular, these remain future work:

- a coherent safe-Rust semantic subset beyond the certified R02 slice;
- broad `std` support or a general `println!` story;
- Rust ↔ CTS type mappings;
- consuming .NET APIs from Rust;
- consuming Rust APIs from C#/VB/F#;
- `.rsproj` / MSBuild / `dotnet new rust`;
- NuGet and ProjectReference integration.

Those claims advance only when their roadmap milestones meet their Definition of Done.
