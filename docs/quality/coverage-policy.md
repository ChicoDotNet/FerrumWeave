# Coverage policy

FerrumWeave treats coverage as an engineering signal, not a vanity score.

## Rust code coverage

Line coverage is the initial code-coverage metric.

- **Hard CI floor:** 80%.
- **Preferred operating band:** 80%–96%.
- **Above 96%:** allowed and reported as above the preferred band, not failed.

The upper edge is intentionally not a failure gate. Very small components can reach 100% naturally, and FerrumWeave will not introduce untested code or low-value tests merely to force a percentage downward.

As the workspace grows, the floor should be evaluated per materially testable crate or component where tooling can report that boundary independently. A high aggregate percentage must not hide a weak component.

## Functional coverage

Functional coverage measures implemented, passing executable contracts against the declared functional-contract ledger.

- **Hard CI floor:** 96%.
- **Target:** 100%.
- Every known functional contract belongs in the ledger, including contracts not yet implemented.
- A contract counts as covered only when its mapped executable test exists and passes.

The initial R00 ledger contains one contract: running the bootstrap binary must succeed and print `Hello FerrumWeave`.

## What coverage does not mean

Neither metric proves correctness by itself. Tests must continue to trace to observable behavior, public contracts, regression risks, edge conditions, or other identifiable reasons.

FerrumWeave will not:

- add meaningless tests only to raise a percentage;
- add intentionally uncovered code only to lower a percentage;
- remove known functional contracts from the denominator to make the report look healthier;
- change expected behavior merely because the implementation currently differs.

Coverage thresholds can evolve as the project gains evidence, but lowering a gate should be an explicit engineering decision rather than a convenient response to a failing build.
