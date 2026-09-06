use ferrumweave_projection_types::{RustReferenceSemantics, managed_object_mapping};

#[test]
fn shared_borrow_is_rejected_instead_of_erased_into_gc_reachability() {
    let error = managed_object_mapping(RustReferenceSemantics::SharedBorrow)
        .expect_err("shared Rust borrow must not escape as a managed object reference");

    assert_eq!(error.rust, RustReferenceSemantics::SharedBorrow);
    assert_eq!(
        error.reason,
        "CLR object references preserve managed identity but do not encode Rust borrow semantics"
    );
}

#[test]
fn exclusive_borrow_is_rejected_instead_of_erased_into_gc_reachability() {
    let error = managed_object_mapping(RustReferenceSemantics::ExclusiveBorrow)
        .expect_err("exclusive Rust borrow must not escape as a managed object reference");

    assert_eq!(error.rust, RustReferenceSemantics::ExclusiveBorrow);
    assert_eq!(
        error.reason,
        "CLR object references preserve managed identity but do not encode Rust borrow semantics"
    );
}
