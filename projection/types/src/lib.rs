#![forbid(unsafe_code)]

mod cts_policy;
pub use cts_policy::*;

/// Stable semantic descriptor for managed API operations recognized by FerrumWeave.
///
/// Rust marker spellings are a projection concern. The rustc backend consumes this
/// descriptor and remains responsible only for MIR shape and CIL lowering.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ManagedIntrinsic {
    SystemMathAbs,
    SystemMathSign,
    SystemObjectNew,
    SystemTextStringBuilderNew,
    SystemObjectToString,
    SystemTextStringBuilderToString,
    SystemTextStringBuilderLength,
}

/// Resolve a Rust-side marker into the managed semantic operation it represents.
#[must_use]
pub fn managed_intrinsic_from_marker(marker: &str) -> Option<ManagedIntrinsic> {
    match marker {
        "ferrumweave_system_math_abs" => Some(ManagedIntrinsic::SystemMathAbs),
        "ferrumweave_system_math_sign" => Some(ManagedIntrinsic::SystemMathSign),
        "ferrumweave_system_object_new" => Some(ManagedIntrinsic::SystemObjectNew),
        "ferrumweave_system_text_string_builder_new" => {
            Some(ManagedIntrinsic::SystemTextStringBuilderNew)
        }
        "ferrumweave_system_object_to_string" => Some(ManagedIntrinsic::SystemObjectToString),
        "ferrumweave_system_text_string_builder_to_string" => {
            Some(ManagedIntrinsic::SystemTextStringBuilderToString)
        }
        "ferrumweave_system_text_string_builder_length" => {
            Some(ManagedIntrinsic::SystemTextStringBuilderLength)
        }
        _ => None,
    }
}

#[cfg(test)]
mod managed_intrinsic_tests {
    use super::{ManagedIntrinsic, managed_intrinsic_from_marker};

    #[test]
    fn marker_resolution_is_centralized_and_exact() {
        let cases = [
            (
                "ferrumweave_system_math_abs",
                ManagedIntrinsic::SystemMathAbs,
            ),
            (
                "ferrumweave_system_math_sign",
                ManagedIntrinsic::SystemMathSign,
            ),
            (
                "ferrumweave_system_object_new",
                ManagedIntrinsic::SystemObjectNew,
            ),
            (
                "ferrumweave_system_text_string_builder_new",
                ManagedIntrinsic::SystemTextStringBuilderNew,
            ),
            (
                "ferrumweave_system_object_to_string",
                ManagedIntrinsic::SystemObjectToString,
            ),
            (
                "ferrumweave_system_text_string_builder_to_string",
                ManagedIntrinsic::SystemTextStringBuilderToString,
            ),
            (
                "ferrumweave_system_text_string_builder_length",
                ManagedIntrinsic::SystemTextStringBuilderLength,
            ),
        ];

        for (marker, expected) in cases {
            assert_eq!(managed_intrinsic_from_marker(marker), Some(expected));
        }
        assert_eq!(managed_intrinsic_from_marker("ferrumweave_unknown"), None);
    }
}
