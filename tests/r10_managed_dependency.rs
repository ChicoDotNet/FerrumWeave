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
        "ferrumweave-r10-managed-dependency-{}-{nonce}",
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
fn created_project_restores_a_real_nuget_dependency_and_uses_a_supported_dotnet_api() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sdk_project = repo.join("sdk/FerrumWeave.Sdk/FerrumWeave.Sdk.csproj");
    let temp = unique_temp_dir();
    let feed = temp.join("feed");
    let project = temp.join("ManagedConsumer");
    fs::create_dir_all(&feed).expect("create isolated alpha package feed");
    fs::create_dir_all(project.join("src")).expect("create external FerrumWeave project");

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
            "<configuration><packageSources><clear/><add key=\"ferrumweave-alpha\" value=\"{}\"/><add key=\"nuget.org\" value=\"https://api.nuget.org/v3/index.json\" protocolVersion=\"3\"/></packageSources></configuration>",
            xml_path(&feed)
        ),
    )
    .expect("write project-scoped package sources");
    fs::write(
        project.join("ManagedConsumer.rsproj"),
        format!(
            "<Project Sdk=\"FerrumWeave.Sdk/{SDK_VERSION}\"><PropertyGroup><TargetFramework>net10.0</TargetFramework><OutputType>Exe</OutputType></PropertyGroup><ItemGroup><PackageReference Include=\"Newtonsoft.Json\" Version=\"13.0.3\" /></ItemGroup></Project>"
        ),
    )
    .expect("write external FerrumWeave project with a real NuGet dependency");
    fs::write(
        project.join("src/main.rs"),
        "fn main() {\n    println!(\"managed-api-ok\");\n}\n",
    )
    .expect("write supported Rust source using the managed console path");

    let restore = Command::new("dotnet")
        .args(["restore", "ManagedConsumer.rsproj", "--nologo"])
        .current_dir(&project)
        .output()
        .expect("normal dotnet restore must execute for the created project");
    assert!(
        restore.status.success(),
        "created FerrumWeave project must restore through normal dotnet tooling:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&restore.stdout),
        String::from_utf8_lossy(&restore.stderr),
    );

    let assets = fs::read_to_string(project.join("obj/project.assets.json"))
        .expect("normal restore must create project.assets.json");
    assert!(
        assets.contains("Newtonsoft.Json/13.0.3"),
        "created FerrumWeave project must participate in the real NuGet dependency graph"
    );

    let run = Command::new("dotnet")
        .args([
            "run",
            "--project",
            "ManagedConsumer.rsproj",
            "--configuration",
            "Debug",
            "--no-restore",
            "--nologo",
        ])
        .current_dir(&project)
        .output()
        .expect("normal dotnet run must execute the created FerrumWeave project");
    assert!(
        run.status.success(),
        "created FerrumWeave project must build and execute after NuGet restore:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr),
    );
    assert!(
        String::from_utf8_lossy(&run.stdout)
            .lines()
            .any(|line| line.trim() == "managed-api-ok"),
        "supported Rust must reach the managed System.Console output path"
    );

    let _ = fs::remove_dir_all(temp);
}
