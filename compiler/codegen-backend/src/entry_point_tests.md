# Entry-point lowering invariants

The R10 entry-point slice intentionally fails closed.

- `rustc` remains authoritative for discovering the binary crate entry function.
- FerrumWeave consumes `TyCtxt::entry_fn` rather than inferring `main` from a symbol string.
- The initial supported source body is only no-op `fn main() -> ()`.
- Any assignment, control-flow expansion, non-unit return, or richer `Termination` behavior requires another executable lowering contract before it can be emitted.
- CIL emission owns CLR identity and entry metadata; it does not own source-language semantics.
