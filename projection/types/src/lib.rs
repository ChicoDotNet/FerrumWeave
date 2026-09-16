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

/// Stable semantic descriptor for external managed operations recognized by FerrumWeave.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExternalManagedOperation {
    Transform,
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

/// Resolve a Rust-side marker into an external managed semantic operation.
#[must_use]
pub fn external_managed_operation_from_marker(marker: &str) -> Option<ExternalManagedOperation> {
    match marker {
        "ferrumweave_external_managed_transform" => Some(ExternalManagedOperation::Transform),
        _ => None,
    }
}

/// Project a rustc-owned export symbol into the public CLR method identity.
///
/// The projection is deterministic and source-causal: snake_case words become
/// PascalCase while already contiguous identifier content is otherwise preserved.
/// This keeps the historical `answer -> Answer` surface without letting compiler
/// lowering or CIL emission own a fixture-specific method name.
#[must_use]
pub fn managed_method_name_from_export_symbol(export_symbol: &str) -> String {
    export_symbol
        .split('_')
        .filter(|segment| !segment.is_empty())
        .map(|segment| {
            let mut chars = segment.chars();
            let Some(first) = chars.next() else {
                return String::new();
            };
            first.to_uppercase().chain(chars).collect::<String>()
        })
        .collect()
}

#[cfg(test)]
mod managed_intrinsic_tests {
    use super::{
        ExternalManagedOperation, ManagedIntrinsic, external_managed_operation_from_marker,
        managed_intrinsic_from_marker, managed_method_name_from_export_symbol,
    };

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

    #[test]
    fn external_managed_marker_resolution_is_centralized_and_exact() {
        assert_eq!(
            external_managed_operation_from_marker("ferrumweave_external_managed_transform"),
            Some(ExternalManagedOperation::Transform)
        );
        assert_eq!(
            external_managed_operation_from_marker("ferrumweave_external_managed_unknown"),
            None
        );
    }

    #[test]
    fn rust_export_symbols_project_to_stable_managed_method_names() {
        assert_eq!(managed_method_name_from_export_symbol("answer"), "Answer");
        assert_eq!(
            managed_method_name_from_export_symbol("compute_result"),
            "ComputeResult"
        );
        assert_eq!(managed_method_name_from_export_symbol("already"), "Already");
    }
}
