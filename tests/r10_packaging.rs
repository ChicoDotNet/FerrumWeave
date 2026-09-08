use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn ferrumweave_sdk_packs_as_versioned_alpha_nuget_artifact() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project = repo.join("sdk/FerrumWeave.Sdk/FerrumWeave.Sdk.csproj");
    assert!(
        project.is_file(),
        "R10 requires a packable FerrumWeave SDK project at {}",
        project.display()
    );

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    let output_dir = std::env::temp_dir().join(format!(
        "ferrumweave-r10-pack-{}-{nonce}",
        std::process::id()
    ));
    fs::create_dir_all(&output_dir).expect("R10 packaging output directory must be creatable");

    let output = Command::new("dotnet")
        .arg("pack")
        .arg(&project)
        .arg("--nologo")
        .arg("--configuration")
        .arg("Release")
        .arg("--output")
        .arg(&output_dir)
        .arg("-p:PackageVersion=0.1.0-alpha.1")
        .output()
        .expect("R10 requires dotnet to execute the SDK packaging contract");

    if !output.status.success() {
        panic!(
            "R10 FerrumWeave SDK pack failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
    }

    let package = output_dir.join("FerrumWeave.Sdk.0.1.0-alpha.1.nupkg");
    assert!(
        package.is_file(),
        "R10 pack must produce the versioned alpha artifact {}",
        package.display()
    );

    let _ = fs::remove_dir_all(output_dir);
}
