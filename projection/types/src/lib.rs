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

/// ECMA-335 ELEMENT_TYPE values used by FerrumWeave's CTS signatures.
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
    String = 0x0E,
    NativeInt = 0x18,
    NativeUInt = 0x19,
    Object = 0x1C,
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

/// A Rust scalar represented by a named CTS value type instead of a primitive ELEMENT_TYPE.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NamedCtsValueTypeMapping {
    pub rust: RustScalar,
    pub namespace: &'static str,
    pub name: &'static str,
}

impl NamedCtsValueTypeMapping {
    #[must_use]
    pub fn full_name(self) -> String {
        format!("{}.{}", self.namespace, self.name)
    }
}

/// Rust-side reference semantics relevant at a managed public boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RustReferenceSemantics {
    SharedBorrow,
    ExclusiveBorrow,
    ManagedObjectHandle,
}

/// A projection whose CLR representation carries managed object identity.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ManagedObjectMapping {
    pub element_type: CliElementType,
    pub system_type: &'static str,
    pub preserves_object_identity: bool,
}

/// Why Rust reference semantics cannot be represented as a CLR object reference directly.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoManagedObjectMapping {
    pub rust: RustReferenceSemantics,
    pub reason: &'static str,
}

/// Why a Rust scalar cannot be represented as a direct CLI scalar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoDirectCliMapping {
    pub rust: RustScalar,
    pub reason: &'static str,
}

/// FerrumWeave's lossless Rust UTF-8 -> CLR `System.String` projection boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SystemStringProjection {
    pub element_type: CliElementType,
    pub system_type: &'static str,
    pub clr_encoding: &'static str,
}

/// A CLR UTF-16 payload that cannot be represented as a valid Rust `String`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct InvalidClrString {
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

/// Return a named CTS value-type mapping when the Rust scalar has no direct CLI primitive.
#[must_use]
pub const fn named_cts_value_type_mapping(rust: RustScalar) -> Option<NamedCtsValueTypeMapping> {
    match rust {
        RustScalar::I128 => Some(NamedCtsValueTypeMapping {
            rust,
            namespace: "System",
            name: "Int128",
        }),
        RustScalar::U128 => Some(NamedCtsValueTypeMapping {
            rust,
            namespace: "System",
            name: "UInt128",
        }),
        _ => None,
    }
}

/// Return the CLR object-reference mapping only for an explicit managed handle.
///
/// A CLR object reference preserves managed object identity, but it does not
/// encode Rust's shared/exclusive borrow rules. `&T` and `&mut T` therefore
/// cannot cross this boundary merely by being re-labelled as `System.Object`.
pub const fn managed_object_mapping(
    rust: RustReferenceSemantics,
) -> Result<ManagedObjectMapping, NoManagedObjectMapping> {
    match rust {
        RustReferenceSemantics::ManagedObjectHandle => Ok(ManagedObjectMapping {
            element_type: CliElementType::Object,
            system_type: "System.Object",
            preserves_object_identity: true,
        }),
        RustReferenceSemantics::SharedBorrow | RustReferenceSemantics::ExclusiveBorrow => {
            Err(NoManagedObjectMapping {
                rust,
                reason: "CLR object references preserve managed identity but do not encode Rust borrow semantics",
            })
        }
    }
}

/// Return the CLR signature/encoding policy for Rust strings.
#[must_use]
pub const fn system_string_projection() -> SystemStringProjection {
    SystemStringProjection {
        element_type: CliElementType::String,
        system_type: "System.String",
        clr_encoding: "UTF-16",
    }
}

/// Encode a valid Rust UTF-8 string as the UTF-16 code units carried by `System.String`.
#[must_use]
pub fn rust_string_to_utf16(value: &str) -> Vec<u16> {
    value.encode_utf16().collect()
}

/// Decode CLR UTF-16 code units into a Rust `String` without silently replacing invalid data.
///
/// CLR `System.String` can contain unpaired surrogate code units while Rust `String`
/// must be valid Unicode scalar data. Such CLR payloads are rejected explicitly.
pub fn utf16_to_rust_string(value: &[u16]) -> Result<String, InvalidClrString> {
    String::from_utf16(value).map_err(|_| InvalidClrString {
        reason: "System.String contains an unpaired UTF-16 surrogate that cannot be represented by Rust String",
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
        assert_eq!(CliElementType::String.code(), 0x0E);
        assert_eq!(CliElementType::NativeInt.code(), 0x18);
        assert_eq!(CliElementType::NativeUInt.code(), 0x19);
        assert_eq!(CliElementType::Object.code(), 0x1C);
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

    #[test]
    fn i128_and_u128_map_to_named_system_value_types() {
        let cases = [
            (RustScalar::I128, "System.Int128"),
            (RustScalar::U128, "System.UInt128"),
        ];

        for (rust, expected) in cases {
            let mapping = named_cts_value_type_mapping(rust)
                .expect("128-bit integer must have named CTS mapping");
            assert_eq!(mapping.rust, rust);
            assert_eq!(mapping.full_name(), expected);
        }
    }

    #[test]
    fn direct_cli_scalars_do_not_gain_named_value_type_aliases() {
        for rust in [
            RustScalar::Bool,
            RustScalar::I32,
            RustScalar::U64,
            RustScalar::Isize,
            RustScalar::Usize,
            RustScalar::Char,
        ] {
            assert_eq!(named_cts_value_type_mapping(rust), None);
        }
    }

    #[test]
    fn explicit_managed_handle_maps_to_system_object_identity() {
        assert_eq!(
            managed_object_mapping(RustReferenceSemantics::ManagedObjectHandle),
            Ok(ManagedObjectMapping {
                element_type: CliElementType::Object,
                system_type: "System.Object",
                preserves_object_identity: true,
            })
        );
    }

    #[test]
    fn rust_borrows_are_not_relabelled_as_managed_object_references() {
        for rust in [
            RustReferenceSemantics::SharedBorrow,
            RustReferenceSemantics::ExclusiveBorrow,
        ] {
            let error = managed_object_mapping(rust)
                .expect_err("Rust borrow semantics require a projection policy");
            assert_eq!(error.rust, rust);
            assert!(error.reason.contains("do not encode Rust borrow semantics"));
        }
    }

    #[test]
    fn rust_strings_project_to_system_string_and_round_trip_utf16() {
        let projection = system_string_projection();
        assert_eq!(projection.element_type, CliElementType::String);
        assert_eq!(projection.system_type, "System.String");
        assert_eq!(projection.clr_encoding, "UTF-16");

        let original = "FerrumWeave 🦀 Δ nul:\0";
        let clr_payload = rust_string_to_utf16(original);
        assert!(clr_payload.len() > original.chars().count());
        assert_eq!(utf16_to_rust_string(&clr_payload), Ok(original.to_owned()));
    }

    #[test]
    fn inbound_unpaired_clr_surrogates_are_rejected_without_replacement() {
        let error = utf16_to_rust_string(&[0xD800])
            .expect_err("unpaired CLR surrogate must not be silently replaced");
        assert!(error.reason.contains("unpaired UTF-16 surrogate"));
    }
}
