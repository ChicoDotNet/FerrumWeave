# Python → Rust verification inventory

This file describes the **current** verification-authority boundary. Git history preserves the migration chronology; the current tree should not retain duplicate executable witnesses or historical narration solely for provenance.

A Python verifier remains only while it protects a current observable that Rust does not yet reproduce with equivalent executable evidence. Once Rust owns that observable on the supported CI platforms, the Python verifier is removed from the critical path and deleted from the tree.

## States

- `PYTHON_ONLY`: Python still owns the material contract.
- `RUST_PARTIAL`: Rust protects part of the contract, but a material observable remains Python-only.
- `DUAL`: temporary transition state while Python and Rust independently protect the same contract before cutover.
- `RUST_AUTHORITATIVE`: Rust owns the product gate; no duplicate Python witness is required.

## Rust-authoritative contracts

| Contract family | Rust authority | Current Python status |
| --- | --- | --- |
| R05 managed-consumption census | `tests/evidence_classification.rs` | no Python witness retained |
| Functional coverage policy | `tests/functional_coverage_policy.rs` plus the authoritative Rust test run | no Python witness retained |
| i32 argument/local flow | `tests/i32_local_flow_backend.rs` | no Python witness retained |
| i32 arithmetic | `tests/i32_arithmetic_backend.rs` | no Python witness retained |
| i32 control flow | `tests/i32_control_flow_backend.rs` | no Python witness retained |
| Checked i32 overflow | `tests/i32_overflow_backend.rs` | no Python witness retained |
| Direct Rust function call | `tests/direct_rust_function_call_backend.rs` | no Python witness retained |
| Managed static call | `tests/managed_static_call_backend.rs` | no Python witness retained |
| Managed construction | `tests/managed_construction_backend.rs` | no Python witness retained |
| Managed instance call | `tests/managed_instance_call_backend.rs` | no Python witness retained |
| Managed property read/write | `tests/managed_property_backend.rs` | no Python witness retained |

The four managed-consumption replays above preserve the former end-to-end observables rather than only emitter primitives: Rust source mutation, managed metadata/IL shape, export identity, managed artifact causality and CoreCLR execution are all asserted from Rust integration tests.

Historical upstream-only R02/R03/`rustc_codegen_clr` characterization is retired from the current verification inventory. Git retains its provenance; it is not a product dependency or current gate.

## Remaining Python authority

| Entry point | Current responsibility | Existing Rust evidence | Missing before deletion | State |
| --- | --- | --- | --- | --- |
| `eng/verification/verify_codegen_backend_boundary.py` | Full `rustc → FerrumWeave` boundary: backend loading, source mutation causality, managed execution, export identity and invalid-Rust rejection | Rust unit/integration tests cover individual backend and repository-boundary pieces | One Rust E2E replay covering the complete positive and negative boundary contract | `RUST_PARTIAL` |
| `eng/verification/verify_external_managed_assembly_backend.py` | Build an independent managed dependency, consume it through AssemblyRef/MemberRef/call, and execute the Rust-driven observable on CoreCLR | R05 and emitter tests cover managed metadata/call primitives | Rust E2E that independently builds the dependency, compiles through FerrumWeave and executes the managed consumer | `RUST_PARTIAL` |
| `eng/verification/verify_multiple_rust_exports_backend.py` | Preserve multiple source-owned Rust exports in one managed artifact | Rust export/emitter tests cover multiple-export primitives | Same-artifact managed consumer replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_heterogeneous_rust_exports_backend.py` | Preserve heterogeneous supported export shapes in one managed artifact | Rust component tests cover the individual export shapes | Same-artifact heterogeneous managed consumer replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_mixed_constant_arithmetic_exports_backend.py` | Preserve constant + arithmetic exports in one codegen invocation | Constant and arithmetic paths are independently Rust-certified | Same-artifact mixed-export replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_mixed_constant_control_flow_exports_backend.py` | Preserve constant + control-flow exports in one codegen invocation | Constant and control-flow paths are independently Rust-certified | Same-artifact mixed-export replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_mixed_constant_direct_call_exports_backend.py` | Preserve constant + direct-call exports, including the managed call opcode, in one artifact | Constant/direct-call component paths are independently Rust-certified | Same-artifact mixed-export replay in Rust | `RUST_PARTIAL` |
| `tools/quality/check_code_coverage.py` | Enforce the Rust line-coverage threshold/policy over `cargo llvm-cov` output | Coverage production is already Rust-native tooling | Rust-native or otherwise non-Python policy enforcement | `PYTHON_ONLY` |

## Migration order

1. External managed assembly consumption — last critical managed-consumption Python E2E.
2. Multiple/heterogeneous/mixed export aggregation as one coherent crate-aggregation block.
3. Complete codegen-backend boundary replay, including its negative invalid-Rust contract.
4. Line-coverage policy parser, independently from product semantics.

A verifier may be deleted only under the current cutover policy in `r09-rust-authority-cutover.md`. The goal is not to preserve duplicate implementations; it is to preserve observable product confidence while continuously reducing obsolete verification LOC.
