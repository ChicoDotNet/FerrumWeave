# Upstream: `rustc_codegen_clr`

FerrumWeave treats [`FractalFir/rustc_codegen_clr`](https://github.com/FractalFir/rustc_codegen_clr) as major prior art and an upstream dependency/contribution target for real Rust → CLR code generation.

## R02 pinned integration snapshot

- Repository: `FractalFir/rustc_codegen_clr`
- Branch inspected: `main`
- Commit: `a9aa553b136fce00eceb41fba30758830500a63f`
- Commit message: `Updated rustc version`
- Rust toolchain: `nightly-2025-10-14`
- Required components: `rust-src`, `rustc-dev`, `llvm-tools-preview`
- License: MIT OR Apache-2.0

At this revision the upstream workspace contains both the `rustc_codegen_clr` compiler backend and `cilly`, the assembly/linker infrastructure used to produce managed output.

R02 checks out this exact upstream commit in CI, builds the backend and linker from source, and invokes real `rustc` with the backend through `-Z codegen-backend`. The generated PE/CLI artifact is then independently inspected and executed by CoreCLR on Linux and Windows.

## Why the integration is isolated

FerrumWeave's normal repository development stays on its supported stable Rust baseline. `rustc_codegen_clr` necessarily integrates with private/nightly rustc compiler APIs, so R02 keeps that dependency in a dedicated compatibility lane instead of forcing the whole repository onto nightly.

This isolation is intentional:

- application/repository code remains stable-Rust-first;
- compiler-internals coupling is explicit and reproducibly pinned;
- upstream source is checked out by CI rather than vendored;
- changes in rustc internals cannot silently redefine the certified R02 contract;
- upgrading the pin is a deliberate compatibility change that must pass the same Linux/Windows contract suite.

The architectural decision is recorded in [`ADR 0002`](../architecture/adr/0002-r02-isolated-upstream-codegen-lane.md).

## Upstream-first policy

FerrumWeave does not copy or permanently fork `rustc_codegen_clr` merely to make local progress.

Preferred lifecycle:

```text
consume
  ↓
patch locally only if necessary
  ↓
upstream contribution
  ↓
remove local divergence
```

R02 required no FerrumWeave-maintained source patch to the pinned upstream backend. The integration work lives in FerrumWeave's fixtures, verifier, CI wiring, contracts, and compatibility documentation.

Future milestones must refresh the upstream snapshot when they require new semantics rather than assuming the R02 pin is permanent.

## Provenance rule

FerrumWeave may learn from and execute the public architecture and behavior of upstream projects, but copied/adapted source must never enter the repository without explicit provenance and license review.
