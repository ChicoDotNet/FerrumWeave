# R10 agent handoff — installed .NET template-language DX

Status: **active engineering handoff** for R10 / `0.1-alpha`.

This document exists so a fresh contributor or autonomous agent can continue R10 from repository evidence alone. It is operational guidance, not a release claim. Executable contracts and current CI results outrank this document if they diverge.

## Read this before changing R10

Read these repository paths in this order:

1. `docs/roadmap/r10-agent-handoff.md` — this execution boundary.
2. `docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md` — accepted public template-language decision.
3. `docs/architecture/adr/0005-r10-rust-to-clr-identity-projection.md` — accepted Rust-source / CLR-identity projection decision for entry points, namespaces, types, and members.
4. `docs/roadmap/template-release-plan.md` — release sequence and the meaning of supported.
5. `docs/getting-started.md` — target installed-package experience and packaging boundary.
6. `docs/roadmap/README.md` — milestone status and transversal Definition of Done.
7. `docs/quality/r09-python-to-rust-inventory.md` — remaining Python/Rust verification authority and migration order.
8. `docs/quality/r09-rust-authority-cutover.md` — evidence required before a Python witness can be retired.
9. `tests/r08/contracts.toml` — historical SDK contracts that R10 must preserve, explicitly supersede, or retire through certified migration.
10. `tests/r10/contracts.toml` — active R10 contract ledger.

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

is historical R08 behavior, not the R10 public target. Historical paths such as `sdk/templates/rust/` may remain in an integration tree until an explicit migration retires them.

Rust source must remain valid Rust. Do not add C#-style source syntax such as `namespace`, do not require the user to rename Rust `main` to CLR `Main`, and do not satisfy CLR ergonomics by translating the project to generated C#.

FerrumWeave owns the CLR projection boundary. For executable projects, ordinary Rust `fn main` must project through the FerrumWeave backend to a static managed `<RootNamespace>.Program.Main` adapter (or an explicitly superseding accepted identity) whose `MethodDef` is the CLI entry point. For broader application surfaces, semantic Rust module paths provide namespace organization and CLR-exportable types/members receive stable namespace-qualified .NET identities. See ADR 0005 and contracts `FW-R10-CLR-ENTRY-001` and `FW-R10-CLR-SURFACE-001`.

The portable managed artifact remains the `.dll`; normal SDK/apphost output may additionally produce a platform host such as `.exe` on Windows. Do not confuse the apphost with the managed assembly contract.

Do not infer evidence authority from a filename alone. During R10, an older R08/R09-named test or fixture may itself be under `COPY`, `TRANSFORM`, `ADAPT`, retirement, or reclassification. Read the active branch diff, contract ledgers, and authority-cutover evidence before restoring, deleting, or classifying it.

Do not rewrite or delete historical evidence merely to make history look like R10. Add equivalent or stronger R10 evidence first; retire or reclassify historical behavior only through an explicit contract and certified migration.

## Primary installed-package contract

The primary R10 distribution/console contract is:

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
- ordinary Rust `fn main` reaches the managed executable entry point through the ADR 0005 projection instead of promoting an unrelated exported method;
- no generated C#, source parser, legacy emitter, native sidecar, or upstream oracle substitutes for the product backend.

### Required falsifiers

