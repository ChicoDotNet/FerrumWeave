# Contributing to FerrumWeave

Thank you for helping build FerrumWeave.

FerrumWeave is an experimental effort to make Rust a first-class language for .NET. Contributions may eventually touch compiler code, CLR metadata, project-system integration, SDK tooling, documentation, tests, developer experience, or community infrastructure.

The project is currently **pre-alpha / architectural discovery**. Small, evidence-producing contributions are more valuable than broad speculative rewrites.

## Two ecosystems, one project

FerrumWeave should feel familiar to contributors from both Rust and .NET.

If you come from Rust, concepts such as Cargo workspaces, `rustc`, MIR, traits, lifetimes, `Result`, rustfmt, Clippy, and compile-time diagnostics are first-class concerns here.

If you come from .NET, concepts such as the CLR, CTS, CIL, assembly metadata, MSBuild, NuGet, SDK-style projects, `.slnx`, analyzers, PDBs, and `dotnet` CLI workflows are equally first-class concerns.

Neither ecosystem is a compatibility shim around the other.

When a design crosses the boundary, explain the semantics on both sides.

## Before writing code

For typo fixes, small documentation improvements, focused tests, or obviously local corrections, a pull request is usually enough.

For changes that affect any of the following, open an issue or design discussion first:

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

Do not surprise the community with a large implementation of a design that has not been discussed.

## Branch and pull-request flow

The repository uses:

- `main` for stable, reviewed repository state;
- `dev` as the integration branch;
- short-lived branches for individual increments.

Contributions should normally target `dev` unless maintainers explicitly request another base.

Prefer one coherent purpose per pull request.

A good PR should answer:

1. What contract or problem does this change address?
2. What is deliberately out of scope?
3. What evidence shows the change works?
4. Does it affect Rust-facing behavior, .NET-facing behavior, or both?
5. Does it create divergence from an upstream project?

## Evidence before claims

FerrumWeave treats compatibility as a contract.

When implementation begins, claims such as “supports `System.String`”, “works with `ProjectReference`”, or “preserves Rust semantics” should be backed by executable tests whenever practical.

For cross-language work, prefer tests that exercise the real boundary rather than mocks of that boundary.

Examples of valuable evidence include:

- compile-pass / compile-fail tests;
- emitted CIL or metadata assertions;
- Rust → C# and C# → Rust interop tests;
- equivalent VB and F# consumption where relevant;
- end-to-end `dotnet build`, `dotnet run`, `dotnet test`, or `dotnet pack` scenarios;
- differential checks against known Rust or .NET behavior;
- regression tests for previously fixed contracts.

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

FerrumWeave expects to build on existing Rust and .NET infrastructure rather than permanently fork it without necessity.

If a requirement belongs naturally in an upstream project, prefer this lifecycle:

1. identify the missing capability;
2. isolate any temporary local adaptation;
3. open or participate in the upstream discussion;
4. contribute upstream when practical;
5. remove local divergence once the upstream capability is available.

Document material temporary divergence.

## Style and tooling

Follow the conventions of the component you are changing.

Rust code should use normal Rust conventions and the repository's pinned Rust tooling once those files exist.

.NET code should use the repository's .NET build/analyzer conventions once those files exist.

Web/community code should follow the tooling already defined in its component.

Do not perform unrelated formatting or style churn in the same PR as a behavioral change.

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

At this early stage, useful contributions include:

- improving architecture documentation;
- identifying relevant upstream capabilities and constraints;
- turning design assumptions into executable experiments;
- improving the website and contributor experience;
- proposing small cross-language contracts that can become future tests;
- reviewing terminology for accuracy from either the Rust or .NET perspective.

As implementation areas become real, the repository will add component-specific build and test instructions rather than documenting commands that do not yet exist.

## Code of Conduct

Participation in FerrumWeave is governed by [`CODE_OF_CONDUCT.md`](CODE_OF_CONDUCT.md).
