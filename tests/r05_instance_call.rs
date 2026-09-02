use ferrumweave::managed::{
    method_calls_instance_member_ref, resolve_public_instance_member_ref,
    resolve_public_static_method,
};

#[test]
fn resolved_public_instance_member_ref_is_the_callvirt_target_in_emitted_il() {
    let image = ferrumweave_cil::emit_probe_assembly();

    let entry = resolve_public_static_method(&image, "", "<Module>", "Main")
        .expect("R05 should resolve the managed entry point");
    let instance_method =
        resolve_public_instance_member_ref(&image, "System", "Object", "ToString")
            .expect("R05 should resolve System.Object.ToString through CLR metadata");

    assert_eq!(instance_method.namespace, "System");
    assert_eq!(instance_method.type_name, "Object");
    assert_eq!(instance_method.method_name, "ToString");
    assert!(
        method_calls_instance_member_ref(&image, &entry, &instance_method)
            .expect("R05 should inspect callvirt in the emitted managed method body")
    );
}
