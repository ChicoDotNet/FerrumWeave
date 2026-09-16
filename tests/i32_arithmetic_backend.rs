mod support;

use std::fs;
use std::path::{Path, PathBuf};

use support::{
    assert_success, build_codegen_backend, compile_rust_source, create_work_dir,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

#[test]
fn i32_arithmetic_is_source_causal_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("i32-arithmetic-rust");

    let add_artifact = compile_source(&backend, &work, "add", "+", "answer");
    let sub_artifact = compile_source(&backend, &work, "sub", "-", "answer");
    let renamed_artifact = compile_source(&backend, &work, "renamed", "+", "compute_result");

    execute_from_csharp(&add_artifact, 211, &work, "add", "Answer");
    execute_from_csharp(&sub_artifact, 63, &work, "sub", "Answer");
    execute_from_csharp(&renamed_artifact, 211, &work, "renamed", "ComputeResult");

    let add_bytes = fs::read(&add_artifact).expect("add artifact should be readable");
    let sub_bytes = fs::read(&sub_artifact).expect("sub artifact should be readable");
    let renamed_bytes = fs::read(&renamed_artifact).expect("renamed artifact should be readable");

    assert_ne!(
        add_bytes, sub_bytes,
        "changing only Rust operator + -> - must change the managed artifact"
    );
    assert_ne!(
        fs::read(&add_artifact).expect("add artifact should remain readable"),
        renamed_bytes,
        "changing only Rust export identity answer -> compute_result must change managed metadata"
    );

    remove_work_dir(&work);
}

fn rust_source(operator: &str, export_name: &str) -> String {
    assert!(matches!(operator, "+" | "-"));
    format!(
        "#[no_mangle]\npub extern \"C\" fn {export_name}(left: i32, right: i32) -> i32 {{\n    let result = left {operator} right;\n    result\n}}\n"
    )
}

fn compile_source(
    backend: &Path,
    work: &Path,
    label: &str,
    operator: &str,
    export_name: &str,
) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("arithmetic_{label}.rs"),
        &format!("arithmetic_{label}.dll"),
        &rust_source(operator, export_name),
        &format!(
            "FerrumWeave should lower Rust arithmetic {label} using {operator} for export {export_name}"
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
    let program =
        format!("System.Console.WriteLine(FerrumWeave.RustApi.{method_name}(137, 74));\n");
    let project = make_csharp_consumer(artifact, root, label, &program);
    let output = run_managed_consumer(
        &project,
        &format!("C# consumer should execute Rust arithmetic {label}"),
    );
    assert_success(
        &output,
        &format!("C# consumer should execute Rust arithmetic {label}"),
    );

    let stdout = String::from_utf8(output.stdout).expect("consumer stdout should be UTF-8");
    let observed = stdout.lines().next_back().unwrap_or_default().trim();
    assert_eq!(
        observed,
        expected.to_string(),
        "Rust arithmetic {label} should drive the managed observable"
    );
}
