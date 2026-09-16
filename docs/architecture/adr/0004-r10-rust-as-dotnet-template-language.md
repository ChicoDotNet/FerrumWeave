<!--
doc-id: architecture.adr.0004-rust-as-dotnet-template-language
locale: en
-->

# ADR 0004 — Register Rust as a .NET template language

**English** · [Deutsch](0004-r10-rust-as-dotnet-template-language.de.md) · [Español](0004-r10-rust-as-dotnet-template-language.es.md) · [Français](0004-r10-rust-as-dotnet-template-language.fr.md) · [Italiano](0004-r10-rust-as-dotnet-template-language.it.md) · [Português (Brasil)](0004-r10-rust-as-dotnet-template-language.pt-BR.md) · [Русский](0004-r10-rust-as-dotnet-template-language.ru.md) · [简体中文](0004-r10-rust-as-dotnet-template-language.zh-Hans.md) · [日本語](0004-r10-rust-as-dotnet-template-language.ja.md)

- Status: Accepted for R10 and the prerelease roadmap
- Date: 2026-09-15
- Milestone: R10 — Developer experience / 0.1 alpha

## Context

FerrumWeave's purpose is to make Rust a first-class language in the .NET ecosystem. The original R10 prototype exposed a dedicated template command:

```console
dotnet new rust
```

That command is technically convenient, but it models Rust as though it were a project type. In the .NET template model, `console`, `classlib`, `web`, test projects, desktop projects, and other workload families are project templates; the source language is a separate selection dimension.

A .NET developer already expects equivalent project families to differ by language rather than by unrelated template names. FerrumWeave should preserve that mental model.

## Decision

FerrumWeave's primary template UX is:

```console
dotnet new <template> -lang Rust
```

The canonical first-use contract is therefore:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

`dotnet new rust` is not the target public product contract.

FerrumWeave templates must declare Rust as their language and participate in the normal .NET template families wherever the template engine allows that integration cleanly. The implementation must be validated by executable template-resolution contracts rather than assuming that matching built-in template metadata is sufficient.

Installing FerrumWeave must not break or alter the normal behavior of the existing C#, F#, or Visual Basic template variants.

## Release sequence

Template support is introduced in these capability releases:

- `0.1-alpha`: `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi`;
- `0.2-alpha`: `mvc`, `winforms`;
- `0.3-beta`: `worker`;
- `0.4-beta`: `wpf`;
- `0.5-beta`: `grpc`;
- `0.6-beta`: `blazor`.

The detailed support contracts and stable-release gate are maintained in [`docs/roadmap/template-release-plan.md`](../../roadmap/template-release-plan.md).

## Supported means executable

A generated directory is not sufficient evidence that a template is supported.

For prerelease compatibility claims, the template must at minimum:

- scaffold through the installed FerrumWeave distribution;
- restore and build without a FerrumWeave source checkout;
- execute the default meaningful workflow (`dotnet run` or `dotnet test` as appropriate);
- reach FerrumWeave's real `rustc -> CodegenBackend -> CIL/metadata -> CoreCLR` product path;
- pass the relevant CI/platform contracts;
- document limitations honestly.

Framework-specific templates must prove the framework behavior that distinguishes them from a generic console application.

## 1.0 gate

FerrumWeave does not become stable only because synthetic conformance tests pass.

Before `1.0.0`, every template family in the committed prerelease sequence must have at least one real project using that FerrumWeave template successfully.

A real project is not a compiler fixture, template smoke test, or documentation sample. It must have a genuine non-test purpose and reproducible evidence appropriate to that project family.

## Consequences

Positive consequences:

- FerrumWeave behaves like an extension of .NET rather than a parallel CLI ecosystem;
- template commands remain unsurprising to C#, F#, and Visual Basic developers;
- the product roadmap becomes measurable by project-family contracts;
- future project templates can be added without inventing FerrumWeave-specific command taxonomies;
- the 1.0 gate ties compatibility claims to real-world evidence.

Costs and constraints:

- the R10 prototype template metadata and tests that assume `dotnet new rust` must be revised;
- template resolution must be verified against the real .NET template engine;
- framework templates may require new compiler/runtime capabilities before they can honestly be marked supported;
- desktop and framework-specific templates retain their own platform/build constraints.

## Validation

R10 must add executable falsifiers proving at least:

```console
dotnet new console -lang Rust
dotnet new console -lang VB
dotnet new console -lang "F#"
```

coexist after FerrumWeave installation, and that the Rust-generated project uses the causal FerrumWeave product backend rather than a legacy emitter or source-language substitute.
