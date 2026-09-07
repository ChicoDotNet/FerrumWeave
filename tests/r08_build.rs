use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r08-build-{}-{nonce}",
        std::process::id()
    ))
}

#[test]
fn dotnet_build_produces_a_managed_ferrumweave_assembly() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust");
    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated build test directory");

    fs::copy(
        template.join("HelloFerrum.rsproj"),
        temp.join("HelloFerrum.rsproj"),
    )
    .expect("copy canonical R08 project fixture");
    fs::copy(template.join("src/main.rs"), source_dir.join("main.rs"))
        .expect("copy canonical R08 Rust source fixture");

    let build = Command::new("dotnet")
        .args(["build", "HelloFerrum.rsproj", "--no-restore"])
        .current_dir(&temp)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute");

    assert!(
        build.status.success(),
        "FerrumWeave.Sdk must own a successful dotnet build:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let assembly = temp.join("bin/Debug/net10.0/HelloFerrum.dll");
    assert!(
        assembly.is_file(),
        "dotnet build must produce the managed FerrumWeave assembly at the standard TargetPath"
    );
    let bytes = fs::read(&assembly).expect("read managed build artifact");
    assert!(
        bytes.starts_with(b"MZ"),
        "FerrumWeave build output must be a PE/CLI assembly, not a native Rust artifact or placeholder"
    );

    let _ = fs::remove_dir_all(temp);
}
