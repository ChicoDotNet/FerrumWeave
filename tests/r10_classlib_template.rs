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
        "ferrumweave-r10-classlib-{}-{nonce}",
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

fn create_classlib(language_args: &[&str], name: &str, output: &Path) -> Output {
    let mut command = Command::new("dotnet");
    command.args(["new", "classlib"]);
    command.args(language_args);
    command.args(["-n", name, "-o"]);
    command.arg(output);
    command
        .output()
        .expect("dotnet new classlib must execute for R10 contract tests")
}

#[test]
fn rust_classlib_is_a_real_dotnet_language_variant_consumable_by_csharp() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template_dir = repo.join("sdk/templates/classlib");
    let root = unique_temp_dir();
    let rust_library = root.join("RustLibrary");
    let csharp_library = root.join("CSharpLibrary");
    let fsharp_library = root.join("FSharpLibrary");
    let vb_library = root.join("VbLibrary");
    let consumer = root.join("Consumer");
    fs::create_dir_all(&root).expect("create isolated classlib template test directory");

    let install = Command::new("dotnet")
        .args(["new", "install"])
        .arg(&template_dir)
        .output()
        .expect("dotnet new install must execute for Rust classlib template");
    assert_success(&install, "install FerrumWeave Rust classlib language");

    let rust = create_classlib(&["-lang", "Rust"], "RustLibrary", &rust_library);
    assert_success(&rust, "dotnet new classlib -lang Rust");

    let csharp = create_classlib(&[], "CSharpLibrary", &csharp_library);
    assert_success(&csharp, "dotnet new classlib default C#");

    let fsharp = create_classlib(&["-lang", "F#"], "FSharpLibrary", &fsharp_library);
    assert_success(&fsharp, "dotnet new classlib -lang F#");

    let vb = create_classlib(&["-lang", "VB"], "VbLibrary", &vb_library);
    assert_success(&vb, "dotnet new classlib -lang VB");

    let _ = Command::new("dotnet")
        .args(["new", "uninstall"])
        .arg(&template_dir)
        .output();

    let project = fs::read_to_string(rust_library.join("RustLibrary.rsproj"))
        .expect("Rust classlib must generate an .rsproj");
    assert!(project.contains("<Project Sdk=\"FerrumWeave.Sdk\">"));
    assert!(project.contains("<TargetFramework>net10.0</TargetFramework>"));
    assert!(project.contains("<OutputType>Library</OutputType>"));

    let source = fs::read_to_string(rust_library.join("src/main.rs"))
        .expect("Rust classlib must generate Rust source");
    assert!(source.contains("pub extern \"C\" fn answer() -> i32"));
    assert!(source.contains("42"));

    assert!(csharp_library.join("CSharpLibrary.csproj").is_file());
    assert!(fsharp_library.join("FSharpLibrary.fsproj").is_file());
    assert!(vb_library.join("VbLibrary.vbproj").is_file());

    fs::create_dir_all(&consumer).expect("create C# classlib consumer directory");
    fs::write(
        consumer.join("Consumer.csproj"),
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
    <ImplicitUsings>disable</ImplicitUsings>
    <Nullable>enable</Nullable>
  </PropertyGroup>
  <ItemGroup>
    <ProjectReference Include="../RustLibrary/RustLibrary.rsproj" />
  </ItemGroup>
</Project>
"#,
    )
    .expect("write C# consumer project");
    fs::write(
        consumer.join("Program.cs"),
        "using System;\nConsole.WriteLine(FerrumWeave.RustApi.Answer());\n",
    )
    .expect("write C# consumer source");

    let run = Command::new("dotnet")
        .args(["run", "--project", "Consumer.csproj"])
        .current_dir(&consumer)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("C# consumer must execute through ProjectReference to Rust classlib");
    assert_success(
        &run,
        "consume Rust classlib from C# through ProjectReference",
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout).trim(), "42");

    let _ = fs::remove_dir_all(root);
}
