# R10 contract evidence

The authoritative contract ledger is `contracts.toml`.

Current executable support lanes include:

- `FW-R10-DIST-001` — installed/distributable SDK boundary;
- `FW-R10-CLR-ENTRY-001` — Rust `fn main` to managed `Program.Main` / CLI entry-point projection;
- `FW-R10-DX-003` — meaningful installed console `dotnet run` developer experience;
- `FW-R10-CLR-SURFACE-001` — namespace-qualified CLR surface projection;
- web/webapi contracts listed in the ledger.

A narrower GREEN does not imply a broader contract is GREEN. In particular, CLR entry-point execution may be certified while the console developer-experience contract remains RED until visible behavior is source-causal from Rust.
