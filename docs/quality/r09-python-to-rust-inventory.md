# R09 Python → Rust verification inventory

This document is the migration ledger for retiring Python from FerrumWeave's critical verification path without losing observable contracts or historical evidence.

The census is intentionally strict: related Rust unit tests do **not** make a Python verifier `DUAL` unless Rust protects essentially the same observable. A row may continue to name a deleted Python entry point after cutover so that the migration history remains auditable. Once a row reaches `RUST_AUTHORITATIVE`, the Rust evidence is the product gate and the retired Python witness is no longer required. Historical upstream-only oracles may instead become `OBSOLETE` when they no longer represent a useful product gate; this is a retirement decision, not a claim of Rust parity.

## States

- `PYTHON_ONLY`: the executable contract is currently protected only by Python.
- `RUST_PARTIAL`: Rust protects part of the same behavior, but not the full Python observable/end-to-end path.
- `DUAL`: Python and Rust independently protect the relevant contract.
- `RUST_AUTHORITATIVE`: Rust is the product gate; Python is no longer required for that contract and may be retired.
- `OBSOLETE`: the contract no longer represents supported/current product evidence and has an explicit retirement rationale.

## Migration census

| Python entry point | Test / contract family | Protected behavior | Fixture / input | Expected observable | Category | Severity | Existing Rust evidence | Missing Rust evidence | State |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| retired `eng/r02/verify.py` | R02 real Rust → CLR slice | Historical proof that real Rust compiled through the pinned upstream backend into managed PE/CLI; invalid borrow was rejected; managed artifact executed; no source-language substitution | `tests/fixtures/r02/managed_console.rs`, `borrow_invalid.rs`, R02 ledger retained as history | Valid PE/CLI + CLR metadata, IL-only entry point, `Hello FerrumWeave`, E0502, exact fixture census | historical oracle / differential | historical | FerrumWeave now owns independent product-backend causal certification; R02 ledger/fixtures remain auditable historical evidence | None: no product replacement is required for an upstream-only historical oracle | `OBSOLETE` |
| retired `eng/r03/verify.py` | R03 safe-Rust semantics | Historical native/upstream characterization of a declared safe-Rust semantic slice plus borrow rejection | `tests/fixtures/r03/semantics_s01.rs`, `borrow_invalid.rs`, R03 ledger retained as history | Exact historical stdout and E0502 | historical oracle / differential | historical | Current semantic product claims require FerrumWeave-owned backend evidence; the old oracle ledger remains historical | None: reintroduce differential comparison only for a scoped research question | `OBSOLETE` |
| retired `eng/verification/check_managed_consumption_causality.py` | `FW-R05-DOTNET-009..013` census | All advertised R05 source-causal managed-consumption families remain implemented with source-causal evidence and replayable proof | `tests/r05/contracts.toml` | Five required IDs present; `implemented = true`; `evidence_level = source-causal-certified`; non-pending proof | product ledger gate | high | `tests/evidence_classification.rs::r05_managed_consumption_source_causality_census_is_complete` replays the exact five-contract census and proof policy | None; Python was removed from CI, then deleted after both cutover gates passed | `RUST_AUTHORITATIVE` |
| `eng/verification/verify_codegen_backend_boundary.py` | rustc → FerrumWeave codegen boundary | rustc reaches FerrumWeave MIR lowering; product mode proves source mutation causality, managed execution, export identity and negative Rust type checking | Generated Rust sources and generated C# consumer | Managed artifact shape/execution changes with Rust-only mutation; invalid Rust rejected | product backend E2E | critical | Rust tests cover backend/lowering components and repository boundary ownership | One Rust integration harness reproducing the complete source→artifact→consumer contract | `RUST_PARTIAL` |
| retired `eng/verification/verify_direct_rust_function_call_backend.py` | Direct Rust function-call lowering | Direct Rust helper call becomes a real managed call; helper semantics and export identity remain source-causal | Generated Rust helper/export + fixed C# consumer | Managed call opcode; observable changes with helper semantics; rename appears in CLR metadata | product backend E2E | critical | `tests/direct_rust_function_call_backend.rs` builds the real backend, compiles add/subtract/renamed helper variants, requires a real managed `call` opcode, executes managed consumers and compares source-causal artifacts | None; Python was removed from convergence, then deleted after CI-only and post-delete gates passed | `RUST_AUTHORITATIVE` |
| `eng/verification/verify_external_managed_assembly_backend.py` | External managed assembly consumption | Rust consumes independently compiled C# dependency through AssemblyRef/MemberRef/call | Generated fixed `External.Managed` C# assembly + Rust payload mutation | Metadata references external assembly/type/member; executable output follows Rust payload | product interoperability E2E | critical | Rust tests cover managed metadata/emission primitives | Full independent dependency build + FerrumWeave compile + CoreCLR execution in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_heterogeneous_rust_exports_backend.py` | Heterogeneous crate exports | One crate preserves a constant export and an argument/local-flow export together | Generated Rust crate with two heterogeneous exports | Both CLR exports and both expected observables survive one codegen invocation | product backend E2E | high | Rust crate-export/backend tests cover aggregation primitives | Same-artifact executable consumer proof in Rust | `RUST_PARTIAL` |
| retired `eng/verification/verify_i32_arithmetic_backend.py` | i32 arithmetic lowering | Rust `+`/`-` MIR arithmetic and export identity drive managed behavior | Generated Rust arithmetic variants + fixed C# arguments | Correct arithmetic output, distinct artifacts/behavior under Rust mutation, CLR export rename | product backend E2E | critical | `tests/i32_arithmetic_backend.rs` builds the real FerrumWeave backend, compiles add/subtract/renamed Rust variants, executes fixed C# consumers and compares managed artifacts | None; Python was removed from convergence and deleted after green cutover evidence | `RUST_AUTHORITATIVE` |
| retired `eng/verification/verify_i32_control_flow_backend.py` | i32 control-flow lowering | Rust branch predicate and export identity survive MIR→CLR lowering | Generated `==`/`!=` Rust variants + fixed C# consumer | Branch-dependent outputs and renamed CLR export | product backend E2E | critical | `tests/i32_control_flow_backend.rs` builds the real FerrumWeave backend, compiles equal/not-equal/renamed Rust variants, executes fixed C# consumers for selector `0` and `1`, and compares managed artifacts | None; Python was removed from convergence and deleted after green cutover evidence | `RUST_AUTHORITATIVE` |
| retired `eng/verification/verify_i32_local_flow_backend.py` | i32 argument/local data flow | Selected Rust argument flows through a local to the return value; export identity preserved | Generated left/right-selection Rust variants + fixed consumer | Output tracks selected Rust argument; renamed export appears in metadata | product backend E2E | critical | `tests/i32_local_flow_backend.rs` builds the real FerrumWeave backend, compiles left/right/renamed Rust variants, executes fixed C# consumers and compares managed artifacts | None; Python was removed from convergence and deleted after green cutover evidence | `RUST_AUTHORITATIVE` |
| retired `eng/verification/verify_i32_overflow_backend.py` | Checked i32 overflow | Debug Rust overflow semantics are not silently lowered to wrapping CLR `add` | Generated ordinary `left + right`; consumer invokes `int.MaxValue + 1` | Overflow path is preserved rather than returning a wrapped value | product semantic E2E | critical | `tests/i32_overflow_backend.rs` builds the real FerrumWeave backend, compiles ordinary checked Rust addition and requires the fixed C# `Answer(int.MaxValue, 1)` consumer to take a non-success overflow path | None; Python was removed from convergence and deleted after green cutover evidence | `RUST_AUTHORITATIVE` |
| `eng/verification/verify_managed_construction_backend.py` | Managed constructor call | Rust marker selection causes a real CLR `newobj` for the selected managed type | Generated Object/StringBuilder marker variants + payload/export mutations | Constructor metadata/opcode changes with source; executable observable follows payload | product managed-consumption E2E | critical | Rust emitter/lowering tests cover construction primitives | Full source-causal construction replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_managed_instance_call_backend.py` | Managed instance dispatch | Rust marker selection causes the expected CLR instance `callvirt` | Generated Object/StringBuilder `ToString` marker variants | Selected receiver/member metadata, real `callvirt`, source-causal payload/export observable | product managed-consumption E2E | critical | Rust emitter/lowering tests cover instance-call primitives | Full source-causal instance replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_managed_property_backend.py` | Managed property read/write | Rust marker causes CLR property setter/getter emission | Generated StringBuilder.Length marker + payload/export mutation | `set_Length` + `get_Length`, executable getter value, export identity | product managed-consumption E2E | critical | Rust property/emission primitives exist | Full property source-causality replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_managed_static_call_backend.py` | Managed static call | Rust marker selects supported `System.Math` static call; Rust-only mutations cause metadata/behavior changes | Generated `Abs`/`Sign` marker variants + integer payload/export mutations | Correct MemberRef/call and managed output derived from Rust source | product managed-consumption E2E | critical | Rust emission tests and R05 ledger evidence exist | Full source→managed execution replay in Rust | `RUST_PARTIAL` |
| retired `eng/verification/verify_managed_static_call_causality.py` | Historical managed static-call upstream oracle | Pinned `rustc_codegen_clr` characterized source causality independently of FerrumWeave | `tests/fixtures/managed_static_call.rs`; historical value 137→211 and WriteLine→Write mutation | Upstream managed observable changed with source | historical oracle / differential | historical | FerrumWeave has a product-backend managed-static-call verifier and R05 source-causal evidence; the upstream fixture remains historical | None: upstream replay no longer contributes product confidence | `OBSOLETE` |
| `eng/verification/verify_mixed_constant_arithmetic_exports_backend.py` | Constant + arithmetic aggregation | One crate preserves independently supported constant and arithmetic exports | Generated mixed Rust crate + fixed managed consumer | Both methods execute correctly from one managed artifact | product backend aggregation E2E | high | Rust crate-export/lowering tests cover component paths | Same-artifact managed execution replay in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_mixed_constant_control_flow_exports_backend.py` | Constant + control-flow aggregation | One crate preserves constant and control-flow exports | Generated mixed Rust crate + fixed managed consumer | Constant and both branch outputs execute correctly from one artifact | product backend aggregation E2E | high | Rust crate-export/control-flow tests cover component paths | Same-artifact executable contract in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_mixed_constant_direct_call_exports_backend.py` | Constant + direct-call aggregation | One crate preserves constant export and direct-call export including managed call opcode | Generated mixed Rust crate + fixed managed consumer | Both exports execute; direct-call export retains managed call | product backend aggregation E2E | high | Rust crate-export/direct-call tests cover component paths | Same-artifact executable contract in Rust | `RUST_PARTIAL` |
| `eng/verification/verify_multiple_rust_exports_backend.py` | Multiple homogeneous exports | More than one source-owned Rust export survives one codegen invocation | Generated `alpha_value=137`, `beta_value=211` + fixed C# consumer | Both CLR methods exist and return independent expected constants | product backend aggregation E2E | high | Rust crate-export backend tests cover multiple-export primitives | Full managed consumer proof in Rust | `RUST_PARTIAL` |
| `tools/quality/check_code_coverage.py` | Rust line-coverage policy gate | Enforce minimum Rust line coverage and report preferred operating band | `cargo llvm-cov --json` report | Fail below configured minimum; informational notice above preferred maximum | quality infrastructure | high | Coverage producer itself is Rust tooling; policy parser remains Python | Rust-native policy parser or another non-Python enforcement mechanism | `PYTHON_ONLY` |
| retired `tools/quality/check_functional_coverage.py` | Functional contract coverage gate | Discover mapped `tests/functional.rs` tests, execute each implemented contract and enforce ledger minimum | `tests/functional/contracts.toml` + `tests/functional.rs`; historical Python additionally listed/ran exact mapped tests | Ledger non-empty; implemented contracts map to real Rust tests; mapped percentage meets `minimum_percent`; mapped tests pass | quality infrastructure | critical | `tests/functional_coverage_policy.rs` enforces ledger/mapping/percentage natively; the authoritative `cargo test --workspace --tests` run executes every mapped `tests/functional.rs` test | None; Python was removed from CI and deleted after both cutover gates passed | `RUST_AUTHORITATIVE` |

## Current migration order

The first authority cutover is complete. Remaining work is ordered by product leverage and evidence dependency:

1. **Completed:** R05 managed-consumption census, functional-coverage policy, i32 local flow, i32 arithmetic, i32 control flow, checked overflow and direct Rust function call are `RUST_AUTHORITATIVE`.
2. **Retired as obsolete upstream characterization:** R02, R03 and the managed-static-call `rustc_codegen_clr` executable oracle lanes. Their fixtures/ledgers remain as historical records, but no product or CI gate builds the upstream backend.
3. Complete Rust E2E parity for managed-consumption families: static call, construction, instance call, property access and external assembly consumption.
4. Complete Rust E2E parity for multiple/mixed/heterogeneous export aggregation.
5. Reproduce the complete codegen-backend boundary contract natively in Rust.
6. Migrate the line-coverage policy parser independently; it is quality infrastructure rather than product semantics.

The detailed batch cutover evidence is recorded in `docs/quality/r09-rust-authority-cutover.md`.

## Historical acceptance notes

Iterations 1–7 below record the state at the time each parity increment was completed. Statements saying a Python verifier remained present are historical and are superseded by the later authority cutover where applicable.

## Iteration 1 acceptance

Iteration 1 completed the five-contract R05 managed-consumption census in Rust:

- `tests/evidence_classification.rs::r05_managed_consumption_source_causality_census_is_complete` enforces the exact five-contract census and proof policy;
- at this stage the Python gate remained unchanged for differential comparison;
- the contract reached `DUAL` before the later batch authority cutover.

## Iteration 2 acceptance

Iteration 2 adapted the functional-coverage policy to the native Rust suite without recursively spawning Cargo from Cargo:

- `tests/functional_coverage_policy.rs` parses the existing ledger with no new dependency;
- the Rust policy test rejects an empty ledger, invalid `implemented` values, missing/empty mappings and mappings to nonexistent `#[test]` functions;
- the Rust policy test enforces the existing `minimum_percent` denominator exactly over all declared contracts;
- ordinary `cargo test --workspace --tests` executes both the policy test and every mapped functional test, so any mapped-test regression fails the same Rust certification run;
- the contract reached `DUAL` before the later batch authority cutover.

