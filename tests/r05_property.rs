use ferrumweave::managed::{
    method_calls_property_accessors, resolve_public_property_accessors, resolve_public_static_method,
};

#[test]
fn resolved_public_property_accessors_are_both_called_in_emitted_il() {
    let image = ferrumweave_cil::emit_probe_assembly();

    let entry = resolve_public_static_method(&image, "", "<Module>", "Main")
        .expect("R05 should resolve the managed entry point");
    let property = resolve_public_property_accessors(
        &image,
        "System",
        "Environment",
        "CurrentDirectory",
    )
    .expect("R05 should resolve System.Environment.CurrentDirectory accessors");

    assert_eq!(property.namespace, "System");
    assert_eq!(property.type_name, "Environment");
    assert_eq!(property.property_name, "CurrentDirectory");
    assert!(
        method_calls_property_accessors(&image, &entry, &property)
            .expect("R05 should inspect getter and setter calls in emitted managed IL")
    );
}
