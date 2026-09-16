#![forbid(unsafe_code)]

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn artifact_probe_modules_are_named_by_responsibility() {
    let cil_source = repo_root().join("compiler/cil/src");

    assert!(
        !cil_source.join("r06.rs").exists(),
        "artifact-level managed API probe must not keep milestone module name r06.rs"
    );
    assert!(
        !cil_source.join("r07.rs").exists(),
        "artifact-level disposable-resource probe must not keep milestone module name r07.rs"
    );
    assert!(
        cil_source.join("managed_api_probe.rs").exists(),
        "artifact-level managed API probe must use the responsibility-named module managed_api_probe.rs"
    );
    assert!(
        cil_source.join("disposable_resource_probe.rs").exists(),
        "artifact-level disposable-resource probe must use the responsibility-named module disposable_resource_probe.rs"
    );
}

#[test]
fn cil_root_declares_responsibility_named_probe_modules() {
    let root_path = repo_root().join("compiler/cil/src/root.rs");
    let root = fs::read_to_string(&root_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", root_path.display()));

    assert!(
        root.contains("mod managed_api_probe;"),
        "compiler/cil root must declare managed_api_probe by responsibility"
    );
    assert!(
        root.contains("mod disposable_resource_probe;"),
        "compiler/cil root must declare disposable_resource_probe by responsibility"
    );
    assert!(
        !root.contains("mod r06;"),
        "compiler/cil root must not retain milestone module declaration r06"
    );
    assert!(
        !root.contains("mod r07;"),
        "compiler/cil root must not retain milestone module declaration r07"
    );
}
