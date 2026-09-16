use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

pub const TOOLCHAIN: &str = "nightly-2025-10-14";
pub const ASSEMBLY_FILE: &str = "FerrumWeave.Generated.dll";

pub fn build_codegen_backend() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let manifest = root.join("compiler/codegen-backend/Cargo.toml");
    let output = Command::new("cargo")
        .arg(format!("+{TOOLCHAIN}"))
        .args([
            "build",
            "--ignore-rust-version",
            "--release",
            "--manifest-path",
        ])
        .arg(&manifest)
        .current_dir(root)
        .output()
        .expect("cargo should start to build the FerrumWeave codegen backend");

    assert_success(
        &output,
        "FerrumWeave codegen backend should build with the pinned nightly toolchain",
    );

    let backend = root
        .join("compiler/codegen-backend/target/release")
        .join(backend_filename());
    assert!(
        backend.is_file(),
        "FerrumWeave backend should exist at {}",
        backend.display()
    );
    backend
}

pub fn create_work_dir(label: &str) -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    let work = std::env::temp_dir().join(format!(
        "ferrumweave-{label}-{}-{nanos}",
        std::process::id()
    ));
    fs::create_dir_all(&work).expect("temporary backend replay directory should be created");
    work
}

pub fn compile_rust_source(
    backend: &Path,
    work: &Path,
    source_name: &str,
    artifact_name: &str,
    source_text: &str,
    context: &str,
) -> PathBuf {
    let source = work.join(source_name);
    let artifact = work.join(artifact_name);
    fs::write(&source, source_text).expect("Rust backend replay source should be written");

    let output = Command::new("rustc")
        .arg(format!("+{TOOLCHAIN}"))
        .arg("-Z")
        .arg(format!("codegen-backend={}", backend.display()))
        .args(["--edition", "2021", "--crate-type", "lib"])
        .arg(&source)
        .arg("-o")
        .arg(&artifact)
        .output()
        .expect("rustc should start with the FerrumWeave backend");

    assert_success(&output, context);
    assert!(
        artifact.is_file(),
        "managed artifact should be produced at {}",
        artifact.display()
    );
    artifact
}

pub fn make_csharp_consumer(
    artifact: &Path,
    root: &Path,
    label: &str,
    program_source: &str,
) -> PathBuf {
    let consumer = root.join(format!("consumer_{label}"));
    fs::create_dir(&consumer).expect("C# consumer directory should be created");
    fs::copy(artifact, consumer.join(ASSEMBLY_FILE))
        .expect("managed artifact should be copied into the C# consumer");

    fs::write(
        consumer.join("Consumer.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework></PropertyGroup>\n  <ItemGroup><Reference Include=\"FerrumWeave.Generated\"><HintPath>FerrumWeave.Generated.dll</HintPath><Private>true</Private></Reference></ItemGroup>\n</Project>\n",
    )
    .expect("C# consumer project should be written");
    fs::write(consumer.join("Program.cs"), program_source)
        .expect("C# consumer source should be written");

    consumer.join("Consumer.csproj")
}

pub fn run_managed_consumer(project: &Path, context: &str) -> Output {
    Command::new("dotnet")
        .args(["run", "--project"])
        .arg(project)
        .arg("--nologo")
        .env("DOTNET_NOLOGO", "1")
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1")
        .output()
        .unwrap_or_else(|error| {
            panic!("{context}: dotnet should start the managed consumer: {error}")
        })
}

pub fn assert_success(output: &Output, context: &str) {
    assert!(
        output.status.success(),
        "{context}; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

pub fn remove_work_dir(work: &Path) {
    fs::remove_dir_all(work).expect("temporary backend replay directory should be removable");
}

fn backend_filename() -> &'static str {
    if cfg!(target_os = "windows") {
        "ferrumweave_codegen_backend.dll"
    } else if cfg!(target_os = "linux") {
        "libferrumweave_codegen_backend.so"
    } else if cfg!(target_os = "macos") {
        "libferrumweave_codegen_backend.dylib"
    } else {
        panic!("unsupported platform for the FerrumWeave codegen backend replay")
    }
}
