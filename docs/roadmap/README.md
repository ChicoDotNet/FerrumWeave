# FerrumWeave capability roadmap

FerrumWeave is developed through **capability milestones**, not calendar promises.

A milestone is complete only when it changes a technical fact about the project and that fact is reproducible from a clean checkout. The roadmap intentionally becomes less prescriptive as it moves farther from executable evidence: R01-R03 are concrete enough to implement now; later milestones define outcomes and Definition of Done without pretending that architectural discoveries have already been made.

> **Executable evidence outranks diagrams, estimates, and aspirations.**

The prerelease project-template sequence from `0.1-alpha` through `1.0` is defined in [Template release plan](template-release-plan.md). ADR 0004 records the decision that Rust participates as a .NET **language choice** through `dotnet new <template> -lang Rust`, rather than as a standalone project type named `rust`.

## Current status

| Milestone | Status | New technical truth |
| --- | --- | --- |
| R00 — Repository baseline | **Done** | FerrumWeave has a reproducible Rust bootstrap, CI, quality gates, community foundations, branding, Pages, and dependency maintenance. |
| R01 — CLR artifact probe | **Done** | FerrumWeave can produce and execute a valid managed .NET assembly. |
| R02 — Rust → CLR vertical slice | **Done — historical oracle characterization** | Bootstrap evidence proved that real Rust can pass through `rustc` and execute as managed code on CoreCLR; that historical evidence is not FerrumWeave-backend product evidence. |
| R03 — Core Rust semantics | **Done — historical oracle characterization** | The declared safe-Rust semantic and negative families remain useful differential characterization; product claims require FerrumWeave-owned backend evidence. |
| R04 — CLR / CTS foundation | **Done — re-audited** | CTS mapping/reflection evidence remains valid at its declared artifact/type-system scope; it does not claim managed API consumption from Rust source. |
| R05 — Rust consumes .NET | **Done — FerrumWeave backend re-certified** | Required managed-call families are source-causally certified through the FerrumWeave `rustc` CodegenBackend on Linux and Windows; upstream remains oracle-only. |
| R06 — .NET consumes Rust | **Done — FerrumWeave backend re-certified** | C#/VB consumers, reflection and no-native-ABI contracts consume Rust-source-causal assemblies emitted through the FerrumWeave `rustc` CodegenBackend on Linux and Windows. |
| R07 — Semantic interoperability | **Done — FerrumWeave backend re-certified** | All declared semantic-interoperability contracts are source-causally certified through the FerrumWeave `rustc` CodegenBackend on Linux and Windows; unsupported crossings fail diagnostically before CLR emission. |
| R08 — `.rsproj` and FerrumWeave SDK | **Done — FerrumWeave backend re-certified** | Normal `.rsproj` build now follows `FerrumWeave.Sdk -> rustc -> FerrumWeave CodegenBackend`; Rust-only mutation changes the managed artifact and CoreCLR observable on Linux and Windows without `ferrumweave_emit` in the product path. |
| R09 — Mixed `.slnx` proof | **Done — promoted and re-certified on `main`** | The canonical mixed-language `.slnx` is source-causal through `.rsproj -> rustc -> FerrumWeave`; the final R09 promotion is independently GREEN at `main@eab90a620b0598dbbce8ea6bf51f7d5b725f5818`. |
| R10 — Developer experience / 0.1 alpha | **Active — resumed after R09 certification** | R10 now owns the installed-package DX contract: Rust participates as a .NET language through `dotnet new <template> -lang Rust`, with uncertified commands kept explicitly as release targets rather than current capability claims. |

### Evidence lifecycle after the convergence audit

R02/R03 historical evidence is `oracle-proven`. Those milestones remain valuable bootstrap history and differential characterization, but their upstream-backed runs do not establish implementation ownership by FerrumWeave.

`rustc_codegen_clr` is not a FerrumWeave runtime, SDK, or product dependency; its pin is retained only in optional oracle/test/dev characterization lanes so historical behavior can be replayed and compared without occupying the product-backend slot.

For current product capability, product capability claims require `FerrumWeave-backend-proven` evidence and, where source semantics are claimed, source-causal certification from Rust source through the FerrumWeave `rustc` CodegenBackend. Oracle evidence may support characterization but cannot substitute for that causal path.

---

# Definition of Done — all milestones

Every milestone inherits this Definition of Done. A milestone-specific DoD may add requirements, but may not silently weaken these.

## Capability

