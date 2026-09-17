# R10 CLR object-model scope handoff

Status: **active scope guard for R10 / 0.1-alpha**.

Read after:

1. ADR 0005 — Rust -> CLR identity projection;
2. ADR 0006 — naming/visibility;
3. ADR 0007 — lifetime/GC/resource ownership;
4. ADR 0008 — CLR object-model projection policy;
5. `docs/quality/clr-oop-senior-engineer-exam.md`;
6. `docs/roadmap/clr-object-model-conformance-plan.md`.

## Why this handoff exists

The senior-engineer examination defines the complete object-model denominator for the road to 1.0. It does **not** make every RED answer an R10 blocker.

R10 must activate only object-model slices required to make the seven `0.1-alpha` template families truthful through the installed package boundary.

## R10 slices by capability family

### OM-001 — identity/naming/accessibility

Already active in `tests/r10/contracts.toml` through:

- `FW-R10-CLR-ENTRY-001`;
- `FW-R10-CLR-SURFACE-001`;
- `FW-R10-CLR-NAMING-001`;
- `FW-R10-CLR-VISIBILITY-001`.

Do not create duplicate contracts for the same semantics.

### OM-002 — ordinary object/member surface

Activate only slices actually required by `classlib`, xUnit, NUnit, MSTest, web, or webapi.

Expected early reusable primitives include:

- sealed CLR reference-class projection for ordinary exported Rust structs;
- supported static/instance methods;
- explicit valid constructors where a framework/consumer needs construction;
- ordinary signatures/nullability from already-certified CTS primitives.

Do not pull general value-type, enum, public-field, property/indexer, boxing or custom `System.Object` override work into R10 unless a causal RED from a 0.1 template proves it is required.

### OM-003 — interfaces/dispatch

R10 owns only the minimal interface/slot capability required by the accepted ASP.NET Core host seam.

`FW-R10-WEB-HOST-001` remains the product contract. Reuse existing `InterfaceImpl` evidence and generalize only enough to satisfy the selected official hosting interface with exact slot identity.

General CLR class inheritance, virtual/newslot/override/final and protected-family behavior are not R10 blockers unless the chosen 0.1 hosting path proves otherwise.

### OM-006 — boundary safety

R10 must preserve all R07 invariants. If web/test/framework execution creates new managed aliases, mutable entries or thread crossings, add the smallest executable safety contract necessary for that crossing.

Do not claim framework support by weakening Rust ownership/borrowing, allowing raw panic unwind, or treating CLR reachability as proof of Rust thread safety.

### OM-008 — custom attributes/framework discovery

xUnit, NUnit and MSTest T1 evidence must prove **real test discovery and execution**, not merely scaffold a project or call a test method manually.

When runner discovery requires managed attributes, activate the smallest explicit `CustomAttribute` projection needed by that runner:

- exact managed attribute constructor identity;
- exact target method/type;
- required constructor/named arguments;
- reflection evidence;
- actual runner discovery as the end-to-end observable.

Do not copy arbitrary Rust attributes into CLR metadata by spelling.

## Explicitly not required to finish R10

Unless a 0.1 RED proves otherwise, R10 does not need to complete:

- general CLR value-type projection;
- fieldless enum projection;
- arbitrary base-class inheritance;
- general virtual/newslot/override/final;
- delegates/events/operators;
- produced public CLR generic definitions/variance;
- Rust Future -> Task;
- WPF/WinForms-specific thread-affinity/object-model semantics;
- every senior-exam row.

These are defined architecture, not 0.1 release promises.

## Activation rule

A new object-model contract enters `tests/r10/contracts.toml` only when at least one of these is true:

1. a current R10 template has a reproducible RED whose first responsible divergence is that capability;
2. an existing active R10 contract explicitly requires that capability to become GREEN;
3. independent Systems verification reveals an observable ambiguity that would make a current 0.1 claim misleading.

When activated, reference both the `FW-CLR-OM-*` family and exact senior-exam question(s) in the proof text or adjacent documentation.

## Concurrency rule

Before implementing an object-model slice, fresh-read PR #36, current R10 HEAD, open branches/PRs, and the exact target files. If another lane is already changing the same semantic slice, take a non-overlapping verifier, metadata/reflection, diagnostic, or framework-discovery slice instead of creating a competing implementation.

The senior-exam roadmap expands future certainty; it must not expand current WIP beyond the Head + Principals operating model.
