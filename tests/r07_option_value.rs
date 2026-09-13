use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const CONSUMER_ASSEMBLY_NAME: &str = "FerrumWeave.R07.OptionValueConsumer";

#[test]
fn option_value_type_projects_to_nullable_from_rust_source() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let root = unique_temp_dir();
    let source_dir = root.join("rust-source/src");
    let rust_project = root.join("rust-source");
    let consumer = root.join("csharp-consumer");
    fs::create_dir_all(&source_dir).expect("create R07 Rust source directory");
    fs::create_dir_all(&consumer).expect("create R07 nullable C# consumer directory");
    fs::copy(template, rust_project.join("RustLibrary.rsproj")).expect("copy canonical rsproj");

    let mut previous_artifact: Option<Vec<u8>> = None;
    for value in [42, 73] {
        fs::write(
            source_dir.join("main.rs"),
            format!(
                "#[no_mangle]\npub extern \"C\" fn option_some_i32() -> Option<i32> {{ Some({value}) }}\n\n#[no_mangle]\npub extern \"C\" fn option_none_i32() -> Option<i32> {{ None }}\n"
            ),
        )
        .expect("write R07 Rust Option value source");

        let build = dotnet_build(&repo, &rust_project);
        assert!(
            build.status.success(),
            "R07 Option value source must build through .rsproj -> rustc -> FerrumWeave; a RED here must come from unsupported FerrumWeave lowering, not a legacy emitter:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr),
        );

        let assembly = rust_project.join("bin/Debug/net10.0/RustLibrary.dll");
        assert!(
            assembly.is_file(),
            "R07 Rust source did not produce managed DLL"
        );
        let bytes = fs::read(&assembly).expect("read R07 managed artifact");
        if let Some(previous) = &previous_artifact {
            assert_ne!(
                previous, &bytes,
                "mutating only the Rust Some payload must mutate the managed artifact",
            );
        }
        previous_artifact = Some(bytes);

        fs::copy(&assembly, consumer.join("RustLibrary.dll"))
            .expect("place Rust-produced assembly beside R07 nullable consumer project");
        let run = build_and_run_consumer(&consumer, "RustLibrary.dll");
        assert!(
            run.status.success(),
            "C# consumer must execute Rust-source causal Nullable<int> Option behavior:\n{}\n{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr),
        );

        let stdout = String::from_utf8_lossy(&run.stdout);
        let lines: Vec<_> = stdout.lines().collect();
        assert_eq!(lines, [value.to_string(), "-1".to_string()]);
    }

    let _ = fs::remove_dir_all(root);
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
      <HintPath>{assembly_file}</HintPath>
      <Private>true</Private>
    </Reference>
  </ItemGroup>
</Project>
"#
        ),
    )
    .expect("write R07 nullable C# consumer project");
    fs::write(
        consumer.join("Program.cs"),
        "using System;\n\nint? some = FerrumWeave.RustApi.OptionSomeI32();\nint? none = FerrumWeave.RustApi.OptionNoneI32();\nConsole.WriteLine(some.HasValue ? some.Value : -1);\nConsole.WriteLine(none.HasValue ? none.Value : -1);\n",
    )
    .expect("write R07 nullable C# consumer source");

    let build = Command::new("dotnet")
        .args(["build", "--configuration", "Release", "--nologo"])
        .current_dir(consumer)
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
    Command::new("dotnet")
        .arg(&consumer_dll)
        .current_dir(consumer)
        .output()
        .expect("execute R07 nullable C# consumer on CoreCLR")
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r07-option-value-{}-{nonce}",
        std::process::id(),
    ))
}
