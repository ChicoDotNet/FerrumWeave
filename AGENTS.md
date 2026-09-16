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