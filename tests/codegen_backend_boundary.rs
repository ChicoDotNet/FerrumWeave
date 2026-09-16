mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use support::{
    TOOLCHAIN, assert_success, build_codegen_backend, compile_rust_source, create_work_dir,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

const LOWERING_MARKER: &str = "FERRUMWEAVE_MIR_LOWERING_FAILED";

#[test]
fn codegen_backend_boundary_is_source_causal_and_rejects_invalid_rust() {
    let backend = build_codegen_backend();
    let work = create_work_dir("codegen-backend-boundary");

    let answer_137 = compile_variant(&backend, &work, "answer_137", 137, "answer");
    let answer_211 = compile_variant(&backend, &work, "answer_211", 211, "answer");
    let renamed = compile_variant(&backend, &work, "compute_result_137", 137, "compute_result");

    assert_managed_shape(&answer_137, "Answer");
    assert_managed_shape(&answer_211, "Answer");
    assert_managed_shape(&renamed, "ComputeResult");

    assert_eq!(
        run_consumer(&answer_137, &work, "answer_137", "Answer"),
        137
    );
    assert_eq!(
        run_consumer(&answer_211, &work, "answer_211", "Answer"),
        211
    );
    assert_eq!(
        run_consumer(&renamed, &work, "compute_result_137", "ComputeResult"),
        137
    );

    let baseline = fs::read(&answer_137).expect("baseline boundary artifact should be readable");
    assert_ne!(
        baseline,
        fs::read(&answer_211).expect("payload-mutated boundary artifact should be readable"),
        "changing only Rust source 137 -> 211 must change the managed artifact"
    );
    assert_ne!(
        baseline,
        fs::read(&renamed).expect("renamed boundary artifact should be readable"),
        "changing only Rust export answer -> compute_result must change managed metadata"
    );

    assert_invalid_rust_rejected_before_lowering(&backend, &work);
    remove_work_dir(&work);
}

fn rust_source(value: i32, export_name: &str) -> String {
    format!("#[no_mangle]\npub extern \"C\" fn {export_name}() -> i32 {{ {value} }}\n")
}

fn compile_variant(
    backend: &Path,
    work: &Path,
    name: &str,
    value: i32,
    export_name: &str,
) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("{name}.rs"),
        &format!("{name}.dll"),
        &rust_source(value, export_name),
        &format!("FerrumWeave should compile boundary variant {name}"),
    )
}

fn assert_managed_shape(artifact: &Path, method_name: &str) {
    let image = fs::read(artifact).expect("boundary artifact should be readable");
    for expected in [
        "MZ",
        "BSJB",
        "FerrumWeave.Generated",
        "FerrumWeave",
        "RustApi",
        method_name,
    ] {
        assert!(
            image
                .windows(expected.len())
                .any(|window| window == expected.as_bytes()),
            "boundary artifact is missing {expected:?}"
        );
    }
}

fn run_consumer(artifact: &Path, root: &Path, label: &str, method_name: &str) -> i32 {
    let program = format!("System.Console.WriteLine(FerrumWeave.RustApi.{method_name}());\n");
    let project = make_csharp_consumer(artifact, root, label, &program);
    let output = run_managed_consumer(&project, &format!("boundary consumer for {label}"));
    assert_success(
        &output,
        &format!("boundary consumer should pass for {label}"),
    );

    let stdout = String::from_utf8(output.stdout).expect("consumer stdout should be UTF-8");
    let observed = stdout
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .expect("boundary consumer should produce an observable")
        .trim();
    observed.parse::<i32>().unwrap_or_else(|error| {
        panic!("unexpected boundary observable for {label}: {observed:?}: {error}")
    })
}

fn assert_invalid_rust_rejected_before_lowering(backend: &Path, work: &Path) {
    let source = work.join("invalid_answer.rs");
    let artifact = work.join("invalid_answer.dll");
    fs::write(
        &source,
        "#[no_mangle]\npub extern \"C\" fn answer() -> i32 { \"not-an-i32\" }\n",
    )
    .expect("invalid Rust source should be written");

    let output = Command::new("rustc")
        .arg(format!("+{TOOLCHAIN}"))
        .arg("-Z")
        .arg(format!("codegen-backend={}", backend.display()))
        .args(["--edition", "2021", "--crate-type", "lib"])
        .arg(&source)
        .arg("-o")
        .arg(&artifact)
        .output()
        .expect("rustc should start for the negative boundary contract");

    assert!(
        !output.status.success(),
        "invalid Rust must fail before producing a managed artifact"
    );
    assert!(
        !artifact.exists(),
        "invalid Rust must not leave behind a managed artifact"
    );
    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
    assert!(
        !combined.contains(LOWERING_MARKER),
        "invalid Rust must fail in rustc semantics before FerrumWeave lowering; output:\n{combined}"
    );
}
