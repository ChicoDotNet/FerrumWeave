# R04 CTS scalar mapping status

R04 is **in progress**. This document records only the type mappings that have executable product-level evidence; it is not a claim that CLR projection or managed interoperability is complete.

## R04 S01 — direct CLI scalars

| Rust type | CLI / CTS representation | Status |
| --- | --- | --- |
| `bool` | `ELEMENT_TYPE_BOOLEAN` / `System.Boolean` | Certified by Rust unit tests |
| `i8` | `ELEMENT_TYPE_I1` / `System.SByte` | Certified by Rust unit tests |
| `u8` | `ELEMENT_TYPE_U1` / `System.Byte` | Certified by Rust unit tests |
| `i16` | `ELEMENT_TYPE_I2` / `System.Int16` | Certified by Rust unit tests |
| `u16` | `ELEMENT_TYPE_U2` / `System.UInt16` | Certified by Rust unit tests |
| `i32` | `ELEMENT_TYPE_I4` / `System.Int32` | Certified by Rust unit tests |
| `u32` | `ELEMENT_TYPE_U4` / `System.UInt32` | Certified by Rust unit tests |
| `i64` | `ELEMENT_TYPE_I8` / `System.Int64` | Certified by Rust unit tests |
| `u64` | `ELEMENT_TYPE_U8` / `System.UInt64` | Certified by Rust unit tests |
| `isize` | `ELEMENT_TYPE_I` / `System.IntPtr` | Certified by Rust unit tests |
| `usize` | `ELEMENT_TYPE_U` / `System.UIntPtr` | Certified by Rust unit tests |
| `i128` / `u128` | No direct CLI primitive; named CTS value-type work pending | Explicitly unsupported as a direct mapping |
| Rust `char` | **Not** directly `System.Char` | Explicitly unsupported as a lossy direct mapping |

## Why `char` is intentionally different

Rust `char` is a Unicode scalar value. CLR `System.Char` is one UTF-16 code unit. These domains are not equivalent: a Rust scalar outside the BMP requires a surrogate pair in UTF-16, so a direct one-value-to-one-`System.Char` mapping would lose information.

R04 must choose and certify a projection policy rather than silently narrow the value.

## Not yet certified

R04 still needs executable evidence for named 128-bit CTS value types, managed references/object identity, `System.String`, arrays, public-boundary nullability, and emitted-signature round trips inspected independently by the CLR.

The canonical implementation policy for the current slice lives in `projection/types`; see ADR 0003 for the separation between direct CLI scalars and richer CTS projections.
