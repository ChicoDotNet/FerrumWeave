mod support;

use std::path::Path;

use support::{
    build_codegen_backend, compile_rust_source, create_work_dir, make_csharp_consumer,
    remove_work_dir, run_managed_consumer,
};

const RUST_SOURCE: &str = "#[no_mangle]\npub extern \"C\" fn answer(left: i32, right: i32) -> i32 {\n    left + right\n}\n";

#[test]
fn checked_i32_overflow_takes_the_failure_path_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("i32-overflow-rust");

    let artifact = compile_rust_source(
        &backend,
        &work,
        "checked_add.rs",
        "checked_add.dll",
        RUST_SOURCE,
        "FerrumWeave should lower checked Rust i32 addition through the pinned rustc lane",
    );
    let project = make_consumer(&artifact, &work);
    let output = run_managed_consumer(&project, "managed checked-overflow consumer");

    assert!(
        !output.status.success(),
        "Rust checked i32 overflow must take the failure path instead of returning a wrapped value; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    remove_work_dir(&work);
}

fn make_consumer(artifact: &Path, root: &Path) -> std::path::PathBuf {
    make_csharp_consumer(
        artifact,
        root,
        "overflow",
        "System.Console.WriteLine(FerrumWeave.RustApi.Answer(int.MaxValue, 1));\n",
    )
}
