use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const CONSUMER_ASSEMBLY_NAME: &str = "FerrumWeave.R06.CSharpInstanceConsumer";

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r06-csharp-instance-{}-{nonce}",
        std::process::id(),
    ))
}

fn dotnet_build(repo: &Path, project: &Path) -> Output {
    Command::new("dotnet")
        .args(["build", "RustLibrary.rsproj"])
        .current_dir(project)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute")
}

fn build_and_run_consumer(consumer: &Path) -> Output {
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
    <Reference Include="RustLibrary">
      <HintPath>RustLibrary.dll</HintPath>
      <Private>true</Private>
    </Reference>
  </ItemGroup>
</Project>
"#
        ),
    )
    .expect("write C# instance consumer project");
    fs::write(
        consumer.join("Program.cs"),
        "using System;\n\nvar value = new FerrumWeave.RustValue();\nConsole.WriteLine(value.Answer());\n",
    )
    .expect("write C# instance consumer source");

    let build = Command::new("dotnet")
        .args(["build", "--configuration", "Release", "--nologo"])
        .current_dir(consumer)
        .output()
        .expect("build C# instance consumer against Rust-produced managed assembly");
    assert!(
        build.status.success(),
        "C# must compile against the Rust-source causal constructible managed type:\n{}\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let consumer_dll = consumer
        .join("bin")
        .join("Release")
        .join("net10.0")
        .join(format!("{CONSUMER_ASSEMBLY_NAME}.dll"));
    Command::new("dotnet")
        .arg(&consumer_dll)
        .current_dir(consumer)
        .output()
        .expect("execute C# instance consumer on CoreCLR")
}

#[test]
fn csharp_constructs_rust_source_causal_type_and_calls_instance_behavior() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let root = unique_temp_dir();
    let source_dir = root.join("rust-source/src");
    let rust_project = root.join("rust-source");
    let consumer = root.join("csharp-instance-consumer");
    fs::create_dir_all(&source_dir).expect("create R06 Rust source directory");
    fs::create_dir_all(&consumer).expect("create R06 C# instance consumer directory");
    fs::copy(template, rust_project.join("RustLibrary.rsproj")).expect("copy canonical rsproj");

    let mut previous_artifact: Option<Vec<u8>> = None;
    for value in [137, 211] {
        fs::write(
            source_dir.join("main.rs"),
            format!(
                "pub struct RustValue;\n\nimpl RustValue {{\n    pub fn answer(&self) -> i32 {{ {value} }}\n}}\n\n#[no_mangle]\npub extern \"C\" fn answer(value: &RustValue) -> i32 {{ value.answer() }}\n"
            ),
        )
        .expect("write R06 Rust instance source");

        let build = dotnet_build(&repo, &rust_project);
        assert!(
            build.status.success(),
            "R06 Rust instance source must project through .rsproj -> rustc -> FerrumWeave:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr),
        );
        let assembly = rust_project.join("bin/Debug/net10.0/RustLibrary.dll");
        assert!(assembly.is_file(), "R06 Rust instance source did not produce managed DLL");

        let bytes = fs::read(&assembly).expect("read R06 instance managed artifact");
        if let Some(previous) = &previous_artifact {
            assert_ne!(
                previous, &bytes,
                "mutating only R06 Rust instance source 137 -> 211 must mutate the managed artifact",
            );
        }
        previous_artifact = Some(bytes);

        fs::copy(&assembly, consumer.join("RustLibrary.dll"))
            .expect("place Rust-produced assembly beside C# instance consumer");
        let run = build_and_run_consumer(&consumer);
        assert!(
            run.status.success(),
            "C# instance consumer must execute Rust-source causal behavior:\n{}\n{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&run.stdout).trim(),
            value.to_string(),
            "C# instance observable must follow the Rust-only source mutation",
        );
    }

    let _ = fs::remove_dir_all(root);
}
