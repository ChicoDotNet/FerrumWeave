# ADR 0008 — Define a conservative Rust-to-CLR object model without inventing a C#-like Rust dialect

- Status: Accepted for the cross-runtime object-model roadmap
- Date: 2026-09-16
- Active context: R10 developer experience plus post-R10 CLR object-model conformance
- Builds on: ADR 0005 identity projection, ADR 0006 naming/visibility, ADR 0007 lifetime/GC/resource ownership

## Context

FerrumWeave already proves important vertical slices: CTS scalar/reference mappings, calls into managed APIs, Rust-defined static and instance members consumed from .NET, reflection, `Option<T>`/nullability, `Result<T,E>`/exceptions, `Drop`/`IDisposable`, managed identity/lifetime, and rejection of unsupported borrow escapes.

Those slices do not by themselves define the complete object model a senior .NET/CLR engineer expects. The CLR distinguishes reference types and value types, classes and interfaces, base classes, virtual slots, overrides, abstract/final members, fields, properties, events, delegates, overloads, generic definitions/constraints/variance, boxing, custom attributes, and normal metadata/reflection behavior.

FerrumWeave must answer those questions without turning Rust into C# syntax and without inventing semantics from names.

This ADR closes that semantic gap. Exact configuration syntax may evolve, but the meaning of each projection is fixed here. A deferred `.rsproj` schema is not a deferred semantic decision.

## Decision principles

1. **Rust remains Rust.** Parsing, type checking, ownership, borrowing, traits, moves, and Rust method bodies remain under `rustc`.
2. **CLR projection is explicit when it adds a concept Rust does not natively have.** Class inheritance, CLR virtual slots, properties, events, overload groups, delegates, reified CLR generics, and custom attributes are never inferred solely from spelling.
3. **Conservative defaults outrank magical ergonomics.** The default projection must not create copying, inheritance, aliasing, dispatch, lifetime, or thread-safety semantics that Rust did not authorize.
4. **Supported projections are ordinary CLR metadata.** C#, Visual Basic, F#, reflection, frameworks, and tooling must not require a FerrumWeave-specific lookup protocol.
5. **Unsupported public shapes fail diagnostically.** FerrumWeave does not emit misleading metadata merely to make compilation succeed.
6. **Existing certified slices remain exact evidence.** This ADR does not retroactively generalize R04-R07 tests beyond their proven shapes.

## Type projection kinds

### Default Rust struct projection

An externally visible Rust `struct` that is exported as an object-like CLR type projects by default as a **sealed CLR reference class**.

The default is deliberately not a CLR value type because CLR value types introduce implicit copying/boxing semantics and a Rust struct may own non-copy state, have `Drop`, contain invariants, or rely on ownership in ways that would be changed by unmanaged copying from C#/VB/F#.

Conceptually:

```text
public Rust struct
        |
        v
sealed CLR class : System.Object
```

The reference-class projection does not make CLR aliasing legal Rust aliasing. Managed entry adapters must still preserve Rust safety and ADR 0007 lifetime rules.

### Explicit CLR value-type projection

A Rust struct may project as a CLR value type only through an explicit value-type projection and only when the compiler can prove the accepted boundary requirements.

The minimum policy is:

- the Rust type is `Copy` for the supported public value projection;
- the type does not implement `Drop`;
- no exported representation contains a borrow/reference whose lifetime would be erased by CLR copying;
- every projected field has a supported CLR value representation;
- copying, boxing, unboxing, and default construction cannot duplicate a logically unique resource;
- the exact projected fields/layout semantics are part of the contract rather than inferred from native Rust ABI layout.

`#[repr(C)]`, `#[repr(transparent)]`, or another Rust representation attribute does not by itself mean "CLR value type".

### Rust enums

A fieldless Rust enum may project as a CLR enum only when it has a stable supported integer discriminant representation and the public projection explicitly selects CLR-enum semantics.

A data-carrying Rust enum does **not** project as a CLR enum. Until a dedicated discriminated-union projection is accepted, exporting such an enum directly as a normal CLR enum is rejected diagnostically. Consumers may use an explicit wrapper/API projection instead.

### CLR nested types

Rust modules project to namespaces, not nested CLR types. FerrumWeave 1.0 does not automatically synthesize Rust-defined nested CLR `TypeDef`s from module nesting. Existing nested CLR types may still be consumed by their metadata identity. A later explicit nested-type feature may be added without changing the default module-to-namespace rule.

