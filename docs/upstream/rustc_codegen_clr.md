# Upstream: `rustc_codegen_clr`

FerrumWeave treats [`FractalFir/rustc_codegen_clr`](https://github.com/FractalFir/rustc_codegen_clr) as important prior art and a historical **characterization oracle / differential reference** for Rust → CLR behavior. It is **not a FerrumWeave runtime, SDK, product dependency, or active CI dependency**, and it no longer occupies the product-backend slot.

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

## Historical R02/R03 oracle snapshot

- Repository: `FractalFir/rustc_codegen_clr`
- Branch inspected: `main`
- Commit: `a9aa553b136fce00eceb41fba30758830500a63f`
- Commit message: `Updated rustc version`
- Rust toolchain: `nightly-2025-10-14`
- Required components: `rust-src`, `rustc-dev`, `llvm-tools-preview`
- License: MIT OR Apache-2.0

At this revision the upstream workspace contained both the `rustc_codegen_clr` compiler backend and `cilly`, the assembly/linker infrastructure used to produce managed output.

Historically, R02, R03, and the managed-static-call causality oracle checked out this exact upstream commit in CI, built the backend and linker from source, and invoked real `rustc` through `-Z codegen-backend`. The generated PE/CLI artifacts were independently inspected and executed by CoreCLR on Linux and Windows. That evidence remains valuable bootstrap history at the `oracle-proven` level, but it never established FerrumWeave implementation ownership.

The executable upstream oracle lanes were **retired from active CI after R09 established FerrumWeave-owned causal backend evidence**. Their fixtures and contract ledgers remain in the repository as an auditable historical record; reproducing the old upstream experiment is no longer a product or release gate.

## Why the oracle was isolated

FerrumWeave's normal repository development stays on its supported stable Rust baseline. Compiler codegen backends necessarily integrate with private/nightly rustc compiler APIs, so the historical oracle was kept in dedicated characterization lanes rather than forcing the stable workspace onto nightly.

That isolation taught several durable lessons:

- normal repository code remains stable-Rust-first;
- compiler-internals coupling must be explicit and reproducibly pinned;
- upstream drift must not silently redefine FerrumWeave product evidence;
- the FerrumWeave-owned backend needs its own pinned compiler-private lane and causal certifiers;
- changing an external oracle is characterization work, not a product-backend upgrade.

The lifecycle change is recorded in [`ADR 0002`](../architecture/adr/0002-r02-isolated-upstream-codegen-lane.md).

## Retired-oracle policy

`rustc_codegen_clr` may still be consulted as public prior art when a concrete research question justifies it, but FerrumWeave no longer carries an executable CI contract that builds or runs the upstream backend.

A historical oracle result is never PASS evidence for a FerrumWeave product capability. Product claims require `FerrumWeave-backend-proven` evidence and, when Rust-source semantics are claimed, source-causal certification through FerrumWeave MIR lowering and CIL/metadata emission.

If a future investigation genuinely needs differential comparison with upstream, it should be introduced as a scoped research task with an explicit question rather than restoring a permanent product gate.

## No-copy / provenance rule

FerrumWeave may study public interfaces, architecture, integration patterns, and observable behavior from upstream projects. FerrumWeave does not copy `rustc_codegen_clr` lowering or IR implementation as a shortcut to product ownership.

If a small piece of generic compiler-interface glue is ever adapted, it must have explicit provenance and license review. MIR lowering, CTS/projection integration, CIL emission, and product causality remain FerrumWeave-owned.
