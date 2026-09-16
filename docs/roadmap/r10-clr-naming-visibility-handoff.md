# R10 CLR naming and visibility handoff

Status: **active design/verification handoff** for the CLR surface lane.

Read together with:

1. `docs/architecture/adr/0005-r10-rust-to-clr-identity-projection.md`
2. `docs/architecture/adr/0006-r10-clr-naming-and-visibility-policy.md`
3. `tests/r10/contracts.toml`
4. `docs/roadmap/r10-agent-handoff.md`

ADR 0006 amends the earlier broad ADR 0005 visibility wording. The active rule is based on **rustc effective visibility**.

## Default naming model

Do not rewrite Rust source names to imitate C#.

Maintain conceptually separate identities:

```text
RustIdentity  -> semantic source identity owned by rustc
ClrIdentity   -> deterministic .NET-facing metadata identity
InteropKey    -> normalized comparison identity for collision checks
InternalSymbol-> compiler-owned implementation identity; stable mangling allowed
```

Default projection examples:

```text
controllers             -> Controllers
weather_api             -> WeatherApi
servicio_de_clima       -> ServicioDeClima
obtener_pronostico      -> ObtenerPronostico
```

Do not guess acronym intent. Prefer deterministic projection plus an explicit future override.

`main` is semantic. Resolve the Rust entry through rustc and project it to the accepted CLR entry facade; do not scan source text and do not ask users to write `Main`.

## Collision policy

Public CLR API must not be silently mangled.

A projected public collision is a compile-time diagnostic when any of these occurs:

- two Rust identities collapse under the naming transformation;
- identities differ only by case in a way that is ambiguous to a supported language such as Visual Basic;
- a projected identity collides with a reserved FerrumWeave identity such as `<RootNamespace>.Program.Main`;
- a keyword/interoperability constraint makes the public API ambiguous or impractical for an officially supported external language.

Diagnostics should identify both Rust identities and the resulting CLR identity.

Private/compiler-owned implementation symbols may use stable mangling because they are not public compatibility surface.

## Visibility mapping

Use rustc effective visibility, not the declaration token alone.

```text
Rust effective accessibility       Default CLR accessibility
----------------------------------------------------------------
externally reachable pub           Public / public
pub(crate)                          Assembly / internal
pub(super)                          non-public implementation
pub(in path)                        non-public implementation
private / pub(self)                 non-public implementation
```

A syntactically `pub` item behind a private ancestor is not automatically CLR public.

A public re-export must be evaluated through the effective Rust public surface.

`pub(crate)` is the natural default mapping to CLR `internal`.

Restricted Rust visibility is more granular than CLR namespaces. Preserve the Rust rule through rustc; choose only the minimum CLR non-public accessibility needed by emitted implementation topology.

## No-widening invariant

Any explicit CLR accessibility projection may narrow Rust-authorized reachability but must never widen it:

```text
CLR-authorized accessibility ⊆ Rust-authorized accessibility
```

Examples:

```text
Rust externally public -> public/internal/private      allowed narrowing
Rust pub(crate)         -> internal/private             allowed
Rust pub(crate)         -> public                       forbidden
Rust private/restricted -> public                       forbidden
```

## Protected-family rule

Do not invent Rust syntax for `protected`.

CLR `Family`, `FamORAssem`, or `FamANDAssem` only belong to an explicit managed inheritance projection. They are never inferred from Rust modules or `pub(super)`/`pub(in ...)`.

Traits should follow their interface/trait projection contract rather than being forced into C# class-inheritance semantics.

## Smallest RED slices

### `FW-R10-CLR-NAMING-001`

Establish executable failures/proofs for:

1. semantic Rust `main` -> accepted CLR `Program.Main` facade;
2. `obtener_pronostico` -> deterministic `ObtenerPronostico`;
3. nested module -> deterministic namespace segment;
4. raw identifier escape syntax does not leak into CLR metadata;
5. two source identities that collapse to the same projected name fail diagnostically;
6. case-only public ambiguity fails before a VB consumer sees it;
7. reserved entry-facade collision fails diagnostically;
8. accepted surface is consumable by independent C#, VB, and F# projects.

### `FW-R10-CLR-VISIBILITY-001`

Establish executable failures/proofs for:

1. externally reachable `pub struct ApiDeClima` -> CLR public;
2. `mod hidden { pub struct X; }` does not expose public CLR `X` solely because of syntactic `pub`;
3. `pub use hidden::X` changes effective reachability and therefore the projection decision;
4. `pub(crate)` -> CLR internal;
5. `pub(super)` / `pub(in ...)` / private do not become external API;
6. an attempted override from `pub(crate)` to CLR public fails;
7. protected-family metadata cannot be requested without an accepted managed inheritance projection.

## Implementation boundary

Principal Backend owns:

- consuming rustc semantic identity/effective visibility;
- deterministic naming projection;
- collision detection before metadata emission;
- CLR accessibility selection;
- no-widening enforcement.

Principal Infrastructure owns:

- transporting project/root namespace and future explicit overrides through `.rsproj`/MSBuild without changing Rust syntax;
- package/template reproducibility.

Principal Systems / Verification owns:

- independent metadata/reflection assertions;
- C#/VB/F# consumers;
- collision/no-widening falsifiers;
- exact-SHA PASS/FAIL/UNKNOWN classification.

## Stop conditions

Do not accept a GREEN claim if:

- codegen decides CLR public from the syntactic `pub` token without effective-visibility evidence;
- a private/restricted Rust item is widened for CLR convenience;
- a public collision is resolved by silent suffix/hash mangling;
- `protected` is inferred from Rust module hierarchy;
- naming is only reflection-valid but ambiguous to an officially supported .NET language;
- source is changed to non-Rust syntax to obtain CLR accessibility or naming.

The durable rule is: **Rust decides semantic identity and authorized reachability; FerrumWeave projects those decisions into a deterministic, cross-language-safe, non-widening CLR surface.**
