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
| Independent external managed assembly consumption | `tests/external_managed_assembly_backend.rs` | no Python witness retained |
| Multiple, heterogeneous and mixed crate aggregation | `tests/crate_aggregation_backend.rs` | no Python witness retained |
| Rust line-coverage gate | pinned `cargo-llvm-cov 0.8.7 --fail-under-lines 80` in `.github/workflows/rust-ci.yml` | no Python parser retained |

The managed-consumption replays preserve end-to-end observables rather than only emitter primitives: Rust source mutation, managed metadata/IL shape, export identity, managed artifact causality and CoreCLR execution are asserted from Rust integration tests. The external-assembly replay additionally builds its dependency independently and requires AssemblyRef/MemberRef/call behavior. The crate-aggregation replay preserves multiple exports, heterogeneous signatures, mixed arithmetic/control-flow shapes and the managed direct-call opcode in same-artifact scenarios.

Coverage policy is enforced directly by the pinned Rust coverage tool. The hard gate remains 80% line coverage. The preferred 80%–96% operating band remains guidance; coverage above 96% is allowed and therefore requires no separate executable upper-bound parser.

Historical upstream-only R02/R03/`rustc_codegen_clr` characterization is retired from the current verification inventory. Git retains its provenance; it is not a product dependency or current gate.

## Remaining Python authority

| Entry point | Current responsibility | Existing Rust evidence | Missing before deletion | State |
| --- | --- | --- | --- | --- |
| `eng/verification/verify_codegen_backend_boundary.py` | Full `rustc → FerrumWeave` boundary: backend loading, source mutation causality, managed execution, export identity and invalid-Rust rejection | `tests/codegen_backend_boundary.rs` replays the complete positive and negative product contract | One clean Windows/Linux DUAL run before cutover | `DUAL` |

## Migration order

1. Certify `tests/codegen_backend_boundary.rs` on Windows and Linux alongside the existing Python boundary gate.
2. Remove the final Python boundary steps and delete `verify_codegen_backend_boundary.py`.
3. Confirm the repository contains zero `.py` files and re-run the final Rust-native gates.

A verifier may be deleted only under the current cutover policy in `r09-rust-authority-cutover.md`. The goal is not to preserve duplicate implementations; it is to preserve observable product confidence while continuously reducing obsolete verification LOC.
