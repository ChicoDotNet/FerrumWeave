# FerrumWeave CLR automation/security-QA senior examination

Status: **adversarial certification denominator defined; executable evidence remains incremental**.

This examination complements, rather than replaces, [`CLR/OOP senior-engineer examination`](clr-oop-senior-engineer-exam.md).

The OOP examination asks:

> What semantics does FerrumWeave promise?

This examination asks:

> Can an independent senior automation/security-QA engineer falsify those promises through authorized, unauthorized, cross-language, cross-assembly, metadata, concurrency, and adversarial paths?

FerrumWeave does not consider a language/CLR feature implemented merely because one happy-path program runs. Accessibility, dispatch, type shape, ownership, framework discovery, and metadata are observable behavior. Their negative space is part of the contract.

## Authority and allowed states

Authority order:

1. executable FerrumWeave contracts and exact-SHA CI evidence;
2. accepted ADRs and the CLR/OOP examination;
3. this examination as the adversarial acceptance denominator.

Allowed states:

- `CERTIFIED` — executable evidence proves the exact positive and negative acceptance surface claimed by the question;
- `DEFINED-BUT-RED` — the expected behavior and falsifiers are fixed, but the complete evidence matrix is not yet green;
- `UNSUPPORTED-BY-DESIGN` — the exact requested crossing is deliberately rejected, and that rejection must itself be executable and diagnostic.

`UNDEFINED` is forbidden. If an adversarial case reveals a new semantic question, architecture must decide it before the corresponding capability can be certified.

At the creation of this examination, questions are deliberately conservative: existing R04-R10 evidence may satisfy useful subcases, but a question remains `DEFINED-BUT-RED` until its full stated adversarial matrix is independently certified.

## Threat-model boundary

CLR/C# accessibility and application security are related but not identical.

`Public`, `Assembly`, `Family`, `FamORAssem`, `FamANDAssem`, and `Private` define metadata/language accessibility domains. They are important correctness and encapsulation guarantees. They are **not**, by themselves, a sandbox or confidentiality boundary against fully trusted code executing in the same process.

Therefore:

- ordinary language/compiler access tests prove accessibility semantics;
- metadata/reflection tests prove emitted CLR truth;
- `BindingFlags.NonPublic` discovery is an adversarial characterization of the runtime surface, not automatically an accessibility failure;
- secrets must not be considered protected merely because a field/property is private;
- strong-name identity must not be described as publisher authentication or a security sandbox;
- untrusted-code isolation is outside ordinary member accessibility and requires an appropriate process/OS/runtime isolation model.

Relevant platform references include Microsoft Learn documentation for C# accessibility, `InternalsVisibleToAttribute`, `System.Reflection.BindingFlags`, .NET secure-coding guidance, and strong-named assemblies.

## Required certification dimensions

When relevant to a capability, certification should combine:

```text
POSITIVE
  authorized consumer compiles and executes

NEGATIVE
  unauthorized consumer fails at the correct boundary

METADATA
  independent inspection confirms exact CLR flags/rows/signatures

CROSS-LANGUAGE
  C# + Visual Basic + F# agree on the same CLR contract

CROSS-ASSEMBLY
  same/external/derived/friend topologies agree

ADVERSARIAL
  reflection/reentrancy/concurrency/identity behavior matches the threat model

SOURCE-CAUSAL
  changing only the Rust/projection source changes the relevant observable
```

Not every question needs every dimension. The certifier must state which dimensions apply and why omitted dimensions are irrelevant.

---

# Block S1 — Accessibility matrix

## S01 — Does every CLR accessibility level authorize exactly the intended caller topology?

**Expected behavior:** `Public`, `Assembly`, `Family`, `FamORAssem`, `FamANDAssem`, and `Private` must match the accepted CLR accessibility table. Test at least declaring type, derived type same assembly, unrelated type same assembly, derived type external assembly, and unrelated type external assembly where meaningful.

**Required falsifier:** at least one consumer that would compile if FerrumWeave accidentally widened the member must fail to compile.

**State:** `DEFINED-BUT-RED`.

## S02 — Are property/event accessors certified independently rather than by aggregate member flags?

**Expected behavior:** a public getter with private/protected/internal setter must expose exactly those accessor-specific domains. `CanRead`/`CanWrite` or the existence of a `Property` row is insufficient evidence.

**Required proof:** inspect `get_*` and `set_*` MethodDefs individually and compile authorized/unauthorized callers.

**State:** `DEFINED-BUT-RED`.

## S03 — Does containing-type accessibility constrain member accessibility correctly?

**Expected behavior:** a nominally public member inside a non-public type must not become an externally usable public API merely because the member MethodDef is marked public. Constituent parameter/return types must not create an accessibility leak either.

