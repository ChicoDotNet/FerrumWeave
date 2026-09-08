use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn project_reference_resolves_rust_assembly_without_manual_copying() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/r09");

    for project in [
        "Domain/Domain.csproj",
        "Legacy/Legacy.vbproj",
        "Analytics/Analytics.fsproj",
    ] {
        let text = fs::read_to_string(fixture.join(project))
            .unwrap_or_else(|error| panic!("R09 requires {project}: {error}"));
        assert!(
            text.contains("<ProjectReference Include=\"../RiskEngine/RiskEngine.rsproj\" />"),
            "R09 requires {project} to resolve RiskEngine through ProjectReference"
        );
        assert!(
            !text.contains("<Copy ") && !text.contains("<Exec "),
            "R09 forbids fixture-level manual copy/exec steps for RiskEngine in {project}"
        );
    }

    let clean = Command::new("dotnet")
        .args(["clean", "Enterprise.slnx", "--nologo"])
        .current_dir(&fixture)
        .output()
        .expect("R09 requires normal dotnet clean tooling");
    assert_command_succeeded("clean", &clean);

    let build = Command::new("dotnet")
        .args(["build", "Enterprise.slnx", "--nologo", "--no-restore"])
        .current_dir(&fixture)
        .output()
        .expect("R09 requires normal dotnet build tooling");
    assert_command_succeeded("build", &build);

    for consumer in ["Domain", "Legacy", "Analytics"] {
        let resolved = fixture
            .join(consumer)
            .join("bin/Debug/net10.0/RiskEngine.dll");
        assert!(
            resolved.is_file(),
            "R09 requires ProjectReference to place RiskEngine.dll in {consumer}'s build output; missing {}",
            resolved.display()
        );
    }
}

fn assert_command_succeeded(stage: &str, output: &std::process::Output) {
    assert!(
        output.status.success(),
        "R09 {stage} failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}
