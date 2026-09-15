# ADR 0002: R02 isolated upstream codegen lane

- **Status:** Accepted for R02 bootstrap; superseded for the product backend path by the 2026-09-09 convergence amendment below
- **Date:** 2026-09-01
- **Scope:** R02 — Rust → CLR vertical slice

## Context

R02 must prove FerrumWeave's central architectural proposition with executable evidence:

> Real Rust source passes through the real Rust compiler frontend and borrow checker, reaches CLR-oriented code generation, and executes as managed code on CoreCLR.

ADR 0001 deliberately kept R01 independent of rustc internals. R01 proved the PE/CLI boundary with a tiny local emitter on stable Rust and required R02 to revisit the long-term code-generation strategy before extending that emitter into a competing compiler backend.

`FractalFir/rustc_codegen_clr` already provides the relevant prior art: a rustc codegen backend plus `cilly` infrastructure for producing .NET assemblies. The inspected revision is dual MIT/Apache-2.0 and therefore compatible with FerrumWeave's project license policy.

The integration has an unavoidable toolchain constraint: rustc codegen backends depend on private compiler APIs and therefore require a matching nightly compiler and compiler-development components. Moving the entire FerrumWeave repository to that nightly would make normal development depend on compiler-internal volatility that only the codegen boundary currently needs.

## Decision

R02 will consume `rustc_codegen_clr` **upstream-first** through an isolated compatibility lane.

The lane is pinned to:

- `FractalFir/rustc_codegen_clr@a9aa553b136fce00eceb41fba30758830500a63f`;
- Rust `nightly-2025-10-14`;
- `rust-src`;
- `rustc-dev`;
- `llvm-tools-preview`.

CI checks out the upstream source at that exact commit, builds the backend and linker from source, and invokes `rustc` directly with the CLR backend. FerrumWeave does not vendor the upstream repository and does not copy its source into the FerrumWeave tree.

FerrumWeave's normal repository toolchain remains on the supported stable Rust line. The nightly compiler exists only inside the R02 compatibility workflow and later compiler-internals lanes that explicitly require it.

## Certification boundary

R02 does not trust successful console output alone. The permanent verifier must independently prove that:

1. real `.rs` source is compiled through the pinned rustc/backend path;
2. the produced artifact is PE/CLI and contains a valid CLI header and CLR metadata root;
3. the artifact is marked IL-only and has a managed `MethodDef` entry point;
4. CoreCLR executes the artifact on Linux;
5. CoreCLR executes the artifact on Windows;
6. deliberately invalid Rust is rejected by the borrow checker with the expected Rust diagnostic before a managed executable can be produced;
7. no C#, Visual Basic, F#, C, or C++ source substitutes for the Rust implementation.

These facts are represented by the stable `FW-R02-CLR-*` contracts in `tests/r02/contracts.toml`.

## Why not extend the R01 emitter

The R01 writer proved FerrumWeave's understanding of the managed artifact boundary. Extending it into an independent Rust compiler backend would duplicate difficult work already represented upstream and would bypass the upstream-first strategy established by ADR 0001.

R02 therefore uses the smallest real integration that preserves rustc parsing, type checking, borrow checking, and MIR participation.

## Consequences

### Positive

- R02 proves real Rust semantics rather than source translation or a native wrapper.
- The repository's everyday toolchain remains stable and predictable.
- Compiler-internal volatility is isolated behind an explicit compatibility contract.
- Upstream provenance is clear and no large third-party source tree is vendored.
- Linux and Windows exercise the same conceptual Rust → rustc → CLR path.
- Updating the upstream commit or nightly becomes an observable compatibility change rather than an accidental dependency drift.

### Costs

- The R02 CI lane is heavier than the normal repository CI because it installs rustc-dev components and builds the upstream backend from source.
- The upstream commit and nightly compiler must move together when rustc internal APIs change.
- FerrumWeave temporarily depends on an experimental upstream implementation whose supported Rust subset is much smaller than the language as a whole.
- R02 certification does not imply broad `std`, CTS projection, `.rsproj`, or interoperability support.

## Upstream convergence policy — original R02 bootstrap

The original lifecycle was:

```text
consume upstream
    ↓
patch locally only when necessary
    ↓
contribute the fix upstream
    ↓
remove local divergence
```

R02 required no FerrumWeave-maintained source patch to the pinned upstream backend. That remains historically correct for the R02 bootstrap proof.

## 2026-09-09 convergence amendment

A strict causal audit of R05-R09 exposed an architectural split that R02 did not need to solve: R02/R03 used real `rustc` semantics through the upstream backend, while later `.rsproj`/SDK work used FerrumWeave-owned emitters that could synthesize managed behavior without deriving it from Rust MIR. Continuing to expand interoperability only through `rustc_codegen_clr` would therefore certify the upstream backend rather than transfer compiler ownership to FerrumWeave.

The product architecture is now explicitly:

```text
.rs / .rsproj
    ↓
rustc frontend + type system + borrow checker + MIR
    ↓
FerrumWeave CodegenBackend adapter
    ↓
FerrumWeave-owned MIR lowering
    ↓
FerrumWeave CTS / projection / CIL / metadata
    ↓
managed assembly
    ↓
CoreCLR
```

Accordingly:

- `rustc` and the compiler-private `CodegenBackend` interface remain legitimate dependencies of FerrumWeave's compiler lane.
- `rustc_codegen_clr` is no longer the intended product backend.
- The pinned upstream backend is retained only as a characterization/differential oracle where it helps establish expected semantics or discover compiler-integration behavior.
- A FerrumWeave product claim is not certified merely because the same Rust source works through the oracle; FerrumWeave itself must be in the causal codegen path.
- FerrumWeave will not copy the upstream lowering implementation as a shortcut. Small compiler-interface glue may be informed by public rustc/upstream examples with provenance and license compatibility, while MIR lowering remains FerrumWeave-owned.
- The compiler-private adapter is isolated from the stable workspace so normal repository development remains on stable Rust.

The first convergence gate is intentionally smaller than code generation: build FerrumWeave's own `dylib`, load it through `rustc -Z codegen-backend=...`, and prove that execution reaches FerrumWeave's `codegen_crate`. Until MIR lowering exists, that boundary fails explicitly and diagnostically. This proves ownership of the compiler handoff without pretending that code generation is already implemented.

After that handshake, capabilities migrate from oracle-only evidence to FerrumWeave-owned evidence incrementally: basic MIR values/control flow, function calls, managed calls, construction, instance/property access, external assemblies, then SDK/MSBuild integration.

## Retirement condition for the upstream oracle

`rustc_codegen_clr` can be removed from a product-critical test path once the corresponding FerrumWeave capability has:

1. a FerrumWeave-backend causal test from Rust source;
2. mutation/falsification evidence;
3. Linux and Windows evidence when portable;
4. replay against earlier compiler contracts; and
5. no product build dependency on the upstream checkout/backend.

The oracle may remain as an optional differential test or historical reference after product dependency reaches zero.

## Revisit conditions

Revisit this amended decision when one of the following becomes true:

- rustc exposes a materially more stable external codegen interface;
- FerrumWeave's own lowering demonstrates that the pinned compiler-private API is no longer appropriate;
- a new upstream standard makes maintaining a separate adapter clearly wasteful without surrendering FerrumWeave's product ownership;
- compatibility or CI cost requires repackaging the compiler-private adapter without weakening reproducibility.

## Non-decision

This ADR does not by itself define the full Rust ↔ CTS mapping, ownership/GC policy, managed API projection, `.rsproj`, NuGet integration, or debugger behavior. Those capabilities still require their own causal contracts and implementation evidence.
