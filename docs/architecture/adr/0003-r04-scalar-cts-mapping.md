# ADR 0003 — Separate direct CLI scalars from richer CTS projections

- Status: Accepted for R04 S01
- Date: 2026-09-01
- Milestone: R04 — CLR / CTS foundation

## Context

R03 proved a coherent safe-Rust subset can execute through the pinned CLR backend. R04 now needs a stable vocabulary for the type boundary before managed references, strings, arrays, and public interoperability are built on top of it.

A dangerous shortcut would be to treat every superficially similar Rust and .NET type as interchangeable. Some are direct ECMA-335 scalar encodings; others require named CTS value types or projection policy; some are not semantically equivalent at all.

## Decision

FerrumWeave centralizes Rust-to-CTS type policy under `projection/types` and distinguishes **direct CLI scalar mappings** from richer projection classes.

For R04 S01:

- Rust `bool` maps directly to `ELEMENT_TYPE_BOOLEAN` / `System.Boolean`.
- `i8/u8/i16/u16/i32/u32/i64/u64` map directly to their signed/unsigned CLI scalar element types and corresponding `System.*` CTS names.
- `isize/usize` map to CLI native integer element types and `System.IntPtr` / `System.UIntPtr`.
- `i128/u128` are **not** encoded as direct CLI primitives because ECMA-335 defines no direct 128-bit integer element type. A later R04 slice must evaluate named `System.Int128` / `System.UInt128` value-type mappings and prove them through emitted metadata and CLR reflection.
- Rust `char` is **not** mapped directly to `System.Char`: Rust `char` represents a Unicode scalar value, while CLR `System.Char` represents one UTF-16 code unit. Silent narrowing would be semantically incorrect.

The mapping policy is product logic and therefore lives in Rust with Rust unit tests. Orchestration languages may invoke and independently inspect compiled artifacts, but they must not become the implementation of the mapping itself.

## Consequences

This creates a deliberate seam for later R04 work:

1. direct CLI scalars;
2. named CTS value types;
3. managed references and object identity;
4. strings and arrays;
5. nullability/public-boundary policy.

It also means some superficially convenient mappings remain unsupported until they can be represented losslessly and certified. That is intentional: explicit unsupported behavior is preferable to metadata that compiles but changes Rust semantics.

## Validation

R04 S01 is validated by Rust unit tests for every direct scalar mapping, the stable ECMA-335 element codes used by FerrumWeave, and explicit rejection of direct `char` and 128-bit integer mappings.

Later R04 slices must add independent CLR metadata/reflection evidence before public managed signatures are considered certified.
