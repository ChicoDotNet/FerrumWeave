# ADR 0003 — Separate direct CLI scalars from richer CTS projections

- Status: Accepted and certified for R04
- Date: 2026-09-01
- Milestone: R04 — CLR / CTS foundation

## Context

R03 proved a coherent safe-Rust subset can execute through the pinned CLR backend. R04 needs a stable vocabulary for the Rust/CTS type boundary before broader managed interoperability is built on top of it.

A dangerous shortcut would be to treat every superficially similar Rust and .NET type as interchangeable. Some mappings are direct ECMA-335 scalar encodings; others require named CTS value types or explicit projection policy; some are not semantically equivalent at all.

## Decision

FerrumWeave centralizes Rust-to-CTS type policy under `projection/types` and separates five concerns:

1. direct CLI scalar mappings;
2. named CTS value types;
3. managed references and object identity;
4. strings and arrays;
5. nullability and public-boundary policy.

Direct scalar mappings use the corresponding ECMA-335 element types for `bool`, fixed-width integers, and native integers. Rust `i128` / `u128` are represented as named `System.Int128` / `System.UInt128` value types rather than invented primitive element codes.

Rust `char` is not mapped directly to `System.Char`: Rust `char` represents a Unicode scalar value while CLR `System.Char` represents one UTF-16 code unit. Silent narrowing would be semantically incorrect.

## Ownership and GC boundary

Managed object identity is represented explicitly rather than inferred from Rust references.

- An explicit managed-object handle may project to `System.Object` and preserve CLR object identity.
- Rust `&T` and `&mut T` are not relabelled as CLR managed references. GC reachability does not encode Rust shared/exclusive borrowing guarantees.
- A required managed reference is non-null at the supported public boundary.
- An optional managed reference may project to CLR nullable-reference semantics.
- `Option<T>` for value types is not silently converted to `System.Nullable<T>`; that requires an explicit later projection contract.
- R04 does not claim a general ownership transfer, pinning, finalization, `Drop`/`IDisposable`, or lifetime model. Those richer cross-runtime semantics belong to later milestones and must remain explicit.

This keeps Rust safety semantics authoritative while allowing CLR-managed identity to exist as a distinct property.

## String and array boundary

Rust strings project to `System.String` through UTF-16. Valid Rust strings round-trip losslessly; inbound CLR strings with unpaired UTF-16 surrogates are rejected rather than replacement-decoded.

Supported owned one-dimensional zero-based arrays project as `SZARRAY` and reuse the certified element mapping policy. Unsupported element mappings do not bypass that policy.

## Implementation ownership

Mapping policy is product logic and lives in Rust with Rust tests. Python may orchestrate or inspect artifacts, but it must not implement the CTS semantics being claimed.

Independent CLR evidence is provided by a .NET reflection verifier that loads FerrumWeave-emitted metadata and inspects representative signatures on Linux and Windows.

## Consequences

The type boundary remains deliberately conservative:

- lossless direct mappings are easy and centralized;
- richer mappings are explicit rather than hidden casts;
- Rust borrowing is not erased by CLR GC semantics;
- unsupported combinations fail clearly;
- future projection layers can add ergonomics without changing the underlying contract silently.

## Validation

R04 is certified by:

- Rust unit and integration tests for direct mappings, named value types, object identity, strings, arrays, nullability, and explicit rejection cases;
- round-trip tests for supported string/array and managed-boundary behavior;
- FerrumWeave-emitted representative signatures independently inspected through `System.Reflection` for `System.Int32`, `System.Boolean`, `System.String`, `System.Object`, `System.Int32[]`, `System.Int128`, and `System.UInt128`;
- the reflection gate running successfully on Ubuntu and Windows;
- Rust CI and the R03 semantic regression suite remaining green.
