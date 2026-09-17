# FerrumWeave CLR/OOP senior-engineer examination

Status: **semantic denominator defined; implementation evidence remains incremental**.

This examination asks the questions a senior .NET/CLR engineer should be able to answer before depending on FerrumWeave's managed object model. It is deliberately adversarial: a prototype capability does not pass a question merely because the CLR could theoretically support it.

Authority order for answers:

1. executable FerrumWeave contracts/evidence;
2. accepted ADRs 0005-0008;
3. this examination as a compact cross-reference.

Allowed states:

- `CERTIFIED` — executable FerrumWeave evidence exists for the exact stated scope;
- `DEFINED-BUT-RED` — the semantic answer is fixed, but full executable evidence is pending;
- `UNSUPPORTED-BY-DESIGN` — the exact requested surface is deliberately rejected for the stated baseline.

**`UNDEFINED` is forbidden.** If a future exam question exposes a genuinely new semantic gap, architecture must decide it before claiming that question is part of the denominator.

The status is intentionally strict. A row can be `DEFINED-BUT-RED` even when a smaller vertical slice is already GREEN.

---

## Block 1 — Managed identity, entry point, naming, reflection

### Q01 — What does ordinary Rust `fn main()` become in a .NET executable?

**Answer:** `rustc` remains the authority for the semantic Rust entry point. FerrumWeave projects that semantic entry to a static managed `<RootNamespace>.Program.Main` adapter and designates that exact `MethodDef` as the CLI entry point. Rust source is not renamed to `Main` and no C# source is generated.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0005/0006 and `FW-R10-CLR-ENTRY-001`; installed `dotnet run` evidence remains pending.

### Q02 — Where do assembly, namespace, module, and nested-type identities come from?

**Answer:** `AssemblyName`/`RootNamespace` come from SDK/project inputs. Semantic Rust module paths project to CLR namespace segments. Filesystem folders alone are not authority. Rust module nesting does not automatically become nested CLR types. Rust-defined nested CLR types are not part of the 1.0 default surface; existing nested managed types may still be consumed by metadata identity.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0005/0008 and `FW-R10-CLR-SURFACE-001`.

### Q03 — How are naming collisions, case-only differences, raw identifiers, and PascalCase transforms handled?

**Answer:** Public CLR naming is deterministic and validated before emission. Raw `r#` escape spelling does not leak into CLR names. Case-insensitive collisions, keyword/interoperability ambiguity, transform-induced collapse, and collisions with generated identities fail diagnostically. FerrumWeave never silently suffixes/hash-mangles public APIs. An intentional overload requires an explicit overload-group projection and distinct CLR signatures.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0006 and `FW-R10-CLR-NAMING-001`.

### Q04 — Are FerrumWeave public APIs ordinary CLR metadata/reflection surface or a Rust-specific lookup system?

**Answer:** Ordinary CLR metadata is mandatory. Supported types/members must be discoverable and invokable by normal managed reflection and normal .NET language/tooling rules. A FerrumWeave-specific registry cannot substitute for public metadata.

**State:** `CERTIFIED` for current R06 supported static/instance/reflection slices; every new metadata shape still requires its own evidence.

**Authority/evidence:** `FW-R06-DOTNET-001` through `FW-R06-DOTNET-008`, especially independent reflection in `FW-R06-DOTNET-007`.

---

## Block 2 — Accessibility and exportability

### Q05 — Does Rust `pub` become CLR `public`?

**Answer:** Effectively externally reachable Rust `pub` becomes CLR `Public` by default when the item has a supported CLR projection. FerrumWeave uses rustc effective visibility, not the syntactic token alone; a `pub` item behind a private ancestor is not accidentally public CLR API, while an effective public re-export is honored.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0006 and `FW-R10-CLR-VISIBILITY-001`.

### Q06 — What happens to `pub(crate)`, `pub(super)`, `pub(in ...)`, and private Rust items?

**Answer:** `pub(crate)` maps by default to CLR `Assembly` (`internal`). More restricted Rust visibility remains non-public implementation. Because CLR namespaces do not create Rust-like module access domains, `pub(super)`/`pub(in ...)` are preserved by Rust semantics and lowered with no more CLR visibility than generated topology requires.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0006.

### Q07 — Where do `protected`, `protected internal`, and `private protected` fit?

**Answer:** FerrumWeave models CLR `Family`, `FamORAssem`, and `FamANDAssem` respectively. They exist only when an explicit CLR inheritance projection makes family access meaningful. Rust module visibility never implies protected-family access.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0006/0007 terminology clarification.

