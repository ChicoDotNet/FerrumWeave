use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave_cil::{PROBE_ASSEMBLY_FILE, write_r06_static_api_artifact};

const CONSUMER_ASSEMBLY_NAME: &str = "FerrumWeave.R07.OptionReferenceConsumer";

#[test]
fn option_some_managed_reference_preserves_value_at_clr_boundary() {
    let output = run_csharp_consumer(
        "string? value = FerrumWeave.RustApi.OptionSomeString();\nConsole.WriteLine(value ?? \"<null>\");",
    );
    assert_eq!(output, "FerrumWeave");
}

#[test]
fn option_none_managed_reference_projects_to_clr_null() {
    let output = run_csharp_consumer(
        "string? value = FerrumWeave.RustApi.OptionNoneString();\nConsole.WriteLine(value is null ? \"NULL\" : value);",
    );
    assert_eq!(output, "NULL");
}

fn run_csharp_consumer(program_body: &str) -> String {
    let root = unique_temp_dir();
    let produced = root.join("rust-produced");
    let consumer = root.join("csharp-consumer");
    fs::create_dir_all(&consumer).expect("create R07 C# consumer directory");

    let assembly =
        write_r06_static_api_artifact(&produced).expect("emit Rust-produced managed assembly");
    fs::copy(&assembly, consumer.join(PROBE_ASSEMBLY_FILE))
        .expect("place Rust-produced managed assembly beside R07 C# consumer project");

    write_project(&consumer);
    fs::write(
        consumer.join("Program.cs"),
        format!("using System;\n\n{program_body}\n"),
    )
    .expect("write R07 C# consumer source");

    let build = Command::new("dotnet")
        .args(["build", "--configuration", "Release", "--nologo"])
        .current_dir(&consumer)
        .output()
        .expect("build R07 C# consumer against Rust-produced managed assembly");
    assert!(
        build.status.success(),
        "C# must compile against the Rust-defined Option reference projection:\n{}\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let consumer_dll = consumer
        .join("bin")
        .join("Release")
        .join("net10.0")
        .join(format!("{CONSUMER_ASSEMBLY_NAME}.dll"));
    let run = Command::new("dotnet")
        .arg(&consumer_dll)
        .current_dir(&consumer)
        .output()
        .expect("execute R07 C# consumer on CoreCLR");
    assert!(
        run.status.success(),
        "C# consumer must execute the Rust-defined Option reference projection:\n{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let stdout = String::from_utf8_lossy(&run.stdout).trim().to_owned();
    let _ = fs::remove_dir_all(root);
    stdout
}

fn write_project(consumer: &Path) {
    fs::write(
        consumer.join("Consumer.csproj"),
        format!(
            r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
    <AssemblyName>{CONSUMER_ASSEMBLY_NAME}</AssemblyName>
    <ImplicitUsings>disable</ImplicitUsings>
    <Nullable>enable</Nullable>
  </PropertyGroup>
  <ItemGroup>
    <Reference Include="FerrumWeave.Probe">
      <HintPath>{PROBE_ASSEMBLY_FILE}</HintPath>
      <Private>true</Private>
    </Reference>
  </ItemGroup>
</Project>
"#
        ),
    )
    .expect("write R07 C# consumer project");
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r07-option-reference-{}-{nonce}",
        std::process::id()
    ))
}