## System.Object, equality, hashing, stringification, boxing

A projected reference class derives from `System.Object` unless an explicit supported CLR base class is selected.

FerrumWeave does **not** automatically translate:

- `PartialEq` / `Eq` into `Object.Equals`;
- `Hash` into `Object.GetHashCode`;
- `Debug` or `Display` into `Object.ToString`.

Those traits have Rust semantics that are not identical to the corresponding CLR virtual members. A future explicit projection may bind compatible implementations.

Without such an override, a reference-class projection uses normal CLR object identity/equality/hash/stringification behavior inherited from `System.Object`.

An explicit CLR value-type projection participates in normal CLR boxing/unboxing. Boxing creates the CLR boxed representation of the value; unboxing requires the exact compatible value type. FerrumWeave must not treat boxing as Rust ownership transfer or as permission to recover an expired Rust reference.

## Class inheritance

### Default is sealed and non-extensible

Because ordinary Rust structs do not declare class inheritance, a default projected reference class is `sealed`/`final` from the CLR perspective.

This prevents a managed consumer from creating a derived type whose behavior Rust never modeled.

### Explicit single CLR base class

A Rust type may derive from one supported CLR base class only through an explicit managed base-type projection. The projection must define and verify:

- the exact CLR base type identity;
- the selected base constructor and argument mapping;
- initialization ordering;
- inherited virtual slots that are overridden;
- any required protected-family accessibility;
- lifetime/ownership rules for Rust state attached to the derived managed object.

The CLR supports one class base type; FerrumWeave therefore does not support multiple class inheritance.

A type with an explicit managed base projection may remain sealed or may explicitly opt into further CLR derivation.

### Rust-owned abstract CLR classes

FerrumWeave 1.0 does not infer a Rust-owned abstract CLR class from a Rust trait or struct. Rust-owned polymorphic abstraction defaults to CLR interfaces.

Consuming an existing CLR abstract base class and deriving a Rust-backed managed type from it is an intended feature under the explicit base/override projection above.

Creating a new Rust-owned abstract CLR base class is **unsupported by design for the 1.0 object-model baseline**. It may be introduced later through a separate accepted projection contract rather than by guessing from traits.

## Virtual dispatch, new slots, overrides, and final members

Ordinary projected Rust methods are **non-virtual by default**.

FerrumWeave never infers virtuality from method spelling.

When an extensible managed class explicitly declares a new virtual member, FerrumWeave emits a new CLR virtual slot (`virtual` + new-slot semantics).

When a Rust-backed derived type explicitly binds a method to an inherited CLR virtual member, FerrumWeave emits a true override that reuses the inherited slot rather than creating a same-named new slot.

A sealed override is emitted as virtual/final according to CLR semantics.

A collision with an inherited method name is never enough to infer override. The exact base member signature/slot must be selected.

## Traits and CLR interfaces

### Projectable trait subset

A public Rust trait may project to a CLR interface when its externally visible contract can be represented faithfully in CLR metadata.

The initial projectable subset requires:

- every exposed parameter/result type has a supported CLR projection;
- receiver semantics are compatible with a managed interface instance call;
- associated types/associated constants do not leak into the CLR contract unless separately projected;
- generic trait methods are not exposed until public CLR-generic method semantics are certified;
- no unsupported lifetime/borrow escapes occur.

A trait that does not meet the projectable subset remains Rust-only or fails if the user explicitly requests a CLR interface projection.

### Multiple interfaces

A projected CLR class may implement multiple projectable Rust traits/interfaces. FerrumWeave emits ordinary CLR interface implementation metadata for each accepted interface.

### Default trait methods

A Rust trait default body is **not automatically emitted as a CLR default interface method**. The CLR interface contract remains an interface contract; Rust default-method semantics may be used when generating a Rust implementation, but managed implementers do not receive a CLR default body unless a future explicit default-interface-method contract says so.

### Implicit and explicit interface implementation

FerrumWeave prefers an ordinary public/implicit interface implementation when one projected method can satisfy the interface member without ambiguity.

When collisions or compatibility requirements demand explicit implementation, FerrumWeave uses an explicit CLR `MethodImpl`-style binding to the exact interface slot. Explicit implementation is never inferred by name alone.

