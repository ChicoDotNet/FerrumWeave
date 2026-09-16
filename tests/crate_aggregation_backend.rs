mod support;

use std::fs;
use std::path::{Path, PathBuf};

use support::{
    assert_success, build_codegen_backend, compile_rust_source, create_work_dir,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

#[test]
fn crate_aggregation_is_preserved_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("crate-aggregation");

    replay_multiple_exports(&backend, &work);
    replay_heterogeneous_exports(&backend, &work);
    replay_mixed_constant_arithmetic(&backend, &work);
    replay_mixed_constant_control_flow(&backend, &work);
    replay_mixed_constant_direct_call(&backend, &work);

    remove_work_dir(&work);
}

fn replay_multiple_exports(backend: &Path, work: &Path) {
    let artifact = compile(
        backend,
        work,
        "multiple_exports",
        r#"#[no_mangle]
pub extern "C" fn alpha_value() -> i32 { 137 }

#[no_mangle]
pub extern "C" fn beta_value() -> i32 { 211 }
"#,
    );
    assert_metadata(&artifact, &["AlphaValue", "BetaValue"]);
    assert_observable(
        &artifact,
        work,
        "multiple_exports",
        "var type = typeof(FerrumWeave.RustApi);\nvar alpha = type.GetMethod(\"AlphaValue\");\nvar beta = type.GetMethod(\"BetaValue\");\nif (alpha is null || beta is null) throw new System.Exception(\"multiple-export metadata lost a source-owned export\");\nSystem.Console.WriteLine($\"{FerrumWeave.RustApi.AlphaValue()},{FerrumWeave.RustApi.BetaValue()}\");\n",
        "137,211",
    );
}

fn replay_heterogeneous_exports(backend: &Path, work: &Path) {
    let artifact = compile(
        backend,
        work,
        "heterogeneous_exports",
        r#"#[no_mangle]
pub extern "C" fn constant_value() -> i32 { 137 }

#[no_mangle]
pub extern "C" fn echo_value(value: i32) -> i32 { value }
"#,
    );
    assert_metadata(&artifact, &["ConstantValue", "EchoValue"]);
    assert_observable(
        &artifact,
        work,
        "heterogeneous_exports",
        "var type = typeof(FerrumWeave.RustApi);\nvar constant = type.GetMethod(\"ConstantValue\");\nvar echo = type.GetMethod(\"EchoValue\");\nif (constant is null || echo is null) throw new System.Exception(\"heterogeneous metadata lost a source-owned export\");\nSystem.Console.WriteLine($\"{FerrumWeave.RustApi.ConstantValue()},{FerrumWeave.RustApi.EchoValue(211)}\");\n",
        "137,211",
    );
}

fn replay_mixed_constant_arithmetic(backend: &Path, work: &Path) {
    let artifact = compile(
        backend,
        work,
        "mixed_constant_arithmetic",
        r#"#[no_mangle]
pub extern "C" fn constant_value() -> i32 { 137 }

#[no_mangle]
pub extern "C" fn add_values(left: i32, right: i32) -> i32 {
    let result = left + right;
    result
}
"#,
    );
    assert_metadata(&artifact, &["ConstantValue", "AddValues"]);
    assert_observable(
        &artifact,
        work,
        "mixed_constant_arithmetic",
        "var type = typeof(FerrumWeave.RustApi);\nvar constant = type.GetMethod(\"ConstantValue\");\nvar add = type.GetMethod(\"AddValues\");\nif (constant is null || add is null) throw new System.Exception(\"mixed arithmetic metadata lost a source-owned export\");\nSystem.Console.WriteLine($\"{FerrumWeave.RustApi.ConstantValue()},{FerrumWeave.RustApi.AddValues(137, 74)}\");\n",
        "137,211",
    );
}

