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
        "ferrumweave-r06-no-native-abi-{}-{nonce}",
        std::process::id()
    ))
}

fn dotnet_build(repo: &Path, project: &Path) -> Output {
    Command::new("dotnet")
        .args(["build", "RustLibrary.rsproj"])
        .current_dir(project)
        .env("MSBuildSDKsPath", repo.join("sdk"))
        .output()
        .expect("dotnet build must execute")
}

fn run_metadata_verifier(verifier: &Path, assembly: &Path) -> Output {
    Command::new("dotnet")
        .args([
            "run",
            "--project",
            verifier
                .join("MetadataVerifier.csproj")
                .to_str()
                .expect("verifier path is UTF-8"),
            "--configuration",
            "Release",
            "--",
            assembly.to_str().expect("assembly path is UTF-8"),
        ])
        .output()
        .expect("run independent managed metadata verifier")
}

#[test]
fn rust_source_causal_managed_api_requires_no_pinvoke_or_native_abi_layer() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let root = unique_temp_dir();
    let rust_project = root.join("rust-source");
    let source_dir = rust_project.join("src");
    fs::create_dir_all(&source_dir).expect("create R06 Rust source directory");
    fs::copy(template, rust_project.join("RustLibrary.rsproj")).expect("copy canonical rsproj");

    let verifier = root.join("metadata-verifier");
    fs::create_dir_all(&verifier).expect("create managed metadata verifier directory");
    fs::write(
        verifier.join("MetadataVerifier.csproj"),
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net10.0</TargetFramework>
    <ImplicitUsings>enable</ImplicitUsings>
    <Nullable>enable</Nullable>
  </PropertyGroup>
</Project>
"#,
    )
    .expect("write metadata verifier project");

    fs::write(
        verifier.join("Program.cs"),
        r#"using System.Reflection;
using System.Reflection.Metadata;
using System.Reflection.Metadata.Ecma335;
using System.Reflection.PortableExecutable;

using var stream = File.OpenRead(Path.GetFullPath(args.Single()));
using var pe = new PEReader(stream);
if (!pe.HasMetadata)
    throw new InvalidOperationException("R06 artifact must carry CLR metadata");

var corHeader = pe.PEHeaders.CorHeader
    ?? throw new InvalidOperationException("R06 artifact must carry a CLR header");
if ((corHeader.Flags & CorFlags.ILOnly) == 0)
    throw new InvalidOperationException($"R06 artifact must be IL-only, flags were {corHeader.Flags}");

var metadata = pe.GetMetadataReader();
var moduleRefCount = metadata.GetTableRowCount(TableIndex.ModuleRef);
if (moduleRefCount != 0)
    throw new InvalidOperationException($"native module references found: {moduleRefCount}");

foreach (var handle in metadata.MethodDefinitions)
{
    var method = metadata.GetMethodDefinition(handle);
    if ((method.Attributes & MethodAttributes.PinvokeImpl) != 0)
        throw new InvalidOperationException("P/Invoke MethodDef found in Rust-defined managed API");

    var import = method.GetImport();
    if (!import.Module.IsNil)
        throw new InvalidOperationException("native import metadata found in Rust-defined managed API");
}

Console.WriteLine("R06 no-native-ABI contract verified");
"#,
    )
    .expect("write managed metadata verifier source");

    let mut previous_artifact: Option<Vec<u8>> = None;
    for value in [137, 211] {
        fs::write(
            source_dir.join("main.rs"),
            format!("#[no_mangle]\npub extern \"C\" fn answer() -> i32 {{ {value} }}\n"),
        )
        .expect("write R06 Rust source");

        let build = dotnet_build(&repo, &rust_project);
        assert!(
            build.status.success(),
            "R06 Rust source must build through .rsproj -> rustc -> FerrumWeave:\nstdout:\n{}\nstderr:\n{}",
            String::from_utf8_lossy(&build.stdout),
            String::from_utf8_lossy(&build.stderr),
        );

        let assembly = rust_project.join("bin/Debug/net10.0/RustLibrary.dll");
        assert!(
            assembly.is_file(),
            "R06 Rust source did not produce managed DLL"
        );

        let bytes = fs::read(&assembly).expect("read R06 managed artifact");
        if let Some(previous) = &previous_artifact {
            assert_ne!(
                previous, &bytes,
                "mutating only R06 Rust source 137 -> 211 must mutate the managed artifact",
            );
        }
        previous_artifact = Some(bytes);

        let run = run_metadata_verifier(&verifier, &assembly);
        assert!(
            run.status.success(),
            "managed metadata verifier must prove the FerrumWeave-produced artifact requires no native ABI layer:\n{}\n{}",
            String::from_utf8_lossy(&run.stdout),
            String::from_utf8_lossy(&run.stderr)
        );
        assert!(String::from_utf8_lossy(&run.stdout).contains("R06 no-native-ABI contract verified"));
    }

    let _ = fs::remove_dir_all(root);
}
