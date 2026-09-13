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
        "ferrumweave-sdk-managed-consumption-{}-{nonce}",
        std::process::id(),
    ))
}

fn consume_managed_answer(assembly: &Path, temp: &Path) -> Output {
    let consumer = temp.join("consumer");
    fs::create_dir_all(&consumer).expect("create managed consumer directory");
    fs::write(
        consumer.join("Consumer.csproj"),
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
  </PropertyGroup>
</Project>
"#,
    )
    .expect("write managed consumer project");
    fs::write(
        consumer.join("Program.cs"),
        r#"using System.Reflection;
var assembly = Assembly.LoadFrom(args[0]);
var type = assembly.GetType("FerrumWeave.RustApi", throwOnError: true)!;
var answer = type.GetMethod("Answer", BindingFlags.Public | BindingFlags.Static)!;
System.Console.Write(answer.Invoke(null, null));
"#,
    )
    .expect("write managed consumer program");

    Command::new("dotnet")
        .args([
            "run",
            "--project",
            "Consumer.csproj",
            "--",
            assembly.to_str().expect("assembly path must be UTF-8"),
        ])
        .current_dir(&consumer)
        .output()
        .expect("managed consumer must execute")
}

#[test]
fn sdk_build_produces_a_managed_library_consumable_by_dotnet() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust");
    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated SDK consumption test directory");

    fs::copy(
        template.join("HelloFerrum.rsproj"),
        temp.join("HelloFerrum.rsproj"),
    )
    .expect("copy canonical SDK project fixture");
    fs::copy(template.join("src/main.rs"), source_dir.join("main.rs"))
        .expect("copy canonical Rust source fixture");

    let build = Command::new("dotnet")
        .args(["build", "HelloFerrum.rsproj"])
        .current_dir(&temp)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute");
    assert!(
        build.status.success(),
        "FerrumWeave.Sdk must build the Rust project through the managed-library product path:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );

    let assembly = temp.join("bin/Debug/net10.0/HelloFerrum.dll");
    assert!(
        assembly.is_file(),
        "the SDK build must materialize the managed library produced by FerrumWeave",
    );

    let consume = consume_managed_answer(&assembly, &temp);
    assert!(
        consume.status.success(),
        "a normal .NET consumer must load and invoke the FerrumWeave artifact:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&consume.stdout),
        String::from_utf8_lossy(&consume.stderr),
    );
    assert_eq!(
        String::from_utf8_lossy(&consume.stdout).trim(),
        "42",
        "the managed consumer must observe behavior originating in the canonical Rust source",
    );

    let _ = fs::remove_dir_all(temp);
}
