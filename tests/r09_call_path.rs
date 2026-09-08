use std::path::PathBuf;
use std::process::Command;

#[test]
fn existing_dotnet_code_calls_rust_and_returns_from_managed_dependency() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/r09");

    let run = Command::new("dotnet")
        .args([
            "run",
            "--project",
            "Domain/Domain.csproj",
            "--configuration",
            "Debug",
            "--nologo",
        ])
        .current_dir(&fixture)
        .output()
        .expect("R09 requires the canonical existing .NET entrypoint to execute");

    assert!(
        run.status.success(),
        "R09 business call path must execute existing .NET code -> Rust -> managed .NET dependency:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr),
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout).trim(), "risk-score=42");
}