Installing FerrumWeave must not break or silently replace the normal template variants. At minimum, preserve executable evidence for:

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
```

The Rust registration must coexist with existing .NET languages.

## Parallel R10 lane model

R10 may advance in parallel where the dependency graph allows it. A fresh agent must not serialize unrelated work merely because `FW-R10-DX-003` is still RED.

The expected lanes are:

1. **Distribution / console certification** — drives `FW-R10-CLR-ENTRY-001` and `FW-R10-DX-003` through managed entry-point projection, package installation, external build/run causality, C#/VB/F# falsifiers, and Windows/Linux certification.
2. **0.1-alpha template families** — may independently advance `classlib`, `xunit`, `nunit`, `mstest`, `web`, and `webapi` through their own T0/T1/T2 evidence. Shared CLR surface work should reuse `FW-R10-CLR-SURFACE-001` rather than inventing family-local namespace/export conventions.
3. **Verification-authority cleanup** — may remove Python or differential/oracle witnesses only after the documented authority cutover for their observable contracts.

Before editing a family, inspect overlapping PRs. Do not overwrite another active family lane just to make one branch look self-contained.

Parallel progress does **not** relax the definition of supported. A family may have a GREEN scaffold or repository-local test while still remaining unsupported for release.

### Template evidence levels

- **T0 — scaffold:** `dotnet new <template> -lang Rust` resolves the standard template family, generates the intended `.rsproj`/Rust source shape, and coexists with the built-in languages that family already supports.
- **T1 — runnable/framework-meaningful:** the generated project completes the meaningful `dotnet run` or `dotnet test` workflow through the real FerrumWeave backend and demonstrates behavior that distinguishes the framework family from a generic console project.
- **T2 — idiomatic:** the Rust-facing project shape and framework use are credible for real developers rather than merely mechanically runnable.

Prerelease support requires at least T1. Repository-local template installation may be useful T0/T1 engineering evidence, but installed/distributable package evidence remains required for release claims.

## Smallest next implementation slices

### Distribution / console lane

Drive the installed console boundary RED -> GREEN without waiting for every other template family:

1. preserve the already-GREEN `FW-R10-DIST-001` package/install/build evidence;
2. reproduce the current `FW-R10-DX-003` `MissingMethodException` from the installed package on Linux and Windows;
3. RED `FW-R10-CLR-ENTRY-001` with metadata/reflection assertions for ordinary Rust `fn main` -> static `<RootNamespace>.Program.Main` plus the correct CLI entry-point `MethodDef`;
4. implement the smallest backend/CIL projection that satisfies that entry-point contract without generated C# or an arbitrary-export shortcut;
5. make direct `dotnet run` expose a Rust-source-causal observable;
6. mutate Rust-only source and prove both managed artifact and direct execution observable change;
7. execute the C#/VB/F# template coexistence falsifiers;
8. certify the exact candidate on Linux and Windows.

### CLR surface lane

Treat `FW-R10-CLR-SURFACE-001` as a reusable interoperability capability, not a template-specific naming hack:

1. establish deterministic `AssemblyName` / `RootNamespace` inputs at the `.rsproj` boundary;
2. RED one nested Rust module -> CLR namespace projection;
3. RED multiple CLR-exportable Rust types under that namespace;
4. project supported public members with deterministic .NET-facing names while retaining Rust source naming and rustc semantics;
5. consume the resulting identities from an independent .NET project (Visual Basic, C#, or F# as appropriate to the evidence slice);
6. preserve the existing CTS/ownership contracts rather than inferring class/value/interface semantics from names alone;
7. add explicit naming override semantics later only when a real compatibility contract requires them.

### Template-family lane

A parallel family lane should take one family at a time:

1. declare the family contract and its current T0/T1/T2 state in `tests/r10/contracts.toml`;
2. RED the standard `dotnet new <template> -lang Rust` behavior;
3. make T0 scaffold evidence GREEN without changing unrelated families;
4. RED the framework-specific runtime/test behavior;
5. implement only the managed/compiler capabilities needed for that observable;
6. prove source causality and native-language coexistence where applicable;
7. certify Windows/Linux before promoting a cross-platform framework claim;
8. keep `implemented = false` until the contract's required support level is actually proven.

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

R02/R03 and other upstream-backed evidence are historical/differential evidence unless an explicit authority-cutover change retires them. `rustc_codegen_clr` never occupies the FerrumWeave product-backend slot.

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
- topic/increment PRs merge normally into `dev`, preserving detailed causal/TDD/fix history there;
- `main` records milestone/release promotions only;
- promotion is a **direct `dev -> main` pull request**; do not create a synthetic promotion branch from `main` merely to compress history;
- merge `dev -> main` with **squash**, yielding one clean promotion commit on `main` while detailed history remains on `dev`;
- because squash does not preserve ancestry, immediately reconcile the resulting `main` commit back into `dev` with a content-preserving merge: the reconciliation tree must remain identical to `dev`, and its purpose is only to make the promoted `main` commit an ancestor of `dev`;
- before opening or merging the next `dev -> main` PR, verify the merge base is the current promoted `main`; unexpectedly reappearing already-promoted files or a huge stale diff means ancestry reconciliation is missing;
- do not force-update shared branches unless a maintainer explicitly authorizes repository surgery;
- before merging or promoting, fresh-read the destination branch and certify the exact head SHA being merged.

This preserves both desired histories simultaneously:

```text
topic branches --normal merge--> dev   # detailed engineering history
                                |
                                +--squash--> main   # clean promotion history
                                      |
                                      +--content-preserving ancestry merge--> dev
```

## Stop conditions

Do not claim R10 progress as certified when any of these are true:

- the release claim relies on repository-local installation instead of the distributable package boundary;
- `dotnet new rust` is used as proof of the R10 public contract;
- the generated project needs the FerrumWeave source checkout to build;
- `fn main` is replaced in source by a non-Rust `Main`/`namespace` syntax solely to satisfy CLR conventions;
- direct execution is made green by promoting an unrelated arbitrary export instead of projecting the Rust entry point through the accepted CLR entry contract;
- output can be explained by generated C#, a legacy emitter, native FFI, or an upstream oracle;
- a family-local T0 scaffold is presented as supported framework behavior;
- only one operating system passed for a cross-platform claim;
- the built-in language variants applicable to that template family were not falsified;
- a Python witness was removed before authority cutover evidence existed;
- documentation says a target is released before executable evidence exists.

## Definition of the next safe handoff

The next agent should be able to answer all of these from the repository alone:

1. Which R10 contract is currently RED/GREEN, and at what T0/T1/T2 level?
2. Which exact SHA was certified?
3. Which Windows and Linux CI runs certify it?
4. What observable changed because Rust source changed?
5. Which Rust source identity projected to which CLR assembly/namespace/type/member identity for the active slice?
6. Which legacy evidence remains intentionally present, migrated, or retired and why?
7. Which parallel lane owns the next smallest contract?

If those answers are not recorded, the iteration is not yet ready to hand off.