### Q08 — Can a CLR projection widen what Rust allowed, and is visibility the same as exportability?

**Answer:** No. `CLR-authorized accessibility ⊆ Rust-authorized accessibility`. A projection may narrow but never widen Rust reachability. Separately, an externally public Rust item may still be non-exportable when FerrumWeave lacks a safe CTS representation; that must fail or stay unprojected rather than emit misleading public metadata.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0006.

---

## Block 3 — CTS type kind, value/reference semantics, Object

### Q09 — Which basic Rust values already have certified CTS mappings?

**Answer:** The certified denominator includes fixed-width integers, native integers, `bool`, explicit `char` non-equivalence, named `Int128/UInt128` handling, managed object identity, `System.String`, SZARRAY round-trip, and public-boundary nullability rules for the supported slices.

**State:** `CERTIFIED`.

**Authority/evidence:** R04 `FW-R04-CTS-001` through `FW-R04-CTS-011`.

### Q10 — Does every Rust `struct` become a CLR `struct`?

**Answer:** No. The default externally exported object-like Rust struct is a **sealed CLR reference class**. CLR value-type projection is explicit and conservative: the baseline requires `Copy`, no `Drop`, no escaping borrow semantics, projectable fields, and no resource/ownership duplication through copying or boxing. Rust ABI `repr` attributes alone do not select CLR value-type semantics.

**State:** `DEFINED-BUT-RED` for the general policy. R06 already certifies one Rust ADT projected as a constructible managed reference type, but not this complete selection policy.

**Authority/evidence:** ADR 0008; R06 instance-type slice.

### Q11 — How do Rust enums project?

**Answer:** A fieldless Rust enum may explicitly project as a CLR enum only with a stable supported integer discriminant representation. A data-carrying Rust enum is not a CLR enum and direct CLR-enum projection is rejected. Until a dedicated discriminated-union feature exists, use a deliberate wrapper/API boundary.

**State:** `DEFINED-BUT-RED` for fieldless CLR-enum projection; data-carrying-enum-as-CLR-enum is `UNSUPPORTED-BY-DESIGN` within this answer.

**Authority/evidence:** ADR 0008.

### Q12 — What are the default `System.Object`, equality/hash/string, and boxing semantics?

**Answer:** Projected reference classes derive from `System.Object` unless an explicit supported base class is selected. `PartialEq/Eq`, `Hash`, and `Debug/Display` do not automatically override `Equals`, `GetHashCode`, or `ToString`. A CLR value projection uses normal CLR boxing/unboxing; boxing never grants new Rust ownership/lifetime authority.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008; R04 proves supported object/value primitives but not the complete object-override policy.

---

## Block 4 — Class inheritance and virtual dispatch

### Q13 — Are projected Rust classes inheritable by default?

**Answer:** No. A default projected Rust reference type is sealed/final and its ordinary methods are non-virtual. Rust did not declare class inheritance, so FerrumWeave does not invent extension points.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

### Q14 — Can a Rust-backed type derive from a CLR base class? Can it have multiple base classes?

**Answer:** One explicit supported CLR base class may be selected through managed projection metadata. Base constructor, initialization order, inherited slots, access, and Rust-state lifetime must all be defined. Multiple class inheritance is `UNSUPPORTED-BY-DESIGN` because the CLR has one class base type. Multiple interfaces remain separate and supported by the intended model.

**State:** `DEFINED-BUT-RED` for single-base derivation.

**Authority/evidence:** ADR 0008.

### Q15 — What is the rule for `virtual`, `newslot`, `override`, and `final`?

**Answer:** New virtual slots are explicit. Ordinary methods are non-virtual. A true override must bind to the exact inherited virtual signature/slot and reuses that slot; same spelling is insufficient. A sealed override is emitted virtual/final. Accidental same-name hiding never silently becomes override.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

### Q16 — What about abstract classes and abstract members?

**Answer:** FerrumWeave 1.0 does not infer or synthesize Rust-owned abstract CLR base classes from traits/structs; Rust-owned polymorphic abstraction defaults to interfaces. Deriving a Rust-backed managed type from an existing CLR abstract base and implementing/overriding its required members is part of the explicit base/override model. Authoring a new Rust-owned abstract CLR base is `UNSUPPORTED-BY-DESIGN` for the 1.0 object-model baseline and requires a later dedicated contract if added.

**State:** `DEFINED-BUT-RED` for deriving from existing abstract CLR bases.

