use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r08-e2e-{}-{nonce}",
        std::process::id(),
    ))
}

fn run(command: &str, args: &[&str], current_dir: &Path, repo: &Path) -> Output {
    Command::new(command)
        .args(args)
        .current_dir(current_dir)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .unwrap_or_else(|error| panic!("{command} must execute: {error}"))
}

fn assert_success(label: &str, output: &Output) {
    assert!(
        output.status.success(),
        "{label} must succeed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

fn consume_managed_answer(assembly: &Path, temp: &Path) -> Output {
    let consumer = temp.join("consumer");
    fs::create_dir_all(&consumer).expect("create managed consumer directory");
    fs::write(
        consumer.join("Consumer.csproj"),
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
  </PropertyGroup>
</Project>
"#,
    )
    .expect("write managed consumer project");
    fs::write(
        consumer.join("Program.cs"),
        r#"using System.Reflection;
var assembly = Assembly.LoadFrom(args[0]);
var type = assembly.GetType("FerrumWeave.RustApi", throwOnError: true)!;
var answer = type.GetMethod("Answer", BindingFlags.Public | BindingFlags.Static)!;
System.Console.Write(answer.Invoke(null, null));
"#,
    )
    .expect("write managed consumer program");

    Command::new("dotnet")
        .args([
            "run",
            "--project",
            "Consumer.csproj",
            "--",
            assembly.to_str().expect("assembly path must be UTF-8"),
        ])
        .current_dir(&consumer)
        .output()
        .expect("managed consumer must execute")
}

#[test]
fn documented_prerequisites_drive_the_complete_supported_sdk_lifecycle() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let prerequisites = fs::read_to_string(repo.join("docs/r08-sdk-prerequisites.md"))
        .expect("R08 SDK prerequisites must be readable");

    assert!(
        prerequisites.contains("# R08 SDK prerequisites"),
        "R08 certification prerequisites must be documented before the lifecycle is claimed complete",
    );
    for prerequisite in ["Rust 1.98", ".NET 10 SDK"] {
        assert!(
            prerequisites.contains(prerequisite),
            "R08 prerequisites must name {prerequisite}",
        );
    }

    assert_success(
        "rustc prerequisite",
        &run("rustc", &["--version"], &repo, &repo),
    );
    assert_success(
        "cargo prerequisite",
        &run("cargo", &["--version"], &repo, &repo),
    );
    let dotnet_sdks = run("dotnet", &["--list-sdks"], &repo, &repo);
    assert_success("dotnet prerequisite", &dotnet_sdks);
    assert!(
        String::from_utf8_lossy(&dotnet_sdks.stdout)
            .lines()
            .any(|line| line.starts_with("10.")),
        "R08 certification requires an installed .NET 10 SDK",
    );

    let template = repo.join("sdk/templates/rust");
    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated R08 lifecycle directory");
    fs::copy(
        template.join("HelloFerrum.rsproj"),
        temp.join("HelloFerrum.rsproj"),
    )
    .expect("copy canonical R08 project fixture");
    fs::copy(template.join("src/main.rs"), source_dir.join("main.rs"))
        .expect("copy canonical R08 Rust source fixture");

    assert_success(
        "dotnet restore",
        &run("dotnet", &["restore", "HelloFerrum.rsproj"], &temp, &repo),
    );
    assert_success(
        "dotnet build",
        &run(
            "dotnet",
            &["build", "HelloFerrum.rsproj", "--no-restore"],
            &temp,
            &repo,
        ),
    );

    let assembly = temp.join("bin/Debug/net10.0/HelloFerrum.dll");
    assert!(
        assembly.is_file(),
        "dotnet build must materialize HelloFerrum.dll"
    );

    let consume = consume_managed_answer(&assembly, &temp);
    assert_success("managed consumer", &consume);
    assert_eq!(
        String::from_utf8_lossy(&consume.stdout).trim(),
        "42",
        "the SDK lifecycle must expose behavior originating in the canonical Rust source through the managed artifact",
    );

    let test_output = run(
        "dotnet",
        &["test", "HelloFerrum.rsproj", "--no-restore"],
        &temp,
        &repo,
    );
    assert!(
        !test_output.status.success(),
        "dotnet test must not report false success while the Rust test adapter is unavailable",
    );
    let test_diagnostics = format!(
        "{}\n{}",
        String::from_utf8_lossy(&test_output.stdout),
        String::from_utf8_lossy(&test_output.stderr),
    );
    assert!(
        test_diagnostics.contains("FerrumWeave.Sdk does not yet provide a Rust test adapter"),
        "dotnet test must expose the documented SDK limitation",
    );

    assert_success(
        "dotnet clean",
        &run("dotnet", &["clean", "HelloFerrum.rsproj"], &temp, &repo),
    );
    assert!(
        !assembly.exists(),
        "dotnet clean must remove the supported managed build artifact",
    );

    let _ = fs::remove_dir_all(temp);
}