**State:** `DEFINED-BUT-RED`.

## S04 — Can any CLR projection accidentally widen effective Rust visibility?

**Expected behavior:** never. Every exported member/type must satisfy the non-widening rule from ADR 0006. Public re-exports are honored; syntactic `pub` hidden behind private Rust ancestry is not promoted accidentally.

**Required falsifier:** Rust source shapes that would expose a hidden item under token-only `pub` analysis.

**State:** `DEFINED-BUT-RED`.

---

# Block S2 — Cross-assembly and protected-family enforcement

## S05 — Does an unrelated external assembly fail to consume `Assembly`/internal members?

**Expected behavior:** normal C#/VB/F# compilation fails without friend access. Reflection characterization is separate and does not turn the compile-time failure into a defect.

**State:** `DEFINED-BUT-RED`.

## S06 — Can an external derived type consume `Family` while an unrelated external type cannot?

**Expected behavior:** yes, subject to the CLR/language protected receiver rules.

**Required falsifier:** an unrelated external consumer and an invalid receiver topology from a derived consumer.

**State:** `DEFINED-BUT-RED`.

## S07 — Does `FamANDAssem` require both family relationship and authorized assembly domain?

**Expected behavior:** yes. Same-assembly unrelated code fails; external derived code fails; same-assembly derived code succeeds. Friend-assembly behavior is tested separately in S11.

**State:** `DEFINED-BUT-RED`.

## S08 — Do C#, Visual Basic, and F# observe an interoperable accessibility surface?

**Expected behavior:** supported languages must agree on the CLR accessibility contract. Language-specific syntax may differ, but FerrumWeave must not emit metadata that is usable from one supported language only because another sees an ambiguous or incompatible surface.

**State:** `DEFINED-BUT-RED`.

---

# Block S3 — Friend assemblies and assembly identity

## S09 — Does `InternalsVisibleTo` grant exactly the intended internal/family-combination access?

**Expected behavior:** the specified friend assembly gains the CLR-permitted internal access and no unrelated assembly does.

**Required proof:** positive friend consumer plus negative same-shaped non-friend consumer.

**State:** `DEFINED-BUT-RED`.

## S10 — Does friendship leave `Private` inaccessible through ordinary language access?

**Expected behavior:** yes. Friend access must not silently widen private members.

**State:** `DEFINED-BUT-RED`.

## S11 — Does friend access to `FamANDAssem` still require derivation?

**Expected behavior:** yes. Friendship extends the assembly side of the accessibility domain; it does not remove the family/derived requirement.

**State:** `DEFINED-BUT-RED`.

## S12 — Is friend assembly identity resolved exactly rather than approximately?

**Expected behavior:** a wrong assembly identity must not inherit friend rights. Any strong-name/public-key requirements of the active .NET toolchain must be followed exactly where applicable.

**Security note:** assembly/strong-name identity is not treated as publisher authentication or a sandbox.

**State:** `DEFINED-BUT-RED`.

---

# Block S4 — Metadata truth versus language observations

## S13 — Do emitted TypeDef/MethodDef/Field/Property/Event flags match the accepted contract exactly?

**Expected behavior:** independent metadata/reflection inspection must agree with the source/projection contract. Runtime success alone is insufficient.

**State:** `DEFINED-BUT-RED`.

## S14 — Can mixed-access properties/events be mis-certified as uniformly public?

**Expected behavior:** no. Certifiers must inspect each accessor/add/remove method, not only aggregate reflection convenience properties.

**State:** `DEFINED-BUT-RED`.

## S15 — Are `Virtual`, `NewSlot`, `Final`, `Abstract`, `MethodImpl`, and interface-slot bindings exact?

**Expected behavior:** dispatch metadata must match ADR 0008. Same-name methods are not accepted as evidence of true override.

**State:** `DEFINED-BUT-RED`.

## S16 — Do compile-time observations, reflection, and actual runtime dispatch agree?

**Expected behavior:** no layer may contradict another. A method that reflection reports as a true override must dispatch as that inherited slot from an independent consumer.

**State:** `DEFINED-BUT-RED`.

---

# Block S5 — Reflection and security-boundary characterization

## S17 — Does ordinary public reflection expose only the intended public surface?

**Expected behavior:** public/default reflection queries must not accidentally present non-public FerrumWeave implementation members as public API.

**State:** `DEFINED-BUT-RED`.

## S18 — Does explicit non-public reflection behave according to .NET rather than FerrumWeave claiming secrecy?

**Expected behavior:** tests may use `BindingFlags.NonPublic` to characterize discoverability/invocation where the runtime permits it. Such discovery is not itself a failure of CLR `Private` metadata.

