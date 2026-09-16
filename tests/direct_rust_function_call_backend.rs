mod support;

use std::fs;
use std::path::{Path, PathBuf};

use support::{
    assert_success, build_codegen_backend, compile_rust_source, create_work_dir,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

#[test]
fn direct_rust_function_call_is_source_causal_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("direct-rust-call");

    let add_artifact = compile_variant(&backend, &work, "helper_add", "+", "answer");
    let sub_artifact = compile_variant(&backend, &work, "helper_sub", "-", "answer");
    let renamed_artifact =
        compile_variant(&backend, &work, "helper_add_renamed", "+", "compute_result");

    assert_ne!(
        fs::read(&add_artifact).expect("add artifact should be readable"),
        fs::read(&sub_artifact).expect("sub artifact should be readable"),
        "Rust-only helper mutation must change the managed artifact"
    );
    assert_ne!(
        fs::read(&add_artifact).expect("add artifact should remain readable"),
        fs::read(&renamed_artifact).expect("renamed artifact should be readable"),
        "changing only the Rust export name answer -> compute_result must change managed metadata"
    );

    assert_eq!(run_consumer(&add_artifact, &work, "add", "Answer"), 211);
    assert_eq!(run_consumer(&sub_artifact, &work, "sub", "Answer"), 63);
    assert_eq!(
        run_consumer(&renamed_artifact, &work, "renamed", "ComputeResult"),
        211
    );

    remove_work_dir(&work);
}

fn rust_source(operator: &str, export_name: &str) -> String {
    assert!(matches!(operator, "+" | "-"));
    format!(
        "#[inline(never)]\nfn helper(left: i32, right: i32) -> i32 {{\n    left {operator} right\n}}\n\n#[no_mangle]\npub extern \"C\" fn {export_name}(left: i32, right: i32) -> i32 {{\n    helper(left, right)\n}}\n"
    )
}

fn compile_variant(
    backend: &Path,
    work: &Path,
    name: &str,
    operator: &str,
    export_name: &str,
) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("{name}.rs"),
        &format!("{name}.dll"),
        &rust_source(operator, export_name),
        &format!("FerrumWeave should lower direct Rust function call variant {name}"),
    )
}

fn run_consumer(artifact: &Path, root: &Path, name: &str, method_name: &str) -> i32 {
    let program = format!(
        "var method = typeof(FerrumWeave.RustApi).GetMethod(\"{method_name}\");\n\
var il = method?.GetMethodBody()?.GetILAsByteArray();\n\
if (il is null || System.Array.IndexOf(il, (byte)0x28) < 0)\n\
    throw new System.Exception(\"{method_name} does not contain a managed call opcode\");\n\
System.Console.WriteLine(FerrumWeave.RustApi.{method_name}(137, 74));\n"
    );
    let project = make_csharp_consumer(artifact, root, name, &program);
    let output = run_managed_consumer(&project, &format!("managed consumer for {name}"));
    assert_success(&output, &format!("managed consumer should pass for {name}"));

    let stdout = String::from_utf8(output.stdout).expect("consumer stdout should be UTF-8");
    let observed = stdout
        .lines()
        .filter(|line| !line.trim().is_empty())
        .next_back()
        .expect("managed consumer should produce a direct-call observable")
        .trim();
    observed.parse::<i32>().unwrap_or_else(|error| {
        panic!("unexpected managed observable for {name}: {observed:?}: {error}")
    })
}