- The milestone introduces an end-to-end capability that did not previously exist.
- The component being proven is real. Mocks, test doubles, generated C#, native wrappers, subprocess tricks, or FFI may not replace the capability that the milestone exists to demonstrate.
- The capability can be reproduced from a clean checkout using documented commands.
- Known limitations are documented as limitations rather than hidden behind optimistic product language.

## Contracts and TDD

- Known functional behavior is declared as explicit contracts before or while implementation begins.
- Each contract receives a stable identifier such as `FW-R02-CLR-001`.
- New known contracts discovered during implementation enter the functional-coverage denominator; they are not deferred merely to protect the percentage.
- Work follows a RED → GREEN → REFACTOR → CERTIFY loop whenever the behavior can be driven that way.
- CI acts as sensor, memory, and final certifier of the milestone.

## Quality gates

- `cargo fmt --all --check` is green for Rust changes.
- Clippy is green with warnings denied for supported Rust targets.
- Unit and functional tests are green.
- Windows and Linux are mandatory whenever the milestone claims CLR execution, code generation, interoperability, or developer workflow portability.
- Code coverage is **at least 80%**.
- The preferred code-coverage operating band is **80%-96%**. Natural coverage above 96% is accepted and must never be reduced artificially.
- Functional coverage is **at least 96%**, with **100% as the target**.
- Quality policy remains defined in [`docs/quality/coverage-policy.md`](../quality/coverage-policy.md).

## Architecture and provenance

- Difficult-to-reverse semantic or architectural decisions are recorded as ADRs.
- Upstream code, patches, forks, and borrowed designs have explicit provenance.
- When FerrumWeave diverges from an upstream project, the reason and intended convergence/upstreaming strategy are documented.
- Compatibility matrices and architectural documentation are updated whenever the milestone changes their truth.

## Toolchain and reproducibility

- Tooling uses the newest appropriate stable or LTS line supported by the project and is pinned where reproducibility requires it.
- Lockfiles are committed where the ecosystem provides them.
- Dependency automation may propose upgrades, but upgrades must pass the same gates as human changes.

## Honesty

- The README, website, examples, and roadmap may describe future intent, but implemented capability and intended capability must remain clearly distinguishable.
- A milestone is not Done because a branch contains code. It is Done when the advertised capability is certified.

---

# Execution model

FerrumWeave grows vertically.

For each milestone:

1. Declare the smallest useful end-to-end contract.
2. Add the contract to the functional ledger.
3. Make the contract fail for the right reason.
4. Implement the smallest architecture that can make it true without closing future paths unnecessarily.
5. Add conformance cases as newly understood behavior becomes known.
6. Refactor only after the capability is observable.
7. Certify the final state in CI.
8. Update documentation and compatibility truth.

The project should prefer a narrow real capability over a wide simulated architecture.

---

# R00 — Repository baseline

**Status:** Done.

R00 establishes the project as something that can be developed safely before claiming any CLR capability.

It includes:

- dual MIT / Apache-2.0 licensing;
- contribution, governance, security, support, DCO, and Code of Conduct foundations;
- FerrumWeave branding and GitHub Pages;
- a minimal native Rust `Hello FerrumWeave` executable;
- Windows and Linux CI;
- unit tests and functional-contract tracking;
- code and functional coverage gates;
- stable/LTS toolchain policy and dependency automation.

R00 deliberately does **not** claim that FerrumWeave can compile Rust to the CLR.

---

# R01 — CLR artifact probe

**Status: Done.**

## Goal

Prove the output side of the architecture before coupling it to `rustc`:

> FerrumWeave can create a valid managed .NET assembly that the CLR can load and execute on Windows and Linux.

The source of that assembly does **not** yet need to be Rust. R01 isolates PE/CLI, CIL, metadata, entry-point, and runtime assumptions from Rust frontend complexity.

## Initial contracts

### `FW-R01-CLR-001` — Managed assembly

Given FerrumWeave's R01 emitter, when it produces the probe artifact, then the result is a valid managed PE/CLI assembly with CLR metadata and a managed entry point.

### `FW-R01-CLR-002` — CLR execution

Given the generated managed assembly, when it is executed with `dotnet`, then it exits successfully and prints exactly:

```text
Hello FerrumWeave
```

### `FW-R01-CLR-003` — Cross-platform managed artifact

The managed artifact must execute on both Linux and Windows without compiling a platform-specific executable for each target.

### `FW-R01-CLR-004` — No hidden native implementation

