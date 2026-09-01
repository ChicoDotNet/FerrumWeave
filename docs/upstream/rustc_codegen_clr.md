# Upstream: `rustc_codegen_clr`

FerrumWeave treats [`FractalFir/rustc_codegen_clr`](https://github.com/FractalFir/rustc_codegen_clr) as major prior art and a likely upstream dependency/contribution target for real Rust → CLR code generation.

## Snapshot inspected for R01

- Repository: `FractalFir/rustc_codegen_clr`
- Branch: `main`
- Commit: `a9aa553b136fce00eceb41fba30758830500a63f`
- Commit message: `Updated rustc version`
- License: MIT OR Apache-2.0

The upstream workspace contains `cilly`, described as a tool for creating and optimizing .NET assemblies. At the inspected revision, `cilly` is an internal path dependency of `rustc_codegen_clr` and enables nightly Rust language features in its crate root.

## FerrumWeave policy

FerrumWeave does not vendor or copy `rustc_codegen_clr` in R01.

R01 is intentionally an output-format probe and uses a tiny local ECMA-335 emitter so the repository can remain on stable Rust while proving the managed artifact boundary.

Before R02 begins real Rust → CLR code generation, the project must refresh this upstream snapshot and evaluate:

1. current supported `rustc` revision/toolchain;
2. current `cilly` API and packaging status;
3. Windows and Linux behavior;
4. the smallest integration surface FerrumWeave actually needs;
5. changes that should be proposed upstream instead of maintained locally.

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

## Provenance rule

FerrumWeave may learn from the public architecture and behavior of upstream projects, but copied/adapted source must never enter the repository without explicit provenance and license review.
