# R10 CLR lifecycle handoff — Rust semantic lifetime vs CLR storage/resource lifetime

Status: **accepted architecture / implementation scope remains slice-specific**.

This handoff operationalizes ADR 0007 without silently enlarging the R10 release denominator beyond the contracts actually exercised by current R10 work.

## Read first

1. `docs/architecture/adr/0005-r10-rust-to-clr-identity-projection.md`
2. `docs/architecture/adr/0006-r10-clr-naming-and-visibility-policy.md`
3. `docs/architecture/adr/0007-r10-rust-lifetime-clr-gc-and-resource-ownership.md`
4. `tests/r07/contracts.toml` for already-certified semantic slices
5. `tests/r10/contracts.toml` for active R10 release contracts

## Three lifetime domains

Do not use the word "lifetime" without identifying which domain is meant.

| Domain | Authority | Meaning |
| --- | --- | --- |
| Rust semantic lifetime | `rustc` / Rust semantics | when a value/reference is legally usable, moved, borrowed, or dropped |
| CLR managed-storage lifetime | CLR GC | how long managed storage remains rooted/reachable/allocated |
| resource lifetime | explicit Drop/Dispose/resource contract | when external/scarce state is acquired and released |

A managed object may remain physically allocated after the corresponding Rust value has reached semantic end. That fact never permits later Rust use.

## Non-negotiable rules

- GC does not replace Rust ownership or borrowing.
- Rust code does not manually free CLR-managed object storage.
- CLR reachability cannot resurrect an expired/moved/dropped Rust value or borrow.
- `Drop` is not a CLR finalizer.
- A supported public resource-owning Rust type may project one logical cleanup operation through both Rust `Drop` and managed `IDisposable.Dispose`.
- Finalizers require a separate explicit unmanaged-resource contract and are never inferred from `Drop` alone.
- Rooting keeps managed storage alive; it does not grant new Rust ownership or extend a borrow.
- Pinning is only for contracts that require address stability; it is not the normal managed-liveness mechanism.
- `Rc<T>`, `Arc<T>`, `Weak<T>`, and ownership-sensitive `Box<T>` are not silently projected as ordinary CLR references.

## Existing evidence that must not be generalized

R07 already certifies narrow source-causal slices for:

- panic boundary rejection/policy;
- one `Drop` -> `IDisposable` exactly-once cleanup scenario;
- supported managed identity/lifetime under CLR GC;
- rejection of Rust borrows that cannot safely escape the managed boundary.

Treat those as GREEN only for the exact supported scenarios declared in `tests/r07/contracts.toml`.

Do not infer from those tests that FerrumWeave already supports:

- arbitrary `Drop` -> `IDisposable` projection;
- CLR finalizers;
- `SafeHandle` projection;
- general smart-pointer crossing;
- async disposal;
- arbitrary pinning;
- every GC/rooting scenario.

## CLR accessibility terminology

Compiler implementation should speak CLR metadata terminology:

```text
Public
Assembly
Family
FamORAssem
FamANDAssem
Private
```

C# aliases are documentation vocabulary:

```text
public
internal
protected
protected internal
private protected
private
```

`private protected` is valid and corresponds to CLR `FamANDAssem`.

ADR 0006 remains authoritative for effective Rust visibility and the no-widening rule. Family-based accessibility is emitted only under a real managed-inheritance projection.

## When R10 must create a new executable contract

Do not add a lifecycle item to the R10 denominator merely because ADR 0007 exists.

Create an R10 executable contract when an active R10 slice materially depends on a currently uncertified lifetime/resource behavior. Examples:

- a template requires a general public `Drop` -> `IDisposable` projection beyond the R07 fixture;
- ASP.NET hosting requires a managed object to remain rooted across a Rust callback lifetime;
- a supported public type crosses a smart-pointer boundary;
- a runtime feature requires pinning;
- a managed-resource wrapper would require finalization or a safe-handle-style contract.

At that point, RED the exact observable before implementation.

## Required falsifiers for new lifecycle work

A lifecycle test should distinguish semantic correctness from mere process survival. Depending on the slice, prove as appropriate that:

- deterministic cleanup happens at the accepted semantic/resource boundary;
- managed storage may remain alive without enabling illegal Rust use;
- cleanup cannot execute twice;
- rooting prevents premature collection but does not widen Rust lifetime;
- no finalizer is emitted for ordinary `Drop` types;
- unsupported smart-pointer/borrow crossings fail explicitly;
- a Rust-only mutation changes the managed artifact and the independent CLR observable when source causality is claimed.

## Scope relationship to the OOP/CLR exam

The forthcoming CLR object-model exam is a **discovery denominator**, not automatically the R10 release denominator.

Its job is to classify each senior-level CLR/OOP question as:

```text
SUPPORTED
DEFINED-BUT-RED
UNSUPPORTED-BY-DESIGN
UNDEFINED
```

ADR 0007 resolves the conceptual lifecycle/GC/resource questions before that exam is materialized. The exam may expose additional contracts, but those should be assigned deliberately to the appropriate milestone instead of being smuggled into R10 by documentation alone.

## Safe implementation rule

If a proposed shortcut makes either of these statements true, stop and redesign the slice:

```text
"The CLR keeps it alive, therefore Rust may still use it."
"Rust has Drop, therefore emit a CLR finalizer."
```

Both violate ADR 0007.
