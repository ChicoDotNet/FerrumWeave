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
        "ferrumweave-sdk-backend-path-{}-{nonce}",
        std::process::id(),
    ))
}

#[test]
fn rsproj_build_rejects_invalid_rust_in_rustc_before_managed_artifact_emission() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated sdk backend-path test directory");
    fs::copy(template, temp.join("HelloFerrum.rsproj")).expect("copy canonical rsproj");

    // Deliberately malformed Rust that the legacy source parser accepted. A real
    // .rsproj -> rustc -> FerrumWeave CodegenBackend path must reject it in the
    // Rust frontend before any managed artifact can be published.
    fs::write(
        source_dir.join("main.rs"),
        "fn main() { println!(\"legacy emitter must not accept this\"); let broken = ; }\n",
    )
    .expect("write invalid Rust source");

    let build = Command::new("dotnet")
        .args(["build", "HelloFerrum.rsproj"])
        .current_dir(&temp)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute");

    let combined = format!(
        "{}\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );

    assert!(
        !build.status.success(),
        ".rsproj must be gated by rustc; invalid Rust unexpectedly produced a successful build.\n{combined}",
    );
    assert!(
        combined.contains("expected expression") && combined.contains("src/main.rs"),
        ".rsproj failed, but not because rustc rejected the malformed Rust source; this must not become a false GREEN.\n{combined}",
    );
    assert!(
        !combined.contains("ferrumweave_emit"),
        "legacy ferrumweave_emit must not remain in the product compilation path.\n{combined}",
    );
    assert!(
        !temp.join("bin/Debug/net10.0/HelloFerrum.dll").exists(),
        "invalid Rust must not leave a valid managed product artifact",
    );

    let _ = fs::remove_dir_all(temp);
}
