use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r08-sdk-lifecycle-{}-{nonce}",
        std::process::id(),
    ))
}

fn contains_manual_build_script(root: &Path) -> bool {
    fs::read_dir(root)
        .expect("template directory must be readable")
        .filter_map(Result::ok)
        .any(|entry| {
            let path = entry.path();
            if path.is_dir() {
                return contains_manual_build_script(&path);
            }
            matches!(
                path.extension().and_then(|value| value.to_str()),
                Some("sh" | "ps1" | "cmd" | "bat")
            ) || path.file_name().and_then(|value| value.to_str()) == Some("Makefile")
        })
}

#[test]
fn dotnet_sdk_owns_the_supported_project_lifecycle_without_manual_build_scripts() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust");

    assert!(
        !contains_manual_build_script(&template),
        "the canonical Rust template must not require a bespoke manual build script",
    );

    let project = fs::read_to_string(template.join("HelloFerrum.rsproj"))
        .expect("canonical R08 project must be readable");
    assert!(
        project.contains("<Project Sdk=\"FerrumWeave.Sdk\">"),
        "the generated project must delegate its lifecycle to FerrumWeave.Sdk",
    );

    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated SDK lifecycle test directory");
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
        .expect("dotnet build must execute through the SDK boundary");

    assert!(
        build.status.success(),
        "the supported lifecycle must work through dotnet/MSBuild without a manual build script:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );

    assert!(
        temp.join("bin/Debug/net10.0/HelloFerrum.dll").is_file(),
        "SDK-owned dotnet build must materialize the managed artifact",
    );

    let _ = fs::remove_dir_all(temp);
}
