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
        "ferrumweave-r08-build-{}-{nonce}",
        std::process::id()
    ))
}

#[test]
fn dotnet_build_produces_a_managed_ferrumweave_assembly() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust");
    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated build test directory");

    fs::copy(
        template.join("HelloFerrum.rsproj"),
        temp.join("HelloFerrum.rsproj"),
    )
    .expect("copy canonical R08 project fixture");
    fs::copy(template.join("src/main.rs"), source_dir.join("main.rs"))
        .expect("copy canonical R08 Rust source fixture");

    let build = Command::new("dotnet")
        .args(["build", "HelloFerrum.rsproj", "--no-restore"])
        .current_dir(&temp)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute");

    assert!(
        build.status.success(),
        "FerrumWeave.Sdk must own a successful dotnet build:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let assembly = temp.join("bin/Debug/net10.0/HelloFerrum.dll");
    assert!(
        assembly.is_file(),
        "dotnet build must produce the managed FerrumWeave assembly at the standard TargetPath"
    );

    let probe = temp.join("probe");
    fs::create_dir_all(&probe).expect("create CLR inspection probe");
    fs::write(
        probe.join("Probe.csproj"),
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
  </PropertyGroup>
</Project>
"#,
    )
    .expect("write CLR inspection project");
    fs::write(
        probe.join("Program.cs"),
        r#"using System.Reflection;
var assembly = Assembly.LoadFrom(args[0]);
var type = assembly.GetType("FerrumWeave.RustApi", throwOnError: true)!;
var answer = type.GetMethod("Answer", BindingFlags.Public | BindingFlags.Static)!;
Console.Write(answer.Invoke(null, null));
"#,
    )
    .expect("write CLR inspection program");

    let inspect = Command::new("dotnet")
        .args([
            "run",
            "--project",
            "Probe.csproj",
            "--",
            assembly.to_str().unwrap(),
        ])
        .current_dir(&probe)
        .output()
        .expect("CLR inspection probe must execute");

    assert!(
        inspect.status.success(),
        "build output must load as a managed FerrumWeave assembly:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&inspect.stdout),
        String::from_utf8_lossy(&inspect.stderr)
    );
    assert_eq!(
        String::from_utf8_lossy(&inspect.stdout).trim(),
        "42",
        "dotnet build must expose FerrumWeave-emitted Rust API behavior, not an empty SDK placeholder assembly"
    );

    let _ = fs::remove_dir_all(temp);
}
