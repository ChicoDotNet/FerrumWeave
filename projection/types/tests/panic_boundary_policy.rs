use ferrumweave_projection_types::{
    ManagedPanicBoundaryPolicy, managed_panic_boundary_policy,
};

#[test]
fn uncontained_rust_panic_is_rejected_before_managed_export() {
    let policy = managed_panic_boundary_policy();

    assert_eq!(policy, ManagedPanicBoundaryPolicy::RejectUncontainedPanic);
    assert!(!policy.permits_unwind_across_clr());
    assert_eq!(
        policy.diagnostic(),
        "uncontained Rust panic cannot cross a managed CLR public boundary; contain or model failure explicitly"
    );
}
