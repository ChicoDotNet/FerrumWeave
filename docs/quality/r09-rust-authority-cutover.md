# Rust verification authority cutover policy

This policy defines when an executable Python verifier may be retired in favor of Rust-native certification. Git history preserves prior cutover chronology and evidence; this file defines the rule that applies to the current tree.

## Principle

Rust owns FerrumWeave product-behavior certification. Python may remain temporarily only where it still protects a material observable that Rust does not yet reproduce.

A migration is complete only when removing Python does **not** remove product confidence.

## Required cutover sequence

For each verifier or coherent contract family:

1. **Identify the contract** — behavior, source/fixture, fixed inputs, observable outputs, metadata/IL requirements, negative cases and source-causal mutations.
2. **Replay it in Rust** — prefer an ordinary `#[test]` integration test reached by the authoritative Rust CI path; introduce no dependency unless the contract requires one.
3. **Prove `DUAL`** — run Python and Rust independently against the same code head on every supported CI platform relevant to the contract.
4. **Resolve every divergence** — do not weaken either oracle merely to obtain GREEN. A difference must be explained as an implementation bug, an accidental harness dependency or an intentional contract correction.
5. **Cut authority** — remove the Python invocation from the critical workflow only after equivalent Rust evidence is green.
6. **Delete the Python witness** — once the Rust-only critical path is valid, remove the superseded `.py` file rather than retaining duplicate obsolete LOC.
7. **Certify the post-delete tree** — the same product and quality gates must remain green with the Python witness physically absent.
8. **Update the current-state inventory** — `r09-python-to-rust-inventory.md` should describe only the resulting authority boundary and remaining migration work.

## What parity means

Related unit tests are not sufficient by themselves. Rust must preserve every **material observable** owned by the Python contract, which may include:

- source-only mutations changing the managed artifact;
- fixed managed-consumer outputs;
- CLR export identity and rename behavior;
- required metadata/member references;
- required IL opcodes such as `call`, `callvirt` or `newobj`;
- multiple exports surviving one codegen invocation;
- independently compiled managed dependencies;
- negative compiler behavior or failure paths.

Harness mechanics do not need line-by-line translation. `COPY`, `TRANSFORM` or `ADAPT` are all acceptable when the externally protected contract remains equivalent.

## Current authority boundary

The authoritative Rust families and remaining Python responsibilities are listed in `r09-python-to-rust-inventory.md`. That inventory, the current workflows and executable tests are the source of truth; old verifier files and old cutover narration are not retained merely as provenance.

When the last Python-owned observable has an equivalent Rust gate and survives this sequence, Python can disappear from FerrumWeave's verification codebase entirely.
