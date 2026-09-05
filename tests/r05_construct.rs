use ferrumweave::managed::{
    method_constructs_member_ref, resolve_public_constructor_member_ref,
    resolve_public_static_method,
};

#[test]
fn resolved_public_constructor_is_the_newobj_target_in_emitted_il() {
    let image = ferrumweave_cil::emit_probe_assembly();

    let entry = resolve_public_static_method(&image, "", "<Module>", "Main")
        .expect("R05 should resolve the managed entry point");
    let constructor = resolve_public_constructor_member_ref(&image, "System", "Object")
        .expect("R05 should resolve System.Object::.ctor through CLR metadata");

    assert_eq!(constructor.namespace, "System");
    assert_eq!(constructor.type_name, "Object");
    assert_eq!(constructor.method_name, ".ctor");
    assert!(
        method_constructs_member_ref(&image, &entry, &constructor)
            .expect("R05 should inspect newobj in the emitted managed method body")
    );
}
