# CLR object-model conformance plan

Status: **accepted planning denominator; implementation remains milestone-driven**.

This plan converts the 40-question [`CLR/OOP senior-engineer examination`](../quality/clr-oop-senior-engineer-exam.md) into dependency-ordered capability families. It does **not** make every senior-level CLR capability a blocker for R10 / `0.1-alpha`.

The architectural answers are already fixed by ADRs 0005-0008. This document answers a different question:

> In what dependency order should FerrumWeave turn those answers into executable product contracts, and which prerelease actually needs each slice?

## Governing rules

1. **No semantic gaps.** The examination may contain `CERTIFIED`, `DEFINED-BUT-RED`, and explicit `UNSUPPORTED-BY-DESIGN` subcases, but no `UNDEFINED` answers.
2. **Capability families are not release blockers by themselves.** A prerelease takes only the smallest slice of a family required to prove that release's advertised workflows.
3. **A GREEN primitive does not certify a broader family.** Existing R04-R07 evidence is reused, but broader object-model claims remain RED until independently proven.
4. **Real CLR metadata is the acceptance surface.** Public projection is inspected by ordinary reflection/tooling and consumed by independent .NET code where appropriate.
5. **Source causality remains mandatory.** Product behavior must flow through Rust source -> `rustc` -> FerrumWeave CodegenBackend -> CLR metadata/CIL -> CoreCLR.
6. **Unsupported-by-design behavior must fail clearly.** A stable diagnostic is a valid product outcome for a deliberately unsupported shape.
7. **The 1.0 stable object-model gate is stricter than any individual prerelease.** Before 1.0, every baseline exam question must either be certified for its accepted semantics or contain only explicitly unsupported, diagnostically enforced subcases.

---

# Capability DAG

FerrumWeave's 35 RED senior-exam answers collapse into eight root capability families.

```text
FW-CLR-OM-001  Identity / naming / accessibility
        |
        +------------------------------+
        |                              |
        v                              v
FW-CLR-OM-002  Type kind / members   FW-CLR-OM-008  Metadata adornments / verification
        |
        +-------------------+
        |                   |
        v                   v
FW-CLR-OM-003  Inheritance /       FW-CLR-OM-004  Signature algebra /
                interface dispatch                 overloads / generics
        |                   |                       |
        +---------+---------+-----------------------+
                  |
                  v
FW-CLR-OM-005  Delegates / events / operators
                  |
                  +------------------+
                  |                  |
                  v                  v
FW-CLR-OM-006  Boundary semantics   FW-CLR-OM-007  Async / Task adaptation
(error/lifetime/concurrency)          (also depends on OM-004 and OM-006)
```

The arrows describe architectural dependency, not mandatory release sequencing. A milestone may certify one narrow vertical slice through several families before the complete families are GREEN.

---

## `FW-CLR-OM-001` — Public identity, naming, exportability, accessibility

**Purpose:** make every public Rust->CLR identity deterministic, non-widening, cross-language addressable, and independently inspectable.

**Exam coverage:** Q01-Q08, with Q04 already GREEN for existing R06 shapes.

**Existing authority:** ADR 0005, ADR 0006; R10 contracts `FW-R10-CLR-ENTRY-001`, `FW-R10-CLR-SURFACE-001`, `FW-R10-CLR-NAMING-001`, `FW-R10-CLR-VISIBILITY-001`.

**Slices:** semantic Rust entry point -> CLI `Program.Main`; project/module/member identity; naming/collision diagnostics; rustc effective visibility; public re-exports; protected-family access only after a real OM-003 family relation; C#/VB/F# addressability and reflection.

**Release need:** **R10 / 0.1 blocker for its advertised public surfaces.** This is already part of R10's critical path and must not be duplicated under a second R10 ledger.

## `FW-CLR-OM-002` — Type representation and ordinary member surface

**Purpose:** define and certify the ordinary CTS shape of Rust-owned exported state without changing Rust ownership semantics.

**Exam coverage:** Q10-Q12 and Q21-Q24; reuses Q09 CTS foundation and Q37 nullability primitives.

**Required slices:** default Rust struct -> sealed CLR reference class; explicit safe value-type projection; supported fieldless Rust enum -> explicit CLR enum; `System.Object` behavior; static/instance methods; explicit `.ctor`; explicit properties/indexers; explicit fields; boxing/unboxing; nullability annotations consistent with Option/non-Option semantics.

**Dependencies:** OM-001 plus R04 CTS primitives and ADR 0007 lifetime rules.

**Release need:** **0.1** only needs ordinary reference-class/static-instance/constructor/member slices required by classlib and test projects; **0.2** takes richer properties/member shapes for MVC/WinForms; **1.0** requires the complete accepted family including value-type/enum/boxing rules.

## `FW-CLR-OM-003` — Class inheritance, virtual slots, interfaces, polymorphic dispatch