This was an `ADAPT`, not a line-by-line translation: Python owned orchestration of individual `cargo test` subprocesses, while Rust owned the ledger/mapping policy and the authoritative Rust test runner owned execution.

## Iteration 3 acceptance

Iteration 3 replayed the first product-backend E2E verifier natively in Rust:

- `tests/i32_local_flow_backend.rs` builds the isolated FerrumWeave codegen backend with the pinned `nightly-2025-10-14` toolchain and introduces no new crate dependency;
- fixed C# arguments `(137, 211)` observe `137` when Rust selects `left` and `211` when Rust selects `right`;
- renaming only the Rust export from `answer` to `compute_result` is consumed as `ComputeResult` from C# while preserving the `137` observable;
- both the local-selection mutation and export-identity mutation must change the emitted managed artifact;
- the Rust replay is an ordinary integration test reached by `cargo test --workspace --tests`.

This was a contract-level `TRANSFORM`: the Python subprocess/oracle structure became idiomatic Rust `Command` + assertions, while source variants, fixed managed consumer inputs, observable outputs and source-causal mutations remained equivalent.

## Iteration 4 acceptance

Iteration 4 replayed the i32 arithmetic product-backend E2E verifier natively in Rust:

- `tests/i32_arithmetic_backend.rs` builds the FerrumWeave codegen backend with the same pinned `nightly-2025-10-14` toolchain and introduces no new crate dependency;
- fixed C# arguments `(137, 74)` observe `211` for Rust `left + right` and `63` for Rust `left - right`;
- renaming only the Rust export from `answer` to `compute_result` is consumed as `ComputeResult` from C# while preserving the `211` observable;
- both the arithmetic mutation `+ -> -` and export-identity mutation must change the emitted managed artifact;
- the Rust replay is an ordinary integration test reached by `cargo test --workspace --tests` on both Ubuntu and Windows.

