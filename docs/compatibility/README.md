# Compatibility status

This document records **implemented and evidenced capability**, with evidence ownership stated explicitly. Historical oracle characterization is preserved, but it is not interchangeable with FerrumWeave product implementation evidence.

## R00 — Native Rust bootstrap

| Capability | Linux | Windows | Status |
| --- | --- | --- | --- |
| Native `Hello FerrumWeave` bootstrap | ✅ | ✅ | Implemented |
| Rust formatting / Clippy / tests | ✅ | ✅ | Implemented |
| Code-coverage gate | ✅ | CI-hosted | Implemented |
| Functional-contract gate | ✅ | exercised by test matrix | Implemented |

R00 does not claim managed CLR code generation.

## R01 — CLR artifact probe

**Status: Certified artifact-level foundation.**

R01 proves that FerrumWeave-owned stable-Rust infrastructure can emit a deterministic IL-only PE/CLI assembly, expose valid CLR metadata and a managed entry point, and execute it directly on CoreCLR on Linux and Windows. It does not claim Rust-language lowering.

## Evidence lifecycle

R02/R03 historical evidence is `oracle-proven`. Those milestones remain useful bootstrap history and differential characterization through the pinned `rustc_codegen_clr` backend, but they do not establish FerrumWeave ownership of the product backend.

Current product capability requires `FerrumWeave-backend-proven` evidence. Where a claim begins with Rust source, the required causal path is:

```text
.rsproj -> rustc -> FerrumWeave `rustc` CodegenBackend
        -> FerrumWeave MIR lowering
        -> FerrumWeave CIL / metadata
        -> managed assembly
        -> CoreCLR
```

The upstream oracle may support differential verification, but it is not a FerrumWeave runtime, SDK, or product dependency and cannot be the primary PASS evidence for current product capability.

## R02 — Rust → CLR vertical slice

**Status: Done as historical bootstrap / oracle characterization.**

The pinned R02 lane demonstrates that real `.rs` source can enter the real `rustc` frontend, pass through borrow checking and MIR, reach CLR-oriented code generation, produce an IL-only PE/CLI artifact, and execute on CoreCLR on Linux and Windows. The lane pins `FractalFir/rustc_codegen_clr@a9aa553b136fce00eceb41fba30758830500a63f` with `nightly-2025-10-14`.

That evidence remains reproducible and valuable, but its ownership level is `oracle-proven`. It characterizes expected Rust → CLR behavior; it does not certify that FerrumWeave itself implements that backend path.

## R03 — Core Rust semantics

**Status: Done as historical differential / oracle characterization.**

The R03 lane preserves a small safe-Rust characterization set across Linux and Windows: primitive values, locals, arithmetic, calls/returns, conditionals, loops, tuples/structs, field access, shared/mutable references, and an invalid-borrowing negative case. Positive cases compare native Rust behavior against the pinned CLR oracle; the negative case preserves the expected Rust compiler rejection.

These families remain `oracle-proven` historical evidence. They are useful as differential expectations while FerrumWeave expands its own MIR lowering, but they do not substitute for FerrumWeave backend certification of the same family.

## R04 — CLR / CTS foundation

**Status: Done — re-audited at artifact/type-system scope.**

R04 preserves FerrumWeave-owned CTS/projection and CLR reflection evidence. It establishes the declared type-system/artifact boundary; it does not by itself claim Rust-source consumption of managed APIs.

## R05-R09 — Current FerrumWeave product path

**Status: Done — FerrumWeave backend re-certified on the declared portable claims.**

R05-R09 carry current product evidence through the FerrumWeave-owned backend rather than the oracle. The certified path includes Rust-source causal managed calls and interop families, managed consumption of FerrumWeave-produced assemblies, `.rsproj` / MSBuild integration, and the mixed `.slnx` proof.

At the declared contract scope, the product evidence demonstrates:

- FerrumWeave's own `rustc` CodegenBackend is loaded through `rustc -Z codegen-backend=<FerrumWeave>`;
- real MIR-derived values and control flow reach FerrumWeave-owned CIL/metadata emission;
- managed static calls, construction, instance access, properties, and external managed assembly use are source-causally certified through FerrumWeave;
- Rust-produced managed assemblies are consumed from .NET without a native ABI substitute;
- `FerrumWeave.Sdk` drives `.rsproj -> rustc -> FerrumWeave` directly;
- the canonical mixed `.slnx` path preserves Rust-source causality into the final CoreCLR-observed business result;
- mutation/falsification checks reject hardcoded substitute behavior;
- Linux and Windows evidence is required where the claim is portable.

This does **not** imply general Rust or `std` compatibility, complete CTS coverage, general NuGet semantics, debugger completeness, or production readiness beyond the explicit contracts.

## R10

R10 remains paused pending explicit post-convergence replay and governance. Draft developer-experience work is preserved but is not being extended while backend ownership and evidence boundaries are converged.

## Evidence rule

Compatibility claims advance only when the implementation that owns the capability is in the causal path. For current product capability, successful `rustc_codegen_clr` execution may be secondary oracle evidence, never primary FerrumWeave PASS evidence.