**Authority/evidence:** ADR 0008.

---

## Block 5 — Traits, interfaces, and polymorphism

### Q17 — Does a Rust trait automatically become a CLR interface?

**Answer:** No. A public trait may explicitly project as a CLR interface only when every exposed signature is CLR-projectable and unsupported associated types/constants/generic methods/borrow escapes do not leak into the contract. Non-projectable traits remain Rust-only or fail when interface export is explicitly requested.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008. R07 `IDisposable` proves an interface implementation slice, not general trait projection.

### Q18 — Can a Rust-backed type implement multiple CLR interfaces?

**Answer:** Yes. Every accepted projectable trait/interface contributes ordinary CLR interface implementation metadata. Multiple interfaces do not imply multiple class inheritance.

**State:** `DEFINED-BUT-RED`; one `IDisposable` implementation mechanism is already proven, but the multiple-interface/general-trait denominator is not.

**Authority/evidence:** ADR 0008; R07 disposable slice.

### Q19 — How are implicit versus explicit interface implementations chosen?

**Answer:** Use an ordinary public implementation when one unambiguous projected method satisfies the interface slot. Use explicit `MethodImpl`-style binding when collision/compatibility requires it. Exact interface slot identity is authoritative; names alone are insufficient.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

### Q20 — Can `dyn Trait` cross into .NET as an interface reference?

**Answer:** Only when the trait has a certified CLR-interface projection and the concrete object/lifetime semantics are safe. FerrumWeave exposes the CLR interface reference, not Rust's internal vtable ABI.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008/0007.

---

## Block 6 — Methods, constructors, properties, fields

### Q21 — What determines static versus instance methods?

**Answer:** A Rust associated function with no receiver may project as CLR static. A supported `&self` receiver may project as instance. Current FerrumWeave already certifies both public static and receiver-backed instance shapes. `&mut self` requires managed-boundary exclusivity before entering Rust.

**State:** `CERTIFIED` for existing static and basic `&self` instance slices; exclusivity for general `&mut self` is `DEFINED-BUT-RED` within the answer.

**Authority/evidence:** R06 `FW-R06-DOTNET-001` through `006`; ADR 0008 for `&mut self` safety.

### Q22 — How are constructors and static constructors determined?

**Answer:** Names like `new`/`create` are not constructor heuristics. `.ctor` is an explicit projection to a valid Rust initialization path. A parameterless constructor exists only when zero-argument initialization is semantically valid. Derived constructors must invoke an accepted base `.ctor`. `.cctor` is compiler/runtime initialization infrastructure unless a dedicated explicit type-initialization projection exists.

**State:** `DEFINED-BUT-RED` generally; R06 certifies one trivial public parameterless `.ctor` slice.

**Authority/evidence:** ADR 0008; R06 instance construction.

### Q23 — Do Rust getter/setter names automatically become CLR properties or indexers?

**Answer:** No. CLR properties/indexers are explicit metadata projections with exact accessors. Getter/setter accessibility may differ only within Rust-authorized visibility. Rust indexing traits do not automatically create CLR indexers.

**State:** `DEFINED-BUT-RED` for Rust-defined property projection. Calling existing managed properties from Rust is already certified in R05.

**Authority/evidence:** ADR 0005/0008; R05 property-consumption evidence.

### Q24 — Does `pub field` mean public CLR field?

**Answer:** No. Rust public fields remain implementation state unless an explicit CLR field projection is selected. The projection must define field type, static/instance shape, visibility, and mutability/init-only semantics. Public API should normally prefer explicit properties/methods because CLR field semantics can expose copying/layout/mutation that Rust did not intend.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

---

## Block 7 — Overloads, generics, constraints, variance

### Q25 — How does Rust call an overloaded .NET API?

**Answer:** Managed overload resolution uses full CLR metadata signature plus Rust type information, never the method name alone. The selected method token/signature must be deterministic.

**State:** `DEFINED-BUT-RED` for full overload sets. R05 already proves exact managed-member resolution/call mechanics for supported signatures.

**Authority/evidence:** ADR 0008; R05 managed member resolution.

### Q26 — Can distinct Rust functions become one CLR overload set?

**Answer:** Yes, but only explicitly. Distinct Rust source identities may be grouped under one CLR name when projected signatures are distinct and supported .NET languages can resolve them. Accidental PascalCase/name collapse remains a collision error rather than becoming an overload automatically.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0006/0008.

### Q27 — Does Rust monomorphization automatically produce public CLR generics?

