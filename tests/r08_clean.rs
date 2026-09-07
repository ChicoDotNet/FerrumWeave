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
        "ferrumweave-r08-clean-{}-{nonce}",
        std::process::id(),
    ))
}

#[test]
fn dotnet_clean_removes_the_managed_build_output() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust");
    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated clean test directory");

    fs::copy(
        template.join("HelloFerrum.rsproj"),
        temp.join("HelloFerrum.rsproj"),
    )
    .expect("copy canonical R08 project fixture");
    fs::copy(template.join("src/main.rs"), source_dir.join("main.rs"))
        .expect("copy canonical R08 Rust source fixture");

    let build = Command::new("dotnet")
        .args(["build", "HelloFerrum.rsproj"])
        .current_dir(&temp)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute before clean");

    assert!(
        build.status.success(),
        "precondition dotnet build must succeed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );

    let assembly = temp.join("bin/Debug/net10.0/HelloFerrum.dll");
    assert!(
        assembly.is_file(),
        "precondition build must materialize the managed target assembly",
    );

    let clean = Command::new("dotnet")
        .args(["clean", "HelloFerrum.rsproj"])
        .current_dir(&temp)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet clean must execute");

    assert!(
        clean.status.success(),
        "FerrumWeave.Sdk must own a successful dotnet clean:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&clean.stdout),
        String::from_utf8_lossy(&clean.stderr),
    );
    assert!(
        !assembly.exists(),
        "dotnet clean must remove the managed assembly produced by the supported build lifecycle",
    );

    let _ = fs::remove_dir_all(temp);
}
