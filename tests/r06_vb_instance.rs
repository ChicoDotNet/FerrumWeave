use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave_cil::{PROBE_ASSEMBLY_FILE, write_r06_static_api_artifact};

const CONSUMER_ASSEMBLY_NAME: &str = "FerrumWeave.R06.VbInstanceConsumer";
const EXPECTED_ANSWER: &str = "42";

#[test]
fn vbnet_constructs_rust_defined_type_and_calls_instance_behavior() {
    let root = unique_temp_dir();
    let produced = root.join("rust-produced");
    let consumer = root.join("vb-instance-consumer");
    fs::create_dir_all(&consumer).expect("create R06 VB.NET instance consumer directory");

    let assembly = write_r06_static_api_artifact(&produced)
        .expect("emit current Rust-produced managed assembly");
    fs::copy(&assembly, consumer.join(PROBE_ASSEMBLY_FILE))
        .expect("place Rust-produced managed assembly beside VB.NET consumer project");

    fs::write(
        consumer.join("Consumer.vbproj"),
        format!(
            r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
    <AssemblyName>{CONSUMER_ASSEMBLY_NAME}</AssemblyName>
    <RootNamespace></RootNamespace>
    <OptionStrict>On</OptionStrict>
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
    .expect("write VB.NET instance consumer project");
    fs::write(
        consumer.join("Program.vb"),
        r#"Imports System

Module Program
    Sub Main()
        Dim value As New FerrumWeave.RustValue()
        Console.WriteLine(value.Answer())
    End Sub
End Module
"#,
    )
    .expect("write VB.NET instance consumer source");

    let build = Command::new("dotnet")
        .args(["build", "--configuration", "Release", "--nologo"])
        .current_dir(&consumer)
        .output()
        .expect("build VB.NET instance consumer against Rust-produced managed assembly");
    assert!(
        build.status.success(),
        "VB.NET must compile against the Rust-defined constructible managed type:\n{}\n{}",
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
        .expect("execute VB.NET instance consumer on CoreCLR");
    assert!(
        run.status.success(),
        "VB.NET instance consumer must execute Rust-defined behavior:\n{}\n{}",
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
        "ferrumweave-r06-vb-instance-{}-{nonce}",
        std::process::id()
    ))
}
