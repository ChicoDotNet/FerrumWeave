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
        "ferrumweave-r08-run-{}-{nonce}",
        std::process::id(),
    ))
}

#[test]
fn dotnet_run_executes_the_managed_ferrumweave_project() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust");
    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated run test directory");

    fs::copy(
        template.join("HelloFerrum.rsproj"),
        temp.join("HelloFerrum.rsproj"),
    )
    .expect("copy canonical R08 project fixture");
    fs::copy(template.join("src/main.rs"), source_dir.join("main.rs"))
        .expect("copy canonical R08 Rust source fixture");

    let run = Command::new("dotnet")
        .args(["run", "--project", "HelloFerrum.rsproj"])
        .current_dir(&temp)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet run must execute");

    assert!(
        run.status.success(),
        "FerrumWeave.Sdk must make the managed project directly runnable through dotnet run:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr),
    );

    let stdout = String::from_utf8_lossy(&run.stdout);
    assert_eq!(
        stdout.trim(),
        "Hello from FerrumWeave!",
        "dotnet run must execute the Rust main observable rather than merely launching a placeholder CLR artifact",
    );

    let assembly = temp.join("bin/Debug/net10.0/HelloFerrum.dll");
    assert!(
        assembly.is_file(),
        "dotnet run must execute the same managed project artifact produced by the SDK build lifecycle",
    );

    let _ = fs::remove_dir_all(temp);
}
