# ADR 0001: R01 managed assembly emission strategy

- **Status:** Accepted for R01
- **Date:** 2026-09-01
- **Scope:** R01 — CLR artifact probe

## Context

R01 must prove a narrow fact before FerrumWeave couples CLR emission to `rustc`:

> FerrumWeave can produce a valid managed PE/CLI assembly, with CIL and CLR metadata, that the .NET runtime executes on Windows and Linux.

The milestone must not be satisfied by generating C#, invoking a native Rust executable, hiding behavior behind P/Invoke, or launching another implementation from the emitted assembly.

Several implementation paths were considered.

### `rustc_codegen_clr` / `cilly`

`rustc_codegen_clr` is the most important prior art for the project and remains the expected upstream-first starting point for R02. Its `cilly` crate is dual MIT/Apache-2.0 and directly models CIL assembly construction.

At the upstream revision inspected for R01 (`a9aa553b136fce00eceb41fba30758830500a63f`), `cilly` is an internal workspace crate and currently enables nightly Rust features. Adopting that internal API for the output-only probe would force FerrumWeave away from its stable-toolchain policy before the `rustc` integration milestone actually needs nightly compiler internals.

### `dotnetdll`

`dotnetdll` can generate managed assemblies from scratch on stable Rust. Its current GPL-3.0-or-later license does not match FerrumWeave's MIT OR Apache-2.0 policy for linked production dependencies, so it is not adopted into the emitter.

### External IL assembler or generated .NET source

Using `ilasm`, C#, or another source-language compiler would validate those tools rather than FerrumWeave's ownership of the artifact boundary. That would weaken the R01 proof.

## Decision

For **R01 only**, FerrumWeave will implement the minimum ECMA-335/PE structures necessary for one deterministic managed probe assembly inside `compiler/cil`.

The emitter will own:

- the PE/COFF container required by the CLR;
- the CLI header;
- the metadata root and required streams;
- the minimal metadata tables needed by the probe;
- one tiny managed method body;
- the managed entry-point token;
- the framework-dependent runtime configuration used only to launch the probe.

The implementation is intentionally **not** a promise to build a general-purpose assembly writer from scratch. R01 code should remain small, deterministic, tested, and easy to replace or reshape when R02 integrates compiler-backend prior art.

## Consequences

### Positive

- R01 stays on the latest supported stable Rust line.
- FerrumWeave proves that it understands and owns the CLR artifact boundary rather than delegating the proof to another source compiler.
- The emitted artifact is architecture-neutral IL and can be executed unchanged by Windows and Linux CoreCLR.
- No production dependency changes FerrumWeave's license model.

### Costs

- FerrumWeave temporarily owns a small amount of low-level PE/CLI encoding code.
- ECMA-335 correctness must be protected by structural tests plus execution on the actual runtime.
- The R01 writer must not grow opportunistically into a second long-term CIL framework.

## R02 revisit requirement

R02 must revisit this decision before implementing real Rust → CLR code generation.

The default remains:

```text
consume upstream → patch only when necessary → contribute upstream → remove local divergence
```

In particular, R02 must evaluate current `rustc_codegen_clr`/`cilly` state and decide whether FerrumWeave can consume, adapt, or upstream changes rather than extending the R01 probe writer into a competing backend infrastructure.

## Non-decision

This ADR does **not** define Rust-to-CTS type semantics, ownership/GC mappings, `.rsproj`, SDK behavior, or .NET projection. Those belong to later milestones.
