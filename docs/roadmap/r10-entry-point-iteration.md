# R10 CLR entry-point iteration

Status: **implementation candidate under certification**.

This slice implements the first executable portion of ADR 0005 without claiming the broader `FW-R10-DX-003` console experience.

## Intended verified outcome

For an installed FerrumWeave console project with `<OutputType>Exe</OutputType>`:

```rust
fn main() {}
```

must remain ordinary Rust source and be compiled by `rustc` as a binary crate. FerrumWeave must project that source entry point to:

```text
HelloFerrum.Program.Main
```

where `Main` is static and its `MethodDef` is the PE/CLI entry point. `Program.Main` must call the compiler-owned projected Rust-main helper rather than replacing source semantics with an unrelated export.

The same managed assembly may continue exposing already-certified managed API exports such as `FerrumWeave.RustApi.Answer`; executable identity must not destroy the distribution/interoperability evidence that preceded it.

## Deliberate boundary

This iteration accepts only a no-op `fn main() -> ()`. Non-trivial main bodies, `Result`/`Termination`, command-line arguments, and Rust-facing `System.Console.WriteLine` are still RED and require explicit lowering contracts.

Therefore this slice may make `FW-R10-CLR-ENTRY-001` GREEN while `FW-R10-DX-003` remains RED for the correct next reason: the canonical console still lacks a Rust-source-causal visible output.

## Independent acceptance

`eng/r10/Test-FerrumWeaveClrEntryPoint.ps1` must certify on Windows and Linux that:

1. the distributable package boundary remains GREEN;
2. an independent .NET 10 reflection consumer observes `Assembly.EntryPoint` as `HelloFerrum.Program.Main`;
3. `Main` is static with the initial `void ()` signature;
4. its IL calls `HelloFerrum.Program.__FerrumWeaveRustMain`;
5. normal `dotnet run` exits successfully through CoreCLR;
6. deleting Rust `fn main()` makes the executable build fail closed through rustc (`E0601` / main-not-found), proving the CLR entry point is not an unrelated synthetic export.

No release claim is promoted until exact-SHA CI supplies that evidence.