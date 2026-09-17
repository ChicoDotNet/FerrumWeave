# ADR 0007 — Separate Rust semantic lifetime, CLR managed-storage lifetime, and resource lifetime

- Status: Accepted for R10 and the cross-runtime object-model roadmap
- Date: 2026-09-16
- Milestone: R10 — Developer experience / 0.1 alpha
- Relates to: ADR 0005 CLR identity projection, ADR 0006 naming/visibility projection, R07 semantic-interoperability evidence

## Context

FerrumWeave compiles real Rust through `rustc` into managed CIL/metadata that executes on the CLR. That means a single source value may participate simultaneously in two runtime models that deliberately solve memory and lifetime differently.

Rust does not use a tracing garbage collector as its ordinary memory-management model. Rust ownership, borrowing, moves, lifetimes, and `Drop` determine when source values remain legally usable and when deterministic destruction occurs.

The CLR does use a tracing garbage collector for managed storage. The CLR decides when unreachable managed objects are physically reclaimed, may move managed objects during collection, and does not normally make the exact reclamation instant part of ordinary program semantics.

A third concern is distinct from both memory models: external resources such as file handles, sockets, database connections, mutex guards, native handles, GPU resources, or other scarce/non-memory state often require deterministic release even in garbage-collected systems.

FerrumWeave must preserve Rust semantics without attempting to replace the CLR collector, and it must not confuse managed-memory reclamation with deterministic resource cleanup.

## Decision summary

FerrumWeave models three distinct lifetime domains:

1. **Rust semantic lifetime** — when a Rust value/reference is legally usable according to Rust ownership, borrowing, moves, and lifetime rules.
2. **CLR managed-storage lifetime** — how long the managed object/storage that represents some value remains physically allocated and rooted/reachable to the CLR.
3. **Resource lifetime** — when an owned external/scarce resource is logically acquired and released.

The governing principle is:

> **Rust owns semantic lifetime; the CLR owns managed-storage reclamation; explicit Rust/CLR resource contracts own resource release.**

These domains may overlap, but FerrumWeave must never collapse them into one concept.

## Rust semantic lifetime remains authoritative

FerrumWeave must not reinterpret ordinary Rust ownership as garbage-collected ownership merely because the generated representation runs on the CLR.

If Rust semantics say that a value was moved, borrowed exclusively, dropped, or otherwise left its legal lifetime, the value is no longer usable through the Rust program even if a managed representation still physically exists on the CLR heap.

Conceptually:

```text
Rust value / handle

created -------------------------- semantic end
   |                                  |
   | ownership / borrow rules          +-- Drop when applicable
   |                                  |
   +----------------------------------X  no later Rust use is authorized

Managed representation

allocated -------------------------------- unreachable -------- CLR GC reclaim
     |                                            |
     +------- may remain physically alive --------+
```

The continued physical existence of managed storage does not resurrect a Rust value or extend a Rust borrow.

## CLR owns managed-storage reclamation

Normal Rust source compiled by FerrumWeave must not manually free CLR-managed object storage.

FerrumWeave may generate the runtime bookkeeping needed to keep a managed object rooted while a legal Rust value/reference depends on it. It may also use CLR mechanisms required for implementation correctness, such as handles or explicit keep-alive behavior.

Pinning is not the default lifetime mechanism. Pinning is only justified when address stability is materially required by a supported interop contract; managed liveness and managed address stability are different concerns.

Once no CLR root remains, physical managed-memory reclamation belongs to the CLR collector. FerrumWeave must not expose a normal Rust `delete`/`free` operation for a managed object.

## Resource lifetime is distinct from managed memory

Garbage collection does not by itself define timely release of files, sockets, database connections, OS handles, locks, or other external resources.

FerrumWeave therefore treats deterministic resource cleanup as a separate semantic contract.

For Rust-owned resources, Rust `Drop` remains the source semantic authority for deterministic cleanup.

For a supported Rust-defined type that is projected for normal .NET consumption and owns a disposable resource, FerrumWeave may project the same logical cleanup operation through `System.IDisposable.Dispose`.

The intended relationship is:

```text
Rust semantic end -----------------> Drop
                                      |
                                      v
                             one logical cleanup core
                                      ^
                                      |
.NET deterministic cleanup -------- Dispose()
```

The two entry paths must not create two independent cleanup semantics. Where a type supports both paths, cleanup must remain coherent and exactly-once according to that type's accepted contract.

R07 already contains source-causal evidence for one supported `Drop`/`IDisposable` slice with exactly-once observable cleanup. That evidence is a proven vertical slice, not a claim that every Rust `Drop` implementation is automatically CLR-disposable.

## `Drop` is not a CLR finalizer

FerrumWeave must not translate Rust `Drop` mechanically into a CLR finalizer.

