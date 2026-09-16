# FerrumWeave template release plan

**English** · [Deutsch](template-release-plan.de.md) · [Español](template-release-plan.es.md) · [Français](template-release-plan.fr.md) · [Italiano](template-release-plan.it.md) · [Português (Brasil)](template-release-plan.pt-BR.md) · [Русский](template-release-plan.ru.md) · [简体中文](template-release-plan.zh-Hans.md) · [日本語](template-release-plan.ja.md)

FerrumWeave's developer-experience roadmap treats Rust as a **.NET project language**, not as a project type of its own.

The primary template UX is therefore:

```console
dotnet new <template> -lang Rust
```

For example:

```console
dotnet new console -lang Rust -n HelloFerrum
cd HelloFerrum
dotnet run
```

The earlier `dotnet new rust` form is not the target product experience. A console application is still a `console` project; FerrumWeave adds Rust as another language choice alongside the languages already understood by the .NET template engine.

## What "supported" means

A template is not considered supported merely because scaffolding files can be generated.

For a template to be listed as supported in a FerrumWeave prerelease:

1. `dotnet new <template> -lang Rust` must create the expected `.rsproj` and Rust source layout from an installed FerrumWeave package.
2. The generated project must restore and build through the normal .NET SDK workflow without requiring a FerrumWeave repository checkout.
3. The template's default meaningful workflow must execute successfully: `dotnet run` for runnable applications and `dotnet test` for test projects.
4. The resulting behavior must flow causally through `rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`; legacy source-pattern emitters or generated C#/VB/F# substitutes do not satisfy the contract.
5. The template must have Windows and Linux certification when the underlying .NET project family is cross-platform. Platform-specific templates such as WinForms and WPF are certified on supported Windows environments and must not imply Linux execution.
6. Known limitations must be documented explicitly.

Samples and conformance fixtures may prove a prerelease contract, but they do not satisfy the final 1.0 real-project gate described below.

## Version sequence

| Release | Stage | Templates introduced as supported |
| --- | --- | --- |
| `0.1` | Alpha | `console`, `classlib`, `xunit`, `nunit`, `mstest`, `web`, `webapi` |
| `0.2` | Alpha | `mvc`, `winforms` |
| `0.3` | Beta | `worker` |
| `0.4` | Beta | `wpf` |
| `0.5` | Beta | `grpc` |
| `0.6` | Beta | `blazor` |
| `1.0` | Stable | All committed template families have both certified contracts and at least one real project using FerrumWeave successfully |

Version labels may include normal prerelease suffixes such as `0.1.0-alpha.1` or `0.5.0-beta.1`; the table above defines the capability boundary, not the exact number of patch-level prereleases required to reach it.

## 0.1 alpha — core .NET project language experience

The first alpha establishes that Rust can participate in the most common .NET project workflows without inventing a separate FerrumWeave-only project taxonomy.

Required template contracts:

```console
dotnet new console  -lang Rust
dotnet new classlib -lang Rust
dotnet new xunit    -lang Rust
dotnet new nunit    -lang Rust
dotnet new mstest   -lang Rust
dotnet new web      -lang Rust
dotnet new webapi   -lang Rust
```

The release should prove at least:

- executable entry point and console output;
- a managed class library consumable from another .NET project;
- discovery and execution through xUnit, NUnit, and MSTest runners;
- an ASP.NET Core host that starts through normal `dotnet run`;
- a minimal HTTP request/response path for `web` and `webapi`;
- normal restore, build, run/test, ProjectReference, and NuGet behavior appropriate to the template;
- installation and use from a clean external directory with no FerrumWeave source checkout.

## 0.2 alpha — application frameworks

Adds:

```console
dotnet new mvc      -lang Rust
dotnet new winforms -lang Rust
```

`mvc` must prove an actual MVC request path rather than only hosting ASP.NET Core. `winforms` must prove an actual Windows Forms application lifecycle and supported event/delegate interaction on Windows.

## 0.3 beta — background services

Adds:

```console
dotnet new worker -lang Rust
```

The Worker template must exercise the .NET Generic Host and a real background-service lifecycle rather than a renamed console application.

## 0.4 beta — WPF

Adds:

```console
dotnet new wpf -lang Rust
```

WPF is treated as its own capability boundary because XAML, WindowsDesktop MSBuild targets, generated types/code-behind, application startup, and the WPF event model introduce build/runtime requirements beyond ordinary CLR code generation.

## 0.5 beta — gRPC

Adds:

```console
dotnet new grpc -lang Rust
```

The contract must cover the generated protobuf/gRPC build path and at least one real request/response round trip.

## 0.6 beta — Blazor

Adds the supported Blazor template surface for the .NET SDK version targeted by FerrumWeave. The exact template short name must follow the contemporary .NET SDK rather than freezing an obsolete command in this roadmap.

The beta contract must prove a real rendered component path and the Razor/Blazor build integration required by that template family.

## 1.0 stable gate — evidence from real projects

FerrumWeave does not reach 1.0 merely because all template conformance suites are green.

Before `1.0.0`, **every template family committed above must have at least one real project that uses FerrumWeave successfully**.

For this gate, a real project:

- is not a FerrumWeave compiler fixture, conformance test, template smoke test, or documentation-only sample;
- has a genuine application/library purpose beyond proving FerrumWeave itself;
- uses the corresponding Rust template family in its normal project structure;
- can restore/build and, where applicable, run or test reproducibly from documented instructions;
- exercises the template in a way representative of its intended .NET workload;
- has any FerrumWeave-specific limitations recorded rather than hidden.

The projects may be open source or privately evidenced, but the project must have enough reproducible evidence to justify the compatibility claim.

The stable-release question is therefore not only:

> Can FerrumWeave scaffold and certify every promised project family?

It is also:

> Has every promised project family survived contact with at least one real application?

Only when both answers are yes does FerrumWeave cross the `1.0.0` gate.

## Compatibility rule

Installing FerrumWeave must not change or break the existing behavior of other language variants. The R10 contract suite must explicitly falsify regressions for standard templates such as:

```console
dotnet new console
dotnet new console -lang VB
dotnet new console -lang "F#"
dotnet new console -lang Rust
```

FerrumWeave extends the .NET language ecosystem; it must not hijack it.
