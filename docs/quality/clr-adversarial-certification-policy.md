# CLR adversarial certification policy

Status: **accepted verification policy for object-model and managed-interoperability claims**.

This policy operationalizes [`CLR/OOP senior-engineer examination`](clr-oop-senior-engineer-exam.md) and [`CLR automation/security-QA senior examination`](clr-automation-security-qa-exam.md).

Its purpose is simple:

> FerrumWeave must prove not only that a supported managed behavior works, but also that materially incorrect, widened, bypassed, substituted, or unsafe versions of that behavior fail for the expected reason.

The policy applies incrementally. A prerelease activates only the dimensions relevant to the capability it claims; 1.0 must close the complete baseline denominators committed by the release plan.

## 1. Verification layers

Every managed-language feature belongs to one or more independent verification layers.

### L1 — Rust semantic authority

`rustc` remains authoritative for parsing, typing, ownership, borrowing, visibility, moves, traits, and other Rust-source semantics.

Tests at this layer answer questions such as:

- does invalid Rust fail before managed emission?
- does effective visibility differ correctly from a syntactic `pub` token?
- does a source-only mutation reach a different MIR/product outcome?

The CLR is never used as justification to weaken Rust validity rules.

### L2 — Projection-policy authority

FerrumWeave decides which supported Rust semantic item becomes which CLR identity, type kind, member shape, accessibility, slot, attribute, generic form, delegate/event, or runtime adapter.

Tests at this layer answer:

- was the intended projection selected?
- was an unsupported projection rejected?
- did an override bind the exact inherited slot?
- did a restricted Rust item remain non-public?

### L3 — Metadata truth

An independent reader inspects the emitted PE/CLI metadata.

Depending on the claim, evidence may include:

- `TypeDef` flags/base type;
- `MethodDef` accessibility/static/virtual/newslot/final/abstract/special-name flags;
- `MethodImpl` rows;
- `InterfaceImpl` rows;
- `Property`/`Event` rows plus individual accessors;
- fields and `initonly`/literal semantics;
- generic parameter rows/constraints/variance;
- `CustomAttribute` constructors/arguments;
- CLI entry-point token;
- assembly references and identity.

Successful execution is not a substitute for correct metadata.

### L4 — Managed language/compiler enforcement

Independent C#, Visual Basic, and/or F# consumers compile against the emitted assembly.

For access/typing/overload/dispatch claims, both authorized and unauthorized consumers should be compiled where meaningful.

A negative compile test is first-class evidence. It should assert the semantic failure category rather than rely unnecessarily on a single compiler-version-specific wording.

### L5 — Runtime behavioral enforcement

A managed process executes the supported path and checks observable behavior:

- dispatch reaches the correct slot;
- setters mutate only when authorized;
- interfaces invoke the intended implementation;
- `Result` faults/returns as specified;
- `Dispose` is exactly once;
- framework runners discover and execute the emitted test;
- `Task` completion/error behavior matches the accepted async contract.

### L6 — Adversarial characterization

The certifier attempts misuse relevant to the threat model:

- wrong caller topology;
- wrong assembly/friend identity;
- case/naming collisions;
- reflection with public versus non-public flags;
- reentrant callbacks;
- concurrent `&mut`/dispose entry;
- post-dispose calls;
- malformed metadata requests;
- unsupported projection combinations;
- attempted feature substitution by generated source/native sidecars/manual invocation.

An adversarial test must not demand security properties the CLR contract does not promise.

### L7 — Distribution/system boundary

When a release claim is about installed FerrumWeave rather than a compiler primitive, the same behavior must survive the distributable package/project/template boundary from an external directory and the supported operating systems.

Repository-local success is insufficient for an installed-package claim.

## 2. Positive and negative evidence are peers

For a feature `F`, the preferred acceptance shape is:

```text
F_positive_authorized      -> PASS
F_negative_unauthorized    -> expected failure
F_metadata_exact           -> PASS
F_runtime_exact            -> PASS
F_source_mutation          -> changes relevant observable
F_substitution_guard       -> no alternate implementation path
```

Negative evidence should fail for the right reason. For example, an inaccessible property test is invalid if compilation fails only because the type name is misspelled.

## 3. Accessibility certification policy

### 3.1 Test the domain, not only the flag

An emitted `Family` MethodDef is necessary but insufficient proof of protected behavior.

Where applicable, generate independent consumers representing:

- declaring type;
- derived type in same assembly;
- unrelated type in same assembly;
- derived type in external assembly;
- unrelated type in external assembly;
- friend assembly variants.

The test matrix must also account for source-language protected receiver restrictions when those are part of the supported consumer-language contract.

### 3.2 Accessors are independent members

Properties/events must be certified through their getter/setter/add/remove MethodDefs individually.

Do not infer write authority from `PropertyInfo.CanWrite`, because it reports that a setter exists, not that the current caller is authorized to invoke it.

### 3.3 Containing/constituent accessibility matters

A public member does not create a usable external API if its containing type or signature constituent types make that surface inaccessible/incoherent.

Certifiers must attempt real consumer compilation rather than relying on a superficial public flag.

### 3.4 Friend assemblies are explicit test dimensions

If `InternalsVisibleTo` is supported in a slice:

- specified friend succeeds for the allowed internal domain;
- non-friend with otherwise identical source fails;
- private remains private through ordinary access;
- `FamANDAssem` retains the family condition;
- assembly identity mismatch fails.