This was another contract-level `TRANSFORM`: the protected source mutations, fixed managed inputs, observable outputs and artifact-causality checks were preserved while Python orchestration became idiomatic Rust process execution and assertions.

## Iteration 5 acceptance

Iteration 5 replayed the i32 control-flow product-backend E2E verifier natively in Rust:

- `tests/i32_control_flow_backend.rs` builds the FerrumWeave codegen backend with the same pinned `nightly-2025-10-14` toolchain and introduces no new crate dependency;
- fixed managed arguments `(selector, 137, 211)` exercise both sides of each branch: `selector == 0` produces `137` for selector `0` and `211` for selector `1`, while `selector != 0` produces `211` for selector `0` and `137` for selector `1`;
- renaming only the Rust export from `answer` to `compute_result` is consumed as `ComputeResult` from C# and preserves the equal-predicate behavior for both selector values;
- both the predicate mutation `== -> !=` and export-identity mutation must change the emitted managed artifact;
- the Rust replay is an ordinary integration test reached by `cargo test --workspace --tests` on both Ubuntu and Windows.

This was another contract-level `TRANSFORM`: source variants, fixed inputs, both branch observables and artifact-causality checks remained equivalent while Python orchestration became idiomatic Rust process execution and assertions.

## Iteration 6 acceptance