**State:** `DEFINED-BUT-RED`.

## S19 — Does documentation distinguish encapsulation/accessibility from confidentiality and sandboxing?

**Expected behavior:** yes. FerrumWeave must never claim that a private field makes a secret inaccessible to fully trusted in-process code.

**State:** `DEFINED-BUT-RED` until documentation and executable characterization are linked to relevant public claims.

## S20 — Do security tests use the correct threat model rather than impossible guarantees?

**Expected behavior:** a failing attempt by ordinary unauthorized code is a valid accessibility assertion; a successful full-trust reflection probe must not be mislabeled as a compiler security vulnerability unless it violates a stronger explicit FerrumWeave security contract.

**State:** `DEFINED-BUT-RED`.

---

# Block S6 — Inheritance and dispatch attacks

## S21 — Is a default sealed FerrumWeave class actually non-derivable from managed code?

**Expected behavior:** independent consumer compilation/type emission must reject derivation according to normal CLR rules; metadata must mark the type sealed/final as required.

**State:** `DEFINED-BUT-RED`.

## S22 — Does an explicit override reuse the intended inherited slot rather than creating `NewSlot`?

**Expected behavior:** base-reference invocation must reach the Rust-backed override. Metadata must identify the inherited slot exactly.

**State:** `DEFINED-BUT-RED`.

## S23 — Can a non-virtual method be accidentally substituted through polymorphic dispatch?

**Expected behavior:** no. Same-name derived members may hide according to CLR rules only when explicitly supported; they must not mutate the base slot semantics.

**State:** `DEFINED-BUT-RED`.

## S24 — Is an explicit interface implementation callable only through the intended interface slot/surface?

**Expected behavior:** exact `MethodImpl`/interface mapping must be observable; accidental public-name exposure or wrong-slot dispatch fails certification.

**State:** `DEFINED-BUT-RED`.

---

# Block S7 — State integrity and invariant preservation

## S25 — Can state be mutated only through the accessibility domain promised by the projected setter/member?

**Expected behavior:** authorized paths succeed; unauthorized ordinary-language paths fail. Internal compiler adapters must not widen the public surface merely for convenience.

**State:** `DEFINED-BUT-RED`.

## S26 — Does a read-only or restricted property avoid accidental writable storage exposure?

**Expected behavior:** FerrumWeave must not satisfy a property contract by exposing a public mutable backing field or another wider mutation path unless that path is independently part of the public contract.

**State:** `DEFINED-BUT-RED`.

## S27 — Do exceptional paths preserve object invariants during construction/mutation?

**Expected behavior:** a managed exception/panic translation during a multi-step operation must not leave externally observable Rust state in a contractually impossible partially-mutated form.

**Required proof:** failure injection at relevant adapter boundaries when such multi-step state exists.

**State:** `DEFINED-BUT-RED`.

## S28 — Can CLR adapters bypass Rust invariants by writing projected storage directly?

**Expected behavior:** no. Generated adapters may lower representation, but they must enter the Rust-authorized operation/invariant path unless an ADR explicitly defines equivalent field-level semantics.

**State:** `DEFINED-BUT-RED`.

---

# Block S8 — Ownership, reentrancy, disposal, and concurrency

## S29 — Can two managed aliases obtain simultaneous unrestricted `&mut` access?

**Expected behavior:** no. Managed entry must serialize/enforce exclusivity or reject the surface.

**State:** `DEFINED-BUT-RED`.

## S30 — Can a managed callback re-enter Rust while an exclusive Rust operation is active?

**Expected behavior:** not unless the accepted adapter semantics make reentrancy safe. Reentrant callbacks must be included in exclusivity testing rather than only parallel threads.

**State:** `DEFINED-BUT-RED`.

## S31 — Does GC reachability accidentally turn non-`Send`/non-`Sync` Rust state into unrestricted cross-thread state?

**Expected behavior:** no. A managed reference being reachable is not evidence of Rust thread safety. Unsafe crossings require an explicit affinity/synchronization adapter or rejection.

**State:** `DEFINED-BUT-RED`.

## S32 — Is cleanup exactly once under duplicate aliases, concurrent dispose, exceptions, and post-dispose calls?

**Expected behavior:** resource-owning projections must preserve the accepted ADR 0007/R07 cleanup semantics. Concurrent/duplicate cleanup must not double-drop, resurrect, or permit use-after-release.

**State:** `DEFINED-BUT-RED`; existing R07 exactly-once evidence is a certified primitive, not yet the complete adversarial matrix.

---

# Block S9 — Metadata-driven framework discovery

