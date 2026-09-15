# Upstream: `rustc_codegen_clr`

FerrumWeave treats [`FractalFir/rustc_codegen_clr`](https://github.com/FractalFir/rustc_codegen_clr) as important prior art and a pinned **characterization oracle / differential reference** for Rust → CLR behavior. It is **not a FerrumWeave runtime, SDK, or product dependency**, and it no longer occupies the product-backend slot.

The current product path is owned by the FerrumWeave `rustc` CodegenBackend:

```text
.rs / .rsproj
    ↓
rustc frontend + type system + borrow checker + MIR
    ↓
FerrumWeave `rustc` CodegenBackend
    ↓
FerrumWeave-owned MIR lowering
    ↓
FerrumWeave CTS / projection / CIL / metadata
    ↓
managed assembly
    ↓
CoreCLR
```

## R02 pinned oracle snapshot

- Repository: `FractalFir/rustc_codegen_clr`
- Branch inspected: `main`
- Commit: `a9aa553b136fce00eceb41fba30758830500a63f`
- Commit message: `Updated rustc version`
- Rust toolchain: `nightly-2025-10-14`
- Required components: `rust-src`, `rustc-dev`, `llvm-tools-preview`
- License: MIT OR Apache-2.0

At this revision the upstream workspace contains both the `rustc_codegen_clr` compiler backend and `cilly`, the assembly/linker infrastructure used to produce managed output.

The R02/R03 oracle workflows check out this exact upstream commit in CI, build the backend and linker from source, and invoke real `rustc` through `-Z codegen-backend`. The generated PE/CLI artifacts are independently inspected and executed by CoreCLR on Linux and Windows. This remains valuable historical bootstrap and differential evidence, but its evidence level is `oracle-proven`, not FerrumWeave implementation ownership.

## Why the oracle integration is isolated

FerrumWeave's normal repository development stays on its supported stable Rust baseline. Compiler codegen backends necessarily integrate with private/nightly rustc compiler APIs, so the historical oracle remains in dedicated characterization lanes instead of forcing the stable workspace onto nightly.

This isolation is intentional:

- normal repository code remains stable-Rust-first;
- compiler-internals coupling is explicit and reproducibly pinned;
- upstream source is checked out by characterization CI rather than vendored;
- upstream drift cannot silently redefine FerrumWeave product evidence;
- the FerrumWeave-owned backend has its own pinned compiler-private lane and causal certifiers;
- changing the oracle pin is a deliberate characterization change, not a product-backend upgrade.

The lifecycle change is recorded in [`ADR 0002`](../architecture/adr/0002-r02-isolated-upstream-codegen-lane.md).

## Oracle policy

Every remaining use of `rustc_codegen_clr` must answer a concrete characterization question: what observable behavior, marker shape, compiler integration fact, or differential expectation are we learning that FerrumWeave must reproduce independently?

A successful oracle run is never sufficient PASS evidence for a FerrumWeave product capability. Product claims require `FerrumWeave-backend-proven` evidence and, when Rust-source semantics are claimed, source-causal certification through FerrumWeave MIR lowering and CIL/metadata emission.

The oracle may be refreshed when differential characterization genuinely needs a newer upstream behavior. Product milestones do not depend on refreshing this pin merely to gain implementation capability.

## No-copy / provenance rule

FerrumWeave may study public interfaces, architecture, integration patterns, and observable behavior from upstream projects. FerrumWeave does not copy `rustc_codegen_clr` lowering or IR implementation as a shortcut to product ownership.

If a small piece of generic compiler-interface glue is ever adapted, it must have explicit provenance and license review. MIR lowering, CTS/projection integration, CIL emission, and product causality remain FerrumWeave-owned.
