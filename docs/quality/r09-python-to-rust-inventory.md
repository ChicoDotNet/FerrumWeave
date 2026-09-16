# R09 Python → Rust verification inventory

This document is the migration ledger for retiring Python from FerrumWeave's critical verification path without losing observable contracts or historical evidence.

The census is intentionally strict: related Rust unit tests do **not** make a Python verifier `DUAL` unless Rust protects essentially the same observable. Python files remain present until the corresponding contract reaches `RUST_AUTHORITATIVE` or is explicitly classified `OBSOLETE`.

## States

- `PYTHON_ONLY`: the executable contract is currently protected only by Python.
- `RUST_PARTIAL`: Rust protects part of the same behavior, but not the full Python observable/end-to-end path.
- `DUAL`: Python and Rust independently protect the relevant contract.
- `RUST_AUTHORITATIVE`: Rust is the product gate; Python is no longer required for that contract.
- `OBSOLETE`: the contract no longer represents supported behavior and has an explicit retirement rationale.

## Complete Python entry-point census on `dev`

| Python entry point | Test / contract family | Protected behavior | Fixture / input | Expected observable | Category | Severity | Existing Rust evidence | Missing Rust evidence | State |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| `eng/r02/verify.py` | R02 real Rust → CLR slice | Real Rust compiles through the pinned upstream backend into managed PE/CLI; invalid borrow remains rejected; managed artifact executes; no source-language substitution | `tests/fixtures/r02/managed_console.rs`, `borrow_invalid.rs`, R02 ledger | Valid PE/CLI + CLR metadata, IL-only entry point, `Hello FerrumWeave`, E0502 for invalid borrow, exact fixture census | historical oracle / differential | high | Rust classifies R02 evidence as historical oracle evidence | Native Rust replacement for the executable PE inspection, compile/reject/execute replay | `PYTHON_ONLY` |
| `eng/r03/verify.py` | R03 safe-Rust semantics | Characterize supported safe-Rust semantics against native/upstream compilation plus borrow rejection | `tests/fixtures/r03/semantics_s01.rs`, `borrow_invalid.rs`, R03 ledger | Exact stdout for supported semantics and E0502 for invalid borrow | historical oracle / differential | high | Rust classifies R03 evidence as historical oracle evidence | Native Rust differential harness for the same fixtures and outputs | `PYTHON_ONLY` |
| `eng/verification/check_managed_consumption_causality.py` | `FW-R05-DOTNET-009..013` census | All advertised R05 source-causal managed-consumption families remain implemented with source-causal evidence and replayable proof | `tests/r05/contracts.toml` | Five required IDs present; `implemented = true`; `evidence_level = source-causal-certified`; non-pending proof | product ledger gate | high | `tests/evidence_classification.rs::r05_managed_consumption_source_causality_census_is_complete` replays the exact five-contract census and proof policy | Python remains in CI only for dual comparison; no Rust contract gap remains | `DUAL` |
| `eng/verification/verify_codegen_backend_boundary.py` | rustc → FerrumWeave codegen boundary | rustc reaches FerrumWeave MIR lowering; product mode proves source mutation causality, managed execution, export identity and negative Rust type checking | Generated Rust sources and generated C# consumer | Managed artifact shape/execution changes with Rust-only mutation; invalid Rust rejected | product backend E2E | critical | Rust tests cover backend/lowering components and repository boundary ownership | One Rust integration harness reproducing the complete source→artifact→consumer contract | `RUST_PARTIAL` |
| `eng/verification/verify_direct_rust_function_call_backend.py` | Direct Rust function-call lowering | Direct Rust helper call becomes a real managed call; helper semantics and export identity remain source-causal | Generated Rust helper/export + fixed C# consumer | Managed call opcode; observable changes with helper semantics; rename appears in CLR metadata | product backend E2E | critical | Rust lowerer/emitter tests cover direct-call pieces | Full compile→managed artifact→consumer replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_external_managed_assembly_backend.py` | External managed assembly consumption | Rust consumes independently compiled C# dependency through AssemblyRef/MemberRef/call | Generated fixed `External.Managed` C# assembly + Rust payload mutation | Metadata references external assembly/type/member; executable output follows Rust payload | product interoperability E2E | critical | Rust tests cover managed metadata/emission primitives | Full independent dependency build + FerrumWeave compile + CoreCLR execution in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_heterogeneous_rust_exports_backend.py` | Heterogeneous crate exports | One crate preserves a constant export and an argument/local-flow export together | Generated Rust crate with two heterogeneous exports | Both CLR exports and both expected observables survive one codegen invocation | product backend E2E | high | Rust crate-export/backend tests cover aggregation primitives | Same-artifact executable consumer proof in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_i32_arithmetic_backend.py` | i32 arithmetic lowering | Rust `+`/`-` MIR arithmetic and export identity drive managed behavior | Generated Rust arithmetic variants + fixed C# arguments | Correct arithmetic output, distinct artifacts/behavior under Rust mutation, CLR export rename | product backend E2E | critical | `tests/i32_arithmetic_backend.rs` builds the real FerrumWeave backend, compiles add/subtract/renamed Rust variants, executes fixed C# consumers and compares managed artifacts | Python remains in convergence for dual comparison; no Rust contract gap remains for this observable | `DUAL` |
| `eng/verification/verify_i32_control_flow_backend.py` | i32 control-flow lowering | Rust branch predicate and export identity survive MIR→CLR lowering | Generated `==`/`!=` Rust variants + fixed C# consumer | Branch-dependent outputs and renamed CLR export | product backend E2E | critical | Rust lowering tests cover control-flow primitives | End-to-end mutation and consumer execution witness in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_i32_local_flow_backend.py` | i32 argument/local data flow | Selected Rust argument flows through a local to the return value; export identity preserved | Generated left/right-selection Rust variants + fixed consumer | Output tracks selected Rust argument; renamed export appears in metadata | product backend E2E | critical | `tests/i32_local_flow_backend.rs` builds the real FerrumWeave backend, compiles left/right/renamed Rust variants, executes fixed C# consumers and compares managed artifacts | Python remains in convergence for dual comparison; no Rust contract gap remains for this observable | `DUAL` |
| `eng/verification/verify_i32_overflow_backend.py` | Checked i32 overflow | Debug Rust overflow semantics are not silently lowered to wrapping CLR `add` | Generated ordinary `left + right`; consumer invokes `int.MaxValue + 1` | Overflow path is preserved rather than returning a wrapped value | product semantic E2E | critical | Rust backend has arithmetic/semantic lowerer coverage | Executable checked-overflow contract reproduced in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_managed_construction_backend.py` | Managed constructor call | Rust marker selection causes a real CLR `newobj` for the selected managed type | Generated Object/StringBuilder marker variants + payload/export mutations | Constructor metadata/opcode changes with source; executable observable follows payload | product managed-consumption E2E | critical | Rust emitter/lowering tests cover construction primitives | Full source-causal construction replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_managed_instance_call_backend.py` | Managed instance dispatch | Rust marker selection causes the expected CLR instance `callvirt` | Generated Object/StringBuilder `ToString` marker variants | Selected receiver/member metadata, real `callvirt`, source-causal payload/export observable | product managed-consumption E2E | critical | Rust emitter/lowering tests cover instance-call primitives | Full source-causal instance replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_managed_property_backend.py` | Managed property read/write | Rust marker causes CLR property setter/getter emission | Generated StringBuilder.Length marker + payload/export mutation | `set_Length` + `get_Length`, executable getter value, export identity | product managed-consumption E2E | critical | Rust property/emission primitives exist | Full property source-causality replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_managed_static_call_backend.py` | Managed static call | Rust marker selects supported `System.Math` static call; Rust-only mutations cause metadata/behavior changes | Generated `Abs`/`Sign` marker variants + integer payload/export mutations | Correct MemberRef/call and managed output derived from Rust source | product managed-consumption E2E | critical | Rust emission tests and R05 ledger evidence exist | Full source→managed execution replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_managed_static_call_causality.py` | Historical managed static-call oracle | Pinned `rustc_codegen_clr` characterizes Rust source causality independently of FerrumWeave product backend | `tests/fixtures/managed_static_call.rs`; value 137→211 and WriteLine→Write mutation | Upstream/oracle managed artifact and observable change with source | historical oracle / differential | medium | Rust explicitly isolates/classifies oracle evidence | No product replacement required for historical evidence; decide later whether to retain as optional oracle | `PYTHON_ONLY` |
| `eng/verification/verify_mixed_constant_arithmetic_exports_backend.py` | Constant + arithmetic aggregation | One crate preserves independently supported constant and arithmetic exports | Generated mixed Rust crate + fixed managed consumer | Both methods execute correctly from one managed artifact | product backend aggregation E2E | high | Rust crate-export/lowering tests cover component paths | Same-artifact managed execution replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_mixed_constant_control_flow_exports_backend.py` | Constant + control-flow aggregation | One crate preserves constant and control-flow exports | Generated mixed Rust crate + fixed managed consumer | Constant and both branch outputs execute correctly from one artifact | product backend aggregation E2E | high | Rust crate-export/control-flow tests cover component paths | Same-artifact managed execution replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_mixed_constant_direct_call_exports_backend.py` | Constant + direct-call aggregation | One crate preserves constant export and direct-call export including managed call opcode | Generated mixed Rust crate + fixed managed consumer | Both exports execute; direct-call export retains managed call | product backend aggregation E2E | high | Rust crate-export/direct-call tests cover component paths | Same-artifact executable contract in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_multiple_rust_exports_backend.py` | Multiple homogeneous exports | More than one source-owned Rust export survives one codegen invocation | Generated `alpha_value=137`, `beta_value=211` + fixed C# consumer | Both CLR methods exist and return independent expected constants | product backend aggregation E2E | high | Rust crate-export backend tests cover multiple-export primitives | Full managed consumer proof in Rust | `RUST_PARTIAL` |
| `tools/quality/check_code_coverage.py` | Rust line-coverage policy gate | Enforce minimum Rust line coverage and report preferred operating band | `cargo llvm-cov --json` report | Fail below configured minimum; informational notice above preferred maximum | quality infrastructure | high | Coverage producer itself is Rust tooling; policy parser remains Python | Rust-native policy parser or another non-Python enforcement mechanism | `PYTHON_ONLY` |
| `tools/quality/check_functional_coverage.py` | Functional contract coverage gate | Discover mapped `tests/functional.rs` tests, execute each implemented contract and enforce ledger minimum | `tests/functional/contracts.toml` + `tests/functional.rs`; Python additionally lists/runs exact mapped tests | Ledger non-empty; implemented contracts map to real Rust tests; mapped percentage meets `minimum_percent`; mapped tests pass | quality infrastructure | critical | `tests/functional_coverage_policy.rs` enforces ledger/mapping/percentage natively; the same `cargo test --workspace --tests` run executes every mapped `tests/functional.rs` test | Python remains in CI for differential comparison; no contract gap remains at the authoritative suite level | `DUAL` |

