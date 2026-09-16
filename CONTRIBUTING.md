# Contributing to FerrumWeave

Thank you for helping build FerrumWeave.

FerrumWeave is an experimental effort to make Rust a first-class language for .NET. Contributions may touch compiler code, CLR metadata, project-system integration, SDK tooling, documentation, tests, developer experience, or community infrastructure.

The project is no longer only architectural discovery. R09 has a certified FerrumWeave-owned `rustc -> CodegenBackend -> managed CLR` product path, and **R10 / 0.1-alpha developer experience is the active frontier**. Small, evidence-producing increments remain preferable to broad speculative rewrites.

## Current engineering entry point

Before changing product behavior, determine whether the work belongs to the active milestone or to historical evidence.

For R10 work, read these repository paths before writing code:

1. `docs/roadmap/r10-agent-handoff.md` — operational handoff, current first contract, stop conditions, and branch discipline;
2. `docs/architecture/adr/0004-r10-rust-as-dotnet-template-language.md` — accepted decision that Rust is a language choice for standard .NET templates;
3. `docs/roadmap/template-release-plan.md` — release sequence and support levels;
4. `docs/roadmap/README.md` — milestone truth and transversal Definition of Done;
5. `docs/quality/r09-python-to-rust-inventory.md` and `docs/quality/r09-rust-authority-cutover.md` — verification-authority migration rules.

Executable evidence and current CI truth outrank prose. Before every mutation, fresh-read the destination branch and any overlapping work branch/PR rather than relying on a remembered SHA.

## Two ecosystems, one project

FerrumWeave should feel familiar to contributors from both Rust and .NET.

If you come from Rust, concepts such as Cargo workspaces, `rustc`, MIR, traits, lifetimes, `Result`, rustfmt, Clippy, and compile-time diagnostics are first-class concerns here.

If you come from .NET, concepts such as the CLR, CTS, CIL, assembly metadata, MSBuild, NuGet, SDK-style projects, `.slnx`, analyzers, PDBs, and `dotnet` CLI workflows are equally first-class concerns.

Neither ecosystem is a compatibility shim around the other. When a design crosses the boundary, explain the semantics on both sides.

## Before writing code

For typo fixes, small documentation improvements, focused tests, or obviously local corrections, a pull request is usually enough.

For changes that affect any of the following, open an issue or design discussion first unless an accepted ADR or active milestone contract already fixes the decision:

- public Rust-facing syntax or APIs;
- CLR-visible public surface or metadata;
- ownership/GC/lifetime semantics across the boundary;
- exception / `Result` mapping;
- async / `Future` / `Task` mapping;
- generics, traits, interfaces, delegates, or reflection behavior;
- `.rsproj`, MSBuild, Cargo, NuGet, or `dotnet` CLI behavior;
- compiler architecture or upstream strategy;
- compatibility guarantees;
- licensing, governance, security policy, or contributor requirements;
- a change large enough that review would be difficult as one focused increment.

Do not surprise the community with a large implementation of a design that has not been discussed. Conversely, do not reopen an accepted decision merely because an older prototype in the tree uses a different model.

## Branch and pull-request flow

The repository uses:

- `dev` for detailed integration history, including causal TDD/fix history;
- `main` for milestone/release promotion history;
- short-lived branches for individual increments.

Contributions should normally target `dev` unless maintainers explicitly request another base.

Promotion from `dev` to `main` is intentionally cleaner than ordinary feature integration: use a selective/squash milestone promotion so `main` records the promoted result rather than replaying hundreds of detailed integration commits. Do not force-update shared branches unless a maintainer explicitly authorizes repository surgery.

Before merge or promotion, fresh-read the base and head refs and certify the exact head SHA being merged.

Prefer one coherent purpose per pull request.

A good PR should answer:

1. What contract or problem does this change address?
2. What is deliberately out of scope?
3. What evidence shows the change works?
4. Does it affect Rust-facing behavior, .NET-facing behavior, or both?
5. Does it create divergence from an upstream project?
6. Which exact SHA and CI runs certify the result?

## Evidence before claims

FerrumWeave treats compatibility as a contract.

Claims such as “supports `System.String`”, “works with `ProjectReference`”, or “preserves Rust semantics” must be backed by executable tests whenever practical.

For cross-language work, prefer tests that exercise the real boundary rather than mocks of that boundary.

Examples of valuable evidence include:

- compile-pass / compile-fail tests;
- emitted CIL or metadata assertions;
- Rust -> C# and C# -> Rust interop tests;
- equivalent VB and F# consumption where relevant;
- end-to-end `dotnet build`, `dotnet run`, `dotnet test`, or `dotnet pack` scenarios;
- differential checks against known Rust or .NET behavior;
- regression tests for previously fixed contracts;
- Rust-source mutation that changes both the managed artifact and an independent CoreCLR observable when source causality is claimed.

Historical oracle evidence and product evidence are different evidence classes. `rustc_codegen_clr` may characterize prior behavior, but it must never occupy the FerrumWeave product-backend slot.

## TDD and verification authority

Known behavior should enter a contract ledger before or while implementation begins. Prefer the repository loop:

