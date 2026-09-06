use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave_cil::{PROBE_ASSEMBLY_FILE, write_r07_disposable_artifact};

const CONSUMER_ASSEMBLY_NAME: &str = "FerrumWeave.R07.DisposableConsumer";
const EXPECTED_VALUES: [&str; 3] = ["0", "1", "1"];

#[test]
fn rust_drop_resource_projects_to_idisposable_with_exactly_once_release() {
    let root = unique_temp_dir();
    let produced = root.join("rust-produced");
    let consumer = root.join("csharp-consumer");
    fs::create_dir_all(&consumer).expect("create R07 IDisposable C# consumer directory");

    let assembly =
        write_r07_disposable_artifact(&produced).expect("emit Rust-produced managed assembly");
    fs::copy(&assembly, consumer.join(PROBE_ASSEMBLY_FILE))
        .expect("place Rust-produced managed assembly beside R07 IDisposable consumer project");

    write_project(&consumer);
    fs::write(
        consumer.join("Program.cs"),
        r#"using System;

var resource = new FerrumWeave.RustResource();
Console.WriteLine(resource.ReleaseCount());

((IDisposable)resource).Dispose();
Console.WriteLine(resource.ReleaseCount());

resource.Dispose();
Console.WriteLine(resource.ReleaseCount());
"#,
    )
    .expect("write R07 IDisposable C# consumer source");

    let build = Command::new("dotnet")
        .args(["build", "--configuration", "Release", "--nologo"])
        .current_dir(&consumer)
        .output()
        .expect("build R07 IDisposable C# consumer against Rust-produced managed assembly");
    assert!(
        build.status.success(),
        "C# must compile against the Rust-defined IDisposable resource projection:\n{}\n{}",
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
        .expect("execute R07 IDisposable C# consumer on CoreCLR");
    assert!(
        run.status.success(),
        "C# consumer must execute the Rust-defined IDisposable resource projection:\n{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    let stdout = String::from_utf8_lossy(&run.stdout);
    let observed: Vec<_> = stdout.lines().collect();
    assert_eq!(observed, EXPECTED_VALUES);

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
    .expect("write R07 IDisposable C# consumer project");
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r07-disposable-{}-{nonce}",
        std::process::id()
    ))
}
