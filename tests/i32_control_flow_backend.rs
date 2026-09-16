mod support;

use std::fs;
use std::path::{Path, PathBuf};

use support::{
    assert_success, build_codegen_backend, compile_rust_source, create_work_dir, last_stdout_line,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

#[test]
fn i32_control_flow_is_source_causal_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("i32-control-flow-rust");

    let eq_artifact = compile_source(&backend, &work, "eq", "==", "answer");
    let ne_artifact = compile_source(&backend, &work, "ne", "!=", "answer");
    let renamed_artifact = compile_source(&backend, &work, "renamed", "==", "compute_result");

    execute_from_csharp(&eq_artifact, 0, 137, &work, "eq_zero", "Answer");
    execute_from_csharp(&eq_artifact, 1, 211, &work, "eq_one", "Answer");
    execute_from_csharp(&ne_artifact, 0, 211, &work, "ne_zero", "Answer");
    execute_from_csharp(&ne_artifact, 1, 137, &work, "ne_one", "Answer");
    execute_from_csharp(
        &renamed_artifact,
        0,
        137,
        &work,
        "renamed_zero",
        "ComputeResult",
    );
    execute_from_csharp(
        &renamed_artifact,
        1,
        211,
        &work,
        "renamed_one",
        "ComputeResult",
    );

    let eq_bytes = fs::read(&eq_artifact).expect("equal-predicate artifact should be readable");
    let ne_bytes = fs::read(&ne_artifact).expect("not-equal-predicate artifact should be readable");
    let renamed_bytes =
        fs::read(&renamed_artifact).expect("renamed control-flow artifact should be readable");

    assert_ne!(
        eq_bytes, ne_bytes,
        "changing only Rust predicate == -> != must change the managed artifact"
    );
    assert_ne!(
        fs::read(&eq_artifact).expect("equal-predicate artifact should remain readable"),
        renamed_bytes,
        "changing only Rust export identity answer -> compute_result must change managed metadata"
    );

    remove_work_dir(&work);
}

fn rust_source(predicate: &str, export_name: &str) -> String {
    assert!(matches!(predicate, "==" | "!="));
    format!(
        "#[no_mangle]\npub extern \"C\" fn {export_name}(selector: i32, left: i32, right: i32) -> i32 {{\n    if selector {predicate} 0 {{\n        left\n    }} else {{\n        right\n    }}\n}}\n"
    )
}

fn compile_source(
    backend: &Path,
    work: &Path,
    label: &str,
    predicate: &str,
    export_name: &str,
) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("control_flow_{label}.rs"),
        &format!("control_flow_{label}.dll"),
        &rust_source(predicate, export_name),
        &format!(
            "FerrumWeave should lower Rust control flow {label} using {predicate} for export {export_name}"
        ),
    )
}

fn execute_from_csharp(
    artifact: &Path,
    selector: i32,
    expected: i32,
    root: &Path,
    label: &str,
    method_name: &str,
) {
    let program = format!(
        "System.Console.WriteLine(FerrumWeave.RustApi.{method_name}({selector}, 137, 211));\n"
    );
    let project = make_csharp_consumer(artifact, root, label, &program);
    let output = run_managed_consumer(
        &project,
        &format!("C# consumer should execute Rust control flow {label}, selector={selector}"),
    );
    assert_success(
        &output,
        &format!("C# consumer should execute Rust control flow {label}, selector={selector}"),
    );

    assert_eq!(
        last_stdout_line(&output),
        expected.to_string(),
        "Rust control flow {label}, selector={selector} should drive the managed observable"
    );
}
