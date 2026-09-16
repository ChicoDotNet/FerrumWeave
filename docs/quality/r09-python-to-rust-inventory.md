# Python → Rust verification inventory

FerrumWeave's active verification authority is now Rust-native. Git history preserves the bootstrap and migration chronology; the current tree does not retain duplicate Python witnesses solely for provenance.

## Authority model

Rust owns behavioral certification through executable unit/integration tests and the pinned Rust toolchain. .NET consumers remain the external interoperability witnesses where a CLR observable is required.

## Rust-authoritative contracts

| Contract family | Rust authority |
| --- | --- |
| R05 managed-consumption census | `tests/evidence_classification.rs` |
| Functional coverage policy | `tests/functional_coverage_policy.rs` plus the authoritative Rust test run |
| i32 argument/local flow | `tests/i32_local_flow_backend.rs` |
| i32 arithmetic | `tests/i32_arithmetic_backend.rs` |
| i32 control flow | `tests/i32_control_flow_backend.rs` |
| Checked i32 overflow | `tests/i32_overflow_backend.rs` |
| Direct Rust function call | `tests/direct_rust_function_call_backend.rs` |
| Managed static call | `tests/managed_static_call_backend.rs` |
| Managed construction | `tests/managed_construction_backend.rs` |
| Managed instance call | `tests/managed_instance_call_backend.rs` |
| Managed property read/write | `tests/managed_property_backend.rs` |
| Independent external managed assembly consumption | `tests/external_managed_assembly_backend.rs` |
| Multiple, heterogeneous and mixed crate aggregation | `tests/crate_aggregation_backend.rs` |
| Complete `rustc → FerrumWeave` codegen boundary, including invalid-Rust rejection | `tests/codegen_backend_boundary.rs` |
| Rust line-coverage gate | pinned `cargo-llvm-cov 0.8.7 --fail-under-lines 80` in `.github/workflows/rust-ci.yml` |

The managed and aggregation replays preserve end-to-end observables rather than only emitter primitives: Rust source mutation, managed metadata/IL shape, export identity, managed artifact causality and CoreCLR execution are asserted from Rust integration tests. The external-assembly replay additionally builds its dependency independently and requires external AssemblyRef/MemberRef/call behavior.

`tests/codegen_backend_boundary.rs` owns the complete product boundary contract formerly split across the Python boundary sensor/product gate: successful backend loading and managed artifact production, source mutation 137→211, CLR execution, export rename causality, and rejection of invalid Rust before FerrumWeave lowering.

Coverage policy is enforced directly by the pinned Rust coverage tool. The hard gate remains 80% line coverage. The preferred 80%–96% operating band remains guidance; coverage above 96% is allowed and therefore requires no separate executable upper-bound parser.

Historical upstream-only R02/R03/`rustc_codegen_clr` characterization remains available through Git history and is neither a product dependency nor a current gate.

## Current Python status

**Zero Python entry points remain in the repository.** Product verification, functional-coverage policy, codegen convergence and line-coverage enforcement no longer require Python.

Future Python may be introduced only for a new justified product/tooling need; it must not reappear merely as a duplicate oracle for behavior already certified in Rust.
