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
        "ferrumweave-r08-{}-{nonce}",
        std::process::id()
    ))
}

#[test]
fn dotnet_new_rust_creates_rsproj_and_rust_source() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template_dir = repo.join("sdk/templates/rust");
    let temp = unique_temp_dir();
    let output = temp.join("HelloFerrum");
    fs::create_dir_all(&temp).expect("create isolated template test directory");

    let install = Command::new("dotnet")
        .args(["new", "install"])
        .arg(&template_dir)
        .output()
        .expect("dotnet CLI must be available for R08 contract tests");
    assert!(
        install.status.success(),
        "dotnet new install failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&install.stdout),
        String::from_utf8_lossy(&install.stderr)
    );

    let create = Command::new("dotnet")
        .args(["new", "rust", "-n", "HelloFerrum", "-o"])
        .arg(&output)
        .output()
        .expect("dotnet new rust must execute");

    let _ = Command::new("dotnet")
        .args(["new", "uninstall"])
        .arg(&template_dir)
        .output();

    assert!(
        create.status.success(),
        "dotnet new rust failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&create.stdout),
        String::from_utf8_lossy(&create.stderr)
    );

    let project = fs::read_to_string(output.join("HelloFerrum.rsproj"))
        .expect("template must generate HelloFerrum.rsproj");
    assert!(project.contains("<Project Sdk=\"FerrumWeave.Sdk\">"));
    assert!(project.contains("<TargetFramework>net10.0</TargetFramework>"));

    let source = fs::read_to_string(output.join("src/main.rs"))
        .expect("template must generate Rust source");
    assert!(source.contains("fn main()"));

    let _ = fs::remove_dir_all(temp);
}
