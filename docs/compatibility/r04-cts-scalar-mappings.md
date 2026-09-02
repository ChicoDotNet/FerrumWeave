# R04 CTS mapping status

R04 is **certified complete** for its declared 11-contract scope. The canonical mapping policy lives in `projection/types`; emitted signature evidence is produced by FerrumWeave and inspected independently through CLR reflection on Linux and Windows.

## Certified mappings and boundaries

| Rust / FerrumWeave boundary | CLI / CTS representation | Evidence |
| --- | --- | --- |
| `bool` | `ELEMENT_TYPE_BOOLEAN` / `System.Boolean` | Rust unit tests + emitted CLR reflection |
| `i8` | `ELEMENT_TYPE_I1` / `System.SByte` | Rust unit tests |
| `u8` | `ELEMENT_TYPE_U1` / `System.Byte` | Rust unit tests |
| `i16` | `ELEMENT_TYPE_I2` / `System.Int16` | Rust unit tests |
| `u16` | `ELEMENT_TYPE_U2` / `System.UInt16` | Rust unit tests |
| `i32` | `ELEMENT_TYPE_I4` / `System.Int32` | Rust unit tests + emitted CLR reflection |
| `u32` | `ELEMENT_TYPE_U4` / `System.UInt32` | Rust unit tests |
| `i64` | `ELEMENT_TYPE_I8` / `System.Int64` | Rust unit tests |
| `u64` | `ELEMENT_TYPE_U8` / `System.UInt64` | Rust unit tests |
| `isize` | `ELEMENT_TYPE_I` / `System.IntPtr` | Rust unit tests |
| `usize` | `ELEMENT_TYPE_U` / `System.UIntPtr` | Rust unit tests |
| `i128` | named value type `System.Int128` | Rust policy tests + emitted CLR reflection |
| `u128` | named value type `System.UInt128` | Rust policy tests + emitted CLR reflection |
| managed object handle | `ELEMENT_TYPE_OBJECT` / `System.Object` | identity/boundary tests + emitted CLR reflection |
| Rust string projection | `ELEMENT_TYPE_STRING` / `System.String` | UTF-8↔UTF-16 round-trip tests + emitted CLR reflection |
| supported owned array | `ELEMENT_TYPE_SZARRAY` | round-trip tests + emitted `System.Int32[]` reflection |
| optional managed reference | nullable CLR reference boundary | public-boundary integration tests |

## Explicit non-equivalences and limits

Rust `char` is not mapped directly to `System.Char`. Rust `char` is a Unicode scalar value while CLR `System.Char` is one UTF-16 code unit, so a silent one-to-one mapping would be lossy.

Rust `&T` and `&mut T` are not relabelled as CLR managed references. CLR object identity and GC reachability do not encode Rust shared/exclusive borrowing guarantees.

Optional value types are not silently projected as `System.Nullable<T>`. That representation requires an explicit later projection contract.

Inbound CLR strings containing unpaired UTF-16 surrogates are rejected rather than replacement-decoded when projected into Rust strings.

## Independent emitted-signature gate

`eng/r04/ReflectionVerifier` loads the FerrumWeave-emitted assembly with the CLR and verifies representative emitted signatures through `System.Reflection`, including:

- `System.Int32`;
- `System.Boolean`;
- `System.String`;
- `System.Object`;
- `System.Int32[]`;
- `System.Int128`;
- `System.UInt128`.

The R04 workflow runs that verifier on Ubuntu 22.04 and Windows. This is deliberately independent CLR evidence; Python does not implement or simulate the CTS semantics under test.

## Contract truth

The R04 functional ledger contains 11 known contracts and is complete at **11/11 = 100%**. Unsupported direct mappings fail explicitly instead of narrowing silently.

See ADR 0003 for the architectural separation between direct CLI scalars, named CTS value types, managed references, strings/arrays, nullability, and the ownership/GC boundary.
