# R10 web / webapi hosting handoff — interface-first ASP.NET Core

Status: **active design + implementation handoff** for the `0.1-alpha` `web` and `webapi` template families.

This document is a narrow continuation guide for an agent entering the ASP.NET Core lane. Read `AGENTS.md` and `docs/roadmap/r10-agent-handoff.md` first. Executable contracts and exact-SHA CI evidence outrank this note.

## Why this lane exists

FerrumWeave should not grow a compiler intrinsic for every convenient ASP.NET Core surface API (`WebApplication.CreateBuilder`, `MapGet`, `Run`, and so on) merely to make the first web template run.

The .NET framework already exposes hosting contracts beneath those convenience APIs. The first T1 implementation should prefer a stable framework seam and generalize FerrumWeave's existing CLR-interface machinery instead of encoding one fluent API call at a time.

For .NET / ASP.NET Core 10, the first candidate seam is:

- `Microsoft.AspNetCore.Hosting.IStartup`
  - `IServiceProvider ConfigureServices(IServiceCollection services)`
  - `void Configure(IApplicationBuilder app)`
- `WebHostBuilderExtensions.UseStartup<TStartup>(IWebHostBuilder)` remains a supported hosting integration point.

Other official seams may be better if the RED proves they require less new FerrumWeave semantics. In particular, investigate before committing to an architecture:

- `IHostingStartup`;
- `IWebHostBuilder` / hosting infrastructure contracts;
- `WebHost.Start(RequestDelegate)` for the smallest real HTTP host;
- reusable middleware/host-bridge infrastructure supplied by FerrumWeave.

`IStartup` is the current **first candidate**, not a public syntax commitment.

## Existing FerrumWeave leverage

Do not start from the assumption that CLR interface projection is absent.

R07 already certifies Rust-source-causal interface projection for a specialized case:

```text
Rust struct + Drop
  -> FerrumWeave MIR lowering
  -> CLR TypeDef + InterfaceImpl(System.IDisposable)
  -> Dispose method
  -> managed DLL
  -> independent C# consumer casts to IDisposable
  -> CoreCLR observable changes when only Rust source changes
```

Relevant evidence:

- `compiler/codegen-backend/src/disposable_resource_lowering.rs`
- `compiler/cil/src/disposable_resource_export.rs`
- `tests/r07_disposable.rs`
- `tests/r07/contracts.toml` (`FW-R07-SEM-007`)

That implementation is intentionally specialized to `Drop -> IDisposable`; do **not** copy its hard-coded type/member names into ASP.NET support. Treat it as proof that FerrumWeave already knows how to emit CLR `InterfaceImpl` metadata and interface-consumable managed types.

## Shared foundation for `web` and `webapi`

`web` and `webapi` are distinct standard .NET template families, but their T1 runtime foundation should be shared as much as possible.

Template identities:

```text
web     -> shortName web    -> groupIdentity Microsoft.Web.Empty
webapi  -> shortName webapi -> groupIdentity Microsoft.Web.WebApi
```

The common T1 foundation is:

```text
.rsproj
  -> FerrumWeave.Sdk
  -> rustc
  -> FerrumWeave CodegenBackend
  -> Rust-defined managed application/startup behavior
  -> normal ASP.NET Core hosting/Kestrel
  -> real HTTP request
  -> Rust-source-causal HTTP observable
```

Do not build two unrelated hosting implementations unless executable evidence demonstrates a real framework requirement.

## Evidence levels

### T0 — scaffold

Each family independently proves:

```console
dotnet new web -lang Rust
dotnet new webapi -lang Rust
```

and preserves the built-in language variants applicable to that family.

T0 may install the repository-local template during development. T0 is **not** release support.

### T1 — real framework behavior

A template is prerelease-supportable only after a generated Rust project:

1. builds through the FerrumWeave product backend;
2. starts the normal ASP.NET Core host/Kestrel;
3. accepts an independent HTTP request;
4. produces an observable whose application-specific value is causally owned by Rust source;
5. changes both the managed artifact and HTTP observable when only Rust application source changes;
6. passes Windows and Linux;
7. passes through the installed/distributable product boundary before a release claim.

A reusable FerrumWeave managed host/bootstrap is allowed infrastructure if it is application-agnostic. It must not contain or generate the application-specific answer used as causal proof. Generated per-project C# is not valid substitution for Rust application behavior.