The probe must not satisfy the contract by launching a native Rust executable, invoking generated C#, using P/Invoke as the implementation, or hiding the behavior behind another process.

## DoD

R01 is Done when:

- all transversal DoD requirements are met;
- the generated file is independently recognizable as a managed .NET assembly;
- metadata and entry point are tested, not inferred only from successful console output;
- `dotnet <artifact>.dll` succeeds on both Linux and Windows;
- CI proves cross-platform execution;
- the implementation has a deliberate home in the capability-first repository layout rather than a throwaway script;
- an ADR records the initial strategy for CIL/metadata emission and the relationship with prior art such as `rustc_codegen_clr`;
- no Rust-language capability beyond the existing native bootstrap is claimed yet.

---

# R02 — Rust → CLR vertical slice

**Status: Done as historical bootstrap/oracle characterization. Current product evidence is owned by the FerrumWeave backend convergence lane.**

## Goal

Preserve the bootstrap proof that real Rust source is accepted by the real Rust compiler frontend, borrow checking participates, and the resulting program can execute as managed CLR code. The historical R02 run used the pinned upstream backend and is therefore characterization evidence, not proof that FerrumWeave implements the backend.

Conceptually, the product architecture now owns this slot:

```text
Rust source
    ↓
rustc frontend
    ↓
HIR / MIR
    ↓
FerrumWeave rustc CodegenBackend
    ↓
FerrumWeave MIR lowering
    ↓
FerrumWeave CIL + metadata
    ↓
managed .NET assembly
    ↓
CoreCLR
```

## Initial contracts

### `FW-R02-CLR-001` — Real Rust source to managed assembly

A real `.rs` program containing the R02 Hello FerrumWeave behavior compiles through Rust compiler infrastructure and produces a managed .NET assembly.

### `FW-R02-CLR-002` — Borrow checker remains authoritative

A deliberately invalid Rust borrow/lifetime case must be rejected by the Rust compiler path before CLR code generation succeeds.

The CLR's GC must not be used as a reason to weaken Rust source-language guarantees.

### `FW-R02-CLR-003` — Linux execution

The generated managed program runs successfully on Linux.

### `FW-R02-CLR-004` — Windows execution

The generated managed program runs successfully on Windows.

### `FW-R02-CLR-005` — No source-language substitution

FerrumWeave must not translate the source into C# or another .NET language as the implementation of this milestone.

## DoD

The historical bootstrap satisfied these R02 observations. Their current evidence level is `oracle-proven`; they are not promoted into a FerrumWeave product claim without the FerrumWeave backend causal path.

- `rustc` parsing, type checking, borrow checking, and MIR participate in the observed compiler path;
- valid Rust reaches CLR execution on both Linux and Windows;
- invalid Rust demonstrates that Rust semantics gate code generation;
- the output is managed CIL/metadata rather than a native Rust binary hosted beside .NET;
- the upstream-first strategy is preserved as history and differential characterization rather than rewritten as an error.

---

# R03 — Core Rust semantics

**Status: Done as historical bootstrap/oracle characterization. Current product semantic claims require FerrumWeave backend evidence.**

## Goal

Retain a small coherent safe-Rust characterization set as a differential oracle while FerrumWeave owns the product backend implementation.

## Certified conformance families

The historical R03 ledger characterizes:

- primitive integer and boolean values;
- local variables and assignment;
- arithmetic and comparison;
- function definitions, calls, arguments, and returns;
- conditional control flow;
- loops / branching represented through MIR;
- simple tuples and structs;
- field reads and writes;
- shared references;
- mutable references;
- negative safe-Rust borrowing rejection with `E0502` and no emitted executable artifact.

Positive semantics remain useful differentially: the same cumulative Rust fixture can be compiled natively and through the pinned oracle backend, with native behavior serving as an explicit comparison point. The R02/R03 oracle workflows preserve that reproducible history on Linux and Windows, but do not certify FerrumWeave implementation ownership.

## DoD

The historical R03 characterization remains valid at `oracle-proven` evidence level. Product statements about these semantic families require replay through the FerrumWeave `rustc` CodegenBackend and the relevant source-causal/backend certifiers.

Broader Rust language and standard-library coverage remains outside R03 and must not be inferred from this status.

---

# R04 — CLR / CTS foundation

**Status: Done. Prior certification remains valid at the declared CTS/artifact scope after causal re-audit.**

## Goal

Define principled mappings between the Rust type world and the Common Type System.

