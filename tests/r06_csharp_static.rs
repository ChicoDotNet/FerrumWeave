use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave_cil::{PROBE_ASSEMBLY_FILE, write_probe_artifacts};

const CONSUMER_ASSEMBLY_NAME: &str = "FerrumWeave.R06.CSharpConsumer";
const RUST_NAMESPACE: &str = "FerrumWeave";
const RUST_TYPE: &str = "RustApi";
const RUST_METHOD: &str = "Answer";
const EXPECTED_ANSWER: &str = "42";

#[test]
fn csharp_calls_rust_defined_public_static_behavior() {
    let root = unique_temp_dir();
    let produced = root.join("rust-produced");
    let consumer = root.join("csharp-consumer");
    fs::create_dir_all(&consumer).expect("create R06 C# consumer directory");

    let artifacts = write_probe_artifacts(&produced).expect("emit Rust-produced managed assembly");
    fs::copy(&artifacts.assembly, consumer.join(PROBE_ASSEMBLY_FILE))
        .expect("place Rust-produced managed assembly beside C# consumer project");

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
    .expect("write C# consumer project");
    fs::write(
        consumer.join("Program.cs"),
        format!(
            "using System;\n\nConsole.WriteLine({RUST_NAMESPACE}.{RUST_TYPE}.{RUST_METHOD}());\n"
        ),
    )
    .expect("write C# consumer source");

    let build = Command::new("dotnet")
        .args(["build", "--configuration", "Release", "--nologo"])
        .current_dir(&consumer)
        .output()
        .expect("build C# consumer against Rust-produced managed assembly");
    assert!(
        build.status.success(),
        "C# must compile against the public Rust-defined managed API:\n{}\n{}",
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
        .expect("execute C# consumer on CoreCLR");
    assert!(
        run.status.success(),
        "C# consumer must execute the Rust-defined behavior:\n{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout).trim(), EXPECTED_ANSWER);

    let _ = fs::remove_dir_all(root);
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r06-csharp-static-{}-{nonce}",
        std::process::id()
    ))
}
