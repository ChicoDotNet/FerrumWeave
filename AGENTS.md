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

Do **not** treat historical `dotnet new rust`, `sdk/templates/rust/`, or `tests/r08_dotnet_new.rs` as the R10 target merely because they remain in the tree. They are R08 historical evidence until an explicit, certified migration supersedes or retires them.

The first active R10 contract is `FW-R10-DX-003` in `tests/r10/contracts.toml`.

## Non-negotiable engineering rules

- Product compilation is `.rsproj -> FerrumWeave.Sdk -> rustc -> FerrumWeave CodegenBackend -> CIL/metadata -> CoreCLR`.
- `rustc_codegen_clr` is prior-art/differential-oracle evidence only, never product-backend proof.
- Work known behavior through `RED -> GREEN -> REFACTOR -> CERTIFY`.
- Preserve observable contracts when migrating harnesses; classify migrations as `COPY`, `TRANSFORM`, or `ADAPT` where useful.
- Never delete a Python verifier while its inventory state is `PYTHON_ONLY`, `RUST_PARTIAL`, or merely related-but-not-equivalent Rust coverage. Follow the authority-cutover evidence in `docs/quality/`.
- Cross-platform product claims require Windows and Linux evidence.
- Do not reduce tests, contracts, or coverage denominators to make CI green.
- Do not claim target/release behavior from generated files alone; prove the installed/external developer workflow where R10 requires it.

## Branch discipline

- Short-lived branches and ordinary feature PRs target `dev`.
- `dev` keeps detailed causal/TDD/fix history.
- `main` keeps milestone/release promotion history.
- Promotion to `main` should be selective/squashed rather than replaying all detailed `dev` history.
- Do not force-update shared branches without explicit maintainer authorization.

## Safe handoff requirement

Before ending an iteration, leave enough repository evidence for the next agent to identify:

- the active RED/GREEN contract;
- the exact certified SHA;
- the CI runs that certify Windows/Linux and relevant quality gates;
- the causal observable protected by the change;
- historical/oracle evidence intentionally preserved;
- the next smallest contract.

If those facts exist only in chat context, the iteration is not ready to hand off.
