use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

#[test]
fn published_alpha_instructions_install_sdk_into_clean_project() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let guide = repo.join("docs/getting-started.md");
    assert!(
        guide.is_file(),
        "R10 requires published getting-started instructions at {}",
        guide.display()
    );

    let instructions = fs::read_to_string(&guide)
        .expect("R10 getting-started instructions must be readable as UTF-8 text");
    let sdk_reference = "FerrumWeave.Sdk/0.1.0-alpha.1";
    assert!(
        instructions.contains(sdk_reference),
        "R10 getting-started instructions must publish the exact alpha SDK reference {sdk_reference}"
    );

    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    let temp_root = std::env::temp_dir().join(format!(
        "ferrumweave-r10-install-{}-{nonce}",
        std::process::id()
    ));
    let feed = temp_root.join("feed");
    let consumer = temp_root.join("consumer");
    fs::create_dir_all(&feed).expect("R10 local package feed must be creatable");
    fs::create_dir_all(&consumer).expect("R10 clean consumer directory must be creatable");

    let sdk_project = repo.join("sdk/FerrumWeave.Sdk/FerrumWeave.Sdk.csproj");
    let pack = Command::new("dotnet")
        .arg("pack")
        .arg(&sdk_project)
        .arg("--nologo")
        .arg("--configuration")
        .arg("Release")
        .arg("--output")
        .arg(&feed)
        .arg("-p:PackageVersion=0.1.0-alpha.1")
        .output()
        .expect("R10 requires dotnet to package the alpha SDK for installation proof");
    assert!(
        pack.status.success(),
        "R10 SDK packaging failed before installation proof\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&pack.stdout),
        String::from_utf8_lossy(&pack.stderr)
    );

    let project = consumer.join("External.rsproj");
    fs::write(
        &project,
        format!(
            "<Project Sdk=\"{sdk_reference}\">\n  <PropertyGroup>\n    <TargetFramework>net10.0</TargetFramework>\n  </PropertyGroup>\n</Project>\n"
        ),
    )
    .expect("R10 clean consumer project must be writable");

    let restore = Command::new("dotnet")
        .arg("restore")
        .arg(&project)
        .arg("--nologo")
        .arg("--source")
        .arg(&feed)
        .output()
        .expect("R10 requires dotnet restore to execute the documented SDK installation path");

    if !restore.status.success() {
        panic!(
            "R10 external SDK restore failed\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&restore.stdout),
            String::from_utf8_lossy(&restore.stderr)
        );
    }

    let _ = fs::remove_dir_all(temp_root);
}
