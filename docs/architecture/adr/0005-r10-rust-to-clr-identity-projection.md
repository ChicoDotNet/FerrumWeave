# ADR 0005 — Preserve Rust source semantics while projecting idiomatic CLR identities

- Status: Accepted for R10 and the prerelease interoperability roadmap
- Date: 2026-09-16
- Milestone: R10 — Developer experience / 0.1 alpha

## Context

FerrumWeave exists to make Rust a first-class language in the .NET ecosystem without inventing a Rust-like replacement language. Rust source must remain valid Rust and continue to be parsed, type-checked, borrow-checked, and lowered by `rustc`.

At the same time, a FerrumWeave-produced assembly must behave like a real .NET assembly. That means the CLR surface needs stable assembly, namespace, type, member, and entry-point identities that are natural to consume from C#, Visual Basic, F#, reflection, ASP.NET Core, test frameworks, and other .NET tooling.

R10 exposed this boundary directly. The installed package can already generate and build an external `HelloFerrum.rsproj`, and a Rust-only mutation changes the managed assembly and an independent CoreCLR observable. However, direct `dotnet run` currently fails because the generated assembly has no CLR executable entry point. The product therefore needs an explicit projection rule rather than a one-off patch that turns an arbitrary export into an entry point.

The same issue applies beyond `main`. A real application must be able to organize exported Rust code into a coherent CLR surface. For example, Rust code organized under a `controllers` module should be able to project CLR-visible types such as:

```text
HelloFerrum.Controllers.ApiDeClima
HelloFerrum.Controllers.ServicioDeClima
HelloFerrum.Controllers.RepositorioDeClima
HelloFerrum.Controllers.ContextoDbDeClima
```

so a Visual Basic, C#, or F# consumer can address those types using normal .NET identity rules.

## Decision

FerrumWeave separates **Rust source identity** from **CLR projected identity**.

Rust remains Rust. FerrumWeave does not add C#-style syntax such as:

```rust
namespace HelloFerrum;
```

and does not require Rust authors to rename `main` to `Main` merely to satisfy CLR conventions.

Instead, FerrumWeave owns a deterministic projection layer that maps Rust crate/module/item identity into CLR assembly/namespace/type/member identity.

The projection is part of the product contract. It must be source-causal, reproducible, inspectable through CLR metadata/reflection, and independently consumable from supported .NET languages.

## Executable entry-point projection

The Rust source entry point remains ordinary Rust:

```rust
fn main() {
    // ...
}
```

or another supported Rust `main` signature declared by a later explicit contract.

For an executable `.rsproj`, FerrumWeave projects that Rust entry point into a CLR executable entry adapter whose canonical managed identity is conceptually:

```text
<RootNamespace>.Program.Main
```

For the canonical console project:

```text
HelloFerrum.Program.Main
```

The managed `Main` method must be static and the emitted PE/CLI metadata must designate the corresponding `MethodDef` as the assembly entry point. Merely emitting a method named `Main` is insufficient if the CLI entry-point token is missing or points elsewhere.

The generated managed adapter is allowed to be compiler-owned infrastructure. Rust authors do not write it manually and FerrumWeave must not satisfy the contract by generating C# source or substituting another language frontend.

Conceptually:

```text
Rust source

crate::main()
     |
     v
FerrumWeave CodegenBackend
     |
     v
HelloFerrum.Program.Main   static managed adapter
     |
     +-- CLI EntryPoint MethodDef
     |
     v
Rust main semantics
```

The exact adapter signature and error/argument policy are separate contracts. For example, if FerrumWeave supports a Rust-facing `fn main() -> Result<()>`, the adapter may map success to process exit code `0` and map errors through an explicitly documented FerrumWeave policy. That policy must not be invented implicitly while fixing R10.

## Assembly and apphost identity

A framework-dependent executable project still produces a real managed .NET assembly such as:

```text
HelloFerrum.dll
```

That managed assembly is the portable CLR artifact and contains CIL, metadata, and the managed entry point.

Where the normal .NET SDK/apphost model applies, the build may also produce a platform-specific host executable:

```text
Windows:      HelloFerrum.exe
Linux/macOS:  HelloFerrum
```

FerrumWeave documentation must distinguish the managed assembly from the optional/platform-specific apphost. `HelloFerrum.exe` must not replace the managed assembly as the architectural artifact being claimed.

## Root namespace and assembly name

FerrumWeave projects a deterministic root CLR identity for each `.rsproj`.

The default root namespace is resolved in this order:

1. explicit MSBuild `RootNamespace`, when provided;
2. `AssemblyName`, when provided;
3. the normalized project name.

The assembly name follows normal SDK-style project semantics and defaults to the normalized project name unless explicitly configured.

For the canonical project:

```text
Project:       HelloFerrum.rsproj
Assembly:      HelloFerrum
RootNamespace: HelloFerrum
```

These values belong to the project/CLR projection boundary, not to new Rust syntax.

## Rust modules project to CLR namespace segments

Rust module identity is the semantic source of CLR namespace organization. Physical folders alone are not authoritative because Rust modules may be declared and organized in more than one filesystem shape.

A Rust module path such as:

```rust
pub mod controllers {
    // CLR-exportable public types
}
```

projects under the root namespace as:

```text
HelloFerrum.Controllers
```

Nested modules add nested namespace segments deterministically.

The default CLR namespace spelling should be .NET-idiomatic while remaining mechanically traceable to the Rust module path. A later explicit naming override mechanism may be added when interoperability requires exact pre-existing CLR names, but it must not require inventing non-Rust source syntax.

## Public Rust types project to CLR types

