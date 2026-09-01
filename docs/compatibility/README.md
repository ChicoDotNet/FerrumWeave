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

The R02 compatibility lane pins `FractalFir/rustc_codegen_clr@a9aa553b136fce00eceb41fba30758830500a63f` and `nightly-2025-10-14`. FerrumWeave's normal repository toolchain remains stable; nightly compiler internals are isolated to this compatibility lane. CLR execution for R02 is certified against .NET 10 LTS using SDK `10.0.400` and a `net10.0` runtime configuration.

R02 is deliberately a **vertical slice**, not broad Rust support. The certified positive program is intentionally tiny so the milestone proves the compiler/runtime path without pretending broad language coverage already exists.

## R03 — Core Rust semantics

**Status: Certified in CI — 11/11 functional contracts across Linux and Windows.**

R03 moves FerrumWeave from the ceremonial R02 slice to a deliberately small, coherent safe-Rust subset. Positive contracts compile the same Rust source through native `rustc` and the pinned CLR backend, then compare observable behavior exactly. The negative contract proves that source-language safety still gates code generation.

| Certified semantic family | Linux | Windows | R03 proof |
| --- | --- | --- | --- |
| Primitive integers and booleans | ✅ | ✅ | Native/CLR differential execution |
| Locals and assignment | ✅ | ✅ | Native/CLR differential execution |
| Arithmetic and comparison | ✅ | ✅ | Native/CLR differential execution |
| Functions, arguments, calls, and returns | ✅ | ✅ | Native/CLR differential execution |
| Conditional control flow | ✅ | ✅ | Native/CLR differential execution |
| Loops and MIR branching | ✅ | ✅ | Native/CLR differential execution |
| Tuples and structs | ✅ | ✅ | Native/CLR differential execution |
| Field reads and writes | ✅ | ✅ | Native/CLR differential execution |
| Shared references | ✅ | ✅ | Native/CLR differential execution |
| Mutable references | ✅ | ✅ | Native/CLR differential execution |
| Invalid safe-Rust borrowing | ✅ | ✅ | Compilation must fail with `E0502`; no executable artifact may be produced |

The cumulative positive semantic fixture has an explicit observable oracle of `42`; the native result must satisfy that oracle and the managed result must match native output byte-for-byte. The R03 lane keeps the R02 toolchain pins: `FractalFir/rustc_codegen_clr@a9aa553b136fce00eceb41fba30758830500a63f`, `nightly-2025-10-14`, and .NET 10 LTS / SDK `10.0.400`.

R03 certifies these semantic families only. It does **not** claim general Rust or `std` compatibility.

## Not implemented by R03

These remain future work unless and until their roadmap milestones are certified:

- broad `std` support or a general `println!` story;
- broad Rust language coverage beyond the enumerated R03 subset;
- Rust ↔ CTS type mappings;
- consuming .NET APIs from Rust;
- consuming Rust APIs from C#/VB/F#;
- `.rsproj` / MSBuild / `dotnet new rust`;
- NuGet and ProjectReference integration.

Those claims advance only when their roadmap milestones meet their Definition of Done.
