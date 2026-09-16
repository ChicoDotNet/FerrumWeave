# R10 agent handoff — installed .NET template-language DX

Status: **active engineering handoff** for R10 / `0.1-alpha`.

This document exists so a fresh contributor or autonomous agent can continue R10 from repository evidence alone. It is operational guidance, not a release claim. Executable contracts and current CI results outrank this document if they diverge.

## Read this before changing R10

Read these repository paths in this order:

1. `docs/roadmap/r10-agent-handoff.md` — this execution boundary.
2. `docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md` — accepted public template-language decision.
3. `docs/roadmap/template-release-plan.md` — release sequence and the meaning of supported.
4. `docs/getting-started.md` — target installed-package experience and packaging boundary.
5. `docs/roadmap/README.md` — milestone status and transversal Definition of Done.
6. `docs/quality/r09-python-to-rust-inventory.md` — remaining Python/Rust verification authority and migration order.
7. `docs/quality/r09-rust-authority-cutover.md` — evidence required before a Python witness can be retired.
8. `tests/r08/contracts.toml` — historical SDK evidence that R10 must preserve or deliberately supersede, not silently rewrite.
9. `tests/r10/contracts.toml` — active R10 contract ledger.

Before every mutation, fresh-read `dev`, the working branch, and any open PR that may overlap. Do not assume a remembered SHA is still current.

## Non-negotiable product model

FerrumWeave does **not** invent a Rust project type for .NET.

Rust is a language choice for standard .NET template families:

```console
dotnet new <template> -lang Rust
```

The old prototype command:

```console
dotnet new rust
```

is historical R08 evidence, not the R10 public target. The current path `sdk/templates/rust/`, its `shortName = rust`, and `tests/r08_dotnet_new.rs` belong to that historical prototype. Do not deepen that public model merely because it exists in the tree.

Do not rewrite or delete R08 evidence just to make history look like R10. Add R10 evidence first; retire or reclassify historical behavior only through an explicit contract and certified migration.

## First active contract

The primary R10 contract is:

`FW-R10-DX-003 — installed_alpha_registers_rust_as_a_dotnet_console_language`

### GIVEN

- a clean machine/environment with the supported .NET SDK;
- no FerrumWeave source checkout required by the generated project;
- the FerrumWeave alpha package installed through the normal `dotnet new install` mechanism.

### WHEN

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet build
dotnet run
```

### THEN

- template resolution succeeds as the standard `console` family with language `Rust`;
- the generated project contains an SDK-style `.rsproj` and `src/main.rs`;
- restore/build/run work from outside the FerrumWeave repository;
- compilation reaches the real product path `.rsproj -> FerrumWeave.Sdk -> rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`;
- the observed output is causally owned by Rust source;
- no generated C#, source parser, legacy emitter, native sidecar, or upstream oracle substitutes for the product backend.

### Required falsifiers

Installing FerrumWeave must not break or silently replace the normal template variants. At minimum, preserve executable evidence for:

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
```

The Rust registration must coexist with existing .NET languages.

## Smallest next implementation slice

Do not begin by implementing every `0.1-alpha` template family.

Drive `FW-R10-DX-003` RED -> GREEN first:

1. add an R10 integration test that installs the package/template payload into an isolated environment;
2. prove the current prototype fails the canonical `dotnet new console -lang Rust` contract for the expected reason;
3. adapt package/template metadata so the standard `console` family resolves Rust without requiring the source repository;
4. build and run the generated project through the real FerrumWeave backend;
5. mutate Rust-only source and prove managed artifact/observable causality;
6. execute the C#/VB/F# falsifiers;
7. certify on Linux and Windows;
8. only then broaden to the next `0.1-alpha` template family.

Use `COPY`, `TRANSFORM`, or `ADAPT` explicitly when migrating an existing test/harness. Preserve the observable contract, not the incidental orchestration language.

## Verification authority boundary

Rust owns product behavior certification. .NET owns external consumer/framework certification where a supported .NET testing surface is the thing being proved. Python is legacy harness/oracle infrastructure and must leave the critical path only incrementally.

For every Python verifier listed in `docs/quality/r09-python-to-rust-inventory.md`:

- `PYTHON_ONLY` — preserve it;
- `RUST_PARTIAL` — preserve it until observable parity exists;
- `DUAL` — both witnesses exist, but Python is not yet disposable solely because Rust has a related test;
- `RUST_AUTHORITATIVE` — Rust owns the product gate and the corresponding Python witness may be retired;
- `OBSOLETE` — requires an explicit rationale.

Do not delete a Python verifier before equivalent material observables are protected, the CI-only cutover is green, and the post-delete tree is independently green.

R02/R03 and other upstream-backed evidence remain historical/differential oracle evidence. `rustc_codegen_clr` never occupies the FerrumWeave product-backend slot.

## Certification commands and CI

The authoritative CI definitions are the workflow files themselves. The normal local Rust gates mirror them:

```console
cargo fmt --all
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --lib --locked
cargo test --workspace --tests --locked
```

Backend integration also requires the pinned compiler components declared by CI (`nightly-2025-10-14`, `rustc-dev`, `rust-src`, `llvm-tools-preview`) and the dedicated FerrumWeave codegen-backend convergence workflow on both Ubuntu and Windows.

For documentation-graph changes, the certified commands are:

```console
cargo test --locked -p ferrumweave-docgraph
cargo run --locked -p ferrumweave-docgraph -- render
git diff --check
git diff --exit-code
```

Coverage policy is not optional: line coverage remains at least 80%, functional coverage at least 96%, target 100%. Never remove tests or known contracts to protect a percentage.

## Branch and promotion discipline

- short-lived work branches target `dev`;
- `dev` preserves detailed causal/TDD/fix history;
- `main` records milestone/release promotions;
- promotions from `dev` to `main` use a clean selective/squash promotion rather than importing hundreds of detailed-history commits;
- do not force-update shared branches unless a maintainer explicitly authorizes repository surgery;
- before merging or promoting, fresh-read the destination branch and certify the exact head SHA being merged.

## Stop conditions

Do not claim R10 progress as certified when any of these are true:

- the test installs from a repository path instead of the distributable package boundary;
- `dotnet new rust` is used as proof of the R10 public contract;
- the generated project needs the FerrumWeave source checkout to build;
- output can be explained by generated C#, a legacy emitter, native FFI, or an upstream oracle;
- only one operating system passed for a cross-platform claim;
- existing C#/VB/F# template behavior was not falsified;
- a Python witness was removed before authority cutover evidence existed;
- documentation says a target is released before executable evidence exists.

## Definition of the next safe handoff

The next agent should be able to answer all of these from the repository alone:

1. Which R10 contract is currently RED/GREEN?
2. Which exact SHA was certified?
3. Which Windows and Linux CI runs certify it?
4. What observable changed because Rust source changed?
5. Which legacy evidence remains intentionally present and why?
6. What is the next smallest contract, without broadening the milestone prematurely?

If those answers are not recorded, the iteration is not yet ready to hand off.
