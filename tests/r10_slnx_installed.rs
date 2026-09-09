use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const SDK_VERSION: &str = "0.1.0-alpha.1";

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r10-installed-slnx-{}-{nonce}",
        std::process::id()
    ))
}

fn xml_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create copied canonical solution directory");
    for entry in fs::read_dir(source).expect("read canonical solution directory") {
        let entry = entry.expect("read canonical solution entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if source_path.is_dir() {
            copy_tree(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).expect("copy canonical solution file");
        }
    }
}

#[test]
fn canonical_mixed_slnx_builds_using_only_the_packaged_alpha_sdk() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sdk_project = repo.join("sdk/FerrumWeave.Sdk/FerrumWeave.Sdk.csproj");
    let canonical = repo.join("tests/fixtures/r09");
    let temp = unique_temp_dir();
    let feed = temp.join("feed");
    let solution = temp.join("Enterprise");
    fs::create_dir_all(&feed).expect("create isolated alpha package feed");
    copy_tree(&canonical, &solution);

    let pack = Command::new("dotnet")
        .arg("pack")
        .arg(&sdk_project)
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

    fs::write(
        solution.join("NuGet.Config"),
        format!(
            "<configuration><packageSources><clear/><add key=\"ferrumweave-alpha\" value=\"{}\"/><add key=\"nuget.org\" value=\"https://api.nuget.org/v3/index.json\" protocolVersion=\"3\"/></packageSources></configuration>",
            xml_path(&feed)
        ),
    )
    .expect("write solution-scoped package sources");

    fs::write(
        solution.join("RiskEngine/RiskEngine.rsproj"),
        format!(
            "<Project Sdk=\"FerrumWeave.Sdk/{SDK_VERSION}\"><PropertyGroup><TargetFramework>net10.0</TargetFramework><OutputType>Exe</OutputType></PropertyGroup></Project>"
        ),
    )
    .expect("make the canonical Rust project consume the packaged alpha SDK");

    let restore = Command::new("dotnet")
        .args(["restore", "Enterprise.slnx", "--nologo"])
        .current_dir(&solution)
        .output()
        .expect("normal dotnet restore must execute for the mixed solution");
    assert!(
        restore.status.success(),
        "canonical mixed solution must restore with the packaged alpha SDK:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&restore.stdout),
        String::from_utf8_lossy(&restore.stderr),
    );

    let build = Command::new("dotnet")
        .args(["build", "Enterprise.slnx", "--no-restore", "--nologo"])
        .current_dir(&solution)
        .output()
        .expect("normal dotnet build must execute for the mixed solution");
    assert!(
        build.status.success(),
        "canonical mixed solution must build without checkout-relative FerrumWeave tooling:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );

    let _ = fs::remove_dir_all(temp);
}
