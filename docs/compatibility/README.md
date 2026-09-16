# Compatibility status

This document records **implemented and evidenced capability in the current tree**. Historical bootstrap experiments remain available through Git history; they are not carried forward as executable compatibility machinery.

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

## Evidence ownership

Current product capability requires `FerrumWeave-backend-proven` evidence. Where a claim begins with Rust source, the required causal path is:

```text
.rsproj -> rustc -> FerrumWeave `rustc` CodegenBackend
        -> FerrumWeave MIR lowering
        -> FerrumWeave CIL / metadata
        -> managed assembly
        -> CoreCLR
```

`rustc_codegen_clr` remains acknowledged prior art, but it is not a FerrumWeave runtime, SDK, product dependency, test dependency, or active certification lane. The retired R02/R03 oracle implementation is preserved by repository history rather than by executable code in the current tree.

## R02-R03 — Historical bootstrap

R02 and R03 established early Rust-to-CLR characterization while the product architecture was still being discovered. Those experiments no longer define an executable compatibility surface in the current tree. Current Rust semantic claims must be certified through the FerrumWeave-owned backend.

## R04 — CLR / CTS foundation

**Status: Done — re-audited at artifact/type-system scope.**

R04 preserves FerrumWeave-owned CTS/projection and CLR reflection evidence. It establishes the declared type-system/artifact boundary; it does not by itself claim Rust-source consumption of managed APIs.

## R05-R09 — Current FerrumWeave product path

**Status: Done — promoted to `main` and re-certified on the declared portable claims.**

R05-R09 carry current product evidence through the FerrumWeave-owned backend. The certified path includes Rust-source causal managed calls and interop families, managed consumption of FerrumWeave-produced assemblies, `.rsproj` / MSBuild integration, and the mixed `.slnx` proof.

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

## R10 — Developer experience / 0.1 alpha

**Status: Active — resumed after certified R09 promotion.**

R10 owns the release/developer-experience boundary. Its primary product contract treats Rust as a .NET **language choice**: `dotnet new <template> -lang Rust`. Commands documented for the alpha remain target contracts until their corresponding executable CI evidence is GREEN; documentation must not present an uncertified target as released capability.

The architecture decision is recorded in [`../architecture/adr/0004-r10-rust-as-dotnet-template-language.md`](../architecture/adr/0004-r10-rust-as-dotnet-template-language.md), and the prerelease project-family sequence is defined in [`../roadmap/template-release-plan.md`](../roadmap/template-release-plan.md).

## Evidence rule

Compatibility claims advance only when the implementation that owns the capability is in the causal path.