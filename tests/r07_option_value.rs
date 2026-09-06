use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave_cil::{PROBE_ASSEMBLY_FILE, write_r06_static_api_artifact};

const CONSUMER_ASSEMBLY_NAME: &str = "FerrumWeave.R07.OptionValueConsumer";

#[test]
fn option_value_type_projects_to_nullable_for_some_and_none() {
    let root = unique_temp_dir();
    let produced = root.join("rust-produced");
    let consumer = root.join("csharp-consumer");
    fs::create_dir_all(&consumer).expect("create R07 nullable C# consumer directory");

    let assembly =
        write_r06_static_api_artifact(&produced).expect("emit Rust-produced managed assembly");
    fs::copy(&assembly, consumer.join(PROBE_ASSEMBLY_FILE))
        .expect("place Rust-produced managed assembly beside R07 nullable consumer project");

    write_project(&consumer);
    fs::write(
        consumer.join("Program.cs"),
        r#"using System;

int? some = FerrumWeave.RustApi.OptionSomeI32();
int? none = FerrumWeave.RustApi.OptionNoneI32();

Console.WriteLine(some.HasValue ? some.Value : -1);
Console.WriteLine(none.HasValue ? none.Value : -1);
"#,
    )
    .expect("write R07 nullable C# consumer source");

    let build = Command::new("dotnet")
        .args(["build", "--configuration", "Release", "--nologo"])
        .current_dir(&consumer)
        .output()
        .expect("build R07 nullable C# consumer against Rust-produced managed assembly");
    assert!(
        build.status.success(),
        "C# must compile against the Rust-defined Nullable<int> Option projection:\n{}\n{}",
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
        .expect("execute R07 nullable C# consumer on CoreCLR");
    assert!(
        run.status.success(),
        "C# consumer must execute the Rust-defined Nullable<int> Option projection:\n{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );

    let stdout = String::from_utf8_lossy(&run.stdout);
    let lines: Vec<_> = stdout.lines().collect();
    assert_eq!(lines, ["42", "-1"]);

    let _ = fs::remove_dir_all(root);
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
    .expect("write R07 nullable C# consumer project");
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r07-option-value-{}-{nonce}",
        std::process::id()
    ))
}
