use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn real_nuget_package_is_restored_and_observed_in_the_business_call_path() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/r09");

    let restore = Command::new("dotnet")
        .args(["restore", "Enterprise.slnx", "--nologo"])
        .current_dir(&fixture)
        .output()
        .expect("R09 requires the canonical mixed solution to restore through normal dotnet tooling");

    assert!(
        restore.status.success(),
        "R09 NuGet gate requires a successful normal restore:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&restore.stdout),
        String::from_utf8_lossy(&restore.stderr),
    );

    let assets_path = fixture.join("Domain/obj/project.assets.json");
    let assets = fs::read_to_string(&assets_path)
        .expect("R09 NuGet gate requires Domain project.assets.json after restore");
    assert!(
        assets.contains("Newtonsoft.Json/13.0.3"),
        "R09 must restore the real Newtonsoft.Json 13.0.3 NuGet package into the supported .NET dependency graph"
    );

    let run = Command::new("dotnet")
        .args([
            "run",
            "--project",
            "Domain/Domain.csproj",
            "--configuration",
            "Debug",
            "--no-restore",
            "--nologo",
        ])
        .current_dir(&fixture)
        .output()
        .expect("R09 requires the canonical C# business entrypoint to execute after NuGet restore");

    assert!(
        run.status.success(),
        "R09 NuGet consumer must execute through the supported solution graph:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr),
    );

    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout
            .lines()
            .any(|line| line.trim() == "nuget-risk-score={\"score\":42}"),
        "R09 must observably consume the restored NuGet package using the Rust-produced business value; stdout was:\n{stdout}"
    );
}
