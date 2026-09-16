mod support;

use std::fs;
use std::path::{Path, PathBuf};

use support::{
    assert_success, build_codegen_backend, compile_rust_source, create_work_dir, last_stdout_line,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

#[test]
fn i32_argument_local_flow_is_source_causal_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("i32-local-flow-rust");

    let left_artifact = compile_source(&backend, &work, "left", "answer");
    let right_artifact = compile_source(&backend, &work, "right", "answer");
    let renamed_artifact = compile_source(&backend, &work, "left", "compute_result");

    execute_from_csharp(&left_artifact, 137, &work, "left", "Answer");
    execute_from_csharp(&right_artifact, 211, &work, "right", "Answer");
    execute_from_csharp(
        &renamed_artifact,
        137,
        &work,
        "left_renamed",
        "ComputeResult",
    );

    let left_bytes = fs::read(&left_artifact).expect("left artifact should be readable");
    let right_bytes = fs::read(&right_artifact).expect("right artifact should be readable");
    let renamed_bytes = fs::read(&renamed_artifact).expect("renamed artifact should be readable");

    assert_ne!(
        left_bytes, right_bytes,
        "changing only Rust local selection left -> right must change the managed artifact"
    );
    assert_ne!(
        fs::read(&left_artifact).expect("left artifact should remain readable"),
        renamed_bytes,
        "changing only Rust export identity answer -> compute_result must change managed metadata"
    );

    remove_work_dir(&work);
}

fn rust_source(selected: &str, export_name: &str) -> String {
    assert!(matches!(selected, "left" | "right"));
    format!(
        "#[no_mangle]\npub extern \"C\" fn {export_name}(left: i32, right: i32) -> i32 {{\n    let selected = {selected};\n    selected\n}}\n"
    )
}

fn compile_source(backend: &Path, work: &Path, selected: &str, export_name: &str) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("select_{selected}_{export_name}.rs"),
        &format!("select_{selected}_{export_name}.dll"),
        &rust_source(selected, export_name),
        &format!(
            "FerrumWeave should lower Rust local flow selecting {selected} for export {export_name}"
        ),
    )
}

fn execute_from_csharp(
    artifact: &Path,
    expected: i32,
    root: &Path,
    label: &str,
    method_name: &str,
) {
    let program = format!(
        "System.Console.WriteLine(FerrumWeave.RustApi.{method_name}(137, 211));\n"
    );
    let project = make_csharp_consumer(artifact, root, label, &program);
    let output = run_managed_consumer(
        &project,
        &format!("C# consumer should execute Rust selection {label}"),
    );
    assert_success(
        &output,
        &format!("C# consumer should execute Rust selection {label}"),
    );

    assert_eq!(
        last_stdout_line(&output),
        expected.to_string(),
        "Rust selection {label} should drive the managed observable"
    );
}
