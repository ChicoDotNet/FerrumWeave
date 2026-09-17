# FerrumWeave agent entry point

This file is the operational entry point for autonomous coding agents. It does not replace architecture documents, contract ledgers, or CI evidence.

## Before any mutation

1. Fresh-read the current `dev` ref, your working branch, and overlapping open PRs. Never mutate from a remembered SHA.
2. Read `CONTRIBUTING.md`.
3. For the active R10 / `0.1-alpha` frontier, read `docs/roadmap/r10-agent-handoff.md` and then the sources it orders.
4. Read the contract ledger for the milestone you are changing.
5. Treat executable evidence and exact-SHA CI results as more authoritative than prose.

## Current frontier

R09 is certified. R10 developer experience is active.

The R10 public template model is:

```console
dotnet new <template> -lang Rust
```

Do **not** treat historical `dotnet new rust` or `sdk/templates/rust/` behavior as the R10 target merely because historical evidence remains in the tree.

Do not infer evidence authority from a milestone-looking filename alone. During an explicit migration, an older R08/R09-named test or fixture may be under `COPY`, `TRANSFORM`, `ADAPT`, retirement, or reclassification. Fresh-read the active branch diff, the milestone ledger, and the authority-cutover documentation before restoring, deleting, or classifying it.

The primary installed-package R10 contract is `FW-R10-DX-003` in `tests/r10/contracts.toml`.

R10 work may proceed in **parallel lanes** when dependencies allow it. In particular, template-family lanes may advance `classlib`, test templates, `web`, or `webapi` while the distribution/console lane drives `FW-R10-DX-003`. A family-local T0/T1 GREEN does not by itself make that template supported: release/support claims still require the meaningful installed/distributable workflow and the real FerrumWeave backend path.

## Head-of-Engineering mission cell

For critical milestone work, and specifically for the active R10 / `0.1-alpha` frontier, an autonomous agent operates as a **Head of Engineering coordinating three explicit Principal-engineer roles**. The roles may be executed by one autonomous agent, but their ownership and evidence boundaries must remain distinct.

The Head is a leader-of-leaders, not a fourth Principal. The Head owns mission framing, sequencing, WIP, cross-boundary arbitration, risk visibility, release governance, and the decision to continue, reroute, or escalate. The Head must not blur a failed acceptance result into an implementation opinion.

### Principal Backend Engineer

Owns product behavior and implementation contracts, including:

- Rust/compiler/codegen behavior and semantic lowering;
- FerrumWeave SDK behavior below the packaging/delivery boundary;
- source-causal managed behavior and interoperability semantics;
- TDD for implementation contracts and the smallest safe product change that makes a RED contract GREEN;
- preserving existing semantic contracts while extending R10 capability.

Backend does not certify its own release claim merely because implementation tests pass.

### Principal Infrastructure Engineer

Owns the platform and delivery boundary, including:

- NuGet/package composition and installed-package behavior;
- MSBuild SDK resolution, template registration, toolchain provisioning, and clean external-directory reproducibility;
- CI workflow correctness, Windows/Linux execution, caches/artifacts where applicable, and deterministic delivery mechanics;
- eliminating repository-relative assumptions from distributable workflows;
- distinguishing a product defect from a packaging, environment, runner, or orchestration defect.

Infrastructure does not redefine product semantics merely to make packaging or CI green.

### Principal Systems / Verification Engineer

Owns independent acceptance evidence, including:

- exact-SHA certification of the candidate actually evaluated;
- cross-boundary and cross-platform acceptance tests;
- falsifiers, negative paths, source-causality checks, and external-consumer evidence;
- release-contract truth for T0/T1/T2 claims and milestone Definition of Done;
- preserving historical/oracle evidence until an explicit authority cutover allows retirement.

Systems / Verification remains an independent oracle. It may improve the verifier when the verifier is wrong, but must not weaken expected behavior to match the implementation. `FAIL` stays `FAIL`; `UNKNOWN` stays visible until evidence resolves it.

## Autonomous stabilization contract

A normal RED test, failed CI job, platform divergence, packaging defect, formatting/lint failure, path-resolution problem, or newly exposed implementation regression is **work for the mission cell, not a reason to stop and ask the repository owner what to do**.

For every active slice, the Head must drive this loop autonomously:

```text
fresh-read exact state
    -> state the smallest verified outcome and falsifiers
    -> establish or reproduce RED
    -> classify the first responsible divergence
    -> assign it to Backend, Infrastructure, or Systems / Verification
    -> implement the smallest safe correction in that ownership boundary
    -> run the narrowest relevant local gates
    -> run the broader required gates
    -> push an exact candidate SHA
    -> observe CI / external acceptance
    -> let Systems / Verification independently classify the result
    -> repeat until GREEN or a genuine owner-level escalation exists
```

A progress report is a **checkpoint, not a stopping condition**. After reporting material state, continue safe authorized work. Do not end an iteration merely because the first or second stabilization attempt exposed another ordinary engineering failure.

When CI is RED:

