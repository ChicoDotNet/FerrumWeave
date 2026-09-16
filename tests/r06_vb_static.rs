use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const CONSUMER_ASSEMBLY_NAME: &str = "FerrumWeave.R06.VbConsumer";

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r06-vb-static-{}-{nonce}",
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

fn build_and_run_consumer(consumer: &Path, assembly_file: &str) -> Output {
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
    <Reference Include="RustLibrary">
      <HintPath>{assembly_file}</HintPath>
      <Private>true</Private>
    </Reference>
  </ItemGroup>
</Project>
"#
        ),
    )
    .expect("write VB.NET consumer project");
    fs::write(
        consumer.join("Program.vb"),
        r#"Imports System

Module Program
    Sub Main()
        Console.WriteLine(FerrumWeave.RustApi.Answer())
    End Sub
End Module
"#,
    )
    .expect("write VB.NET consumer source");

    let build = Command::new("dotnet")
        .args(["build", "--configuration", "Release", "--nologo"])
        .current_dir(consumer)
        .output()
        .expect("build VB.NET consumer against Rust-produced managed assembly");
    assert!(
        build.status.success(),
        "VB.NET must compile against the public Rust-defined managed API:\n{}\n{}",
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
        .expect("execute VB.NET consumer on CoreCLR")
}

#[test]
fn vbnet_calls_rust_source_causal_public_static_behavior() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let root = unique_temp_dir();
    let source_dir = root.join("rust-source/src");
    let rust_project = root.join("rust-source");
    let consumer = root.join("vb-consumer");
    fs::create_dir_all(&source_dir).expect("create R06 Rust source directory");
    fs::create_dir_all(&consumer).expect("create R06 VB.NET consumer directory");
    fs::copy(template, rust_project.join("RustLibrary.rsproj")).expect("copy canonical rsproj");

    let mut previous_artifact: Option<Vec<u8>> = None;
    for value in [137, 211] {
        fs::write(
            source_dir.join("main.rs"),
            format!("#[no_mangle]\npub extern \"C\" fn answer() -> i32 {{ {value} }}\n"),
        )
        .expect("write R06 Rust source");

        let build = dotnet_build(&repo, &rust_project);
        assert!(
            build.status.success(),
            "R06 Rust source must build through .rsproj -> rustc -> FerrumWeave:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr),
        );
        let assembly = rust_project.join("bin/Debug/net10.0/RustLibrary.dll");
        assert!(
            assembly.is_file(),
            "R06 Rust source did not produce managed DLL"
        );

        let bytes = fs::read(&assembly).expect("read R06 managed artifact");
        if let Some(previous) = &previous_artifact {
            assert_ne!(
                previous, &bytes,
                "mutating only R06 Rust source 137 -> 211 must mutate the managed artifact",
            );
        }
        previous_artifact = Some(bytes);

        let consumer_assembly = consumer.join("RustLibrary.dll");
        fs::copy(&assembly, &consumer_assembly)
            .expect("place Rust-produced assembly beside VB.NET consumer");
        let run = build_and_run_consumer(&consumer, "RustLibrary.dll");
        assert!(
            run.status.success(),
            "VB.NET consumer must execute Rust-source causal behavior:\n{}\n{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr)
        );
        assert_eq!(
            String::from_utf8_lossy(&run.stdout).trim(),
            value.to_string(),
            "VB.NET observable must follow the Rust-only source mutation",
        );
    }

    let _ = fs::remove_dir_all(root);
}
