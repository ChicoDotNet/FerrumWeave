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
        "ferrumweave-sdk-backend-path-{}-{nonce}",
        std::process::id(),
    ))
}

fn dotnet_build(repo: &Path, project: &Path) -> Output {
    Command::new("dotnet")
        .args(["build", "HelloFerrum.rsproj"])
        .current_dir(project)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute")
}

fn run_managed_consumer(project: &Path, artifact: &Path) -> String {
    let consumer = project.join("consumer");
    fs::create_dir_all(&consumer).expect("create managed consumer directory");
    fs::copy(artifact, consumer.join("HelloFerrum.dll"))
        .expect("copy generated managed assembly for consumer");
    fs::write(
        consumer.join("Consumer.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>net10.0</TargetFramework>\n  </PropertyGroup>\n  <ItemGroup>\n    <Reference Include=\"HelloFerrum\">\n      <HintPath>HelloFerrum.dll</HintPath>\n      <Private>true</Private>\n    </Reference>\n  </ItemGroup>\n</Project>\n",
    )
    .expect("write C# consumer project");
    fs::write(
        consumer.join("Program.cs"),
        "System.Console.WriteLine(FerrumWeave.RustApi.Answer());\n",
    )
    .expect("write C# consumer source");

    let run = Command::new("dotnet")
        .args(["run", "--project", "Consumer.csproj", "--nologo"])
        .current_dir(&consumer)
        .output()
        .expect("managed consumer must execute");
    assert!(
        run.status.success(),
        "managed consumer failed.\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr),
    );
    String::from_utf8_lossy(&run.stdout)
        .trim()
        .lines()
        .last()
        .unwrap_or_default()
        .to_owned()
}

#[test]
fn rsproj_build_uses_rustc_and_ferrumweave_backend_causally() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let temp = unique_temp_dir();
    let source_dir = temp.join("src");
    fs::create_dir_all(&source_dir).expect("create isolated sdk backend-path test directory");
    fs::copy(template, temp.join("HelloFerrum.rsproj")).expect("copy canonical rsproj");

    // RED falsifier: malformed Rust was previously accepted by the legacy source parser.
    // The SDK must now fail inside the real Rust frontend, before managed emission.
    fs::write(
        source_dir.join("main.rs"),
        "fn main() { println!(\"legacy emitter must not accept this\"); let broken = ; }\n",
    )
    .expect("write invalid Rust source");
    let invalid = dotnet_build(&repo, &temp);
    let invalid_output = format!(
        "{}\n{}",
        String::from_utf8_lossy(&invalid.stdout),
        String::from_utf8_lossy(&invalid.stderr),
    );
    assert!(
        !invalid.status.success(),
        ".rsproj must be gated by rustc; invalid Rust unexpectedly produced a successful build.\n{invalid_output}",
    );
    assert!(
        invalid_output.contains("expected expression") && invalid_output.contains("src/main.rs"),
        ".rsproj failed, but not because rustc rejected malformed Rust; this must not become a false GREEN.\n{invalid_output}",
    );
    assert!(
        !invalid_output.contains("ferrumweave_emit"),
        "legacy ferrumweave_emit must not remain in the product compilation path.\n{invalid_output}",
    );

    let artifact = temp.join("bin/Debug/net10.0/HelloFerrum.dll");
    assert!(
        !artifact.exists(),
        "invalid Rust must not leave a valid managed product artifact",
    );

    let mut previous_bytes: Option<Vec<u8>> = None;
    for value in [137, 211] {
        fs::write(
            source_dir.join("main.rs"),
            format!("#[no_mangle]\npub extern \"C\" fn answer() -> i32 {{ {value} }}\n"),
        )
        .expect("write valid source-causal Rust input");

        let build = dotnet_build(&repo, &temp);
        assert!(
            build.status.success(),
            "valid Rust failed through .rsproj -> rustc -> FerrumWeave.\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr),
        );
        assert!(artifact.is_file(), "valid Rust did not produce managed DLL");

        let bytes = fs::read(&artifact).expect("read managed product artifact");
        assert!(bytes.starts_with(b"MZ") && bytes.windows(4).any(|w| w == b"BSJB"));
        if let Some(previous) = &previous_bytes {
            assert_ne!(
                previous, &bytes,
                "mutating only Rust source 137 -> 211 must change the managed artifact",
            );
        }
        previous_bytes = Some(bytes);

        let observed = run_managed_consumer(&temp, &artifact);
        assert_eq!(
            observed,
            value.to_string(),
            "managed observable must follow the Rust-only source mutation",
        );
    }

    let _ = fs::remove_dir_all(temp);
}
