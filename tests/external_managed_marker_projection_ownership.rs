use std::fs;

#[test]
fn external_managed_marker_identity_is_owned_by_projection_policy() {
    let backend_path = "compiler/codegen-backend/src/external_managed_lowering.rs";
    let backend = fs::read_to_string(backend_path)
        .unwrap_or_else(|error| panic!("failed to read {backend_path}: {error}"));

    assert!(
        !backend.contains("EXTERNAL_MANAGED_TRANSFORM_MARKER"),
        "rustc backend still owns the external-managed marker identity; move marker resolution behind projection/types"
    );
    assert!(
        !backend.contains("ferrumweave_external_managed_transform"),
        "rustc backend still embeds the external-managed marker spelling"
    );
    assert!(
        backend.contains("external_managed_operation_from_marker"),
        "rustc backend must delegate external-managed marker interpretation to projection/types"
    );

    let projection_path = "projection/types/src/lib.rs";
    let projection = fs::read_to_string(projection_path)
        .unwrap_or_else(|error| panic!("failed to read {projection_path}: {error}"));

    assert!(
        projection.contains("pub enum ExternalManagedOperation"),
        "projection/types must own a stable semantic descriptor for external managed operations"
    );
    assert!(
        projection.contains("pub fn external_managed_operation_from_marker"),
        "projection/types must own external-managed marker resolution"
    );
}