CLR-exportable public Rust types declared inside a projected module receive CLR identities under that module's projected namespace.

Conceptually:

```rust
pub mod controllers {
    pub struct ApiDeClima { /* ... */ }
    pub struct ServicioDeClima { /* ... */ }
    pub struct RepositorioDeClima { /* ... */ }
    pub struct ContextoDbDeClima { /* ... */ }
}
```

may project as:

```text
HelloFerrum.Controllers.ApiDeClima
HelloFerrum.Controllers.ServicioDeClima
HelloFerrum.Controllers.RepositorioDeClima
HelloFerrum.Controllers.ContextoDbDeClima
```

The exact representation of each Rust type as a CLR class, value type, interface implementation, or another CTS construct remains governed by the applicable type-system contracts and ADRs. Namespace projection does not silently redefine Rust ownership or CTS semantics.

## Rust methods project to CLR members

For CLR-exportable members, FerrumWeave projects a stable .NET-facing name and member shape while preserving the Rust source symbol as the causal implementation identity.

Default direction:

- Rust module/function naming remains idiomatic Rust;
- CLR public member naming is .NET-idiomatic, normally PascalCase;
- a Rust associated function with no receiver may project as a CLR static method;
- a Rust method with a supported receiver may project as an instance method;
- constructor/property/event projections require their own explicit semantics rather than name-only heuristics.

For example, a Rust method such as:

```rust
pub fn obtener_pronostico(&self) -> Pronostico {
    // ...
}
```

may project as the CLR member:

```text
HelloFerrum.Controllers.ServicioDeClima.ObtenerPronostico()
```

A .NET consumer must be able to discover and invoke the projected member through normal metadata/reflection and normal language calls, subject to the type/signature mappings that FerrumWeave actually supports.

## Cross-language acceptance example

A Visual Basic consumer should eventually be able to use a FerrumWeave assembly with ordinary CLR names, conceptually:

```vb
Dim api = New HelloFerrum.Controllers.ApiDeClima()
Dim servicio = New HelloFerrum.Controllers.ServicioDeClima()
Dim repositorio = New HelloFerrum.Controllers.RepositorioDeClima()
Dim contexto = New HelloFerrum.Controllers.ContextoDbDeClima()
```

The same CLR identities must be addressable from C#, F#, reflection, and other supported .NET consumers without a Rust-specific runtime lookup protocol.

## Visibility and export boundary

`pub` in Rust does not automatically mean "public CLR API" for every item. Rust visibility and CLR exportability are related but distinct contracts.

FerrumWeave must make the CLR export boundary explicit enough that:

- internal Rust implementation details do not accidentally become permanent public .NET API;
- intended CLR-facing items have deterministic metadata identities;
- unsupported projections fail diagnostically instead of producing misleading metadata;
- source-to-CLR identity remains traceable for debugging, diagnostics, and compatibility.

The precise opt-in/export policy may evolve, but R10 must not hard-code all public Rust items into one synthetic type such as `FerrumWeave.RustApi` as the long-term application model.

## Consequences

Positive consequences:

- Rust source remains real Rust and stays under `rustc` semantics;
- generated assemblies behave like normal .NET assemblies rather than opaque containers of Rust exports;
- `dotnet run` can be solved through a principled CLR entry-point projection instead of a template-specific hack;
- application code can be organized by Rust modules and still expose familiar CLR namespaces;
- C#, Visual Basic, F#, ASP.NET Core, test frameworks, dependency injection, reflection, and NuGet consumers can address FerrumWeave types using ordinary .NET identities;
- the same projection model can support later `console`, `web`, `webapi`, `worker`, library, and test-template work.

Costs and constraints:

- code generation must carry project/root-namespace context into metadata emission;
- namespace/type/member naming becomes compatibility surface and requires regression tests;
- source-level visibility and CLR export visibility need explicit policy;
- name collisions, generic types, nested modules, traits/interfaces, constructors, properties, async methods, and overloads require later contracts;
- naming conversion must remain deterministic and must support explicit overrides before FerrumWeave can claim compatibility with arbitrary pre-existing CLR APIs.

## R10 validation requirements

R10 must not mark the console developer-experience contract GREEN merely because an arbitrary exported Rust function can be invoked by reflection.

At minimum, the executable path must prove:

1. ordinary Rust `fn main` remains the source entry point;
2. the generated managed assembly contains the expected `<RootNamespace>.Program.Main` projection or an explicitly superseding accepted identity;
3. the managed entry adapter is static;
4. PE/CLI metadata designates the correct managed `MethodDef` as the CLI entry point;
5. `dotnet run` executes the Rust-source-causal behavior on Windows and Linux;
6. no generated C#, native sidecar, legacy emitter, or upstream oracle substitutes for the FerrumWeave product path;
7. the managed assembly remains a real `.dll`, with apphost output treated separately when present.

The broader CLR surface contract must additionally gain executable evidence that at least one nested Rust module and multiple CLR-exportable public types project to stable namespace-qualified identities and can be discovered/consumed from an independent .NET project.

## Non-goals for this ADR

This ADR does not decide:

- the complete `Result`/panic/exception policy for process entry points;
- command-line argument projection;
- whether every public Rust item is automatically exported;
- the final attribute/configuration syntax for CLR naming overrides;
- every Rust-module-to-CLR-namespace casing rule;
- full class/value-type/interface projection for arbitrary Rust types;
- async `Main`, `Task`, or framework-specific hosting semantics.

Those decisions must be introduced by smaller executable contracts without weakening the separation established here: **Rust source semantics remain Rust; FerrumWeave owns the CLR projection.**