Rust `Drop` is deterministic with respect to Rust value semantics. CLR finalization is nondeterministic and occurs under the collector's finalization process.

Therefore:

```text
Rust Drop != CLR Finalize
```

A CLR finalizer is permitted only under a separate explicit contract for a projected type that directly owns unmanaged/native resources and requires a safety net when a managed consumer fails to perform deterministic cleanup.

FerrumWeave must not emit finalizers for ordinary Rust values merely because they implement `Drop`.

When a managed resource wrapper equivalent to the .NET safe-handle pattern can own the native handle, that pattern is preferred over compiler-generated ad-hoc finalizer logic. Exact `SafeHandle` interop syntax/projection remains a later executable contract.

## `IDisposable` is an interop projection, not a replacement for ownership

Projecting a Rust resource-owning type as `IDisposable` does not transfer Rust lifetime authority to the CLR.

For Rust callers, Rust ownership and `Drop` remain authoritative.

For managed callers, `Dispose` provides the normal deterministic .NET resource-release surface.

FerrumWeave must not infer that every type with `Drop` should become public `IDisposable`; some `Drop` implementations may be purely Rust-internal, may have no supported managed representation, or may require semantics that have not yet been certified.

Likewise, FerrumWeave must not infer a CLR finalizer merely from `IDisposable` or `Drop` support.

Post-dispose member behavior, `ObjectDisposedException`, idempotency requirements beyond already certified slices, asynchronous disposal, and inheritance/disposal patterns require their own contracts rather than being guessed from method names.

## Managed liveness cannot widen Rust lifetime

FerrumWeave may keep a managed object physically alive long enough to satisfy a legal Rust operation. It must not do the reverse: CLR reachability cannot authorize Rust use that the borrow checker/lifetime model rejected.

Examples of invalid reasoning include:

```text
"The CLR object is still rooted, therefore an expired Rust reference may be used."
"The CLR object survived collection, therefore a moved Rust value may be observed again."
"A managed alias exists, therefore unique Rust ownership may be silently recreated."
```

All are forbidden.

The invariant is:

> **Managed-storage liveness may support an already-legal Rust lifetime; it may never extend or recreate Rust semantic authority.**

## Reference-counted Rust types are not ordinary CLR references

Rust smart pointers encode semantics that are observably different from ordinary tracing-GC references.

FerrumWeave must not automatically project the following as plain managed object references:

- `Rc<T>`;
- `Arc<T>`;
- `Weak<T>`;
- `Box<T>` when its unique-ownership/destruction semantics would be lost.

Reasons include:

- `Rc<T>` has non-atomic reference-count semantics and deterministic destruction when the strong count reaches zero;
- strong `Rc` cycles can remain allocated, while an ordinary tracing GC may collect unreachable cycles;
- `Arc<T>` has atomic shared-ownership semantics and similarly observable destruction/count behavior;
- `Weak<T>` participates in explicit upgrade/downgrade semantics rather than ordinary strong GC reachability;
- `Box<T>` represents unique ownership and deterministic destruction, which cannot be silently replaced with unrestricted managed aliasing.

A future interop contract may define supported projections for some of these types, but it must preserve or explicitly transform their semantics. "The CLR can keep the object alive" is not sufficient evidence of equivalence.

Unsupported smart-pointer crossings must fail diagnostically rather than silently degrading to ordinary managed references.

## Managed references consumed by Rust

A CLR object consumed from Rust participates in CLR storage/liveness rules while Rust still applies its own local aliasing and lifetime rules to the Rust-visible handle/reference shape.

FerrumWeave must therefore preserve both truths:

```text
CLR: object storage is tracing-GC-managed
Rust: use of the Rust-visible value/reference remains constrained by Rust semantics
```

The exact representation of managed references, rooting handles, pinning, interior mutability, and thread-safety are separate implementation/interop contracts. This ADR prohibits solving those concerns by weakening either runtime's semantics.

## Cycles

FerrumWeave must not claim that CLR cycle collection makes Rust ownership cycles irrelevant.

If a supported Rust abstraction has reference-counted cycle semantics, those semantics remain part of the Rust contract unless an explicit boundary transformation says otherwise.

Conversely, ordinary CLR object graphs remain subject to CLR tracing-GC reachability. FerrumWeave must not artificially impose Rust reference-count cycle behavior on normal CLR references.

The source abstraction and boundary contract decide which semantics apply.

## Relationship to R07 evidence

R07 already proves several important slices relevant to this ADR:

- panic does not silently unwind across an unsupported managed boundary;
- one supported Rust `Drop` resource projects to `IDisposable`-style exactly-once cleanup;
- supported managed identity/lifetime remains coherent under CLR GC in the certified scenario;
- Rust borrows that cannot safely escape a managed boundary are rejected explicitly.

Those GREEN contracts remain authoritative evidence for their exact supported slices.