## 4. Encapsulation is not a sandbox

FerrumWeave distinguishes:

```text
language/CLR accessibility
    !=
confidentiality against fully trusted in-process code
    !=
OS/process sandboxing
```

Therefore:

- `BindingFlags.NonPublic` may be used deliberately to inspect/characterize non-public metadata;
- successful non-public reflection under a runtime/tooling scenario that permits it is not automatically a defect in `Private` projection;
- secrets must not rely solely on member accessibility;
- strong naming may support identity/binding/friend contracts but must never be promoted as a trust boundary;
- executing hostile/untrusted code safely requires an explicit isolation threat model beyond ordinary CLR member modifiers.

Security defects should be filed when FerrumWeave violates an explicit stronger contract, widens a surface unexpectedly, bypasses Rust safety/invariants, or makes a false security claim.

## 5. Source causality and mutation testing

A certifier should prefer mutations that isolate the claimed causal dependency.

Examples:

- change only Rust visibility and require metadata + consumer compilation matrix to change;
- change only a setter projection from `Family` to `Assembly` and require the expected topology rows to invert;
- remove only a test-framework custom attribute and require discovery to disappear;
- change only a Rust method bound to an override and require base-reference runtime behavior/artifact bytes to change;
- alter only a friend assembly identity and require access to fail;
- remove only the Rust `Drop` behavior and require the disposal observable to change.

A test that stays green after removing the claimed causal input is evidence of a weak or substituted test.

## 6. Anti-cheat / substitution rules

A product capability cannot be certified by:

- generated per-project C#, VB, or F# implementing the behavior under test;
- native sidecar/PInvoke replacing a managed interoperability claim;
- hard-coded/baked expected output not causally owned by Rust;
- manually invoking a method when the claim is framework metadata discovery;
- reflection directly calling a private method when the claim is ordinary public API consumption;
- an upstream/oracle backend occupying the FerrumWeave product path;
- a repository-local-only setup when the claim is installed/distributable usage.

Reflection is valid when reflection itself is the asserted metadata/threat-model observable.

## 7. Concurrency and reentrancy policy

Managed aliases and callbacks introduce adversarial shapes absent from a simple single-threaded Rust test.

For any projection that enters mutable Rust state or owns cleanup:

- test simultaneous managed calls when the public contract allows concurrency;
- test reentrant callbacks separately from parallel threads;
- test duplicate/concurrent disposal where aliases may exist;
- test post-dispose behavior;
- test that GC reachability does not reauthorize expired Rust borrows/ownership;
- test thread-affinity/synchronization adapters explicitly when required.

The goal is not to make every object thread-safe. The goal is to ensure the actual supported surface cannot violate Rust safety or claim concurrency semantics it does not provide.

## 8. Framework metadata policy

For framework-driven behavior such as xUnit/NUnit/MSTest discovery, ASP.NET conventions, serializers, DI metadata, or future UI/runtime frameworks:

1. inspect exact emitted metadata;
2. run the real framework/tooling consumer;
3. mutate/remove only the relevant metadata and prove the framework observable changes;
4. reject malformed/unsupported metadata requests explicitly;
5. prohibit hidden FerrumWeave registries from substituting for ordinary framework metadata unless such a registry is itself an explicitly accepted public integration model.

## 9. Evidence-strength vocabulary

A test/contract report should distinguish:

- `primitive-proven` — a lower-level mechanism exists (for example `InterfaceImpl` emission);
- `slice-proven` — one exact end-to-end scenario is certified;
- `matrix-proven` — the relevant positive/negative topology matrix is certified;
- `system-proven` — installed/distributed workflow is certified on applicable platforms;
- `baseline-certified` — all denominator cases for that declared release boundary are closed.

Do not promote `primitive-proven` into a broader compatibility claim without the appropriate higher-level evidence.

## 10. Failure taxonomy

Systems/QA should classify first responsible divergence as one of:

- `RUST_SEMANTIC` — rustc/Rust semantic expectation;
- `PROJECTION_SELECTION` — wrong/unsupported CLR shape selected;
- `METADATA_EMISSION` — wrong PE/CLI rows/flags/signature;
- `MANAGED_LANGUAGE_ENFORCEMENT` — emitted API cannot be consumed as promised;
- `RUNTIME_BEHAVIOR` — dispatch/state/error/lifetime behavior wrong;
- `BOUNDARY_SAFETY` — ownership/aliasing/concurrency/resource invariant violated;
- `FRAMEWORK_DISCOVERY` — framework metadata/integration wrong;
- `DISTRIBUTION` — package/template/external environment divergence;
- `TEST_SUBSTITUTION` — test passes through a path other than the claimed capability;
- `THREAT_MODEL_ERROR` — test or documentation asserts a security property the accepted model does not promise.

The first responsible divergence determines Principal ownership; downstream symptoms should not be patched merely to green CI.

## 11. Release rule

Prereleases need the adversarial dimensions activated by the capabilities they claim. They do not need to complete every S01-S40 question immediately.

For `1.0.0`, the committed senior CLR and senior automation/security-QA baselines must contain no unresolved `DEFINED-BUT-RED` or `UNDEFINED` questions. Any intentionally unsupported subcase must have an executable, stable, diagnostic rejection.

This policy turns language features into falsifiable invariants rather than checklist items.