```text
RED -> GREEN -> REFACTOR -> CERTIFY
```

CI is a sensor during development, memory for known regressions, and the final certifier of a promoted capability.

When migrating verification from Python to Rust, preserve the observable contract rather than translating orchestration line-for-line. Use the repository's `PYTHON_ONLY`, `RUST_PARTIAL`, `DUAL`, `RUST_AUTHORITATIVE`, and `OBSOLETE` states. Do not delete a Python witness before material observable parity, a green CI-only cutover, and a green post-delete tree.

Rust is the authority for FerrumWeave product behavior. .NET tests certify external managed consumption/framework behavior when that is the public surface being proved.

## Preserve both models

Do not make Rust pretend to be C# merely because the CLR is involved.

Do not make .NET APIs pretend to be native Rust merely because Rust is the source language.

When the models differ, make the boundary explicit and document the consequences.

Particular care is required around:

- ownership vs. managed references;
- deterministic destruction vs. garbage collection;
- Rust `String` vs. `System.String`;
- slices/spans/arrays;
- `Option<T>` / nullable/reference-nullability semantics;
- `Result<T, E>` / exceptions;
- traits / CLR interfaces;
- closures / delegates;
- Rust futures / `Task` and `ValueTask`;
- monomorphization / CLR generics;
- unsafe code and unverifiable IL.

## Upstream first

FerrumWeave builds on existing Rust and .NET infrastructure rather than permanently forking it without necessity.

If a requirement belongs naturally in an upstream project, prefer this lifecycle:

1. identify the missing capability;
2. isolate any temporary local adaptation;
3. open or participate in the upstream discussion;
4. contribute upstream when practical;
5. remove local divergence once the upstream capability is available.

Document material temporary divergence.

The upstream-first rule does not turn `rustc_codegen_clr` into product implementation. That project remains prior art / characterization oracle unless an explicit decision changes its evidence role.

## Style and tooling

Follow the conventions of the component you are changing.

The current Rust quality commands are defined by `.github/workflows/rust-ci.yml`; locally, the important gates include:

```bash
cargo fmt --all
cargo clippy --workspace --all-targets --locked -- -D warnings
cargo build --workspace --all-targets --locked
cargo test --workspace --lib --locked
cargo test --workspace --tests --locked
```

Compiler/backend integration uses the pinned toolchain and compiler components declared in CI. Do not silently substitute a different nightly when certifying a backend contract.

Documentation graph changes are certified by `.github/workflows/docs-ci.yml` and `ferrumweave-docgraph`; generated navigation/backlinks must be committed rather than hand-edited inside generated regions.

.NET code should follow the repository's SDK/build conventions. Web/community code should follow the tooling already defined in its component.

Do not perform unrelated formatting or style churn in the same PR as a behavioral change.

## Coverage

FerrumWeave keeps a minimum Rust line-coverage gate of 80%, with 80%-96% as the preferred operating band and naturally higher coverage accepted. Functional coverage remains at least 96%, with 100% as the target.

Never hide a newly discovered contract, delete useful tests, or weaken a denominator merely to preserve a percentage.

## AI-assisted contributions

AI tools are welcome as development aids. They do not transfer responsibility away from the contributor.

If substantial code, tests, or prose were generated or transformed with an AI tool:

- review it as if you had written every line yourself;
- verify licensing and provenance of any material that may have been reproduced from another source;
- do not submit generated code you cannot explain and maintain;
- disclose material AI assistance in the PR when it meaningfully affected the implementation or evidence;
- never use AI output as a substitute for executable validation of compiler/runtime claims.

The author of the PR remains accountable for correctness, licensing, security, and maintainability.

## Contribution licensing and sign-off

FerrumWeave is licensed under **MIT OR Apache-2.0**, at the user's option.

By contributing, you agree that your contribution may be distributed under those project licenses.

FerrumWeave also uses the Developer Certificate of Origin (DCO) sign-off model. Add a `Signed-off-by` line to each commit you contribute:

```text
Signed-off-by: Your Name <you@example.com>
```

The easiest way is:

```bash
git commit -s
```

See [`DCO.md`](DCO.md) for the contribution-certification policy.

## Reviews

Review the change, not the author.

Reviewers should distinguish between:

- correctness or compatibility blockers;
- maintainability concerns;
- optional improvements;
- personal preference.

Contributors should be able to tell which feedback must be resolved before merge.

If a discussion becomes a Rust-vs-.NET argument, restate the concrete contract being designed and return to evidence.

## Getting started

Useful contributions now include:

- advancing the active R10 contract through RED -> GREEN without bypassing the installed-package boundary;
- migrating remaining verification authority from Python to Rust according to the recorded inventory;
- strengthening executable compiler/CLR interoperability evidence;
- improving architecture and compatibility documentation when implementation changes technical truth;
- improving the website and contributor experience;
- reviewing terminology and contracts for accuracy from either the Rust or .NET perspective.

Do not infer that a target is unimplemented merely because an old document says “eventually”, and do not infer that a target is implemented merely because a prototype file exists. Check the milestone ledger and current CI evidence.

## Code of Conduct

Participation in FerrumWeave is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
