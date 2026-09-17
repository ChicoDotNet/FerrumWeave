# ADR 0006 — Preserve Rust naming and visibility semantics while enforcing a safe CLR public surface

- Status: Accepted for R10 and the prerelease interoperability roadmap
- Date: 2026-09-16
- Milestone: R10 — Developer experience / 0.1 alpha
- Amends: ADR 0005 visibility/export wording and makes its naming policy executable

## Context

ADR 0005 establishes the architectural boundary: Rust source remains Rust, while FerrumWeave owns deterministic projection into CLR assembly, namespace, type, member, and entry-point identities.

Two details need a stronger contract before that projection becomes durable public API:

1. Rust and .NET do not enforce naming in exactly the same layer. Rust has semantic names such as `main` and style lints such as `non_snake_case`; .NET has CLR metadata identities, language-specific entry rules such as C# `Main`, and public API conventions enforced mainly through analyzers/tooling.
2. Rust visibility is not equivalent to a single CLR `public`/`private` bit. Rust has `pub`, `pub(crate)`, `pub(super)`, `pub(in path)`, re-exports, and effective reachability. CLR has `Public`, `Assembly`, `Family`, `FamORAssem`, `FamANDAssem`, and `Private`.

FerrumWeave must preserve Rust semantics without exposing ambiguous or accidentally widened CLR APIs.

## Decision summary

FerrumWeave maintains distinct Rust and CLR identities and validates the projected public surface before metadata emission.

The governing principles are:

- Rust source identity is authoritative for Rust semantics.
- CLR projected identity is authoritative for .NET consumption.
- Public CLR identities must be deterministic, traceable, and safe for supported .NET languages.
- CLR accessibility is derived from **effective Rust visibility**, not merely from the presence of the syntactic token `pub`.
- Explicit CLR projection policy may narrow Rust-authorized accessibility but must never widen it.
- CLR inheritance-specific accessibility (`protected` families) is only valid when an explicit managed inheritance projection exists; it is never inferred from Rust module visibility.

## Naming identities

FerrumWeave treats every projected item as having conceptually separate identities:

```text
RustIdentity
    crate::controllers::servicio_de_clima::obtener_pronostico

ClrIdentity
    HelloFerrum.Controllers.ServicioDeClima.ObtenerPronostico

InteropKey
    a normalized comparison key used to detect cross-language collisions

InternalSymbol
    a compiler-owned private identity that may be mangled without becoming public API
```

`RustIdentity` is never rewritten in source merely to satisfy CLR naming conventions.

`ClrIdentity` is deterministic and .NET-facing. Default public naming is .NET-idiomatic while remaining mechanically traceable to the Rust source identity.

`InteropKey` exists only for safety checks. It must catch collisions that CLR metadata might technically permit but a supported case-insensitive or keyword-sensitive consumer could not address unambiguously.

`InternalSymbol` is implementation infrastructure. FerrumWeave may use stable mangling there because it is not public compatibility surface.

## Entry-point naming is semantic, not textual

FerrumWeave must not locate the Rust entry point by scanning source text for a function spelled `main`.

The backend must consume the semantic entry-point decision already made by `rustc` and project that Rust entry identity to the accepted CLR executable facade, currently:

```text
<RootNamespace>.Program.Main
```

The CLR adapter name `Main` does not rename the Rust item. It is a managed projection whose body reaches the Rust entry semantics and whose `MethodDef` is designated as the CLI entry point.

This also permits valid Rust forms in which the semantic entry point is introduced through normal Rust name resolution rather than by a simplistic textual convention.

## Default CLR naming

FerrumWeave uses deterministic mechanical defaults rather than guessing developer intent.

Examples:

```text
Rust                                      CLR
controllers                          ->  Controllers
weather_api                          ->  WeatherApi
servicio_de_clima                   ->  ServicioDeClima
obtener_pronostico                  ->  ObtenerPronostico
ApiDeClima                           ->  ApiDeClima
```

FerrumWeave must not silently guess acronym styling such as `io_stream -> IOStream`; the deterministic default is preferred, with an explicit override mechanism reserved for compatibility requirements.

Raw Rust identifier syntax such as `r#type` does not become part of the CLR identifier itself.

## Public naming collisions are errors

A public CLR projection must be unique under the supported interoperability comparison rules.

For example, two distinct Rust names such as:

```rust
pub fn get_weather() {}
pub fn getWeather() {}
```

must not silently become two indistinguishable public CLR members such as `GetWeather`.