Iteration 6 replayed the checked-i32-overflow product semantic E2E verifier natively in Rust:

- `tests/i32_overflow_backend.rs` builds the FerrumWeave codegen backend with the same pinned `nightly-2025-10-14` toolchain and introduces no new crate dependency;
- the generated Rust remains ordinary `left + right`, relying on the pinned debug rustc lane to carry checked-overflow semantics through MIR;
- the fixed C# consumer calls `FerrumWeave.RustApi.Answer(int.MaxValue, 1)`;
- success is defined exactly as in the Python verifier: the managed process must take a non-success path rather than return a wrapped i32 value;
- the Rust replay deliberately does not require a specific CLR exception type or message, avoiding a stronger runtime-specific contract than the Python verifier protected;
- the Rust replay is an ordinary integration test reached by `cargo test --workspace --tests` on both Ubuntu and Windows.

This contract exposed the correct shared-harness boundary: common support may own backend construction, rustc invocation, temporary workspaces and managed-consumer process mechanics, but each E2E contract must own its observable/oracle.

## Iteration 7 acceptance

Iteration 7 extracted only the stable E2E mechanics and replayed direct Rust function-call lowering natively:

- `tests/support/mod.rs` owns backend construction, pinned rustc invocation, temporary workspaces, C# consumer creation/execution and generic process-success diagnostics;
- contract-specific result parsing and exceptional observables remain local to each integration test; the checked-overflow contract specifically prevented over-generalizing the shared support;
- `tests/direct_rust_function_call_backend.rs` compiles helper-add, helper-subtract and renamed-export variants through the real backend;
- fixed managed inputs `(137, 74)` observe `211`, `63` and `211` respectively;
- the managed consumer inspects the exported method IL and requires opcode `call` (`0x28`), proving the helper call survived as a real managed call;
- helper-semantics mutation and export-identity mutation must both change the emitted managed artifact;
- Rust CI #753 passed fmt/clippy, coverage and the complete integration suite on Ubuntu and Windows;
- backend convergence #387 independently passed the still-present Python witnesses on Ubuntu and Windows.