### Trait objects

A `dyn Trait` may cross a managed boundary as the corresponding CLR interface reference only when that trait has a certified CLR-interface projection and the concrete object/lifetime semantics satisfy ADR 0007. Rust vtable representation itself is not exposed as CLR ABI.

## Static and instance members

A Rust associated function with no receiver may project as a CLR static method.

A supported `&self` receiver may project as a normal CLR instance method.

An exported `&mut self` method requires a managed-boundary exclusivity mechanism; CLR callers can otherwise create aliases that Rust would not permit. FerrumWeave must establish exclusive access in the managed adapter or reject the projection. It must not hand multiple simultaneous managed calls unrestricted `&mut self` access.

## Constructors and type initialization

Rust function names such as `new`, `create`, or `default` do not automatically become `.ctor`.

A CLR instance constructor is an explicit projection that must identify the Rust initialization operation and prove that the resulting Rust state is valid.

A public parameterless `.ctor` is emitted only when FerrumWeave has a certified valid zero-argument initialization path. The existing R06 trivial constructible type remains a certified slice; it is not evidence that every Rust struct has a legal default constructor.

For derived managed classes, every emitted `.ctor` must call an accepted base `.ctor` according to CLR rules before exposing the initialized object.

CLR type initializer `.cctor` is compiler-owned/runtime initialization infrastructure unless an explicit type-initialization contract is introduced. FerrumWeave does not map an arbitrary Rust function named `cctor` or `static_init` by convention.

## Properties and indexers

FerrumWeave does not infer properties from names such as `get_x`, `set_x`, `x`, or `setX`.

A CLR property is an explicit projection that emits a normal `Property` metadata row and exact getter/setter accessors. Getter and setter accessibility may differ only within the maximum access authorized by Rust and ADR 0006.

An indexer/default property is likewise explicit and must identify the parameterized property/accessors; it is not inferred from Rust indexing traits by default.

## Fields

A Rust `pub` field does not automatically become a public CLR field. Rust field visibility, Rust layout, move semantics, interior mutability, and CLR public-field semantics are not equivalent.

Rust fields remain implementation state unless an explicit field projection is selected. Public API design should normally prefer explicit methods/properties.

An explicit CLR field projection must define static/instance, mutability (`initonly` where applicable), type mapping, and visibility. Constants/literals require their own exact metadata semantics rather than being guessed from associated constants.

## Overloads

### Consuming .NET overloads

FerrumWeave resolves managed overloads by full metadata signature plus Rust type information, never by method name alone.

### Producing overloads from Rust

Rust source cannot normally declare two same-scope functions with one identical source name and different parameter lists. FerrumWeave may nevertheless expose an intentional CLR overload set by explicitly grouping distinct Rust source identities under one CLR member name when their CLR signatures are distinct and cross-language overload resolution remains valid.

Accidental naming collapse (for example two different Rust names both becoming the same PascalCase CLR name) remains an ADR 0006 collision error. It becomes an overload only through an explicit overload-group projection.

## Public generics and monomorphization

Rust's internal generic implementation remains governed by `rustc`, including monomorphization where Rust semantics require it.

A **public CLR generic type or method** is a different contract: FerrumWeave must emit real CLR generic parameter metadata so managed consumers see one reified generic definition, not a collection of unrelated monomorphized public types/methods.

FerrumWeave must never claim a public CLR generic API merely because Rust instantiated multiple monomorphized versions internally.

If a Rust generic public item cannot be represented by the supported CLR generic subset, export fails diagnostically or requires an explicit non-generic wrapper.

## Generic constraints

Rust bounds are mapped to CLR generic constraints only when semantics are known to correspond.

Examples of rules:

- a projectable trait bound may become an interface constraint;
- an explicit supported CLR base-type requirement may become a base-class constraint;
- Rust `Copy` does not automatically mean CLR `struct` constraint;
- Rust `'static` does not automatically mean C#/CLR `class` constraint;
- Rust `Send`/`Sync` do not become CLR generic constraints because the CLR type system has no equivalent generic constraint;
- unsupported or non-equivalent bounds must not be erased silently from a claimed public generic contract.

## Variance

CLR generic variance applies only where CLR permits it, principally generic interfaces and delegates.

FerrumWeave's default public generic parameter is **invariant**.