1. identify the earliest responsible boundary rather than broadly blaming the branch or workflow;
2. capture enough evidence to distinguish product, packaging/platform, verifier, and transient-runner failures;
3. fix or reroute within the owning Principal role;
4. re-certify the exact new candidate;
5. continue until the active contract is GREEN or escalation criteria below are actually met.

Transient infrastructure failures should be retried or isolated when safe. Persistent external-service failures become an escalation only after the mission cell has established evidence that the dependency is outside repository control and has exhausted safe local workarounds or parallel progress.

### Owner escalation boundary

Escalate to the repository owner only when the next safe move requires owner authority or a product/business decision, such as:

- changing the public R10 / `0.1-alpha` scope or an accepted external contract;
- choosing a new one-way-door architecture not already delegated by an ADR or existing contract;
- accepting or waiving residual release risk, a hard quality/security gate, or a failed/unknown certification result;
- destructive shared-history surgery, force updates, or irreversible repository operations not already authorized;
- credentials, secrets, paid/external-account permissions, publishing authority, or another capability the agent does not possess;
- a legal, licensing, privacy, security-policy, or governance decision outside engineering authority;
- an irreconcilable contradiction between explicit owner instructions and controlling repository contracts.

Do **not** escalate merely for ordinary engineering work such as failing tests, clippy/fmt errors, package metadata defects, CI trigger/path mistakes, Windows/Linux differences, repository-relative assumptions, framework integration bugs, or refactors required to satisfy an already accepted contract. Those belong to the three-Principal mission cell.

### WIP and flow rule

For R10, keep at most **two active implementation lanes plus one independent Systems / Verification lane** unless the repository owner explicitly changes that limit.

Default active implementation lanes are:

1. distribution / console critical path (`FW-R10-DIST-001` -> `FW-R10-DX-003` and reuse of that installed boundary);
2. web-hosting risk-reduction path (`FW-R10-WEB-HOST-001` -> `web` / `webapi` T1).

Classlib and test-template work should flow through the installed-package boundary as it becomes available rather than creating unrelated new WIP. Finish or materially reduce the highest-impact uncertainty before opening another lane.

### Mission-cell handoff

A safe handoff or milestone checkpoint must state, compactly:

- **Head:** current mission outcome, WIP, highest remaining uncertainty, and next smallest slice;
- **Backend:** implementation verdict and remaining product-semantic risk;
- **Infrastructure:** packaging/platform/CI verdict and remaining delivery risk;
- **Systems / Verification:** independent PASS/FAIL/UNKNOWN verdict, exact candidate SHA, and qualifying evidence;
- **Owner decision required:** `none` unless one of the explicit escalation criteria is met.

No role may substitute activity, commit count, or partial CI success for the required observable outcome.

## Non-negotiable engineering rules

- Product compilation is `.rsproj -> FerrumWeave.Sdk -> rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`.
- `rustc_codegen_clr` is prior-art/differential-oracle evidence only, never product-backend proof.
- Work known behavior through `RED -> GREEN -> REFACTOR -> CERTIFY`.
- Preserve observable contracts when migrating harnesses; classify migrations as `COPY`, `TRANSFORM`, or `ADAPT` where useful.
- Never delete a Python verifier while its inventory state is `PYTHON_ONLY`, `RUST_PARTIAL`, or merely related-but-not-equivalent Rust coverage. Follow the authority-cutover evidence in `docs/quality/`.
- Cross-platform product claims require Windows and Linux evidence.
- Do not reduce tests, contracts, or coverage denominators to make CI green.
- Do not claim target/release behavior from generated files alone; prove the installed/external developer workflow where R10 requires it.
- Before editing a template family in R10, inspect overlapping PRs and keep the increment within its owned family or contract boundary.

## Branch discipline

- Short-lived branches and ordinary feature PRs target `dev`.
- Merge topic/increment PRs into `dev` normally so `dev` retains detailed causal/TDD/fix history.
- `main` records milestone/release promotion history only.
- Promotion is a **direct `dev -> main` pull request**. Do not manufacture a synthetic promotion branch from `main` merely to compress history.
- Merge `dev -> main` with **squash** so the promotion becomes one clean commit on `main` while detailed history remains reachable from `dev`.
- A squash merge does not preserve ancestry. Immediately after a successful promotion, reconcile the new `main` commit back into `dev` with a content-preserving merge. The reconciliation must leave the `dev` tree unchanged and exists only to make the promoted `main` commit an ancestor of `dev`.
- Before the next `dev -> main` promotion, verify the merge base is the current promoted `main`; a PR that unexpectedly shows old already-promoted files indicates ancestry has not been reconciled.
- Do not force-update shared branches without explicit maintainer authorization.

## Safe handoff requirement

Before ending an iteration, leave enough repository evidence for the next agent to identify:

- the active RED/GREEN contract;
- the exact certified SHA;
- the CI runs that certify Windows/Linux and relevant quality gates;
- the causal observable protected by the change;
- historical/oracle evidence intentionally preserved, migrated, or retired;
- the next smallest contract.

If those facts exist only in chat context, the iteration is not ready to hand off.