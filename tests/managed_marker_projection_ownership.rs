use std::fs;

#[test]
fn managed_marker_resolution_is_owned_by_projection_policy_not_backend_lowering() {
    let lowering_path = "compiler/codegen-backend/src/lowering.rs";
    let lowering = fs::read_to_string(lowering_path)
        .unwrap_or_else(|error| panic!("failed to read {lowering_path}: {error}"));

    for backend_owned_marker in [
        "SYSTEM_MATH_ABS_MARKER",
        "SYSTEM_MATH_SIGN_MARKER",
        "SYSTEM_OBJECT_NEW_MARKER",
        "SYSTEM_TEXT_STRING_BUILDER_NEW_MARKER",
        "SYSTEM_OBJECT_TO_STRING_MARKER",
        "SYSTEM_TEXT_STRING_BUILDER_TO_STRING_MARKER",
        "SYSTEM_TEXT_STRING_BUILDER_LENGTH_MARKER",
    ] {
        assert!(
            !lowering.contains(backend_owned_marker),
            "backend MIR lowering still owns CLR API-specific marker resolution via {backend_owned_marker}; move marker-to-managed-member policy behind projection/types"
        );
    }

    assert!(
        lowering.contains("managed_intrinsic_from_marker"),
        "backend lowering must delegate marker interpretation to projection/types"
    );

    let projection_path = "projection/types/src/lib.rs";
    let projection = fs::read_to_string(projection_path)
        .unwrap_or_else(|error| panic!("failed to read {projection_path}: {error}"));

    assert!(
        projection.contains("pub enum ManagedIntrinsic"),
        "projection/types must own the stable managed intrinsic descriptor"
    );
    assert!(
        projection.contains("pub fn managed_intrinsic_from_marker"),
        "projection/types must own marker-to-managed-intrinsic resolution"
    );
}
