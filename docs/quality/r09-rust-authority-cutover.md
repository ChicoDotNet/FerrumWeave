# R09 Rust verification authority cutover

This document records the batch cutover from dual Python/Rust verification to Rust-authoritative verification for the contracts that reached observable parity.

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

| Retired Python verifier | Rust-authoritative evidence | Protected observable |
| --- | --- | --- |
| `eng/verification/check_managed_consumption_causality.py` | `tests/evidence_classification.rs::r05_managed_consumption_source_causality_census_is_complete` | Exact five-contract R05 census, implemented/source-causal state and replayable proof |
| `tools/quality/check_functional_coverage.py` | `tests/functional_coverage_policy.rs` plus the authoritative `cargo test --workspace --tests` run | Ledger validity, mapped Rust tests, minimum functional coverage and executable mapped tests |
| `eng/verification/verify_i32_local_flow_backend.py` | `tests/i32_local_flow_backend.rs` | Argument/local flow, fixed managed observables, export rename and artifact causality |
| `eng/verification/verify_i32_arithmetic_backend.py` | `tests/i32_arithmetic_backend.rs` | `+`/`-` semantics, fixed managed observables, export rename and artifact causality |
| `eng/verification/verify_i32_control_flow_backend.py` | `tests/i32_control_flow_backend.rs` | `==`/`!=` branch semantics on both branch sides, export rename and artifact causality |
| `eng/verification/verify_i32_overflow_backend.py` | `tests/i32_overflow_backend.rs` | Checked i32 overflow takes the required non-success path instead of silently wrapping |
| `eng/verification/verify_direct_rust_function_call_backend.py` | `tests/direct_rust_function_call_backend.rs` | Real managed `call` opcode, helper-semantics mutation, export rename, managed outputs and artifact causality |

These seven contracts were accepted as `DUAL` before cutover because Python and Rust independently protected the same material observable and both verification lanes were green on the baseline HEAD.

## Executed batch cutover

The migration was performed as one controlled batch rather than seven independent retirements.

### Stage 1 — remove Python from the critical CI path

CI-only cutover HEAD: `5d03f16062808f23eb9a047511c4b277bdaa96d5`

Changes:

- removed the five already-dual backend Python E2E steps from `.github/workflows/codegen-backend-convergence.yml`;
- removed the Python functional-coverage / managed-consumption gate job from `.github/workflows/rust-ci.yml`;
- kept all seven Python files present during this validation stage.

Evidence after the CI-only cutover:

- Rust CI #756: success
  - `fmt + clippy`: success
  - code coverage: success
  - Ubuntu build/unit/integration: success
  - Windows build/unit/integration: success
- FerrumWeave codegen backend convergence #390: success on Ubuntu and Windows.

This proves the critical CI path no longer depended on the retired Python witnesses before any file was deleted.

### Stage 2 — delete the retired Python witnesses

Post-delete HEAD: `a5a1a6f34f6a4c924929f99febac6bae151c7be6`

Deleted:

- `eng/verification/check_managed_consumption_causality.py`
- `tools/quality/check_functional_coverage.py`
- `eng/verification/verify_i32_local_flow_backend.py`
- `eng/verification/verify_i32_arithmetic_backend.py`
- `eng/verification/verify_i32_control_flow_backend.py`
- `eng/verification/verify_i32_overflow_backend.py`
- `eng/verification/verify_direct_rust_function_call_backend.py`

Evidence with those files physically absent:

- Rust CI #763: success
  - `fmt + clippy`: success
  - code coverage: success
  - Ubuntu build/unit/integration: success
  - Windows build/unit/integration: success
- FerrumWeave codegen backend convergence #397: success on Ubuntu and Windows.

## Result

The seven contracts in this batch are now `RUST_AUTHORITATIVE`.

Rust owns their product verification authority. Python is no longer retained as a product gate or duplicate executable witness for them.

No verifier that was still `PYTHON_ONLY` or `RUST_PARTIAL` was removed. In particular, R02/R03 historical oracle evidence, the line-coverage policy script, the codegen-boundary verifier, aggregation verifiers and managed-consumption E2E verifiers remain untouched until their own Rust parity exists.

## Promotion consequence

The branch now has a smaller critical verification surface with the same protected observables and two independent green cutover gates: one before deletion and one after deletion. The next promotion step can therefore operate on the Rust-authoritative tree rather than carrying the retired duplicate Python witnesses forward into `dev`.