fn replay_mixed_constant_control_flow(backend: &Path, work: &Path) {
    let artifact = compile(
        backend,
        work,
        "mixed_constant_control_flow",
        r#"#[no_mangle]
pub extern "C" fn constant_value() -> i32 { 137 }

#[no_mangle]
pub extern "C" fn choose_value(selector: i32, left: i32, right: i32) -> i32 {
    if selector == 0 { left } else { right }
}
"#,
    );
    assert_metadata(&artifact, &["ConstantValue", "ChooseValue"]);
    assert_observable(
        &artifact,
        work,
        "mixed_constant_control_flow",
        "var type = typeof(FerrumWeave.RustApi);\nvar constant = type.GetMethod(\"ConstantValue\");\nvar choose = type.GetMethod(\"ChooseValue\");\nif (constant is null || choose is null) throw new System.Exception(\"mixed control-flow metadata lost a source-owned export\");\nSystem.Console.WriteLine($\"{FerrumWeave.RustApi.ConstantValue()},{FerrumWeave.RustApi.ChooseValue(0, 137, 211)},{FerrumWeave.RustApi.ChooseValue(1, 137, 211)}\");\n",
        "137,137,211",
    );
}

fn replay_mixed_constant_direct_call(backend: &Path, work: &Path) {
    let artifact = compile(
        backend,
        work,
        "mixed_constant_direct_call",
        r#"#[no_mangle]
pub extern "C" fn constant_value() -> i32 { 137 }

#[inline(never)]
fn helper(left: i32, right: i32) -> i32 { left + right }

#[no_mangle]
pub extern "C" fn compute_result(left: i32, right: i32) -> i32 {
    helper(left, right)
}
"#,
    );
    assert_metadata(&artifact, &["ConstantValue", "ComputeResult"]);
    assert_observable(
        &artifact,
        work,
        "mixed_constant_direct_call",
        "var type = typeof(FerrumWeave.RustApi);\nvar constant = type.GetMethod(\"ConstantValue\");\nvar compute = type.GetMethod(\"ComputeResult\");\nif (constant is null || compute is null) throw new System.Exception(\"mixed direct-call metadata lost a source-owned export\");\nvar il = compute.GetMethodBody()?.GetILAsByteArray();\nif (il is null || System.Array.IndexOf(il, (byte)0x28) < 0) throw new System.Exception(\"ComputeResult lost its managed call opcode\");\nSystem.Console.WriteLine($\"{FerrumWeave.RustApi.ConstantValue()},{FerrumWeave.RustApi.ComputeResult(137, 74)}\");\n",
        "137,211",
    );
}

fn compile(backend: &Path, work: &Path, name: &str, source: &str) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("{name}.rs"),
        &format!("{name}.dll"),
        source,
        &format!("FerrumWeave should preserve crate aggregation for {name}"),
    )
}

fn assert_metadata(artifact: &Path, expected_methods: &[&str]) {
    let image = fs::read(artifact).expect("crate aggregation artifact should be readable");
    for expected in ["MZ", "BSJB", "FerrumWeave.Generated", "RustApi"]
        .into_iter()
        .chain(expected_methods.iter().copied())
    {
        assert!(
            image
                .windows(expected.len())
                .any(|window| window == expected.as_bytes()),
            "crate aggregation artifact is missing {expected:?}"
        );
    }
}

fn assert_observable(artifact: &Path, work: &Path, label: &str, program: &str, expected: &str) {
    let project = make_csharp_consumer(artifact, work, label, program);
    let output = run_managed_consumer(
        &project,
        &format!("crate aggregation managed consumer for {label}"),
    );
    assert_success(
        &output,
        &format!("crate aggregation managed consumer should pass for {label}"),
    );
    let stdout = String::from_utf8(output.stdout).expect("consumer stdout should be UTF-8");
    let observed = stdout
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .expect("crate aggregation consumer should produce an observable")
        .trim();
    assert_eq!(observed, expected, "unexpected observable for {label}");
}
