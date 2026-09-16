use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const TOOLCHAIN: &str = "nightly-2025-10-14";
const ASSEMBLY_FILE: &str = "FerrumWeave.Generated.dll";
const RUST_SOURCE: &str = "#[no_mangle]\npub extern \"C\" fn answer(left: i32, right: i32) -> i32 {\n    left + right\n}\n";

#[test]
fn checked_i32_overflow_takes_the_failure_path_through_the_ferrumweave_backend() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let backend = build_codegen_backend(root);
    let work = unique_temp_directory();
    fs::create_dir_all(&work).expect("temporary overflow directory should be created");

    let artifact = compile_source(&backend, &work);
    let project = make_consumer(&artifact, &work);
    let output = run_consumer(&project);

    assert!(
        !output.status.success(),
        "Rust checked i32 overflow must take the failure path instead of returning a wrapped value; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );

    fs::remove_dir_all(work).expect("temporary overflow directory should be removable");
}

fn build_codegen_backend(root: &Path) -> PathBuf {
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

fn compile_source(backend: &Path, work: &Path) -> PathBuf {
    let source = work.join("checked_add.rs");
    let artifact = work.join("checked_add.dll");
    fs::write(&source, RUST_SOURCE).expect("checked-add Rust source should be written");

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

    assert_success(
        &output,
        "FerrumWeave should lower checked Rust i32 addition through the pinned rustc lane",
    );
    assert!(
        artifact.is_file(),
        "managed artifact should be produced for checked Rust i32 addition"
    );
    artifact
}

fn make_consumer(artifact: &Path, root: &Path) -> PathBuf {
    let consumer = root.join("consumer");
    fs::create_dir(&consumer).expect("C# consumer directory should be created");
    fs::copy(artifact, consumer.join(ASSEMBLY_FILE))
        .expect("managed artifact should be copied into the C# consumer");

    fs::write(
        consumer.join("Consumer.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework></PropertyGroup>\n  <ItemGroup><Reference Include=\"FerrumWeave.Generated\"><HintPath>FerrumWeave.Generated.dll</HintPath><Private>true</Private></Reference></ItemGroup>\n</Project>\n",
    )
    .expect("C# consumer project should be written");
    fs::write(
        consumer.join("Program.cs"),
        "System.Console.WriteLine(FerrumWeave.RustApi.Answer(int.MaxValue, 1));\n",
    )
    .expect("C# consumer source should be written");

    consumer.join("Consumer.csproj")
}

fn run_consumer(project: &Path) -> Output {
    Command::new("dotnet")
        .args(["run", "--project"])
        .arg(project)
        .arg("--nologo")
        .env("DOTNET_NOLOGO", "1")
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1")
        .output()
        .expect("dotnet should start the managed overflow consumer")
}

fn assert_success(output: &Output, context: &str) {
    assert!(
        output.status.success(),
        "{context}; stdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr)
    );
}

fn unique_temp_directory() -> PathBuf {
    let nanos = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock should be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-i32-overflow-rust-{}-{nanos}",
        std::process::id()
    ))
}
