# ADR 0002: R02 isolated upstream codegen lane

- **Status:** Accepted for R02
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

## Upstream convergence policy

The default lifecycle remains:

```text
consume upstream
    ↓
patch locally only when necessary
    ↓
contribute the fix upstream
    ↓
remove local divergence
```

R02 required no FerrumWeave-maintained source patch to the pinned upstream backend. If a future milestone does require a patch, its provenance, reason, upstream issue/PR, and removal condition must be recorded explicitly.

## Revisit conditions

This decision should be revisited when one of the following becomes true:

- `rustc_codegen_clr` exposes a more stable integration surface;
- rustc gains a materially different supported mechanism for external codegen backends;
- FerrumWeave needs backend behavior that upstream cannot reasonably provide;
- R03 or later semantic work demonstrates that the current pin cannot support the next coherent subset;
- CI build cost becomes significant enough to justify caching or packaging the pinned backend without weakening reproducibility.

## Non-decision

This ADR does not define Rust ↔ CTS type mappings, ownership/GC semantics, managed API projection, `.rsproj`, NuGet integration, or debugger behavior. Those remain owned by later milestones.
