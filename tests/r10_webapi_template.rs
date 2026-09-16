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
        "ferrumweave-r10-webapi-{}-{nonce}",
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

fn create_webapi(language_args: &[&str], name: &str, output: &Path) -> Output {
    let mut command = Command::new("dotnet");
    command.args(["new", "webapi"]);
    command.args(language_args);
    command.args(["-n", name, "-o"]);
    command.arg(output);
    command
        .output()
        .expect("dotnet new webapi must execute for R10 contract tests")
}

#[test]
fn rust_webapi_scaffolds_as_the_standard_dotnet_webapi_language_variant() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template_dir = repo.join("sdk/templates/webapi");
    let root = unique_temp_dir();
    let rust_api = root.join("RustWebApi");
    let csharp_api = root.join("CSharpWebApi");
    let fsharp_api = root.join("FSharpWebApi");
    fs::create_dir_all(&root).expect("create isolated webapi template test directory");

    let install = Command::new("dotnet")
        .args(["new", "install"])
        .arg(&template_dir)
        .output()
        .expect("dotnet new install must execute for Rust webapi template");
    assert_success(&install, "install FerrumWeave Rust webapi language");

    let rust = create_webapi(&["-lang", "Rust"], "RustWebApi", &rust_api);
    assert_success(&rust, "dotnet new webapi -lang Rust");

    let csharp = create_webapi(&["--no-restore"], "CSharpWebApi", &csharp_api);
    assert_success(&csharp, "dotnet new webapi default C#");

    let fsharp = create_webapi(
        &["-lang", "F#", "--no-restore"],
        "FSharpWebApi",
        &fsharp_api,
    );
    assert_success(&fsharp, "dotnet new webapi -lang F#");

    let _ = Command::new("dotnet")
        .args(["new", "uninstall"])
        .arg(&template_dir)
        .output();

    let project = fs::read_to_string(rust_api.join("RustWebApi.rsproj"))
        .expect("Rust webapi must generate an .rsproj");
    assert!(project.contains("<Project Sdk=\"FerrumWeave.Sdk\">"));
    assert!(project.contains("<TargetFramework>net10.0</TargetFramework>"));
    assert!(project.contains("<FrameworkReference Include=\"Microsoft.AspNetCore.App\" />"));

    let source = fs::read_to_string(rust_api.join("src/main.rs"))
        .expect("Rust webapi must generate Rust source");
    assert!(source.contains("pub extern \"C\" fn health_status() -> i32"));
    assert!(source.contains("200"));

    assert!(csharp_api.join("CSharpWebApi.csproj").is_file());
    assert!(fsharp_api.join("FSharpWebApi.fsproj").is_file());

    let _ = fs::remove_dir_all(root);
}
