# R10 adversarial CLR QA scope handoff

Status: **active verification scope guard for R10 / 0.1-alpha**.

Read after:

1. `docs/quality/clr-oop-senior-engineer-exam.md`;
2. `docs/quality/clr-automation-security-qa-exam.md`;
3. `docs/quality/clr-adversarial-certification-policy.md`;
4. `docs/roadmap/clr-object-model-conformance-plan.md`;
5. `docs/roadmap/r10-clr-object-model-scope-handoff.md`;
6. `tests/r10/contracts.toml`.

## Why this handoff exists

The S01-S40 security/automation-QA baseline is a 1.0 denominator. It must not inflate R10 into a complete CLR security/conformance project.

R10 activates only adversarial dimensions that are causally required by the seven 0.1 template families or by object-model slices already active in R10.

A feature does not need every conceivable hostile test. It does need the hostile/negative tests that could falsify the exact claim being made.

## R10 activation examples

### Console entry point

For `FW-R10-CLR-ENTRY-001` / `FW-R10-DX-003`, applicable adversarial evidence includes:

- metadata proves the exact static `Program.Main` and CLI entry-point MethodDef;
- `dotnet run` follows that entry point from installed-package output;
- Rust-only mutation changes managed artifact and direct runtime observable;
- no unrelated export is silently promoted to entry point;
- generated C#/native sidecar/upstream oracle cannot substitute for the product path.

Full access-control/friend-assembly matrices are irrelevant unless the console slice begins exposing such members.

### CLR surface / classlib

For `FW-R10-CLR-SURFACE-001`, `FW-R10-CLR-NAMING-001`, and `FW-R10-CLR-VISIBILITY-001`, activate the smallest relevant subset of S01-S20:

- positive and negative C#/VB/F# consumer compilation where accessibility is claimed;
- exact metadata/reflection of public/non-public surface;
- case/name collision falsifiers;
- containing/signature accessibility checks where they can leak or invalidate a public surface;
- no widening of Rust effective visibility.

Do not add general protected/friend/inheritance matrices until a current 0.1 surface actually emits those accessibility forms.

### xUnit / NUnit / MSTest

For T1 test-template support, S33-S36 and S40 are directly relevant:

- exact framework attribute identity/constructor/arguments;
- real runner discovery and execution;
- removing only the discovery metadata removes discovery;
- malformed/unsupported metadata fails explicitly;
- manual test invocation or a FerrumWeave registry cannot substitute for real runner discovery.

### web / webapi

For `FW-R10-WEB-HOST-001`, activate the adversarial dimensions corresponding to the chosen hosting seam:

- exact interface/slot metadata if an interface-based host is used;
- actual ASP.NET Core/Kestrel framework invocation rather than manual host imitation;
- Rust-only mutation changes HTTP observable;
- mutable/shared Rust state crossing managed callbacks/threads is checked for the required aliasing/thread-safety semantics;
- raw panic/unwind or unsupported boundary shapes fail according to accepted policy.

Do not pull unrelated general inheritance, event, delegate, generic, or async matrices into R10 solely because ASP.NET Core contains those concepts elsewhere.

## Negative tests are first-class R10 evidence

When a current R10 claim has a meaningful forbidden counterpart, Systems should require both.

Examples:

```text
public surface:
  authorized external consumer -> compiles/runs
  hidden/non-exported item      -> cannot be consumed through ordinary API

naming:
  unique projection             -> emits
  case/transform collision      -> stable diagnostic

test discovery:
  correct attribute             -> discovered
  attribute removed             -> not discovered

boundary safety:
  supported owned shape         -> emits/runs
  escaping unsupported borrow   -> stable diagnostic/no artifact
```

A negative test is invalid if it fails for an unrelated setup error.

## Threat-model guard

R10 must not create misleading security claims while adding visibility features.

In particular:

- `private`/`protected`/`internal` are tested as CLR/language accessibility and encapsulation;
- explicit non-public reflection is characterization, not automatically an access-control failure;
- strong naming/friend identity must not be described as publisher trust or sandboxing;
- secrets/untrusted-code isolation are not provided merely by CLR member modifiers.

If R10 introduces an actual stronger security boundary, that boundary requires its own threat model and contract rather than being inferred from access modifiers.

## Systems acceptance template

For each activated object-model capability, Systems should record:

```text
Claim:
Exact candidate SHA:
Rust/source causal input:
Positive witness:
Negative/falsifier witness:
Metadata witness:
Runtime/framework witness:
Cross-language/cross-assembly dimensions used:
Threat-model/adversarial dimension used:
Substitution guard:
Linux/Windows applicability and result:
Classification: PASS / FAIL / UNKNOWN
```

Omitted dimensions should be explicitly irrelevant, not silently forgotten.

## Concurrency rule

Before adding a QA implementation/harness, fresh-read PR #36, current R10 HEAD, open work, and target files. If another lane is implementing the same product behavior, Systems may take a non-overlapping metadata, negative-compile, framework-discovery, or adversarial verifier lane.

The new QA denominator is intended to strengthen certification, not to create a third implementation lane or exceed the existing WIP policy.