**Purpose:** make CLR dispatch metadata exact rather than name-driven.

**Exam coverage:** Q13-Q20 and the inheritance-dependent portion of Q07.

**Required slices:** sealed/non-virtual defaults; explicit one-base-class projection and base constructor; exact `Virtual + NewSlot`; exact inherited-slot override; sealed override; deriving from existing CLR abstract bases; projectable trait -> interface; multiple interfaces; implicit and explicit `MethodImpl` bindings; `dyn Trait` -> certified CLR interface reference; protected-family accessibility only for actual inheritance.

**Unsupported baseline:** multiple class inheritance; automatic Rust-owned abstract CLR class synthesis; automatic trait default bodies as CLR default-interface implementations.

**Dependencies:** OM-001 and OM-002.

**Release need:** **0.1** only the interface/slot subset required by the chosen ASP.NET Core hosting seam; **0.2** class inheritance/virtual dispatch becomes a release capability for MVC/WinForms; **1.0** requires the complete accepted family.

## `FW-CLR-OM-004` — Signature algebra, overloads, CLR generics, constraints, variance

**Purpose:** make method/type identity depend on full CLR signatures and emit real reified generic metadata where public CLR generics are claimed.

**Exam coverage:** Q25-Q28.

**Required slices:** exact-signature CLR overload consumption; explicit produced overload groups; accidental naming collapse remains an error; real CLR generic types/methods; supported bounds -> CLR constraints; non-equivalent bounds are not silently erased; invariance by default; explicit legal covariance/contravariance; reflection plus C#/VB/F# consumption.

**Dependencies:** OM-001 and OM-002; variance additionally depends on the relevant OM-003 interface or OM-005 delegate shape.

**Release need:** exact-signature slices may arrive earlier as frameworks demand them; **0.5 gRPC** makes richer generic/signature support materially important; **0.6 Blazor** broadens generic/variance needs; **1.0** requires the complete accepted family.

## `FW-CLR-OM-005` — Delegates, events, operators and conversions

**Purpose:** bridge CLR callable/event semantics without pretending Rust `Fn*`/operator traits are automatically identical.

**Exam coverage:** Q29-Q32.

**Required slices:** consume CLR delegates; explicit non-capturing Rust function -> delegate; safe owned-capture closures; borrowed escaping capture rejection; reusable `Fn`; synchronized/exclusive `FnMut`; `FnOnce` rejected as ordinary reusable delegate; real CLR event metadata/add-remove; explicit compatible operators/conversions and checked variants only when separately certified.

**Dependencies:** OM-002; OM-003 as required; OM-004 for generic delegates/variance; OM-006 for closure lifetime/thread-safety.

**Release need:** **0.2 WinForms** and **0.4 WPF** make delegates/events release-relevant; **0.6 Blazor** uses broad callback interoperability; **1.0** requires the complete accepted family.

## `FW-CLR-OM-006` — Error, resource, lifetime, aliasing and concurrency boundary

**Purpose:** preserve Rust semantic authority when CLR storage, exceptions, aliases and threads cross the boundary.

**Exam coverage:** Q33, Q34, Q36, Q38, Q39 and cross-cutting portions of Q21/Q30/Q35/Q37.

**Existing GREEN primitives:** R07 `Option`, `Result`, panic rejection, `Drop`/`IDisposable`, managed identity, borrow-boundary rejection.

**Required broader slices:** typed exception projection; stable fallback exception; general panic containment or explicit rejection; the three lifetime domains; exactly-one cleanup; no automatic finalizer; smart-pointer semantics or rejection; `&mut self` managed-entry exclusivity; `Send`/`Sync` or explicit synchronization/affinity adapters; thread-affine enforcement.

**Dependencies:** OM-001/OM-002; specific adapters may depend on OM-003/OM-005.

**Release need:** **0.1** only safety slices actually exercised by classlib/test/web/webapi; web hosting must not become unsound merely to reach T1. Later frameworks exercise progressively more of this family. **1.0** requires the complete accepted family.

## `FW-CLR-OM-007` — Rust async/Future to managed Task

**Purpose:** expose asynchronous Rust behavior through ordinary .NET Task semantics without leaking Rust poll/vtable machinery.

**Exam coverage:** Q35, reusing Q33/Q34 error/panic semantics.

**Required slices:** `Future<Output=T>` -> `Task<T>`; unit -> `Task`; `Result` -> completed/faulted task; panic remains distinct; FerrumWeave-owned polling/waker/completion adapter; correct managed rooting/lifetime; explicit cancellation when `CancellationToken` participates; `ValueTask` only by separate explicit projection.

**Dependencies:** OM-002, OM-004 for `Task<T>` signature correctness, OM-006 for lifetime/error semantics.

**Release need:** **0.3 worker** makes Task-based Generic Host/background lifecycle a release capability; **0.5 gRPC** and **0.6 Blazor** make async central; **1.0** requires the accepted family.

