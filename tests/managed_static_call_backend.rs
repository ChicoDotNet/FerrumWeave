mod support;

use std::fs;
use std::path::{Path, PathBuf};

use support::{
    assert_success, build_codegen_backend, compile_rust_source, create_work_dir,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

#[test]
fn managed_static_call_is_source_causal_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("managed-static-call");

    let abs_137 = compile_variant(
        &backend,
        &work,
        "abs_137",
        "ferrumweave_system_math_abs",
        137,
        "answer",
    );
    let abs_211 = compile_variant(
        &backend,
        &work,
        "abs_211",
        "ferrumweave_system_math_abs",
        211,
        "answer",
    );
    let sign_137 = compile_variant(
        &backend,
        &work,
        "sign_137",
        "ferrumweave_system_math_sign",
        137,
        "answer",
    );
    let renamed = compile_variant(
        &backend,
        &work,
        "renamed_export_137",
        "ferrumweave_system_math_abs",
        137,
        "compute_result",
    );

    assert_managed_target(&abs_137, "Abs", "Answer");
    assert_managed_target(&abs_211, "Abs", "Answer");
    assert_managed_target(&sign_137, "Sign", "Answer");
    assert_managed_target(&renamed, "Abs", "ComputeResult");

    assert_eq!(run_consumer(&abs_137, &work, "abs_137", "Answer"), 137);
    assert_eq!(run_consumer(&abs_211, &work, "abs_211", "Answer"), 211);
    assert_eq!(run_consumer(&sign_137, &work, "sign_137", "Answer"), 1);
    assert_eq!(
        run_consumer(&renamed, &work, "renamed_export_137", "ComputeResult"),
        137
    );

    let baseline = fs::read(&abs_137).expect("baseline managed artifact should be readable");
    assert_ne!(
        baseline,
        fs::read(&abs_211).expect("argument-mutated managed artifact should be readable"),
        "changing only the Rust call argument 137 -> 211 must change the managed artifact"
    );
    assert_ne!(
        baseline,
        fs::read(&sign_137).expect("method-mutated managed artifact should be readable"),
        "changing only the Rust-selected managed method Abs -> Sign must change the managed artifact"
    );
    assert_ne!(
        baseline,
        fs::read(&renamed).expect("renamed managed artifact should be readable"),
        "changing only the Rust export name answer -> compute_result must change managed metadata"
    );

    remove_work_dir(&work);
}

fn rust_source(marker: &str, argument: i32, export_name: &str) -> String {
    assert!(matches!(
        marker,
        "ferrumweave_system_math_abs" | "ferrumweave_system_math_sign"
    ));
    format!(
        "#[inline(never)]\nfn ferrumweave_system_math_abs(value: i32) -> i32 {{ value }}\n\n#[inline(never)]\nfn ferrumweave_system_math_sign(value: i32) -> i32 {{ value }}\n\n#[no_mangle]\npub extern \"C\" fn {export_name}() -> i32 {{ {marker}({argument}) }}\n"
    )
}

fn compile_variant(
    backend: &Path,
    work: &Path,
    name: &str,
    marker: &str,
    argument: i32,
    export_name: &str,
) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("{name}.rs"),
        &format!("{name}.dll"),
        &rust_source(marker, argument, export_name),
        &format!("FerrumWeave should lower managed static call variant {name}"),
    )
}

fn assert_managed_target(artifact: &Path, managed_method: &str, export_method: &str) {
    let image = fs::read(artifact).expect("managed static-call artifact should be readable");
    for expected in [
        "MZ",
        "BSJB",
        "FerrumWeave.Generated",
        "FerrumWeave",
        "RustApi",
        export_method,
        "System",
        "Math",
        managed_method,
    ] {
        assert!(
            image
                .windows(expected.len())
                .any(|window| window == expected.as_bytes()),
            "managed artifact for {managed_method} is missing {expected:?}"
        );
    }
}

fn run_consumer(artifact: &Path, root: &Path, name: &str, method_name: &str) -> i32 {
    let program = format!(
        "System.Console.WriteLine(FerrumWeave.RustApi.{method_name}());\n"
    );
    let project = make_csharp_consumer(artifact, root, name, &program);
    let output = run_managed_consumer(&project, &format!("managed static-call consumer for {name}"));
    assert_success(
        &output,
        &format!("managed static-call consumer should pass for {name}"),
    );

    let stdout = String::from_utf8(output.stdout).expect("consumer stdout should be UTF-8");
    let observed = stdout
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .expect("managed static-call consumer should produce an observable")
        .trim();
    observed.parse::<i32>().unwrap_or_else(|error| {
        panic!("unexpected managed observable for {name}: {observed:?}: {error}")
    })
}
