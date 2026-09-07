use std::path::PathBuf;
use std::process::Command;

#[test]
fn normal_dotnet_build_builds_the_mixed_solution_graph() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let fixture = repo.join("tests/fixtures/r09");

    let build = Command::new("dotnet")
        .args(["build", "Enterprise.slnx", "--nologo"])
        .current_dir(&fixture)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("R09 requires the normal dotnet build workflow to execute");

    assert!(
        build.status.success(),
        "R09 mixed solution must build through one normal 'dotnet build Enterprise.slnx' workflow:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );
}
