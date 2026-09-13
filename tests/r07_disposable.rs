use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const CONSUMER_ASSEMBLY_NAME: &str = "FerrumWeave.R07.DisposableConsumer";

#[test]
fn rust_drop_resource_projects_to_idisposable_from_rust_source() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let root = unique_temp_dir();
    let source_dir = root.join("rust-source/src");
    let rust_project = root.join("rust-source");
    let consumer = root.join("csharp-consumer");
    fs::create_dir_all(&source_dir).expect("create R07 Drop/RAII Rust source directory");
    fs::create_dir_all(&consumer).expect("create R07 IDisposable C# consumer directory");
    fs::copy(template, rust_project.join("RustLibrary.rsproj")).expect("copy canonical rsproj");

    let mut previous_artifact: Option<Vec<u8>> = None;
    for (seed, release_increment) in [(0, 1), (5, 2)] {
        fs::write(
            source_dir.join("main.rs"),
            format!(
                r#"pub struct RustResource {{
    release_count: i32,
    released: bool,
}}

impl RustResource {{
    pub fn new() -> Self {{
        Self {{ release_count: {seed}, released: false }}
    }}

    pub fn release_count(&self) -> i32 {{
        self.release_count
    }}
}}

impl Drop for RustResource {{
    fn drop(&mut self) {{
        if !self.released {{
            self.release_count += {release_increment};
            self.released = true;
        }}
    }}
}}

#[no_mangle]
pub extern "C" fn answer() -> i32 {{
    let resource = RustResource::new();
    resource.release_count()
}}
"#
            ),
        )
        .expect("write R07 Rust Drop/RAII source");

        let build = dotnet_build(&repo, &rust_project);
        assert!(
            build.status.success(),
            "R07 Drop/RAII source must build through .rsproj -> rustc -> FerrumWeave; a RED here must identify unsupported FerrumWeave ownership lowering rather than a legacy emitter:\nstdout:\n{}\nstderr:\n{}",
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
                "mutating only Rust construction/drop semantics must mutate the managed artifact",
            );
        }
        previous_artifact = Some(bytes);

        fs::copy(&assembly, consumer.join("RustLibrary.dll"))
            .expect("place Rust-produced assembly beside R07 IDisposable consumer project");
        let run = build_and_run_consumer(&consumer, "RustLibrary.dll");
        assert!(
            run.status.success(),
            "C# consumer must execute the Rust-source causal IDisposable projection:\n{}\n{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr),
        );

        let stdout = String::from_utf8_lossy(&run.stdout);
        let observed: Vec<_> = stdout.lines().collect();
        let expected = [
            seed.to_string(),
            (seed + release_increment).to_string(),
            (seed + release_increment).to_string(),
        ];
        assert_eq!(observed, expected);
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
    .expect("write R07 IDisposable C# consumer project");
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
        .current_dir(consumer)
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
    Command::new("dotnet")
        .arg(&consumer_dll)
        .current_dir(consumer)
        .output()
        .expect("execute R07 IDisposable C# consumer on CoreCLR")
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r07-disposable-{}-{nonce}",
        std::process::id(),
    ))
}
