<!--
doc-id: architecture.repository-layout
locale: en
-->

<!-- ferrumweave-nav:start -->
[← Documentation](../README.md) · [Project README](../../README.md)
<!-- ferrumweave-nav:end -->

# Repository layout

**English** · [Deutsch](repository-layout.de.md) · [Español](repository-layout.es.md) · [Français](repository-layout.fr.md) · [Italiano](repository-layout.it.md) · [Português (Brasil)](repository-layout.pt-BR.md) · [Русский](repository-layout.ru.md) · [简体中文](repository-layout.zh-Hans.md) · [日本語](repository-layout.ja.md)

FerrumWeave is intentionally a **capability-first monorepo**. The repository should feel familiar to contributors coming from either Rust or .NET without dividing the product into a permanent “Rust side” and “.NET side”.

This document is the target structure. Directories are created only when an increment needs them; the project does not keep empty folders alive with `.gitkeep` files.

## Target structure

```text
FerrumWeave/
├── .cargo/
├── .github/
│   ├── workflows/
│   ├── ISSUE_TEMPLATE/
│   └── PULL_REQUEST_TEMPLATE.md
├── assets/
│   └── brand/
│       ├── hero/
│       ├── logos/
│       ├── mascot/
│       ├── icons/
│       └── merch/
├── compiler/
│   ├── codegen-backend/
│   ├── cil/
│   └── driver/
├── projection/
│   ├── metadata/
│   ├── types/
│   └── support/
├── sdk/
│   ├── FerrumWeave.Sdk/
│   ├── tasks/
│   └── templates/rust/
├── tooling/
│   ├── analyzer/
│   ├── debugger/
│   ├── vscode/
│   └── visualstudio/
├── tools/
│   ├── docgraph/
│   └── quality/
├── tests/
│   ├── ui/
│   ├── codegen/
│   ├── conformance/
│   ├── interop/
│   │   ├── rust-csharp/
│   │   ├── rust-vb/
│   │   └── rust-fsharp/
│   ├── sdk/
│   ├── e2e/
│   └── fixtures/
├── samples/
│   ├── hello-world/
│   ├── consume-dotnet/
│   ├── consumed-by-dotnet/
│   └── mixed-solution/
├── docs/
│   ├── README.md
│   ├── i18n.md
│   ├── architecture/
│   │   └── adr/
│   ├── compatibility/
│   ├── design/
│   ├── roadmap/
│   ├── site/
│   └── upstream/
├── eng/
│   ├── ci/
│   ├── packaging/
│   └── scripts/
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── FerrumWeave.slnx
├── global.json
├── Directory.Build.props
├── Directory.Build.targets
├── README.md
├── CONTRIBUTING.md
├── GOVERNANCE.md
├── SECURITY.md
├── CODE_OF_CONDUCT.md
├── NOTICE
├── LICENSE-MIT
└── LICENSE-APACHE
```

## Rules

### Organize by capability, not implementation language

Top-level product folders describe responsibilities: `compiler`, `projection`, `sdk`, and `tooling`. Avoid a root such as `src/rust` next to `src/dotnet`; that would create an architectural boundary FerrumWeave exists to dissolve.

Within a capability, use the conventions native to the ecosystem implementing it. Rust crates use idiomatic `Cargo.toml` + `src/`; .NET components use SDK-style projects and normal .NET naming.

Repository-maintenance utilities that are not product tooling live under `tools/`. `tools/docgraph`, for example, maintains the documentation graph; it is not part of the compiler or the developer-facing FerrumWeave toolchain.

### Keep the rustc backend boundary owned by FerrumWeave

`compiler/codegen-backend/` is the product boundary loaded by `rustc`. It owns the FerrumWeave implementation of `CodegenBackend` and routes rustc MIR into FerrumWeave lowering, CIL emission, metadata, and projection infrastructure.

The stable workspace remains independent from compiler-private APIs. Nightly/rustc-dev requirements belong to the isolated backend lane rather than leaking into ordinary crates.

### Keep `projection/` independent

CLR metadata projection is a shared contract, not an implementation detail of code generation. The compiler, code analysis, project references, NuGet integration, IntelliSense, and eventually debugging may all depend on the same model of .NET-visible symbols.

Dependency direction should remain explicit. Tooling may consume projection contracts; projection should not depend on IDE-specific concerns.

### Tests are executable interoperability contracts

The test tree intentionally borrows vocabulary from both ecosystems. `tests/ui` and `tests/codegen` should feel familiar to Rust compiler contributors; `tests/interop`, `tests/sdk`, `tests/e2e`, and `tests/fixtures` should be unsurprising to .NET contributors.

Each interop fixture should prove behavior in both directions whenever practical. For example, `tests/interop/rust-csharp/` should eventually contain contracts where Rust consumes a C# surface and where C# consumes a CLR-facing Rust surface.

### Samples sell the architecture with evidence

The canonical samples progress vertically:

1. `hello-world` — Rust → CIL → CLR.
2. `consume-dotnet` — Rust consumes .NET APIs or a .NET project.
3. `consumed-by-dotnet` — another .NET language consumes a Rust-produced assembly.
4. `mixed-solution` — `.rsproj`, `.csproj`, `.fsproj`, and `.vbproj` coexist in one solution.

A sample is only added when the behavior it demonstrates is executable.

### Keep product upstream divergence visible and temporary

FerrumWeave should integrate before reinventing when a dependency is genuinely part of the product path. Local divergence from `rustc`, `rust-analyzer`, the .NET SDK, or another product upstream should be recorded under `docs/upstream/` with the upstream revision, local requirement, issue/PR link, and exit condition.

The preferred lifecycle for real product upstreams is:

```text
consume upstream
      ↓
patch only when required
      ↓
submit upstream
      ↓
remove local divergence
```

`rustc_codegen_clr` is different: it is a pinned **characterization oracle / differential oracle**, not a runtime, SDK, or product backend dependency. Its lifecycle is therefore evidence-oriented:

```text
pin oracle revision
      ↓
characterize observable behavior
      ↓
reproduce the required behavior through FerrumWeave
      ↓
retain only when differential value remains, otherwise remove
```

Oracle provenance and pins still belong under `docs/upstream/`, but FerrumWeave does not patch or consume that backend as the product implementation.

### Do not pre-create the future

The target tree is a map, not a mandate to create empty directories. New folders arrive with the first real contract, implementation, test, or document that belongs there.

This keeps repository history honest: structure follows executable evidence rather than speculative architecture.

## Two familiar entry points

As implementation grows, a contributor should eventually be able to approach the repository naturally from either ecosystem:

```bash
cargo test --workspace
```

or:

```bash
dotnet build FerrumWeave.slnx
```

Those commands should converge on the same product and the same interoperability contracts.

<!-- ferrumweave-backlinks:start -->
## What links here

- [FerrumWeave](../../README.md)
- [FerrumWeave documentation](../README.md)
<!-- ferrumweave-backlinks:end -->
