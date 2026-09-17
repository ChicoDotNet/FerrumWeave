use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};
use std::time::{SystemTime, UNIX_EPOCH};

const SDK_VERSION: &str = "0.1.0-alpha.1";

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r10-installed-sdk-{}-{nonce}",
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

fn isolated_dotnet(root: &Path) -> Command {
    let mut command = Command::new("dotnet");
    command
        .env("DOTNET_CLI_HOME", root.join("dotnet-home"))
        .env("DOTNET_NOLOGO", "1")
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1")
        .env("NUGET_PACKAGES", root.join("nuget-packages"));
    command
}

fn write_nuget_config(root: &Path, feed: &Path) {
    let feed = feed.to_string_lossy().replace('&', "&amp;");
    fs::write(
        root.join("NuGet.Config"),
        format!(
            "<?xml version=\"1.0\" encoding=\"utf-8\"?>\n<configuration>\n  <packageSources>\n    <clear />\n    <add key=\"FerrumWeaveLocal\" value=\"{feed}\" />\n  </packageSources>\n</configuration>\n"
        ),
    )
    .expect("write isolated NuGet.Config");
}

fn build_rust_project(root: &Path, project: &Path) -> Output {
    let mut command = isolated_dotnet(root);
    command
        .args(["build", "HelloFerrum.rsproj", "--nologo"])
        .current_dir(project)
        .output()
        .expect("dotnet build must execute for installed-package contract")
}

fn run_managed_consumer(root: &Path, project: &Path, artifact: &Path) -> String {
    let consumer = project.join("consumer");
    fs::create_dir_all(&consumer).expect("create installed-package consumer directory");
    fs::copy(artifact, consumer.join("HelloFerrum.dll"))
        .expect("copy installed-package managed artifact");
    fs::write(
        consumer.join("Consumer.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <OutputType>Exe</OutputType>\n    <TargetFramework>net10.0</TargetFramework>\n  </PropertyGroup>\n  <ItemGroup>\n    <Reference Include=\"HelloFerrum\">\n      <HintPath>HelloFerrum.dll</HintPath>\n      <Private>true</Private>\n    </Reference>\n  </ItemGroup>\n</Project>\n",
    )
    .expect("write installed-package C# consumer project");
    fs::write(
        consumer.join("Program.cs"),
        "System.Console.WriteLine(FerrumWeave.RustApi.Answer());\n",
    )
    .expect("write installed-package C# consumer source");

    let mut command = isolated_dotnet(root);
    let output = command
        .args(["run", "--project", "Consumer.csproj", "--nologo"])
        .current_dir(&consumer)
        .output()
        .expect("managed consumer must execute for installed-package contract");
    assert_success(
        &output,
        "run managed consumer of installed FerrumWeave artifact",
    );

    String::from_utf8_lossy(&output.stdout)
        .trim()
        .lines()
        .last()
        .unwrap_or_default()
        .to_owned()
}

#[test]
fn installed_sdk_package_builds_external_console_causally() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let package_project = repo.join("sdk/FerrumWeave.Sdk/FerrumWeave.Sdk.csproj");
    assert!(
        package_project.is_file(),
        "R10 RED: FerrumWeave.Sdk must become a real NuGet package project at {}",
        package_project.display(),
    );

    let root = unique_temp_dir();
    let feed = root.join("feed");
    let project = root.join("HelloFerrum");
    fs::create_dir_all(&feed).expect("create isolated FerrumWeave NuGet feed");
    write_nuget_config(&root, &feed);

    let mut pack = isolated_dotnet(&root);
    let pack = pack
        .args(["pack", "--nologo", "-c", "Release", "-o"])
        .arg(&feed)
        .arg(&package_project)
        .output()
        .expect("dotnet pack must execute for FerrumWeave.Sdk");
    assert_success(&pack, "pack FerrumWeave.Sdk");

    let nupkg = feed.join(format!("FerrumWeave.Sdk.{SDK_VERSION}.nupkg"));
    assert!(
        nupkg.is_file(),
        "FerrumWeave.Sdk package must be produced at {}",
        nupkg.display(),
    );

    let mut install = isolated_dotnet(&root);
    let install = install
        .args(["new", "install", &format!("FerrumWeave.Sdk@{SDK_VERSION}")])
        .args(["--add-source"])
        .arg(&feed)
        .current_dir(&root)
        .output()
        .expect("dotnet new install must execute for FerrumWeave.Sdk package");
    assert_success(
        &install,
        "install FerrumWeave.Sdk package by NuGet ID and version",
    );

    let mut scaffold = isolated_dotnet(&root);
    let scaffold = scaffold
        .args(["new", "console", "-lang", "Rust", "-n", "HelloFerrum", "-o"])
        .arg(&project)
        .current_dir(&root)
        .output()
        .expect("dotnet new console -lang Rust must execute after package install");
    assert_success(
        &scaffold,
        "create Rust console from installed FerrumWeave.Sdk package",
    );

    let generated_project = fs::read_to_string(project.join("HelloFerrum.rsproj"))
        .expect("installed package must generate HelloFerrum.rsproj");
    assert!(generated_project.contains("<Project Sdk=\"FerrumWeave.Sdk\">"));
    assert!(generated_project.contains("<FerrumWeaveRustCrateType>bin</FerrumWeaveRustCrateType>"));
    assert!(
        !generated_project.contains(repo.to_string_lossy().as_ref()),
        "generated project must not embed repository-local paths",
    );

    let generated_source = fs::read_to_string(project.join("src/main.rs"))
        .expect("installed package must generate Rust console source");
    assert!(
        generated_source.contains("fn main()"),
        "installed Rust console template must preserve ordinary Rust main"
    );

    let global_json = fs::read_to_string(project.join("global.json"))
        .expect("installed Rust template must pin the FerrumWeave MSBuild SDK version");
    assert!(global_json.contains("\"FerrumWeave.Sdk\""));
    assert!(global_json.contains(SDK_VERSION));

    let source = project.join("src/main.rs");
    let artifact = project.join("bin/Debug/net10.0/HelloFerrum.dll");
    let mut previous_bytes: Option<Vec<u8>> = None;

    for value in [137, 211] {
        fs::write(
            &source,
            format!(
                "#[no_mangle]\npub extern \"C\" fn answer() -> i32 {{ {value} }}\n\nfn main() {{\n    std::process::exit(answer())\n}}\n"
            ),
        )
        .expect("write Rust-only installed-package mutation");

        let build = build_rust_project(&root, &project);
        assert_success(
            &build,
            "external .rsproj must build through the installed FerrumWeave.Sdk package",
        );
        assert!(
            artifact.is_file(),
            "installed SDK build must produce managed DLL"
        );

        let bytes = fs::read(&artifact).expect("read installed-package managed artifact");
        assert!(bytes.starts_with(b"MZ") && bytes.windows(4).any(|window| window == b"BSJB"));
        if let Some(previous) = &previous_bytes {
            assert_ne!(
                previous, &bytes,
                "changing only external Rust source 137 -> 211 must change the managed artifact",
            );
        }
        previous_bytes = Some(bytes);

        let observed = run_managed_consumer(&root, &project, &artifact);
        assert_eq!(
            observed,
            value.to_string(),
            "managed observable must follow Rust-only source mutation through installed package",
        );
    }

    let mut uninstall = isolated_dotnet(&root);
    let _ = uninstall
        .args(["new", "uninstall", "FerrumWeave.Sdk"])
        .current_dir(&root)
        .output();

    let _ = fs::remove_dir_all(root);
}
