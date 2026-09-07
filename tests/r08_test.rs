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
        "ferrumweave-r08-test-{}-{nonce}",
        std::process::id(),
    ))
}

#[test]
fn dotnet_test_reports_the_r08_testing_limitation_explicitly() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust");
    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated test directory");

    fs::copy(
        template.join("HelloFerrum.rsproj"),
        temp.join("HelloFerrum.rsproj"),
    )
    .expect("copy canonical R08 project fixture");
    fs::copy(template.join("src/main.rs"), source_dir.join("main.rs"))
        .expect("copy canonical R08 Rust source fixture");

    let test = Command::new("dotnet")
        .args(["test", "HelloFerrum.rsproj", "--no-restore"])
        .current_dir(&temp)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet test must execute");

    let stdout = String::from_utf8_lossy(&test.stdout);
    let stderr = String::from_utf8_lossy(&test.stderr);
    let output = format!("{stdout}\n{stderr}");

    assert!(
        !test.status.success(),
        "R08 must not silently report test success before a Rust test adapter exists:\n{output}",
    );
    assert!(
        output.contains("FerrumWeave.Sdk does not yet provide a Rust test adapter"),
        "dotnet test must expose the documented R08 limitation explicitly:\n{output}",
    );

    let _ = fs::remove_dir_all(temp);
}
