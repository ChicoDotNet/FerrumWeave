use ferrumweave::managed::{
    method_calls_member_ref, resolve_public_static_member_ref, resolve_public_static_method,
};

#[test]
fn resolved_public_static_member_ref_is_the_call_target_in_emitted_il() {
    let image = ferrumweave_cil::emit_probe_assembly();

    let entry = resolve_public_static_method(&image, "", "<Module>", "Main")
        .expect("R05 should resolve the managed entry point");
    let target = resolve_public_static_member_ref(&image, "System", "Console", "WriteLine")
        .expect("R05 should resolve System.Console.WriteLine through CLR metadata");

    assert_eq!(target.namespace, "System");
    assert_eq!(target.type_name, "Console");
    assert_eq!(target.method_name, "WriteLine");
    assert_eq!(target.token, 0x0A00_0001);
    assert!(
        method_calls_member_ref(&image, &entry, &target)
            .expect("R05 should inspect the emitted managed method body")
    );
}
