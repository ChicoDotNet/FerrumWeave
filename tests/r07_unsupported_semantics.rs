use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const DIAGNOSTIC: &str = "FERRUMWEAVE_SEMANTIC_BOUNDARY_REJECTED";

#[test]
fn unsupported_cross_runtime_semantics_fail_inside_the_ferrumweave_backend() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let root = unique_temp_dir();
    let source_dir = root.join("rust-source/src");
    let rust_project = root.join("rust-source");
    fs::create_dir_all(&source_dir).expect("create R07 semantic-boundary Rust source directory");
    fs::copy(template, rust_project.join("RustLibrary.rsproj")).expect("copy canonical rsproj");

    for source in [
        "#[no_mangle]\npub extern \"C\" fn answer() -> char { '\u{1F600}' }\n",
        "#[no_mangle]\npub extern \"C\" fn answer(value: String) -> i32 { value.len() as i32 }\n",
    ] {
        clean_outputs(&rust_project);
        fs::write(source_dir.join("main.rs"), source)
            .expect("write rejected Rust semantic-boundary source");

        let build = dotnet_build(&repo, &rust_project);
        assert!(
            !build.status.success(),
            "unsupported Rust semantics at a managed export must be rejected by FerrumWeave"
        );
        let diagnostic = format!(
            "{}\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr)
        );
        assert!(
            diagnostic.contains(DIAGNOSTIC),
            "rejection must come from the FerrumWeave semantic boundary policy, not loader/configuration or generic lowering:\n{diagnostic}"
        );
        assert!(
            !managed_artifact(&rust_project).is_file(),
            "rejected semantic boundary must not leave a valid managed artifact"
        );
    }

    clean_outputs(&rust_project);
    fs::write(
        source_dir.join("main.rs"),
        "#[no_mangle]\npub extern \"C\" fn answer() -> i32 { 42 }\n",
    )
    .expect("write supported owned Rust source");

    let supported = dotnet_build(&repo, &rust_project);
    assert!(
        supported.status.success(),
        "mutating the boundary to a supported owned i32 must compile, falsifying an always-reject implementation:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&supported.stdout),
        String::from_utf8_lossy(&supported.stderr),
    );
    assert!(
        managed_artifact(&rust_project).is_file(),
        "supported owned Rust source must produce the managed artifact"
    );

    let _ = fs::remove_dir_all(root);
}

fn dotnet_build(repo: &Path, project: &Path) -> Output {
    Command::new("dotnet")
        .args(["build", "RustLibrary.rsproj"])
        .current_dir(project)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute")
}

fn clean_outputs(project: &Path) {
    let _ = fs::remove_dir_all(project.join("bin"));
    let _ = fs::remove_dir_all(project.join("obj"));
}

fn managed_artifact(project: &Path) -> PathBuf {
    project.join("bin/Debug/net10.0/RustLibrary.dll")
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r07-semantic-boundary-{}-{nonce}",
        std::process::id()
    ))
}