## Iteration order

The migration order deliberately starts with low-risk ledger/policy gates before moving into subprocess-heavy compiler/CLR E2E witnesses:

1. `check_managed_consumption_causality.py` → Rust integration test (`DUAL`).
2. Functional contract ledger enforcement → Rust-native gate while preserving existing Rust functional tests (`DUAL`).
3. Small product-backend E2E verifiers (local flow, arithmetic, control flow, checked overflow).
4. Direct-call and managed-consumption E2E families.
5. Multiple/mixed/heterogeneous export aggregation.
6. Historical R02/R03/oracle scripts: preserve or retire only after their evidence role is explicitly decided.
7. Coverage-policy Python is infrastructure rather than product behavior; migrate it independently after product-contract parity.

## Iteration 1 acceptance

Iteration 1 is complete: the five-contract R05 managed-consumption census is enforced by an ordinary Rust integration test executed by `cargo test`, while the current Python gate remains unchanged. This row is `DUAL`; no Python file has been deleted.

## Iteration 2 acceptance

Iteration 2 adapts the functional-coverage policy to the native Rust suite without recursively spawning Cargo from Cargo:

- `tests/functional_coverage_policy.rs` parses the existing ledger with no new dependency;
- the Rust policy test rejects an empty ledger, invalid `implemented` values, missing/empty mappings and mappings to nonexistent `#[test]` functions;
- the Rust policy test enforces the existing `minimum_percent` denominator exactly over all declared contracts;
- ordinary `cargo test --workspace --tests` executes both the policy test and every mapped functional test, so any mapped-test regression fails the same Rust certification run;
- `tools/quality/check_functional_coverage.py` remains unchanged for dual comparison until this Rust path has sufficient green evidence.