Covariance/contravariance is enabled only through an explicit CLR projection after FerrumWeave verifies the parameter occurs only in CLR-valid variance positions. Rust's own inferred variance is not copied mechanically into CLR metadata.

## Delegates and Rust callables

### Consuming CLR delegates

A CLR delegate consumed from Rust remains a managed delegate object. FerrumWeave invokes it through normal CLR delegate invocation semantics and maintains managed liveness for the legal Rust-visible handle.

### Exporting Rust callables as delegates

A Rust callable may project to a CLR delegate only through an explicit delegate contract.

Initial safe rules:

- non-capturing function items/functions with compatible signatures are eligible;
- a capturing closure must own all escaping capture state and satisfy the required `'static`/managed-lifetime boundary;
- borrowed captures that could outlive their Rust borrow are rejected;
- repeatable `Fn` semantics can back a normal reusable delegate;
- `FnMut` requires an explicit exclusivity/synchronization adapter before managed callers may invoke it concurrently;
- `FnOnce` is not projected as an ordinary reusable CLR delegate, because delegate invocation is not intrinsically single-use.

Multicast combination/removal semantics belong to CLR delegate/event objects; FerrumWeave does not pretend that Rust `Fn*` traits natively have multicast semantics.

## Events

A CLR event is not a public delegate field.

Events require an explicit projection that emits normal event metadata plus exact add/remove methods and a supported delegate type. Rust naming alone never turns a field or method pair into an event.

## Operators and conversions

Rust operator traits such as `Add`, `Sub`, `From`, or `Into` are not automatically converted into CLR operator/conversion methods.

A concrete compatible Rust implementation may be explicitly projected to a CLR operator or conversion special-name method (`op_*`) when parameter/result semantics are unambiguous. Checked-operator forms and user-defined conversions require their exact CLR metadata/signature contracts.

## Async: Future to Task

A supported public Rust `async fn` / `Future<Output = T>` projects by default to `System.Threading.Tasks.Task<T>`; unit output projects to `Task`.

`ValueTask` is not the default and requires an explicit projection because it has materially different consumption/lifetime expectations.

FerrumWeave owns the adapter that drives/polls the Rust future and completes the managed `Task`; managed callers do not see Rust poll/vtable internals.

If the async Rust result is `Result<T,E>`, successful completion produces `T` and the error path faults the `Task` according to the managed `Result`/exception policy.

Cancellation is not invented implicitly. A `CancellationToken` participates only when the public Rust/CLR contract explicitly includes cancellation semantics.

## Result, managed exceptions, and panic

### Result

For the managed public-boundary model:

- `Ok(T)` returns/completes with the projected success value;
- `Err(E)` becomes a managed exception, not a hidden status code;
- if `E` has an explicit supported CLR-exception projection, that exception type is used;
- otherwise the compatibility fallback remains a managed `InvalidOperationException` carrying stable FerrumWeave/Rust error identity and any safely projectable diagnostic payload.

The existing R07 `Result<i32,i32>` failure slice remains valid evidence for the fallback direction.

### Panic

A Rust panic must never unwind raw across a managed public boundary.

Today, statically recognized unsupported panic bodies remain rejected by the certified R07 policy.

The general object-model target is: exported managed boundaries contain unwindable Rust panic and translate it to a dedicated FerrumWeave-managed panic exception; configurations that cannot provide a contained managed boundary must reject that export rather than silently allowing cross-runtime unwind. A panic is not treated as an ordinary `Result::Err`.

The exact public exception class name/package is an implementation contract for the later executable slice; the semantic distinction is fixed here.

## Nullability and Option

FerrumWeave preserves the already accepted distinction:

- `Option<T>` over a supported managed reference may represent CLR nullability;
- `Option<T>` over a supported CLR value type may project as `Nullable<T>` where the contract applies;
- a plain non-optional Rust boundary value is not silently made nullable merely because CLR references can physically contain null.

Nullable annotations/attributes used by C#/VB/F# tooling must reflect this semantic policy rather than inventing nullability from reference-type shape alone.

## Lifetime, GC, smart pointers, and resources

ADR 0007 remains authoritative:

