use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave_cil::write_r06_static_api_artifact;

#[test]
fn managed_r06_api_requires_no_pinvoke_or_native_abi_layer() {
    let root = unique_temp_dir();
    fs::create_dir_all(&root).expect("create R06 native-ABI inspection fixture directory");

    let assembly = write_r06_static_api_artifact(root.join("managed"))
        .expect("emit Rust-defined R06 managed assembly");

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
if (metadata.ModuleReferences.Count != 0)
    throw new InvalidOperationException($"native module references found: {metadata.ModuleReferences.Count}");

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

    let run = Command::new("dotnet")
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
        .expect("run independent managed metadata verifier");

    assert!(
        run.status.success(),
        "managed metadata verifier must prove no native ABI layer is required:\n{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(String::from_utf8_lossy(&run.stdout).contains("R06 no-native-ABI contract verified"));

    let _ = fs::remove_dir_all(root);
}

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
