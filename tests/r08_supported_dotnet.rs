use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn sdk_defaults_to_the_supported_dotnet_10_lts_line() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project = fs::read_to_string(repo.join("sdk/templates/rust/HelloFerrum.rsproj"))
        .expect("canonical R08 project must be readable");

    assert!(
        project.contains("<TargetFramework>net10.0</TargetFramework>"),
        "the canonical FerrumWeave SDK template must default to .NET 10 LTS",
    );

    let installed = Command::new("dotnet")
        .arg("--list-sdks")
        .output()
        .expect("dotnet SDK discovery must execute");

    assert!(
        installed.status.success(),
        "dotnet --list-sdks must succeed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&installed.stdout),
        String::from_utf8_lossy(&installed.stderr),
    );

    let stdout = String::from_utf8_lossy(&installed.stdout);
    assert!(
        stdout.lines().any(|line| line.starts_with("10.")),
        "the certified environment must provide a .NET 10 SDK; installed SDKs:\n{stdout}",
    );
}