**Answer:** No. A claimed public CLR generic type/method must emit real CLR generic parameter metadata. Internal Rust monomorphization remains a rustc concern and cannot masquerade as one reified CLR generic definition. Unsupported generic public shapes fail or use an explicit non-generic wrapper.

**State:** `DEFINED-BUT-RED`. R07 proves one generic boundary (`Nullable<int>`), not general public generic definitions.

**Authority/evidence:** ADR 0008; R07 `FW-R07-SEM-009` as limited precedent.

### Q28 — How do Rust bounds and CLR variance map?

**Answer:** Only semantically corresponding bounds become CLR constraints: projectable traits may become interface constraints and an explicit supported base requirement may become a base-class constraint. `Copy != struct`, `'static != class`, and `Send/Sync` have no direct CLR generic-constraint equivalent. Public generic parameters are invariant by default. Covariance/contravariance is explicit and only legal for CLR-valid interface/delegate positions after position analysis.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

---

## Block 8 — Delegates, events, operators, custom managed behavior

### Q29 — How are existing CLR delegates consumed from Rust?

**Answer:** As managed delegate objects with normal CLR invocation semantics and managed liveness. FerrumWeave does not reinterpret multicast delegates as Rust `Fn*` internals.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

### Q30 — How does a Rust function/closure become a CLR delegate?

**Answer:** Only through an explicit delegate projection. Non-capturing compatible functions are eligible. Escaping capturing closures must own their captures and satisfy the accepted `'static`/managed-lifetime boundary; borrowed escaping captures are rejected. Reusable `Fn` can back a normal delegate. `FnMut` requires exclusivity/synchronization. `FnOnce` is `UNSUPPORTED-BY-DESIGN` as an ordinary reusable CLR delegate.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008/0007.

### Q31 — Is a .NET event just a public delegate field?

**Answer:** No. Event projection is explicit and emits ordinary CLR event metadata plus exact add/remove methods and a supported delegate type. Names alone do not create events.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

### Q32 — Do Rust operator traits automatically become CLR operators/conversions?

**Answer:** No. A concrete compatible Rust implementation may be explicitly projected to an exact CLR `op_*` special-name method. `Add`, `From`, `Into`, etc. are not assumed semantically identical to CLR operators/conversions. Checked operators require their own exact contract.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

---

## Block 9 — Errors, panic, async, deterministic cleanup

### Q33 — What happens to `Result<T,E>` at a managed public boundary?

**Answer:** `Ok(T)` returns the success value; `Err(E)` becomes a managed exception rather than a hidden status code. An explicitly projected CLR exception type for `E` may be thrown; otherwise the compatibility fallback is `InvalidOperationException` with stable FerrumWeave/Rust diagnostic identity/payload where safely projectable.

**State:** `CERTIFIED` for the core Ok/value and Err/managed-exception direction, including current `InvalidOperationException` fallback slice; richer typed-exception projection remains `DEFINED-BUT-RED`.

**Authority/evidence:** R07 `FW-R07-SEM-004` and `005`; ADR 0008.

### Q34 — Can Rust panic unwind across CLR public boundaries?

**Answer:** No. Raw cross-runtime panic unwind is forbidden. Current statically recognized direct panic is rejected. The target general managed boundary contains an unwindable panic and translates it to a distinct FerrumWeave-managed panic exception; a configuration that cannot contain it must reject that public export rather than silently allow raw unwind. Panic is not `Result::Err`.

**State:** `DEFINED-BUT-RED` for the general runtime containment rule; direct rejection is already certified.

**Authority/evidence:** R07 `FW-R07-SEM-006`; ADR 0008.

### Q35 — What does Rust `Future`/`async fn` become?

**Answer:** A supported public future defaults to `Task<T>` (`Task` for unit). `ValueTask` is explicit, not default. FerrumWeave owns the polling/completion adapter; Rust future internals do not leak to CLR callers. `Result` errors fault the task according to the exception policy. Cancellation is explicit and not invented automatically.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

### Q36 — How do `Drop`, `IDisposable`, finalization, and GC relate?

**Answer:** Rust semantic lifetime, CLR storage lifetime, and resource lifetime are distinct. `Drop` is deterministic Rust cleanup, not CLR finalization. A supported resource-owning managed projection may expose the same one logical cleanup through `IDisposable.Dispose`. A finalizer requires a separate explicit unmanaged-resource contract and is never inferred merely from `Drop`/`IDisposable`.

