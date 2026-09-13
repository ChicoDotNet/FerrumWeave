use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const PANIC_BOUNDARY_DIAGNOSTIC: &str = "FERRUMWEAVE_PANIC_BOUNDARY_REJECTED";

#[test]
fn uncontained_panic_is_rejected_by_ferrumweave_backend_before_managed_export() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let root = unique_temp_dir();
    let source_dir = root.join("rust-source/src");
    let rust_project = root.join("rust-source");
    let rust_source = source_dir.join("main.rs");
    fs::create_dir_all(&source_dir).expect("create R07 panic-boundary Rust source directory");
    fs::copy(template, rust_project.join("RustLibrary.rsproj")).expect("copy canonical rsproj");

    fs::write(
        &rust_source,
        r#"#[no_mangle]
pub extern "C" fn answer() -> i32 {
    panic!("panic must not unwind across the managed boundary")
}
"#,
    )
    .expect("write R07 Rust panic-boundary source");

    let rejected = dotnet_build(&repo, &rust_project);
    assert!(
        !rejected.status.success(),
        "an uncontained Rust panic at a supported managed public boundary must be rejected before a valid managed artifact is produced",
    );

    let output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&rejected.stdout),
        String::from_utf8_lossy(&rejected.stderr),
    );
    assert!(
        output.contains(PANIC_BOUNDARY_DIAGNOSTIC),
        "the FerrumWeave backend must reject uncontained panic with an explicit stable diagnostic; current output:\n{output}",
    );

    let assembly = rust_project.join("bin/Debug/net10.0/RustLibrary.dll");
    assert!(
        !assembly.is_file(),
        "panic-boundary rejection must happen before a valid managed artifact exists",
    );

    fs::write(
        &rust_source,
        r#"#[no_mangle]
pub extern "C" fn answer() -> i32 {
    42
}
"#,
    )
    .expect("mutate only Rust source from panic to a supported return");

    let accepted = dotnet_build(&repo, &rust_project);
    assert!(
        accepted.status.success(),
        "removing only the uncontained Rust panic must remove the policy rejection; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&accepted.stdout),
        String::from_utf8_lossy(&accepted.stderr),
    );
    assert!(
        assembly.is_file(),
        "the source-only non-panic mutation must produce the managed artifact",
    );

    let _ = fs::remove_dir_all(root);
}

fn dotnet_build(repo: &Path, project: &Path) -> Output {
    Command::new("dotnet")
        .args(["build", "RustLibrary.rsproj"])
        .current_dir(project)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute")
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r07-panic-boundary-{}-{nonce}",
        std::process::id(),
    ))
}
