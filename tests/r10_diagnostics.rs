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
        "ferrumweave-r10-diagnostics-{}-{nonce}",
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

#[test]
fn invalid_rust_reports_a_useful_source_location_through_dotnet_build() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sdk_project = repo.join("sdk/FerrumWeave.Sdk/FerrumWeave.Sdk.csproj");
    let temp = unique_temp_dir();
    let feed = temp.join("feed");
    let project = temp.join("BrokenFerrum");
    fs::create_dir_all(project.join("src")).expect("create external diagnostic project");
    fs::create_dir_all(&feed).expect("create isolated alpha package feed");

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
        project.join("NuGet.Config"),
        format!(
            "<configuration><packageSources><clear/><add key=\"ferrumweave-alpha\" value=\"{}\"/></packageSources></configuration>",
            xml_path(&feed)
        ),
    )
    .expect("write project-scoped alpha feed configuration");
    fs::write(
        project.join("BrokenFerrum.rsproj"),
        format!(
            "<Project Sdk=\"FerrumWeave.Sdk/{SDK_VERSION}\"><PropertyGroup><TargetFramework>net10.0</TargetFramework><OutputType>Exe</OutputType></PropertyGroup></Project>"
        ),
    )
    .expect("write FerrumWeave project");
    fs::write(
        project.join("src/main.rs"),
        "fn main() {\n    let answer = ;\n}\n",
    )
    .expect("write intentionally invalid Rust source");

    let build = Command::new("dotnet")
        .arg("build")
        .arg("BrokenFerrum.rsproj")
        .current_dir(&project)
        .output()
        .expect("dotnet build must execute through the installed FerrumWeave SDK");

    assert!(
        !build.status.success(),
        "invalid Rust must fail the normal dotnet build path"
    );

    let diagnostic = format!(
        "{}\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    )
    .replace('\\', "/");

    assert!(
        diagnostic.contains("main.rs"),
        "diagnostic must identify the edited Rust source file:\n{diagnostic}"
    );
    assert!(
        diagnostic.contains("main.rs:2") || diagnostic.contains("main.rs(2"),
        "diagnostic must preserve the Rust source line for the invalid edit:\n{diagnostic}"
    );

    let _ = fs::remove_dir_all(temp);
}
