# R09 Rust verification authority cutover

This document records the batch cutover from dual Python/Rust verification to Rust-authoritative verification for the contracts that have already reached observable parity.

## Cutover baseline

Validated branch: `feat/r10-developer-experience`

Validated code HEAD before the cutover: `34b4a56a66d9ac19328e660dec9ef92a673e961b`

Independent evidence on that HEAD:

- Rust CI #753: success
  - `fmt + clippy`: success
  - functional/ledger gates: success
  - code coverage: success
  - `cargo test --workspace --tests --locked` on Ubuntu: success
  - `cargo test --workspace --tests --locked` on Windows: success
- FerrumWeave codegen backend convergence #387: success on Ubuntu and Windows.

The direct Rust function-call replay completed on Ubuntu in 12.70 s and independently preserved the managed `call` opcode, helper-semantics mutation, export rename and managed outputs.

## DUAL contracts accepted for batch cutover

| Retiring Python verifier | Rust-authoritative evidence | Protected observable |
| --- | --- | --- |
| `eng/verification/check_managed_consumption_causality.py` | `tests/evidence_classification.rs::r05_managed_consumption_source_causality_census_is_complete` | Exact five-contract R05 census, implemented/source-causal state and replayable proof |
| `tools/quality/check_functional_coverage.py` | `tests/functional_coverage_policy.rs` plus the authoritative `cargo test --workspace --tests` run | Ledger validity, mapped Rust tests, minimum functional coverage and executable mapped tests |
| `eng/verification/verify_i32_local_flow_backend.py` | `tests/i32_local_flow_backend.rs` | Argument/local flow, fixed managed observables, export rename and artifact causality |
| `eng/verification/verify_i32_arithmetic_backend.py` | `tests/i32_arithmetic_backend.rs` | `+`/`-` semantics, fixed managed observables, export rename and artifact causality |
| `eng/verification/verify_i32_control_flow_backend.py` | `tests/i32_control_flow_backend.rs` | `==`/`!=` branch semantics on both branch sides, export rename and artifact causality |
| `eng/verification/verify_i32_overflow_backend.py` | `tests/i32_overflow_backend.rs` | Checked i32 overflow takes the required non-success path instead of silently wrapping |
| `eng/verification/verify_direct_rust_function_call_backend.py` | `tests/direct_rust_function_call_backend.rs` | Real managed `call` opcode, helper-semantics mutation, export rename, managed outputs and artifact causality |

These seven contracts are accepted as `DUAL` before the cutover because Python and Rust independently protected the same material observable and both verification lanes were green on the baseline HEAD.

## Batch cutover protocol

The migration is intentionally performed as one controlled batch rather than seven independent retirements:

1. Remove only these seven Python verifiers from the critical CI path while keeping the files present.
2. Require the complete Rust CI and remaining backend-convergence suites to pass on Ubuntu and Windows.
3. If the CI-only cutover is green, delete all seven retired Python files together.
4. Re-run the complete CI after deletion.
5. Record the seven contracts as `RUST_AUTHORITATIVE`.

No Python verifier that is still `PYTHON_ONLY` or `RUST_PARTIAL` is part of this batch. In particular, R02/R03 historical oracle evidence, the line-coverage policy script, the codegen-boundary verifier, aggregation verifiers and managed-consumption E2E verifiers remain untouched until their own Rust parity exists.

## Authority boundary

After successful completion of this protocol, Rust owns the product verification authority for these seven contracts. Python is no longer retained as a product gate or duplicate executable witness for them. Historical/oracle Python remains separate where it still carries independent evidentiary value.