Likewise, public CLR identities that differ only by case must be rejected when they would be ambiguous to a supported case-insensitive .NET language such as Visual Basic.

FerrumWeave must fail diagnostically and report:

- both Rust source identities;
- the colliding projected CLR identity;
- the applicable interoperability rule;
- the available remediation, such as a source rename or explicit CLR-name override.

FerrumWeave must never resolve a public collision by silently appending numbers, hashes, suffixes, or arbitrary mangling. Stable mangling remains valid only for non-public compiler-owned implementation symbols.

## Supported-language safety

The public CLR surface should be consumable unambiguously from every .NET language FerrumWeave claims to support for external consumption.

At minimum, R10 validation must account for C#, Visual Basic, and F# naming/interoperability constraints rather than validating only reflection metadata.

The projected public API must therefore reject or explicitly resolve:

- case-insensitive collisions;
- reserved/keyword collisions that make normal consumption ambiguous or impractical;
- deterministic naming transformations that collapse two different Rust identities into one CLR identity;
- reserved FerrumWeave CLR identities such as the executable entry facade when the user projection would otherwise occupy the same identity.

## Effective Rust visibility is the source of CLR accessibility

ADR 0005 previously stated broadly that Rust `pub` does not automatically mean public CLR API. This ADR narrows and corrects that wording.

The rule is:

> A Rust item that is **effectively externally reachable according to rustc** projects to CLR `public` by default when the item has a supported CLR projection.

FerrumWeave must not decide external visibility merely by observing the syntactic token `pub`.

For example:

```rust
mod internals {
    pub struct WeatherService;
}
```

contains a syntactically `pub` type that is not externally reachable through the private parent module. It must not become public CLR API merely because the item declaration contains `pub`.

By contrast:

```rust
mod internals {
    pub struct WeatherService;
}

pub use internals::WeatherService;
```

makes the type externally reachable through the crate public surface. That effective Rust visibility is the relevant input to CLR projection.

The backend should consume rustc visibility/effective-visibility information rather than reimplementing Rust privacy and re-export rules independently.

## Default visibility mapping

The default projection policy is:

| Effective Rust accessibility | Default CLR accessibility |
| --- | --- |
| externally reachable `pub` | `Public` / C# `public` |
| crate-wide `pub(crate)` | `Assembly` / C# `internal` |
| `pub(super)` | non-public implementation |
| `pub(in path)` | non-public implementation |
| private / `pub(self)` | non-public implementation |

`pub(crate) -> internal` is the normal cross-runtime mapping because both mean assembly/crate-wide collaboration without external consumer access.

Rust restricted visibility such as `pub(super)` and `pub(in path)` is more granular than CLR namespace accessibility. CLR namespaces do not themselves create accessibility domains. FerrumWeave therefore preserves those Rust restrictions semantically through rustc and emits only the minimum non-public CLR accessibility required by the generated implementation topology.

The exact private-vs-assembly lowering for such implementation details is a compiler concern and must not widen the supported external API.

## Accessibility may narrow but never widen

FerrumWeave may later support explicit CLR accessibility overrides for interoperability scenarios.

Such a projection is constrained by the invariant:

```text
CLR-authorized accessibility ⊆ Rust-authorized accessibility
```

Examples:

```text
Rust externally-public
    -> CLR public                 valid default
    -> CLR internal               valid narrowing
    -> CLR private                valid narrowing

Rust pub(crate)
    -> CLR internal               valid default
    -> CLR private                valid narrowing
    -> CLR public                 invalid widening

Rust restricted/private
    -> CLR public                 invalid widening
```

No `.rsproj` option, attribute, template, or backend convenience may make an item visible to a CLR consumer that Rust semantics did not authorize.

## Protected accessibility belongs to managed inheritance projection

Rust does not model encapsulation through class inheritance and therefore FerrumWeave must not invent non-Rust source syntax such as:

```rust
protected fn foo() {}
```

or infer CLR `Family` accessibility from Rust module hierarchy.

CLR accessibility forms such as:

- `Family` / `protected`;
- `FamORAssem` / `protected internal`;
- `FamANDAssem` / `private protected`;

are only meaningful when FerrumWeave is explicitly projecting a managed inheritance contract.

A future CLR accessibility override may narrow an externally-public Rust member to a protected CLR member when a supported class-inheritance projection requires it, but the Rust source remains valid Rust and the projection must never widen the Rust-authorized access domain.

Rust traits should normally project through the relevant interface/trait interoperability contract rather than being forced into a class-inheritance accessibility model solely to imitate C# syntax.