- Rust semantic lifetime is distinct from CLR managed-storage lifetime;
- managed reachability cannot resurrect a moved/dropped/expired Rust value or borrow;
- `Drop` is not a CLR finalizer;
- supported resource-owning projections may unify one cleanup core behind Rust `Drop` and `IDisposable.Dispose`;
- `Rc`, `Arc`, `Weak`, and ownership-sensitive `Box` are not degraded silently to ordinary managed references.

## Send, Sync, CLR aliasing, and managed concurrency

The CLR type system does not enforce Rust `Send`/`Sync`. FerrumWeave must therefore prevent managed aliases/threads from creating Rust data races.

For an unrestricted public managed reference object backed by Rust-owned state:

- state that may move across managed threads must satisfy the necessary Rust `Send` requirements;
- state concurrently reachable through shared managed aliases must satisfy the necessary Rust `Sync` requirements, or the projection must provide a compiler-owned synchronization/affinity mechanism;
- an exported `&mut self` call must establish exclusive access before entering Rust;
- a type that is neither safe to share nor safely wrapped must be rejected as an unrestricted public managed object.

A future explicit thread-affine projection may confine an object to its creating/owning thread and perform runtime thread checks. Thread affinity is not inferred from absence of `Send`/`Sync`; it must be a declared managed contract.

FerrumWeave does not advertise a type as "thread safe" merely because the CLR can pass its object reference between threads.

## Custom attributes

Rust attributes and CLR custom attributes are different metadata systems.

FerrumWeave never copies arbitrary Rust attributes into CLR `CustomAttribute` rows by name.

CLR custom attributes are emitted only by an explicit projection or by compiler-owned metadata required for a certified CLR feature (for example interoperability/nullability/tooling metadata). The projection must identify the exact attribute constructor and supported constant/array/type arguments according to CLR custom-attribute rules.

The final surface syntax may be `.rsproj` metadata, a future stable Rust attribute/macro mechanism, or another accepted configuration surface; the semantic contract is unchanged.

## Reflection and ordinary managed identity

Every supported projected class, interface, method, constructor, property, field, event, generic parameter, attribute, base type, interface implementation, and accessibility flag must be represented through ordinary CLR metadata.

Independent managed reflection must be able to discover the same identity and member shape that C#, Visual Basic, F#, frameworks, DI containers, serializers, and test runners consume.

FerrumWeave-specific runtime lookup may support diagnostics/debugging internally but is never a substitute for standard metadata on a claimed public CLR surface.

## Diagnostics and unsupported shapes

Whenever a requested public projection cannot preserve both Rust safety and the claimed CLR semantics, compilation fails before publishing misleading metadata.

Diagnostics should identify:

- the Rust source identity;
- the requested CLR projection;
- the semantic rule that cannot be satisfied;
- whether an explicit wrapper/projection can resolve it.

Examples include non-Copy value-type requests with observable unique ownership, borrowed closure capture escaping as a delegate, public generics with unsupported bounds, unrestricted managed exposure of unsafe non-`Send`/`Sync` state, or name/signature collisions not explicitly declared as overloads.

## Consequences

The result is intentionally asymmetric:

- Rust authors continue writing Rust;
- ordinary Rust constructs receive conservative CLR defaults;
- richer CLR object-model concepts remain available through explicit projection rather than new C#-like Rust keywords;
- managed consumers see normal CLR types, slots, interfaces, members, exceptions, tasks, delegates, and metadata;
- FerrumWeave can reject a shape rather than weaken Rust semantics.

This design also means "first-class .NET language" does not mean "every C# surface construct must have a one-token Rust spelling." It means the CLR behavior is defined, deterministic, discoverable, interoperable, and safe.

## Evidence rule

This ADR defines semantics. It does not claim those semantics are implemented.

The accompanying senior OOP/CLR examination records each question as one of:

- `CERTIFIED` — executable FerrumWeave evidence exists for the exact stated scope;
- `DEFINED-BUT-RED` — semantics are decided here or in ADR 0005-0007 but executable evidence is still required;
- `UNSUPPORTED-BY-DESIGN` — FerrumWeave deliberately rejects that exact surface for the stated baseline.

`UNDEFINED` is not an allowed state after this ADR.

## Non-goals

This ADR does not freeze the final XML/attribute spelling of every projection descriptor. It freezes the semantic meaning and safe default. Exact syntax can evolve without reopening the object-model decision.

It also does not promote every defined feature into R10. Milestone assignment, contract IDs, dependency order, and implementation slices are the next planning iteration.