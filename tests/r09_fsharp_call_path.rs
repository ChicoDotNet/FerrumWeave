use std::path::PathBuf;
use std::process::Command;

#[test]
fn fsharp_calls_ferrumweave_through_the_supported_solution_graph() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/r09");

    let run = Command::new("dotnet")
        .args([
            "run",
            "--project",
            "Analytics/Analytics.fsproj",
            "--configuration",
            "Debug",
            "--nologo",
        ])
        .current_dir(&fixture)
        .output()
        .expect("R09 requires the canonical F# project to execute");

    assert!(
        run.status.success(),
        "R09 F# must execute through the supported project graph and call FerrumWeave Rust:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr),
    );

    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.lines().any(|line| line.trim() == "fs-risk-score=42"),
        "R09 F# must observe the Rust -> managed dependency business result; stdout was:\n{stdout}"
    );
}
