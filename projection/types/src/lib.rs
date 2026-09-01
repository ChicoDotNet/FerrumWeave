#![forbid(unsafe_code)]

/// Rust scalar types whose CLR representation must be decided deliberately.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustScalar {
    Bool,
    I8,
    U8,
    I16,
    U16,
    I32,
    U32,
    I64,
    U64,
    I128,
    U128,
    Isize,
    Usize,
    Char,
}

/// ECMA-335 ELEMENT_TYPE values used by direct CLI scalar signatures.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum CliElementType {
    Boolean = 0x02,
    I1 = 0x04,
    U1 = 0x05,
    I2 = 0x06,
    U2 = 0x07,
    I4 = 0x08,
    U4 = 0x09,
    I8 = 0x0A,
    U8 = 0x0B,
    NativeInt = 0x18,
    NativeUInt = 0x19,
}

impl CliElementType {
    #[must_use]
    pub const fn code(self) -> u8 {
        self as u8
    }
}

/// A lossless direct mapping to a CLI scalar element type and its CTS name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DirectCtsMapping {
    pub rust: RustScalar,
    pub element_type: CliElementType,
    pub system_type: &'static str,
}

/// Why a Rust scalar cannot be represented as a direct CLI scalar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoDirectCliMapping {
    pub rust: RustScalar,
    pub reason: &'static str,
}

/// Return FerrumWeave's centralized direct Rust -> CTS scalar policy.
///
/// `Err` is intentional: not every Rust scalar has a lossless ECMA-335
/// primitive encoding. Named CTS value types and projection-specific types are
/// separate mapping classes and must not be smuggled into this direct layer.
pub const fn direct_cts_mapping(rust: RustScalar) -> Result<DirectCtsMapping, NoDirectCliMapping> {
    let (element_type, system_type) = match rust {
        RustScalar::Bool => (CliElementType::Boolean, "System.Boolean"),
        RustScalar::I8 => (CliElementType::I1, "System.SByte"),
        RustScalar::U8 => (CliElementType::U1, "System.Byte"),
        RustScalar::I16 => (CliElementType::I2, "System.Int16"),
        RustScalar::U16 => (CliElementType::U2, "System.UInt16"),
        RustScalar::I32 => (CliElementType::I4, "System.Int32"),
        RustScalar::U32 => (CliElementType::U4, "System.UInt32"),
        RustScalar::I64 => (CliElementType::I8, "System.Int64"),
        RustScalar::U64 => (CliElementType::U8, "System.UInt64"),
        RustScalar::Isize => (CliElementType::NativeInt, "System.IntPtr"),
        RustScalar::Usize => (CliElementType::NativeUInt, "System.UIntPtr"),
        RustScalar::I128 | RustScalar::U128 => {
            return Err(NoDirectCliMapping {
                rust,
                reason: "128-bit Rust integers require named CTS value-type mapping; ECMA-335 has no direct 128-bit integer element type",
            });
        }
        RustScalar::Char => {
            return Err(NoDirectCliMapping {
                rust,
                reason: "Rust char is a Unicode scalar value while System.Char is one UTF-16 code unit; a direct mapping would be lossy",
            });
        }
    };

    Ok(DirectCtsMapping {
        rust,
        element_type,
        system_type,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fixed_width_integer_mappings_are_explicit() {
        let cases = [
            (RustScalar::I8, CliElementType::I1, "System.SByte"),
            (RustScalar::U8, CliElementType::U1, "System.Byte"),
            (RustScalar::I16, CliElementType::I2, "System.Int16"),
            (RustScalar::U16, CliElementType::U2, "System.UInt16"),
            (RustScalar::I32, CliElementType::I4, "System.Int32"),
            (RustScalar::U32, CliElementType::U4, "System.UInt32"),
            (RustScalar::I64, CliElementType::I8, "System.Int64"),
            (RustScalar::U64, CliElementType::U8, "System.UInt64"),
        ];

        for (rust, element_type, system_type) in cases {
            assert_eq!(
                direct_cts_mapping(rust),
                Ok(DirectCtsMapping {
                    rust,
                    element_type,
                    system_type,
                })
            );
        }
    }

    #[test]
    fn bool_maps_to_system_boolean() {
        assert_eq!(
            direct_cts_mapping(RustScalar::Bool),
            Ok(DirectCtsMapping {
                rust: RustScalar::Bool,
                element_type: CliElementType::Boolean,
                system_type: "System.Boolean",
            })
        );
    }

    #[test]
    fn pointer_sized_integers_use_native_cli_integer_types() {
        assert_eq!(
            direct_cts_mapping(RustScalar::Isize),
            Ok(DirectCtsMapping {
                rust: RustScalar::Isize,
                element_type: CliElementType::NativeInt,
                system_type: "System.IntPtr",
            })
        );
        assert_eq!(
            direct_cts_mapping(RustScalar::Usize),
            Ok(DirectCtsMapping {
                rust: RustScalar::Usize,
                element_type: CliElementType::NativeUInt,
                system_type: "System.UIntPtr",
            })
        );
    }

    #[test]
    fn cli_element_type_codes_match_ecma_335() {
        assert_eq!(CliElementType::Boolean.code(), 0x02);
        assert_eq!(CliElementType::I1.code(), 0x04);
        assert_eq!(CliElementType::U1.code(), 0x05);
        assert_eq!(CliElementType::I2.code(), 0x06);
        assert_eq!(CliElementType::U2.code(), 0x07);
        assert_eq!(CliElementType::I4.code(), 0x08);
        assert_eq!(CliElementType::U4.code(), 0x09);
        assert_eq!(CliElementType::I8.code(), 0x0A);
        assert_eq!(CliElementType::U8.code(), 0x0B);
        assert_eq!(CliElementType::NativeInt.code(), 0x18);
        assert_eq!(CliElementType::NativeUInt.code(), 0x19);
    }

    #[test]
    fn rust_char_is_not_silently_narrowed_to_system_char() {
        let error = direct_cts_mapping(RustScalar::Char).expect_err("char must not map directly");
        assert_eq!(error.rust, RustScalar::Char);
        assert!(error.reason.contains("UTF-16"));
    }

    #[test]
    fn i128_and_u128_are_not_misrepresented_as_cli_primitives() {
        for rust in [RustScalar::I128, RustScalar::U128] {
            let error =
                direct_cts_mapping(rust).expect_err("128-bit integer needs named CTS mapping");
            assert_eq!(error.rust, rust);
            assert!(
                error
                    .reason
                    .contains("no direct 128-bit integer element type")
            );
        }
    }
}