**State:** `DEFINED-BUT-RED` for the full general policy; one exactly-once `Drop`/`IDisposable` slice and managed-lifetime slice are already certified.

**Authority/evidence:** ADR 0007; R07 `FW-R07-SEM-007` and `008`.

---

## Block 10 — Nullability, lifetime safety, concurrency, custom attributes

### Q37 — How do Rust `Option` and CLR nullability/`Nullable<T>` interact?

**Answer:** Supported reference `Option<T>` may project None as CLR null and Some as the managed value. Supported value `Option<T>` may project as `Nullable<T>`. Plain non-optional Rust values are not silently nullable merely because the CLR representation is a reference type. Nullable tooling metadata must reflect this semantic distinction.

**State:** `CERTIFIED` for the R07 supported reference/value slices; broader nullable-annotation emission still requires per-shape evidence.

**Authority/evidence:** R04 nullability policy; R07 `FW-R07-SEM-001` through `003`.

### Q38 — Can GC reachability or smart-pointer lowering extend/change Rust lifetime semantics?

**Answer:** CLR reachability may keep storage alive for an already-legal Rust operation but cannot resurrect moved/dropped values or expired borrows. `Rc`, `Arc`, `Weak`, and ownership-sensitive `Box` are not silently degraded to normal GC references because reference counting, deterministic destruction, cycle behavior, and uniqueness are observable semantics. Unsupported crossings fail diagnostically or require explicit adapters.

**State:** `DEFINED-BUT-RED` for smart-pointer projections; the core no-borrow-widening/managed-identity invariant is already certified.

**Authority/evidence:** ADR 0007; R07 `FW-R07-SEM-008` and `010`.

### Q39 — What prevents managed aliases/threads from violating Rust `Send`, `Sync`, or `&mut` exclusivity?

**Answer:** CLR reachability is not proof of Rust thread safety. Unrestricted managed exposure must satisfy required `Send`/`Sync` conditions or use an explicit synchronization/affinity adapter. Exported `&mut self` calls must establish exclusive entry before invoking Rust. A type that cannot be shared safely and has no accepted wrapper is rejected as an unrestricted managed object. A future thread-affine projection must be explicit and enforce the owning-thread rule at runtime.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008/0007.

### Q40 — Do Rust attributes automatically become CLR custom attributes?

**Answer:** No. CLR `CustomAttribute` metadata is explicit (or compiler-owned for a certified platform feature) and identifies the exact managed attribute constructor/arguments. Arbitrary Rust attributes are not copied by spelling. The configuration surface may evolve, but the resulting CLR metadata semantics are fixed and must remain discoverable by normal reflection.

**State:** `DEFINED-BUT-RED`.

**Authority/evidence:** ADR 0008.

---

# Examination result at this baseline

The important result is not a percentage; it is that **all 40 questions now have an architectural answer**.

Current strict classification:

```text
CERTIFIED exact/core scope: 5
DEFINED-BUT-RED:           35
UNDEFINED:                  0
```

Several `DEFINED-BUT-RED` rows contain smaller certified primitives (constructors, properties consumed from .NET, `IDisposable`, managed identity, interface implementation mechanics, generic `Nullable<T>`). They remain RED at examination level because the broader senior-level rule has not yet been proven as one coherent product contract.

Several answers also contain deliberately unsupported subcases, including:

- multiple class inheritance;
- automatic data-carrying-Rust-enum -> CLR-enum projection;
- automatic Rust-owned abstract CLR base classes in the 1.0 baseline;
- automatic trait-default-body -> CLR default-interface-method projection;
- accidental naming collisions treated as overloads;
- automatic `FnOnce` -> reusable CLR delegate;
- automatic Rust trait/operator/field/property/event/attribute mapping by spelling;
- raw panic unwind across a managed public boundary.

Those are not unknowns. They are explicit constraints.

# Pass criterion for a future senior-level FerrumWeave object-model release

A question may move from `DEFINED-BUT-RED` to `CERTIFIED` only when executable evidence proves the accepted answer through the actual FerrumWeave product path and ordinary CLR consumption/reflection as appropriate.

A complete senior-level object-model certification does **not** require FerrumWeave to imitate every C# keyword. It requires that a senior CLR engineer can predict the resulting metadata, dispatch, accessibility, value/reference behavior, lifetime, exception, generic, delegate/event, and reflection semantics without guessing.

The next planning iteration should convert these 35 RED answers into dependency-ordered contract families and milestones, preserving R10 scope discipline rather than making all 35 blockers for `0.1-alpha`.