This is an `ADAPT`, not a line-by-line translation: Python owns orchestration of individual `cargo test` subprocesses, while Rust owns the ledger/mapping policy and the authoritative Rust test runner owns execution.

## Iteration 3 acceptance

Iteration 3 replays the first product-backend E2E verifier natively in Rust while leaving the Python oracle unchanged:

- `tests/i32_local_flow_backend.rs` builds the isolated FerrumWeave codegen backend with the pinned `nightly-2025-10-14` toolchain and introduces no new crate dependency;
- fixed C# arguments `(137, 211)` observe `137` when Rust selects `left` and `211` when Rust selects `right`;
- renaming only the Rust export from `answer` to `compute_result` is consumed as `ComputeResult` from C# while preserving the `137` observable;
- both the local-selection mutation and export-identity mutation must change the emitted managed artifact;
- the existing Python verifier remains in the backend-convergence workflow for differential evidence and is not deleted or weakened;
- the Rust replay is an ordinary integration test reached by `cargo test --workspace --tests`, so local-flow source causality is now protected by the native Rust certification path.

This is a contract-level `TRANSFORM`: the Python subprocess/oracle structure becomes idiomatic Rust `Command` + assertions, while the source variants, fixed managed consumer inputs, observable outputs and source-causal mutations remain equivalent.

## Iteration 4 acceptance

Iteration 4 replays the i32 arithmetic product-backend E2E verifier natively in Rust while preserving the Python verifier unchanged:

- `tests/i32_arithmetic_backend.rs` builds the FerrumWeave codegen backend with the same pinned `nightly-2025-10-14` toolchain and introduces no new crate dependency;
- fixed C# arguments `(137, 74)` observe `211` for Rust `left + right` and `63` for Rust `left - right`;
- renaming only the Rust export from `answer` to `compute_result` is consumed as `ComputeResult` from C# while preserving the `211` observable;
- both the arithmetic mutation `+ -> -` and export-identity mutation must change the emitted managed artifact;
- the existing Python verifier remains in backend convergence for differential evidence and is not deleted or weakened;
- the Rust replay is an ordinary integration test reached by `cargo test --workspace --tests` on both Ubuntu and Windows.

This is another contract-level `TRANSFORM`: the protected source mutations, fixed managed inputs, observable outputs and artifact-causality checks are preserved while Python orchestration becomes idiomatic Rust process execution and assertions. The second E2E replay intentionally keeps its small helper layer local to the test: current CI evidence shows Cargo reuses the already-built backend between integration binaries, so extracting shared support is not yet justified by a demonstrated runtime bottleneck.