## Exportability and accessibility are separate

An item can be visible in Rust yet still lack a supported CLR representation.

Therefore FerrumWeave distinguishes:

1. **Rust visibility** — what Rust semantics permit;
2. **CLR exportability** — whether FerrumWeave currently knows how to represent the item correctly in CTS/metadata;
3. **CLR accessibility** — how visible that projected representation is to managed consumers.

A supported externally-reachable Rust item defaults to CLR `public`. An unsupported externally-reachable Rust item must fail or remain unprojected according to an explicit contract; it must not be represented with misleading metadata merely to appear exported.

This distinction replaces the earlier overly broad idea that `pub` and CLR `public` are generally unrelated. They are related by default, but only after rustc effective visibility and FerrumWeave exportability are known.

## Reserved generated identities

FerrumWeave-generated public or runtime-facing identities may reserve part of the CLR namespace/type/member space.

For executable projects, the canonical entry facade currently reserves:

```text
<RootNamespace>.Program.Main
```

A user projection that would collide with a reserved identity must produce a diagnostic or require an explicit accepted override. FerrumWeave must not silently merge unrelated user types/members into compiler-owned generated types.

## Configuration overrides

R10 does not require inventing new Rust syntax for CLR naming/accessibility overrides.

When exact compatibility requires an override, project-level metadata in `.rsproj` is an acceptable initial mechanism because it leaves Rust source valid and keeps interoperability policy at the project/CLR boundary.

The final override schema is deliberately deferred until an executable compatibility case requires it. Any future mechanism must remain deterministic, source-traceable, and subject to the no-widening visibility invariant.

## Required contracts

R10 introduces two explicit contract families from this ADR.

### `FW-R10-CLR-NAMING-001`

Must prove at minimum:

- Rust source naming remains unchanged;
- Rust semantic `main` projects to the accepted managed `Main` entry facade rather than being textually renamed;
- snake-case Rust members receive deterministic .NET-facing names;
- nested module identities receive deterministic CLR namespace names;
- raw identifiers do not leak Rust escape syntax into CLR names;
- case-insensitive and transformation-induced public collisions fail diagnostically;
- reserved generated identities cannot be silently occupied by user projection;
- independent C#, VB, and F# consumers can address the accepted projected identities without ambiguity.

### `FW-R10-CLR-VISIBILITY-001`

Must prove at minimum:

- externally reachable Rust `pub` projects to CLR `public` by default when exportable;
- syntactically `pub` but externally unreachable items do not become CLR public merely because of their declaration token;
- `pub use` / re-export effective visibility is honored;
- `pub(crate)` projects to CLR `internal` by default;
- `pub(super)`, `pub(in ...)`, and private items do not become external CLR API;
- an explicit CLR override cannot widen Rust-authorized visibility;
- CLR protected-family accessibility is not inferred from Rust module visibility and is only emitted under an explicit managed inheritance contract.

## Consequences

Positive consequences:

- Rust remains idiomatic Rust rather than C# semantics disguised in Rust syntax;
- normal `pub` APIs naturally become normal public .NET APIs when they are truly part of the effective Rust public surface;
- `pub(crate)` gains a natural CLR `internal` meaning;
- Rust privacy and re-export rules stay owned by rustc instead of being reconstructed imperfectly by FerrumWeave;
- public naming becomes safe for C#, Visual Basic, F#, reflection, and tooling;
- public compatibility cannot be silently changed by compiler mangling;
- CLR inheritance-specific access remains possible without contaminating the Rust language model.

Costs and constraints:

- the backend must carry rustc effective-visibility information into projection;
- public naming/visibility become compatibility surfaces that need exact regression evidence;
- the collision detector must consider supported-language interoperability, not only raw CLR validity;
- some restricted Rust visibility cannot be represented one-for-one in CLR metadata and must remain a Rust semantic restriction plus non-public lowering policy;
- explicit naming/accessibility override syntax remains future work.

## Non-goals

This ADR does not define:

- the final `.rsproj` schema for naming/accessibility overrides;
- full class inheritance projection semantics;
- trait/interface projection details;
- overload resolution policy;
- every Unicode normalization rule beyond the requirement that projected public identities be deterministic and collision-safe;
- arbitrary CLS compliance beyond the supported-language contracts FerrumWeave explicitly claims.

The durable rule is:

> **Rust decides source semantics and authorized reachability; FerrumWeave projects that authority into an idiomatic, deterministic, non-widening CLR surface.**
