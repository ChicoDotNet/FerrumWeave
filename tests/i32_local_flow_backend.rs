use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const TOOLCHAIN: &str = "nightly-2025-10-14";
const ASSEMBLY_FILE: &str = "FerrumWeave.Generated.dll";

#[test]
fn i32_argument_local_flow_is_source_causal_through_the_ferrumweave_backend() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"));
    let backend = build_codegen_backend(root);
    let work = unique_temp_directory();
    fs::create_dir_all(&work).expect("temporary local-flow directory should be created");

    let left_artifact = compile_source(&backend, &work, "left", "answer");
    let right_artifact = compile_source(&backend, &work, "right", "answer");
    let renamed_artifact = compile_source(&backend, &work, "left", "compute_result");

    execute_from_csharp(&left_artifact, 137, &work, "left", "Answer");
    execute_from_csharp(&right_artifact, 211, &work, "right", "Answer");
    execute_from_csharp(
        &renamed_artifact,
        137,
        &work,
        "left_renamed",
        "ComputeResult",
    );

    let left_bytes = fs::read(&left_artifact).expect("left artifact should be readable");
    let right_bytes = fs::read(&right_artifact).expect("right artifact should be readable");
    let renamed_bytes = fs::read(&renamed_artifact).expect("renamed artifact should be readable");

    assert_ne!(
        left_bytes, right_bytes,
        "changing only Rust local selection left -> right must change the managed artifact"
    );
    assert_ne!(
        fs::read(&left_artifact).expect("left artifact should remain readable"),
        renamed_bytes,
        "changing only Rust export identity answer -> compute_result must change managed metadata"
    );

    fs::remove_dir_all(work).expect("temporary local-flow directory should be removable");
}

fn build_codegen_backend(root: &Path) -> PathBuf {
    let manifest = root.join("compiler/codegen-backend/Cargo.toml");
    let output = Command::new("cargo")
        .arg(format!("+{TOOLCHAIN}"))
        .args(["build", "--ignore-rust-version", "--release", "--manifest-path"])
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

fn rust_source(selected: &str, export_name: &str) -> String {
    assert!(matches!(selected, "left" | "right"));
    format!(
        "#[no_mangle]\npub extern \"C\" fn {export_name}(left: i32, right: i32) -> i32 {{\n    let selected = {selected};\n    selected\n}}\n"
    )
}

fn compile_source(backend: &Path, work: &Path, selected: &str, export_name: &str) -> PathBuf {
    let source = work.join(format!("select_{selected}_{export_name}.rs"));
    let artifact = work.join(format!("select_{selected}_{export_name}.dll"));
    fs::write(&source, rust_source(selected, export_name))
        .expect("Rust local-flow source should be written");

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
        &format!(
            "FerrumWeave should lower Rust local flow selecting {selected} for export {export_name}"
        ),
    );
    assert!(
        artifact.is_file(),
        "managed artifact should be produced for selection {selected} / {export_name}"
    );
    artifact
}

fn execute_from_csharp(
    artifact: &Path,
    expected: i32,
    root: &Path,
    label: &str,
    method_name: &str,
) {
    let consumer = root.join(format!("consumer_{label}"));
    fs::create_dir(&consumer).expect("C# consumer directory should be created");
    fs::copy(artifact, consumer.join(ASSEMBLY_FILE))
        .expect("managed artifact should be copied into the C# consumer");

    fs::write(
        consumer.join("Consumer.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>net10.0</TargetFramework>\n  </PropertyGroup>\n  <ItemGroup>\n    <Reference Include=\"FerrumWeave.Generated\">\n      <HintPath>FerrumWeave.Generated.dll</HintPath>\n      <Private>true</Private>\n    </Reference>\n  </ItemGroup>\n</Project>\n",
    )
    .expect("C# consumer project should be written");
    fs::write(
        consumer.join("Program.cs"),
        format!(
            "System.Console.WriteLine(FerrumWeave.RustApi.{method_name}(137, 211));\n"
        ),
    )
    .expect("C# consumer source should be written");

    let output = Command::new("dotnet")
        .args(["run", "--project"])
        .arg(consumer.join("Consumer.csproj"))
        .arg("--nologo")
        .env("DOTNET_NOLOGO", "1")
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1")
        .output()
        .expect("dotnet should start the managed consumer");

    assert_success(
        &output,
        &format!("C# consumer should execute Rust selection {label}"),
    );

    let stdout = String::from_utf8(output.stdout).expect("consumer stdout should be UTF-8");
    let observed = stdout.lines().next_back().unwrap_or_default().trim();
    assert_eq!(
        observed,
        expected.to_string(),
        "Rust selection {label} should drive the managed observable"
    );
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
        "ferrumweave-i32-local-flow-rust-{}-{nanos}",
        std::process::id()
    ))
}