Early scope should include the fundamental types necessary to make managed APIs meaningful, for example:

- Rust integer widths ↔ CLR integer types;
- `bool` ↔ `System.Boolean`;
- character semantics where compatible;
- managed references;
- `System.String`;
- arrays;
- object identity and the initial nullability boundary.

## DoD

R04 is Done when:

- supported Rust ↔ CTS mappings are documented and centralized rather than scattered casts;
- emitted signatures are validated through CLR metadata/reflection tests;
- round-trip tests cover supported primitive and managed-reference cases;
- ownership/GC boundary rules have an ADR;
- nullability behavior is explicit for every supported public boundary;
- unsupported mappings fail clearly.

All of these conditions are satisfied by the certified R04 milestone. The mapping policy is centralized in Rust, round-trip and boundary behavior is executable, ADR 0003 records the ownership/GC constraints, and FerrumWeave-emitted representative signatures are independently inspected through CLR reflection on Ubuntu and Windows. Broader managed API consumption remains R05 scope and must not be inferred from R04 completion.

---

# R05 — Rust consumes .NET

**Status: Done — re-certified through the FerrumWeave `rustc` CodegenBackend.**

## Goal

Allow Rust to consume APIs that already exist in managed assemblies.

This milestone begins the CLR projection layer: .NET metadata becomes Rust-visible types and members without pretending managed assemblies are native libraries.

## DoD

R05 is Done when Rust can, through managed metadata:

- resolve and call a public static .NET method;
- construct a supported public .NET type;
- call an instance method;
- read/write a supported property;
- use at least one `System.*` API;
- consume a user-defined C# assembly compiled independently from FerrumWeave tests;
- perform all of the above on Linux and Windows;
- do so without P/Invoke/FFI being the implementation of managed interoperability;
- demonstrate that the managed call is causally produced from real Rust source compiled through `rustc`/the CLR codegen path rather than injected by an emitter or fixture.

The re-certification closes that causal gate using FerrumWeave-owned codegen rather than `rustc_codegen_clr`: real Rust source reaches FerrumWeave `codegen_crate`, MIR selects each managed-call family, FerrumWeave emits the CIL/metadata, and CoreCLR executes the result. Static call, construction, instance call, property access, and independently compiled external managed assembly consumption each have source-only falsification. Exact implementation baseline `60756627ca1617b517fbcbbb0654b1c2c23cb71b` passed FerrumWeave codegen backend convergence #106 and Rust CI #469, including fmt, clippy, coverage, and Linux/Windows integration. `rustc_codegen_clr` is retained only as historical/differential oracle evidence.

The first projection should favor **CLR-shaped, mechanically predictable semantics** over prematurely clever Rust wrappers. More idiomatic abstractions can be layered later without obscuring the underlying CLR contract.

---

# R06 — .NET consumes Rust

**Status: Done — re-certified through the FerrumWeave `rustc` CodegenBackend on Linux and Windows.**

## Goal

Make interoperability bidirectional.

Rust-generated managed APIs should be consumable as ordinary CLR metadata by existing .NET languages.

## Required languages

- **C# is mandatory.**
- **Visual Basic .NET is mandatory in R06.**
- F# is welcome and likely to appear early, but it becomes a mandatory gate in R09.

## DoD

R06 is Done when:

- a C# project references a managed assembly produced from Rust and calls supported public Rust-defined behavior;
- a Visual Basic project references the same kind of output and calls supported public Rust-defined behavior;
- neither consumer requires P/Invoke or a native ABI layer;
- CLR reflection sees coherent public names, signatures, visibility, and supported types;
- at least static and instance-call shapes are represented in the contract suite;
- Linux and Windows are green for the supported consumer scenarios.

The causal replay is complete: static and instance Rust-defined APIs are built by `FerrumWeave.Sdk -> rustc -> FerrumWeave CodegenBackend`, consumed independently from C# and Visual Basic, reflected as coherent CLR metadata and verified to contain no P/Invoke/native ABI substitute. Rust-only mutations change both artifact bytes and managed observables. The exact supporting SHAs and portable CI runs are recorded in `tests/r06/contracts.toml`; `rustc_codegen_clr` and the legacy R06 emitter are not in the product causal path.

---

# R07 — Semantic interoperability

**Status: Done — re-certified through the FerrumWeave `rustc` CodegenBackend on Linux and Windows.**

## Goal

Turn mechanical interoperability into semantics that developers can trust.

Expected design areas include:

- Rust ownership / borrowing versus managed GC references;
- `Result<T, E>` and CLR exceptions;
- `Option<T>` and CLR nullability / nullable values;
- `Drop` / RAII and `IDisposable`-style resource lifetime;
- supported generics boundaries;
- panic policy across managed public boundaries;
- lifetime rules at the CLR boundary.

## DoD

R07 is Done when:

- each supported cross-model semantic has an ADR and executable contracts;
- success, failure, exception, null/none, and resource-cleanup paths are tested where relevant;
- public projections do not silently erase Rust safety semantics;
- managed lifetime does not introduce known double-free/use-after-free behavior in supported contracts;
- unsupported combinations fail explicitly;
- the compatibility documentation explains what is safe, what is managed, and what remains unsupported.

The backend-convergence audit closes the causal gap: all declared R07 contracts now execute or reject through the FerrumWeave `rustc` CodegenBackend rather than relying on projection/emitter-only evidence. Exact aggregate SHA `6e86017e4b46526eec76a3f862a7bc922af14487` passed Rust CI #561 on Ubuntu and Windows, FerrumWeave codegen backend convergence #198, R02 #237, R03 #264 and R04 #413. `rustc_codegen_clr` remains oracle-only.

## `unsafe` policy

C# **does** support explicitly unsafe code, including pointer operations and `fixed` contexts. Therefore `unsafe` cannot be dismissed merely because FerrumWeave targets .NET.

FerrumWeave's policy is:

- `unsafe` is **eventually in scope as a first-class Rust capability**;
- it is **not a gate for R01-R10 or the first 0.1 alpha**;
- the safe-Rust/managed interoperability model must be proven first;
- implementing `unsafe` requires a dedicated ADR and capability milestone/track because native pointers, unverifiable IL, pinning, GC interaction, and platform ABI concerns change the risk model substantially;
- safe public abstractions must never require application authors to use `unsafe` merely because the backend implementation is incomplete.

Evidence may cause the dedicated unsafe milestone to move earlier, but support must not be smuggled into another milestone without explicit review.

---

# R08 — `.rsproj` and FerrumWeave SDK

**Status: Done — re-certified through `.rsproj -> FerrumWeave.Sdk -> rustc -> FerrumWeave CodegenBackend` on Linux and Windows.**

## Goal

Make a Rust project feel native to the .NET SDK experience after the compiler/interoperability core is real enough to deserve that shell.

Target experience:

```bash
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

Rust is a **language choice** for a .NET project family, not a project type named `rust`. The template-resolution decision and coexistence requirements are recorded in [ADR 0004](../architecture/adr/0004-r10-rust-as-dotnet-template-language.md).

## DoD

R08 is Done when, from a clean machine/environment with documented prerequisites:

- `dotnet new console -lang Rust` creates a valid Rust/.NET console project;
- `.rsproj` is an SDK-style project owned by `FerrumWeave.Sdk`;
- `dotnet restore` performs the supported restore responsibilities;
- `dotnet build` produces the managed FerrumWeave assembly through the real supported Rust compiler/codegen path;
- `dotnet run` executes it;
- `dotnet clean` behaves predictably;
- supported testing hooks have a documented `dotnet test` story or an explicitly documented limitation;
- no bespoke manual build script is required outside the SDK contract;
- the SDK uses the current supported stable/LTS .NET line rather than an obsolete target by default.

Exact product-path replay SHA `9b08fa42e0a0c5ad12700b544a670aabc045569c` passed Rust CI #565 on Ubuntu and Windows, FerrumWeave codegen backend convergence #202, R02 #241, R03 #268 and R04 #417. `tests/sdk_backend_path.rs` proves malformed Rust is rejected by real `rustc`, rejects `ferrumweave_emit` from the product path, and proves a Rust-only `137 -> 211` mutation changes both managed assembly bytes and an independent CoreCLR consumer observable.

---

# R09 — Mixed `.slnx` proof

**Status: Done — promoted and independently re-certified on `main`.**

## Goal

Prove the modernization story that makes FerrumWeave useful beyond compiler experimentation.

The canonical proof should resemble:

```text
Enterprise.slnx
├── Legacy/
│   └── Legacy.vbproj
├── Domain/
│   └── Domain.csproj
├── Analytics/
│   └── Analytics.fsproj
└── RiskEngine/
    └── RiskEngine.rsproj
