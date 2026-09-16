use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const PROJECT_NAME: &str = "LocalFlow";

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-i32-local-flow-{}-{nonce}",
        std::process::id(),
    ))
}

fn rust_source(selected: &str, export_name: &str) -> String {
    assert!(matches!(selected, "left" | "right"));
    format!(
        "#[no_mangle]\npub extern \"C\" fn {export_name}(left: i32, right: i32) -> i32 {{\n    let selected = {selected};\n    selected\n}}\n"
    )
}

fn create_variant(repo: &Path, root: &Path, label: &str, selected: &str, export_name: &str) -> PathBuf {
    let project = root.join(label);
    let source_dir = project.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated local-flow project");
    fs::copy(
        repo.join("sdk/templates/rust/HelloFerrum.rsproj"),
        project.join(format!("{PROJECT_NAME}.rsproj")),
    )
    .expect("copy canonical FerrumWeave project template");
    fs::write(
        source_dir.join("main.rs"),
        rust_source(selected, export_name),
    )
    .expect("write source-causal Rust local-flow variant");
    project
}

fn dotnet_build(repo: &Path, project: &Path) -> Output {
    Command::new("dotnet")
        .args(["build", &format!("{PROJECT_NAME}.rsproj"), "--nologo"])
        .current_dir(project)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("FerrumWeave local-flow project must invoke dotnet build")
}

fn build_variant(repo: &Path, project: &Path) -> Vec<u8> {
    let build = dotnet_build(repo, project);
    assert!(
        build.status.success(),
        "FerrumWeave local-flow variant failed through .rsproj -> rustc -> FerrumWeave backend.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );

    let build_output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );
    assert!(
        !build_output.contains("ferrumweave_emit"),
        "legacy ferrumweave_emit must not participate in the local-flow product path:\n{build_output}",
    );

    let artifact = project.join(format!("bin/Debug/net10.0/{PROJECT_NAME}.dll"));
    assert!(artifact.is_file(), "local-flow build did not emit managed DLL");
    let bytes = fs::read(&artifact).expect("read local-flow managed artifact");
    assert!(
        bytes.starts_with(b"MZ") && bytes.windows(4).any(|window| window == b"BSJB"),
        "local-flow artifact must be a managed PE/CLI assembly",
    );
    bytes
}

fn run_managed_consumer(project: &Path, method_name: &str, expected: i32) {
    let consumer = project.join("consumer");
    fs::create_dir_all(&consumer).expect("create local-flow managed consumer directory");
    fs::copy(
        project.join(format!("bin/Debug/net10.0/{PROJECT_NAME}.dll")),
        consumer.join(format!("{PROJECT_NAME}.dll")),
    )
    .expect("copy local-flow managed artifact into consumer");
    fs::write(
        consumer.join("Consumer.csproj"),
        format!(
            "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>net10.0</TargetFramework>\n  </PropertyGroup>\n  <ItemGroup>\n    <Reference Include=\"{PROJECT_NAME}\">\n      <HintPath>{PROJECT_NAME}.dll</HintPath>\n      <Private>true</Private>\n    </Reference>\n  </ItemGroup>\n</Project>\n"
        ),
    )
    .expect("write local-flow C# consumer project");
    fs::write(
        consumer.join("Program.cs"),
        format!(
            "System.Console.WriteLine(FerrumWeave.RustApi.{method_name}(137, 211));\n"
        ),
    )
    .expect("write local-flow C# consumer source");

    let run = Command::new("dotnet")
        .args(["run", "--project", "Consumer.csproj", "--nologo"])
        .current_dir(&consumer)
        .output()
        .expect("local-flow managed consumer must execute");
    assert!(
        run.status.success(),
        "local-flow C# consumer failed for {method_name}.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr),
    );

    let observed = String::from_utf8_lossy(&run.stdout)
        .trim()
        .lines()
        .last()
        .unwrap_or_default()
        .to_owned();
    assert_eq!(
        observed,
        expected.to_string(),
        "managed observable for {method_name} must follow Rust local selection",
    );
}

#[test]
fn i32_argument_local_flow_is_source_causal_through_managed_consumption() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let temp = unique_temp_dir();

    let left = create_variant(&repo, &temp, "left", "left", "answer");
    let right = create_variant(&repo, &temp, "right", "right", "answer");
    let renamed = create_variant(
        &repo,
        &temp,
        "left-renamed",
        "left",
        "compute_result",
    );

    let left_bytes = build_variant(&repo, &left);
    run_managed_consumer(&left, "Answer", 137);

    let right_bytes = build_variant(&repo, &right);
    run_managed_consumer(&right, "Answer", 211);

    let renamed_bytes = build_variant(&repo, &renamed);
    run_managed_consumer(&renamed, "ComputeResult", 137);

    assert_ne!(
        left_bytes, right_bytes,
        "changing only Rust local selection left -> right must change the managed artifact",
    );
    assert_ne!(
        left_bytes, renamed_bytes,
        "changing only Rust export identity answer -> compute_result must change managed metadata",
    );

    let _ = fs::remove_dir_all(temp);
}
