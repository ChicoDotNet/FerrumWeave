//! Verifies the managed-metadata/emission primitive behind static .NET calls.
//!
//! This test intentionally does not certify that real Rust source caused the
//! managed call. Source causality is a separate R05 re-certification contract.

use ferrumweave::managed::{
    method_calls_member_ref, resolve_public_static_member_ref, resolve_public_static_method,
};

#[test]
fn resolved_public_static_member_ref_is_the_call_target_in_emitted_il() {
    let image = ferrumweave_cil::emit_probe_assembly();

    let entry = resolve_public_static_method(&image, "", "<Module>", "Main")
        .expect("managed call emission should resolve the managed entry point");
    let target = resolve_public_static_member_ref(&image, "System", "Console", "WriteLine")
        .expect("managed call emission should resolve System.Console.WriteLine through CLR metadata");

    assert_eq!(target.namespace, "System");
    assert_eq!(target.type_name, "Console");
    assert_eq!(target.method_name, "WriteLine");
    assert_eq!(target.token, 0x0A00_0001);
    assert!(
        method_calls_member_ref(&image, &entry, &target)
            .expect("managed call emission should inspect the emitted method body")
    );
}
