use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

const SDK_VERSION: &str = "0.1.0-alpha.1";

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r10-debugging-{}-{nonce}",
        std::process::id()
    ))
}

fn xml_path(path: &Path) -> String {
    path.to_string_lossy()
        .replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
}

#[test]
fn debug_build_emits_portable_source_mapping_for_rust_main() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let sdk_project = repo.join("sdk/FerrumWeave.Sdk/FerrumWeave.Sdk.csproj");
    let temp = unique_temp_dir();
    let feed = temp.join("feed");
    let project = temp.join("DebugConsumer");
    let inspector = temp.join("PdbInspector");
    fs::create_dir_all(&feed).expect("create isolated alpha package feed");
    fs::create_dir_all(project.join("src")).expect("create external FerrumWeave project");
    fs::create_dir_all(&inspector).expect("create portable PDB inspector project");

    let pack = Command::new("dotnet")
        .arg("pack")
        .arg(&sdk_project)
        .args(["--configuration", "Release", "--output"])
        .arg(&feed)
        .output()
        .expect("dotnet pack must be available for R10 contract tests");
    assert!(
        pack.status.success(),
        "dotnet pack failed:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&pack.stdout),
        String::from_utf8_lossy(&pack.stderr),
    );

    fs::write(
        project.join("NuGet.Config"),
        format!(
            "<configuration><packageSources><clear/><add key=\"ferrumweave-alpha\" value=\"{}\"/></packageSources></configuration>",
            xml_path(&feed)
        ),
    )
    .expect("write project-scoped alpha feed configuration");
    fs::write(
        project.join("DebugConsumer.rsproj"),
        format!(
            "<Project Sdk=\"FerrumWeave.Sdk/{SDK_VERSION}\"><PropertyGroup><TargetFramework>net10.0</TargetFramework><OutputType>Exe</OutputType><DebugType>portable</DebugType></PropertyGroup></Project>"
        ),
    )
    .expect("write external FerrumWeave debug project");
    fs::write(
        project.join("src/main.rs"),
        "fn main() {\n    println!(\"debug-line\");\n}\n",
    )
    .expect("write supported Rust source");

    let build = Command::new("dotnet")
        .args([
            "build",
            "DebugConsumer.rsproj",
            "--configuration",
            "Debug",
            "--nologo",
        ])
        .current_dir(&project)
        .output()
        .expect("normal dotnet build must execute for the external debug project");
    assert!(
        build.status.success(),
        "debug project must build through the installed alpha SDK:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr),
    );

    let pdb = project.join("bin/Debug/net10.0/DebugConsumer.pdb");
    assert!(
        pdb.is_file(),
        "Debug build must emit a portable PDB next to the managed assembly: {}",
        pdb.display()
    );

    fs::write(
        inspector.join("PdbInspector.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\"><PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework></PropertyGroup></Project>",
    )
    .expect("write CLR metadata inspector project");
    fs::write(
        inspector.join("Program.cs"),
        r#"using System.Reflection.Metadata;
using System.Reflection.Metadata.Ecma335;

using var stream = File.OpenRead(args[0]);
using var provider = MetadataReaderProvider.FromPortablePdbStream(stream);
var reader = provider.GetMetadataReader();
var rustDocument = reader.Documents
    .Select(handle => (Handle: handle, Document: reader.GetDocument(handle)))
    .FirstOrDefault(item => reader.GetString(item.Document.Name).Replace('\\', '/').EndsWith("src/main.rs", StringComparison.Ordinal));
if (rustDocument.Handle.IsNil)
{
    Console.Error.WriteLine("portable PDB does not map src/main.rs");
    return 2;
}

var hasLineTwo = reader.MethodDebugInformation
    .Select(handle => reader.GetMethodDebugInformation(handle))
    .Where(info => info.Document == rustDocument.Handle || info.Document.IsNil)
    .SelectMany(info => info.GetSequencePoints())
    .Any(point => !point.IsHidden && point.StartLine == 2);
if (!hasLineTwo)
{
    Console.Error.WriteLine("portable PDB does not expose a sequence point for Rust line 2");
    return 3;
}

Console.WriteLine("rust-source-map-ok");
return 0;
"#,
    )
    .expect("write portable PDB metadata inspection program");

    let inspect = Command::new("dotnet")
        .args(["run", "--project", "PdbInspector.csproj", "--nologo", "--"])
        .arg(&pdb)
        .current_dir(&inspector)
        .output()
        .expect("CLR metadata inspector must execute");
    assert!(
        inspect.status.success(),
        "portable PDB must contain Rust document and sequence-point mapping:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&inspect.stdout),
        String::from_utf8_lossy(&inspect.stderr),
    );
    assert!(
        String::from_utf8_lossy(&inspect.stdout)
            .lines()
            .any(|line| line.trim() == "rust-source-map-ok"),
        "portable PDB inspection must certify the Rust source mapping"
    );

    let _ = fs::remove_dir_all(temp);
}