```

## Mandatory ecosystem gates

R09 requires:

- Visual Basic;
- C#;
- **F# as a mandatory gate**;
- Rust through FerrumWeave;
- `ProjectReference` integration;
- consumption of at least one real **NuGet** package through the supported .NET dependency path.

F# may arrive earlier; R09 is simply the latest milestone at which it becomes non-optional.

## DoD

R09 is Done when:

- one `.slnx` contains the four project-language families above;
- a single normal .NET build workflow can build the supported solution graph;
- at least one end-to-end business-style call path crosses existing .NET code → Rust → managed .NET dependency and returns successfully;
- changing only the relevant Rust source changes the .NET-observed business result without changing the emitter, consumer, fixture, or expected-value logic;
- Visual Basic can participate in the path without migration to C#;
- F# compiles and interoperates as part of the certified solution, not as a screenshot/demo-only project;
- a real NuGet package is restored and consumed;
- project references resolve without manual copying of assemblies;
- the solution works on Linux and Windows where all chosen project types support the scenario;
- functional contracts prove the language crossings rather than merely checking that projects compile.

The final promoted R09 boundary is `main@eab90a620b0598dbbce8ea6bf51f7d5b725f5818`, a squash of the certified `dev@96a189159cd865726ebf48b502ac8fe501ea9a6f` state with the same tree `2f37276650272b79316780b9792e47e60d4683b9`. That exact `main` SHA independently completed GREEN in Rust CI #720, FerrumWeave codegen backend convergence #350, R02 #390, R03 #417, R04 #570, and managed-consumption causality #273. The source-causal contract remains the same: a Rust-only mutation changes `RiskEngine.dll` and the independent CoreCLR observable while preserving the managed dependency call and rejecting `ferrumweave_emit` from the product path.

This is the canonical **0.1 proof moment**:

> Add `RiskEngine.rsproj` to an existing multi-language `.slnx`; preserve what already works; strengthen the new critical component with Rust.

---

# R10 — Developer experience / 0.1 alpha

**Status: Active — resumed after certified R09 promotion.**

## Goal

Turn the proven compiler and solution capability into something an external developer can realistically try.

Expected areas include:

- installation and versioned packaging;
- FerrumWeave SDK distribution;
- Rust-as-language template integration through `dotnet new <template> -lang Rust`;
- rust-analyzer integration/awareness of projected CLR symbols;
- diagnostics that preserve useful Rust source locations;
- initial source mapping / PDB / debugger experience;
- templates and samples;
- compatibility and limitation documentation;
- reproducible release artifacts.

The `0.1-alpha` target template surface is:

```text
console
classlib
xunit
nunit
mstest
web
webapi
```

The full prerelease sequence and per-template evidence requirements are defined in [Template release plan](template-release-plan.md).

## DoD

R10 is Done when an external contributor, following only published documentation, can:

1. install the alpha toolchain;
2. create each supported 0.1 project family with `dotnet new <template> -lang Rust`;
3. edit Rust with useful Rust diagnostics;
4. consume a supported .NET API/NuGet dependency;
5. build and run/test through normal `dotnet` commands as appropriate to the project family;
6. build the canonical mixed `.slnx` example;
7. perform at least basic source-level debugging for the supported scenario;
8. understand from the compatibility matrix what is and is not implemented.

Additionally:

- release artifacts are reproducible and versioned;
- the alpha packaging path includes NuGet where appropriate for the SDK/tooling model;
- no documented getting-started step depends on unpublished maintainer knowledge or a FerrumWeave source checkout;
- installing FerrumWeave does not break existing C#, F#, or Visual Basic template variants;
- public status is still explicitly alpha and limitations remain visible.

Commands documented in `docs/getting-started.md` are release contracts until the corresponding executable CI evidence is GREEN; they must not be interpreted as already released capability merely because they are documented.

When these conditions are met, FerrumWeave may reasonably publish its first **0.1 alpha** rather than tagging a release merely because some CIL exists.

---

# Prerelease template sequence and stable gate

The agreed project-family sequence is:

| Release | Stage | Template families |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | Every committed template family is certified and has at least one real FerrumWeave project |

A template is not supported merely because scaffolding succeeds. It must satisfy the runnable/testable contract for that project family through the real FerrumWeave backend.

FerrumWeave reaches `1.0.0` only after **every template family above has survived at least one real project**. Compiler fixtures, template smoke tests, documentation samples, and synthetic conformance projects do not satisfy that stable-release gate. The real project must have a genuine application/library purpose and reproducible evidence appropriate to the template family.
