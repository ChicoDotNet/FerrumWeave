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
        "ferrumweave-r10-test-templates-{}-{nonce}",
        std::process::id(),
    ))
}

fn assert_success(output: &Output, label: &str) {
    assert!(
        output.status.success(),
        "{label} failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

fn scaffold_rust_test_template(
    repo: &Path,
    root: &Path,
    short_name: &str,
    project_name: &str,
    framework: &str,
) {
    let template_dir = repo.join(format!("sdk/templates/{short_name}"));
    let project_dir = root.join(project_name);

    let install = Command::new("dotnet")
        .args(["new", "install"])
        .arg(&template_dir)
        .output()
        .expect("dotnet new install must execute for Rust test templates");
    assert_success(&install, &format!("install {short_name} Rust template"));

    let create = Command::new("dotnet")
        .args([
            "new",
            short_name,
            "-lang",
            "Rust",
            "-n",
            project_name,
            "-o",
        ])
        .arg(&project_dir)
        .output()
        .expect("dotnet new test template must execute");

    let _ = Command::new("dotnet")
        .args(["new", "uninstall"])
        .arg(&template_dir)
        .output();

    assert_success(
        &create,
        &format!("dotnet new {short_name} -lang Rust"),
    );

    let project_path = project_dir.join(format!("{project_name}.rsproj"));
    let project = fs::read_to_string(&project_path).expect("test template must generate an .rsproj");
    assert!(project.contains("<Project Sdk=\"FerrumWeave.Sdk\">"));
    assert!(project.contains("<TargetFramework>net10.0</TargetFramework>"));
    assert!(project.contains("<OutputType>Library</OutputType>"));
    assert!(project.contains("<IsTestProject>true</IsTestProject>"));
    assert!(project.contains(&format!(
        "<FerrumWeaveTestFramework>{framework}</FerrumWeaveTestFramework>"
    )));

    let source = fs::read_to_string(project_dir.join("src/main.rs"))
        .expect("test template must generate Rust source");
    assert!(source.contains("pub extern \"C\" fn sample_test_value() -> i32"));

    let test = Command::new("dotnet")
        .args(["test"])
        .arg(&project_path)
        .arg("--no-restore")
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet test must execute for the staged R10 test-template contract");

    let diagnostics = format!(
        "{}\n{}",
        String::from_utf8_lossy(&test.stdout),
        String::from_utf8_lossy(&test.stderr),
    );
    assert!(
        !test.status.success(),
        "{short_name} must not report false test success before the Rust test adapter exists:\n{diagnostics}"
    );
    assert!(
        diagnostics.contains("FerrumWeave.Sdk does not yet provide a Rust test adapter"),
        "{short_name} must fail at the explicit Rust test-adapter boundary:\n{diagnostics}"
    );
}

#[test]
fn xunit_nunit_and_mstest_are_reserved_as_rust_language_template_families() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let temp = unique_temp_dir();
    fs::create_dir_all(&temp).expect("create isolated R10 test-template directory");

    for (short_name, project_name, framework) in [
        ("xunit", "RustXunit", "xUnit"),
        ("nunit", "RustNunit", "NUnit"),
        ("mstest", "RustMstest", "MSTest"),
    ] {
        scaffold_rust_test_template(&repo, &temp, short_name, project_name, framework);
    }

    let _ = fs::remove_dir_all(temp);
}