ADR 0007 generalizes the architectural rule around them; it does not retroactively broaden their implementation claims.

In particular, this ADR does **not** claim general support yet for finalizers, `SafeHandle`, arbitrary `Rc`/`Arc`/`Weak`/`Box` projection, every `Drop` type, async disposal, pinning, or all possible managed-reference lifetime shapes.

## CLR accessibility terminology clarification

The lifecycle model is independent of access control, but the same object-model review exposed an important terminology rule for ADR 0006.

FerrumWeave compiler internals should model **CLR metadata accessibility**, not C# keyword names.

The canonical terms are:

| CLR accessibility | Common C# spelling | Meaning |
| --- | --- | --- |
| `Public` | `public` | accessible without family/assembly restriction |
| `Assembly` | `internal` | accessible within the same assembly |
| `Family` | `protected` | accessible to the declaring type/family/derived types according to CLR rules |
| `FamORAssem` | `protected internal` | family OR same assembly |
| `FamANDAssem` | `private protected` | family AND same assembly |
| `Private` | `private` | declaring-type private |

`private protected` is therefore a valid C# surface spelling for the CLR `FamANDAssem` accessibility. It must not be removed from FerrumWeave's model.

An implementation-facing enum should therefore prefer CLR names conceptually equivalent to:

```rust
pub enum ClrAccessibility {
    Public,
    Assembly,
    Family,
    FamilyOrAssembly,
    FamilyAndAssembly,
    Private,
}
```

C# names are useful documentation aliases but are not the compiler's semantic vocabulary.

ADR 0006 remains authoritative for deriving the maximum allowed CLR accessibility from effective Rust visibility. `Family`, `FamORAssem`, and `FamANDAssem` are still emitted only when an explicit managed-inheritance projection makes family accessibility meaningful; Rust module hierarchy does not imply CLR inheritance.

## Invariants

Any implementation that touches lifetime/resource/GC projection must preserve all of these invariants:

1. **Rust semantic authority** — ownership, moves, borrows, and lifetime legality remain Rust decisions.
2. **No manual managed free** — ordinary Rust code does not directly reclaim CLR-managed storage.
3. **No lifetime resurrection** — CLR reachability cannot revive an expired/moved/dropped Rust value or reference.
4. **Deterministic resource semantics remain deterministic** — GC timing cannot replace a required `Drop`/`Dispose` release point.
5. **Drop is not finalization** — CLR finalizers require a separate explicit unmanaged-resource contract.
6. **Exactly one logical cleanup** — if both Rust `Drop` and managed `Dispose` reach a supported resource, they converge on one coherent cleanup contract.
7. **Smart-pointer semantics are not erased** — `Rc`/`Arc`/`Weak`/`Box` are not silently degraded to plain managed references.
8. **Rooting is implementation, not ownership** — keeping managed storage alive supports legal semantics but does not widen Rust authority.
9. **Pinning is exceptional** — address stability is requested only by contracts that materially require it.
10. **Evidence remains slice-specific** — existing R07 GREEN evidence is not generalized beyond what its executable tests prove.

## Validation requirements for future object-model contracts

Future executable contracts derived from this ADR should include falsifiers that distinguish the three lifetime domains rather than merely showing that no crash occurred.

Useful proof shapes include:

- Rust semantic end triggers deterministic cleanup even when managed storage remains physically live;
- a managed consumer can call the accepted `Dispose` projection and observe the same cleanup semantics as the Rust path;
- cleanup does not execute twice when both managed and Rust lifetime machinery could otherwise converge;
- a managed object remains rooted for the duration of a legal Rust operation without permitting use after Rust semantic end;
- unsupported borrowed/smart-pointer escapes fail before producing misleading metadata;
- cyclic `Rc`/`Arc` semantics are not silently replaced with tracing-GC semantics;
- finalizer metadata is absent for ordinary `Drop` types;
- any future finalizer-enabled type proves why finalization is required and why deterministic cleanup remains the primary path.

## Non-goals

This ADR does not define:

- the final public syntax for managed-reference handles;
- arbitrary pinning APIs;
- finalizer generation syntax;
- exact `SafeHandle` projection syntax;
- general `Rc<T>`, `Arc<T>`, `Weak<T>`, or `Box<T>` public CLR mappings;
- async `DisposeAsync` / Rust async-drop semantics;
- post-dispose behavior for every projected type;
- a general managed-object resurrection policy beyond prohibiting it as a way to widen Rust semantic lifetime;
- the complete class/value-type projection model, which belongs to the broader CLR object-model conformance work.

The durable rule is:

> **The CLR may keep bytes alive after Rust semantics are finished with a value; that physical survival never changes what Rust is allowed to do. Resource cleanup is explicit and deterministic when the source/resource contract requires it.**