## S33 — Are required custom attributes emitted on exactly the intended target with the intended constructor/arguments?

**Expected behavior:** independent reflection verifies exact managed attribute identity and arguments.

**State:** `DEFINED-BUT-RED`.

## S34 — Do xUnit/NUnit/MSTest discover FerrumWeave tests through real framework metadata?

**Expected behavior:** yes. Calling a test method manually or through a FerrumWeave-specific harness is not discovery evidence.

**State:** `DEFINED-BUT-RED`.

## S35 — Does removing only the discovery attribute make the runner stop discovering the test?

**Expected behavior:** yes. This source-only mutation is a key causality falsifier proving that runner discovery depends on emitted metadata rather than fixture naming or hidden registration.

**State:** `DEFINED-BUT-RED`.

## S36 — Do malformed, duplicated, or unsupported attribute projections fail explicitly?

**Expected behavior:** FerrumWeave must not silently emit a different attribute shape, drop required arguments, or compensate with a hidden runner hook.

**State:** `DEFINED-BUT-RED`.

---

# Block S10 — Fail-closed boundaries, compatibility, and anti-cheat certification

## S37 — Do unsupported projections fail closed instead of widening/changing semantics to make a test green?

**Expected behavior:** yes. Unsupported accessibility, lifetime, type, generic, delegate, async, or framework metadata combinations must reject diagnostically rather than degrade to a more permissive or less precise shape.

**State:** `DEFINED-BUT-RED`.

## S38 — Are accessibility/metadata changes observable compatibility changes rather than silent implementation details?

**Expected behavior:** changing public/protected/internal shape, virtual slots, signatures, attributes used by frameworks, or other externally relevant metadata must be detectable by compatibility tests. A release must not silently widen or narrow API contracts without deliberate versioning/migration treatment.

**State:** `DEFINED-BUT-RED`.

## S39 — Are assembly identity, friend access, and strong naming described without false security claims?

**Expected behavior:** yes. They may participate in CLR identity/binding/friend-access contracts, but FerrumWeave must not equate strong naming with code trust, publisher authentication, confidentiality, or isolation.

**State:** `DEFINED-BUT-RED`.

## S40 — Can any test pass through a substitute implementation rather than the claimed FerrumWeave capability?

**Expected behavior:** no. Generated per-project C#/VB/F#, native sidecars/PInvoke as implementation, reflection hacks that bypass the feature under test, baked expected values, manual invocation replacing framework discovery, or an upstream backend/oracle cannot certify the product claim.

**State:** `DEFINED-BUT-RED` at the complete examination level. Existing anti-substitution/source-causality contracts provide important certified primitives.

---

# Accessibility test-topology minimum

For any feature claiming CLR accessibility beyond a trivial `public` method, the test generator/certifier should be able to construct the applicable subset of:

| Consumer topology | Public | Assembly | Family | FamORAssem | FamANDAssem | Private |
| --- | --- | --- | --- | --- | --- | --- |
| declaring type | allow | allow | allow | allow | allow | allow |
| derived / same assembly | allow | allow | allow | allow | allow | deny ordinary direct private access |
| unrelated / same assembly | allow | allow | deny | allow | deny | deny |
| derived / external assembly | allow | deny | allow | allow | deny | deny |
| unrelated / external assembly | allow | deny | deny | deny | deny | deny |

Friend-assembly cases are additional rows, not replacements for the table above. Protected instance-member receiver restrictions must be tested explicitly where the source language imposes them.

The table is a conceptual CLR/C# acceptance matrix. Exact emitted metadata names use CLR terminology rather than C# keywords.

# Property/accessor example

A projected property such as:

```text
Property: Name
get_Name: Public
set_Name: Family
```

is not certified by proving only that `Name` is discoverable or that `PropertyInfo.CanWrite == true`.

The certifier must prove at least:

- public callers can read;
- unauthorized callers cannot compile a write;
- authorized derived callers can write through a valid receiver topology;
- reflection sees the exact getter/setter MethodAttributes;
- the same assembly behaves consistently from the supported managed languages;
- explicit non-public reflection behavior is characterized separately from ordinary accessibility.

# Certification rule

A language/CLR feature is considered certified only when the evidence protects both the intended behavior and the relevant forbidden behavior.

In compact form:

```text
FEATURE CERTIFICATION
    = positive semantics
    + negative semantics
    + metadata truth
    + applicable topology/language matrix
    + source causality
    + threat-model honesty
```

A feature is not complete because FerrumWeave can emit a flag. It is complete when an independent certifier can demonstrate that changing, omitting, widening, narrowing, bypassing, or substituting that flag/semantic causes the expected test to fail for the correct reason.
