use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!("ferrumweave-r08-{}-{nonce}", std::process::id()))
}

fn run_dotnet_new(args: &[&str], output: &Path) -> Output {
    let mut command = Command::new("dotnet");
    command.args(["new", "console"]);
    command.args(args);
    command.args(["-o"]);
    command.arg(output);
    command
        .output()
        .expect("dotnet new console must execute for template-language contract tests")
}

fn assert_success(output: &Output, label: &str) {
    assert!(
        output.status.success(),
        "{label} failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&output.stdout),
        String::from_utf8_lossy(&output.stderr),
    );
}

#[test]
fn rust_participates_in_the_standard_console_template_as_a_language() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template_dir = repo.join("sdk/templates/console");
    let temp = unique_temp_dir();
    fs::create_dir_all(&temp).expect("create isolated template test directory");

    let install = Command::new("dotnet")
        .args(["new", "install"])
        .arg(&template_dir)
        .output()
        .expect("dotnet CLI must be available for R10 template-language contract tests");
    assert_success(
        &install,
        "dotnet new install FerrumWeave Rust console language",
    );

    let rust_short_dir = temp.join("RustShort");
    let rust_short = run_dotnet_new(&["-lang", "Rust", "-n", "RustShort"], &rust_short_dir);
    assert_success(&rust_short, "dotnet new console -lang Rust");

    let rust_long_dir = temp.join("RustLong");
    let rust_long = run_dotnet_new(&["--language", "Rust", "-n", "RustLong"], &rust_long_dir);
    assert_success(&rust_long, "dotnet new console --language Rust");

    let csharp_dir = temp.join("CSharpConsole");
    let csharp = run_dotnet_new(&["-n", "CSharpConsole"], &csharp_dir);
    assert_success(&csharp, "dotnet new console default C#");

    let fsharp_dir = temp.join("FSharpConsole");
    let fsharp = run_dotnet_new(&["-lang", "F#", "-n", "FSharpConsole"], &fsharp_dir);
    assert_success(&fsharp, "dotnet new console -lang F#");

    let vb_dir = temp.join("VbConsole");
    let vb = run_dotnet_new(&["-lang", "VB", "-n", "VbConsole"], &vb_dir);
    assert_success(&vb, "dotnet new console -lang VB");

    let _ = Command::new("dotnet")
        .args(["new", "uninstall"])
        .arg(&template_dir)
        .output();

    for (project_dir, project_name) in
        [(&rust_short_dir, "RustShort"), (&rust_long_dir, "RustLong")]
    {
        let project = fs::read_to_string(project_dir.join(format!("{project_name}.rsproj")))
            .expect("Rust console language must generate an .rsproj");
        assert!(project.contains("<Project Sdk=\"FerrumWeave.Sdk\">"));
        assert!(project.contains("<TargetFramework>net10.0</TargetFramework>"));

        let source = fs::read_to_string(project_dir.join("src/main.rs"))
            .expect("Rust console language must generate Rust source");
        assert!(source.contains("#[no_mangle]"));
        assert!(source.contains("pub extern \"C\" fn answer() -> i32"));
        assert!(source.contains("fn main()"));
        assert!(source.contains("std::process::exit(answer())"));
    }

    assert!(
        csharp_dir.join("CSharpConsole.csproj").is_file(),
        "installing FerrumWeave must preserve the default C# console template"
    );
    assert!(
        fsharp_dir.join("FSharpConsole.fsproj").is_file(),
        "installing FerrumWeave must preserve the F# console template"
    );
    assert!(
        vb_dir.join("VbConsole.vbproj").is_file(),
        "installing FerrumWeave must preserve the Visual Basic console template"
    );

    let _ = fs::remove_dir_all(temp);
}