## `FW-CLR-OM-008` — Custom attributes, metadata adornments and independent CLR verification

**Purpose:** allow framework-discoverable managed metadata without copying arbitrary Rust syntax into CLR attributes.

**Exam coverage:** Q40 and the verification dimension of Q04; every family consumes OM-008 as its metadata oracle.

**Required slices:** explicit CLR `CustomAttribute` constructor/arguments; no automatic arbitrary Rust-attribute copying by spelling; compiler-owned attributes only for accepted platform contracts; independent reflection; framework discovery proves real meaning; every new OM-002..OM-007 metadata shape gains reflection assertions.

**Dependencies:** OM-001, OM-002, and OM-004 when richer attribute argument/signature types are required.

**Release need:** **R10 / 0.1 release-relevant for xUnit/NUnit/MSTest discovery**; reused by later frameworks; **1.0** requires the complete accepted attribute/reflection policy.

---

# Release slicing

| Release | Object-model slices that may become blockers |
| --- | --- |
| **0.1 alpha / R10** | OM-001 R10 identity/naming/visibility; OM-002 ordinary reference class/static-instance/ctor slices needed by classlib/tests; OM-003 interface subset for selected ASP.NET host; OM-006 safety subset exercised by those crossings; OM-008 test-framework attributes/discovery. **No requirement to finish general inheritance, value types, delegates/events, public generics or async.** |
| **0.2 alpha** | OM-002 properties/member surface for MVC/WinForms; OM-003 base-class/virtual/override dispatch; OM-005 delegates/events for WinForms; framework metadata through OM-008. |
| **0.3 beta** | OM-007 Task adaptation for Generic Host/background lifecycle plus required OM-003/OM-006 host semantics. |
| **0.4 beta** | WPF-specific property/event/delegate/attribute/reflection slices across OM-002/OM-005/OM-008, plus any thread-affinity slice from OM-006. |
| **0.5 beta** | gRPC drives richer OM-004 signatures/generics and OM-007 async, plus generated metadata/build integration as required. |
| **0.6 beta** | Blazor drives broad OM-004 generics/variance, OM-005 delegates/callbacks, OM-007 async and OM-008 metadata/reflection. |
| **1.0 stable** | Every senior-exam baseline question is either `CERTIFIED` for accepted semantics or contains only an explicitly `UNSUPPORTED-BY-DESIGN` subcase whose rejection is executable and diagnostic. No baseline `DEFINED-BUT-RED` remains. |

This table is a dependency map, not permission to defer a capability that an earlier vertical slice empirically requires. When a RED proves a supposedly later primitive is needed earlier, the required narrow slice moves earlier; the whole root family does not automatically move with it.

# Contract activation policy

The eight `FW-CLR-OM-*` identifiers are stable **capability-family IDs**, not replacements for milestone-local executable contract IDs.

When a slice becomes active work:

1. add an executable contract to the owning milestone ledger (`tests/r10/contracts.toml` for R10, or the future milestone ledger that owns it);
2. reference the relevant `FW-CLR-OM-*` family and exact exam question(s);
3. state observable and falsifiers before implementation;
4. keep `implemented = false` until source-causal evidence exists;
5. certify Windows/Linux whenever the claimed CLR behavior is portable;
6. promote an exam row only when the row's broader accepted semantics, not merely one narrower primitive, are genuinely covered.

Do **not** create dozens of placeholder active contracts merely because the architecture is defined. The planning denominator is known; the executable denominator grows only as slices enter a real milestone.

# Dependency-compression opportunities

Highest-leverage implementation seams are:

1. **OM-001 projection context** — one identity/accessibility service for entry, namespaces, naming, visibility and collisions.
2. **OM-002 projected type/member descriptor** — one model feeding class/value/enum, constructors, properties, fields and reflection instead of independent emitters.
3. **OM-003 slot binder** — exact base/interface slot resolution unifying interfaces, `newslot`, override/final and family accessibility.
4. **OM-004 signature algebra** — exact signatures unifying overload selection, generics, constraints, variance and cross-language collision checks.
5. **OM-006 boundary guard** — reusable managed-entry safety for panic containment, rooting, mutable exclusivity and thread-safety.
6. **OM-008 metadata verifier** — reusable independent .NET verification for every new metadata shape instead of one-off probes.

# Senior-level stable gate

FerrumWeave 1.0 does not need to implement every C# language feature. It does need a predictable CLR object model.

```text
for every Q01..Q40:
    CERTIFIED(accepted semantics)
    OR
    UNSUPPORTED-BY-DESIGN + certified diagnostic

and never:
    DEFINED-BUT-RED
    UNDEFINED
```

A senior CLR engineer should be able to inspect Rust source plus FerrumWeave projection configuration and predict public metadata, dispatch, accessibility, type kind, lifetime, exception, generic, async, delegate/event and reflection behavior without guessing.
