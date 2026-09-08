use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const SDK_PACKAGE: &str = "FerrumWeave.Sdk::0.1.0-alpha.1";

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r10-create-{}-{nonce}",
        std::process::id()
    ))
}

#[test]
fn installed_alpha_toolchain_creates_a_ferrumweave_project() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let project = repo.join("sdk/FerrumWeave.Sdk/FerrumWeave.Sdk.csproj");
    let temp = unique_temp_dir();
    let feed = temp.join("feed");
    let output = temp.join("HelloFerrum");
    fs::create_dir_all(&feed).expect("create isolated alpha package feed");

    let pack = Command::new("dotnet")
        .arg("pack")
        .arg(&project)
        .args(["--configuration", "Release", "--output"])
        .arg(&feed)
        .output()
        .expect("dotnet pack must be available for R10 contract tests");
    assert!(
        pack.status.success(),
        "dotnet pack failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&pack.stdout),
        String::from_utf8_lossy(&pack.stderr),
    );

    let install = Command::new("dotnet")
        .args(["new", "install", SDK_PACKAGE, "--nuget-source"])
        .arg(&feed)
        .output()
        .expect("dotnet new install must execute");
    assert!(
        install.status.success(),
        "installing the released alpha toolchain failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&install.stdout),
        String::from_utf8_lossy(&install.stderr),
    );

    let create = Command::new("dotnet")
        .args(["new", "rust", "-n", "HelloFerrum", "-o"])
        .arg(&output)
        .output()
        .expect("dotnet new rust must execute from the installed alpha toolchain");

    let _ = Command::new("dotnet")
        .args(["new", "uninstall", "FerrumWeave.Sdk"])
        .output();

    assert!(
        create.status.success(),
        "installed alpha toolchain could not create a Rust project:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&create.stdout),
        String::from_utf8_lossy(&create.stderr),
    );

    let rsproj = fs::read_to_string(output.join("HelloFerrum.rsproj"))
        .expect("installed template must generate HelloFerrum.rsproj");
    assert!(rsproj.contains("<Project Sdk=\"FerrumWeave.Sdk\">"));
    assert!(output.join("src/main.rs").is_file());

    let _ = fs::remove_dir_all(temp);
}