### T2 — idiomatic Rust framework experience

T2 is where direct Rust-facing ergonomics belong, including whichever abstractions ultimately represent:

- `WebApplication` / host builders;
- endpoint routing;
- delegates/closures;
- async request handlers;
- typed results / JSON conveniences;
- middleware composition.

Do not block T1 on implementing the entire Minimal API convenience surface.

## Family-specific T1 observables

The host/bootstrap foundation may be shared, but evidence should still make the generated family meaningful.

Suggested minimal observables:

- `web`: a real root HTTP response whose payload is sourced from Rust;
- `webapi`: a real API endpoint (for example `/health`) with a deterministic status/payload sourced from Rust and an API-appropriate response shape/content type.

These examples are test observables, not permanent public API design.

## Next RED — do this before broad implementation

Do **not** begin by adding `webapplication_*` compiler intrinsics.

The next agent should first determine the smallest framework-interface projection missing between certified R07 and an ASP.NET hosting seam.

Recommended sequence:

1. Fresh-read `dev`, PR #36, PR #57, and any newer overlapping web/template PR.
2. Re-read `FW-R07-SEM-007` and the specialized `IDisposable` lowering/emitter.
3. Inspect the exact .NET 10 reference signatures of `IStartup` and competing official hosting seams.
4. Write a narrow RED that demonstrates the first missing *general* CLR capability, not the final fluent ASP.NET syntax.
5. Prefer an internal lowering/emission contract before inventing new public Rust syntax.
6. If implementing arbitrary external CLR interfaces requires a new Rust-visible annotation/syntax choice, stop and record the design decision/ADR instead of smuggling syntax into a framework-specific patch.
7. Once an external interface implementation can be emitted causally from Rust, RED the actual ASP.NET host bridge and real HTTP observable.

Likely capability gaps to falsify explicitly:

- arbitrary external `TypeRef` as `InterfaceImpl`, rather than hard-coded `System.IDisposable`;
- interface method signatures containing managed reference parameters/returns (`IServiceCollection`, `IServiceProvider`, `IApplicationBuilder`);
- a reusable managed entry point/host bridge;
- the smallest request-handler/middleware bridge needed to turn a Rust-owned value into an HTTP response;
- disposal/lifetime of the running host.

Do not assume every item is missing; prove the first missing capability with a RED.

## Current `webapi` sub-lane

PR #57 / branch `feat/r10-webapi` currently owns only the `webapi` family-local increment.

Its staged T0 contract is `FW-R10-TPL-WEBAPI-001` in `tests/r10/contracts.toml`. The branch contains:

- `sdk/templates/webapi/.template.config/template.json`;
- `sdk/templates/webapi/RustWebApi.rsproj`;
- `sdk/templates/webapi/src/main.rs`;
- `tests/r10_webapi_template.rs`;
- the R10 ledger update.

The branch intentionally keeps `implemented = false`: T0 scaffolding does not satisfy T1.

Before changing it, synchronize from the live PR #36 head without restoring intentionally retired Python/oracle witnesses.

## Things the next agent must not do

- Do not reintroduce Python verifiers merely because an old branch once contained them; follow current authority-cutover evidence.
- Do not duplicate the active test-template lane's work.
- Do not claim `webapi` support from scaffolding alone.
- Do not generate application C# as a substitute for Rust causal behavior.
- Do not hard-code ASP.NET convenience methods into the compiler before testing the framework-interface route.
- Do not invent public Rust syntax for CLR interfaces inside a framework patch without an explicit architectural decision.
- Do not make `web` and `webapi` separate runtime stacks without evidence that their hosting requirements actually diverge.

## Definition of a safe next handoff

Before ending the next iteration, leave repository evidence that answers:

1. Which exact hosting seam was selected, and why was it smaller/safer than the alternatives?
2. What was the first causal RED between R07 `InterfaceImpl` support and that hosting seam?
3. Which general CLR capability became GREEN?
4. Which exact SHA and Windows/Linux CI runs certify it?
5. What remains RED before a real HTTP request can succeed?
6. Is the work shared by `web` and `webapi`, or genuinely family-specific?

If those answers exist only in chat, the lane is not ready to hand off.