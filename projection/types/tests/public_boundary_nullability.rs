use ferrumweave_projection_types::{
    public_boundary_nullability, PublicBoundaryRepresentation, PublicBoundaryNullability,
    RustBoundaryPresence,
};

#[test]
fn required_public_boundaries_are_non_nullable() {
    for representation in [
        PublicBoundaryRepresentation::ValueType,
        PublicBoundaryRepresentation::ManagedReference,
    ] {
        assert_eq!(
            public_boundary_nullability(representation, RustBoundaryPresence::Required),
            Ok(PublicBoundaryNullability::NonNullable)
        );
    }
}

#[test]
fn optional_managed_references_project_to_nullable_clr_references() {
    assert_eq!(
        public_boundary_nullability(
            PublicBoundaryRepresentation::ManagedReference,
            RustBoundaryPresence::Optional,
        ),
        Ok(PublicBoundaryNullability::NullableReference)
    );
}

#[test]
fn optional_value_types_are_not_silently_relabelled_nullable() {
    let error = public_boundary_nullability(
        PublicBoundaryRepresentation::ValueType,
        RustBoundaryPresence::Optional,
    )
    .expect_err("Option<T> value types need an explicit System.Nullable<T> projection");

    assert!(error.reason.contains("System.Nullable<T>"));
}