At this point direct-call and the other six contracts were all validated as `DUAL` and eligible for one batch authority cutover.

## Batch Rust-authority cutover acceptance

The seven mature `DUAL` contracts were promoted together to reduce ceremony without reducing evidence.

### CI-only cutover

HEAD `5d03f16062808f23eb9a047511c4b277bdaa96d5` removed the seven Python witnesses from the critical CI path while leaving their files present.

- Rust CI #756: success for fmt/clippy, code coverage, Ubuntu integration and Windows integration.
- FerrumWeave codegen backend convergence #390: success on Ubuntu and Windows using only the Python verifiers that still lacked Rust parity.

This proved there was no hidden runtime or orchestration dependency on the retiring Python witnesses.

### Physical retirement

The seven Python files were then deleted as one batch. Post-delete HEAD `a5a1a6f34f6a4c924929f99febac6bae151c7be6` was certified independently:

- Rust CI #763: success for fmt/clippy, code coverage, Ubuntu integration and Windows integration.
- FerrumWeave codegen backend convergence #397: success on Ubuntu and Windows.

No `PYTHON_ONLY` or `RUST_PARTIAL` product verifier was deleted in that authority cutover.

Therefore the R05 managed-consumption census, functional-coverage policy, i32 local flow, i32 arithmetic, i32 control flow, checked i32 overflow and direct Rust function-call contracts are `RUST_AUTHORITATIVE`.

## Historical upstream-oracle retirement

After the first Rust-authority batch was promoted, the remaining executable `rustc_codegen_clr` lanes were reviewed separately. R02, R03, and the managed-static-call causality oracle were retired as `OBSOLETE` because they characterize a different upstream backend and no longer contribute release confidence for the FerrumWeave-owned product path.

The retirement deliberately preserves `tests/r02`, `tests/r03`, their fixtures, and `tests/fixtures/managed_static_call.rs` as historical evidence. It removes only the permanent CI/workflow and Python execution machinery. A future differential experiment may reuse those historical inputs, but restoring an upstream backend lane requires a new explicit research question rather than being inherited as a product gate.
