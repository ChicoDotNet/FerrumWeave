use ferrumweave::managed::{
    method_calls_member_ref, resolve_public_static_member_ref, resolve_public_static_method,
};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r09-source-causality-{}-{nonce}",
        std::process::id(),
    ))
}

fn copy_tree(source: &Path, destination: &Path) {
    fs::create_dir_all(destination).expect("create isolated R09 fixture directory");
    for entry in fs::read_dir(source).expect("read canonical R09 fixture") {
        let entry = entry.expect("read canonical R09 fixture entry");
        let source_path = entry.path();
        let destination_path = destination.join(entry.file_name());
        if entry.file_type().expect("read R09 fixture file type").is_dir() {
            copy_tree(&source_path, &destination_path);
        } else {
            fs::copy(&source_path, &destination_path).unwrap_or_else(|error| {
                panic!(
                    "copy canonical R09 fixture {} -> {}: {error}",
                    source_path.display(),
                    destination_path.display()
                )
            });
        }
    }
}

fn rewrite_sdk_imports(repo: &Path, fixture: &Path) {
    let project = fixture.join("RiskEngine/RiskEngine.rsproj");
    let original = fs::read_to_string(&project).expect("read isolated RiskEngine.rsproj");
    let props = repo.join("sdk/FerrumWeave.Sdk/Sdk/Sdk.props");
    let targets = repo.join("sdk/FerrumWeave.Sdk/Sdk/Sdk.targets");
    let rewritten = original
        .replace("../../../../sdk/FerrumWeave.Sdk/Sdk/Sdk.props", &props.display().to_string())
        .replace(
            "../../../../sdk/FerrumWeave.Sdk/Sdk/Sdk.targets",
            &targets.display().to_string(),
        );
    fs::write(project, rewritten).expect("rewrite isolated SDK imports");
}

fn dotnet(fixture: &Path, args: &[&str]) -> Output {
    Command::new("dotnet")
        .args(args)
        .current_dir(fixture)
        .output()
        .expect("R09 dotnet command must execute")
}

fn write_rust_value(fixture: &Path, value: i32) {
    fs::write(
        fixture.join("RiskEngine/src/main.rs"),
        format!(
            "#[inline(never)]\nfn ferrumweave_system_math_abs(value: i32) -> i32 {{\n    value\n}}\n\n#[no_mangle]\npub extern \"C\" fn answer() -> i32 {{\n    ferrumweave_system_math_abs({value})\n}}\n"
        ),
    )
    .expect("write Rust-only R09 mutation");
}

#[test]
fn mixed_solution_business_path_is_causal_from_rust_source() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let canonical = repo.join("tests/fixtures/r09");
    let temp = unique_temp_dir();
    copy_tree(&canonical, &temp);
    rewrite_sdk_imports(&repo, &temp);

    let artifact = temp.join("RiskEngine/bin/Debug/net10.0/RiskEngine.dll");
    let mut previous_bytes: Option<Vec<u8>> = None;

    for value in [42, 73] {
        write_rust_value(&temp, value);

        let clean = dotnet(&temp, &["clean", "Enterprise.slnx", "--nologo"]);
        assert!(
            clean.status.success(),
            "R09 isolated mixed solution must clean before causal replay:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&clean.stdout),
            String::from_utf8_lossy(&clean.stderr),
        );

        let build = dotnet(&temp, &["build", "Enterprise.slnx", "--nologo"]);
        let build_output = format!(
            "{}\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr),
        );
        assert!(
            build.status.success(),
            "R09 mixed solution must build through .slnx -> .rsproj -> rustc -> FerrumWeave backend:\n{build_output}",
        );
        assert!(
            !build_output.contains("ferrumweave_emit"),
            "legacy ferrumweave_emit must not participate in the R09 product call path:\n{build_output}",
        );
        assert!(artifact.is_file(), "R09 Rust project did not emit RiskEngine.dll");

        let bytes = fs::read(&artifact).expect("read source-causal RiskEngine.dll");
        assert!(bytes.starts_with(b"MZ") && bytes.windows(4).any(|window| window == b"BSJB"));
        if let Some(previous) = &previous_bytes {
            assert_ne!(
                previous, &bytes,
                "mutating only RiskEngine Rust source 42 -> 73 must change RiskEngine.dll",
            );
        }
        previous_bytes = Some(bytes.clone());

        let rust_answer = resolve_public_static_method(&bytes, "FerrumWeave", "RustApi", "Answer")
            .expect("R09 must expose the Rust business method through managed metadata");
        let managed_dependency = resolve_public_static_member_ref(&bytes, "System", "Math", "Abs")
            .expect("R09 Rust output must reference the managed System.Math dependency");
        match method_calls_member_ref(&bytes, &rust_answer, &managed_dependency) {
            Ok(calls_dependency) => assert!(calls_dependency),
            Err(ferrumweave::managed::ManagedMetadataError::Unsupported(_)) => {
                let mut encoded_call = vec![0x28];
                encoded_call.extend_from_slice(&managed_dependency.token.to_le_bytes());
                assert!(
                    bytes.windows(encoded_call.len()).any(|window| window == encoded_call),
                    "R09 source-causal artifact must contain a CLR call to System.Math.Abs",
                );
            }
            Err(error) => panic!("R09 must inspect the emitted Rust method body: {error:?}"),
        }

        let run = dotnet(
            &temp,
            &[
                "run",
                "--project",
                "Domain/Domain.csproj",
                "--configuration",
                "Debug",
                "--no-build",
                "--nologo",
            ],
        );
        assert!(
            run.status.success(),
            "R09 managed consumer must execute source-causal Rust output:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr),
        );
        let stdout = String::from_utf8_lossy(&run.stdout);
        assert!(
            stdout.lines().any(|line| line.trim() == format!("risk-score={value}")),
            "R09 CoreCLR observable must follow the Rust-only source mutation; stdout was:\n{stdout}",
        );
    }

    let _ = fs::remove_dir_all(temp);
}
