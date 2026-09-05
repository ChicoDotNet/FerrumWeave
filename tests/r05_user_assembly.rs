use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave::managed::resolve_public_static_method;
use ferrumweave_cil::{
    PROBE_ASSEMBLY_FILE, PROBE_RUNTIME_CONFIG_FILE, emit_probe_assembly_with_external_static_call,
    probe_runtime_config,
};

const USER_ASSEMBLY_NAME: &str = "FerrumWeave.UserFixture";
const USER_NAMESPACE: &str = "UserFixture";
const USER_TYPE: &str = "Api";
const USER_METHOD: &str = "Greeting";
const USER_GREETING: &str = "Hello from independently compiled C#";

#[test]
fn rust_emitted_il_consumes_independently_compiled_csharp_assembly() {
    let root = unique_temp_dir();
    fs::create_dir_all(&root).expect("create R05 user-assembly fixture directory");

    let project = root.join("user");
    fs::create_dir_all(&project).expect("create independent C# project directory");
    fs::write(
        project.join("UserFixture.csproj"),
        r#"<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <TargetFramework>net10.0</TargetFramework>
    <AssemblyName>FerrumWeave.UserFixture</AssemblyName>
    <ImplicitUsings>disable</ImplicitUsings>
    <Nullable>enable</Nullable>
  </PropertyGroup>
</Project>
"#,
    )
    .expect("write independent C# project");
    fs::write(
        project.join("Api.cs"),
        format!(
            "namespace {USER_NAMESPACE};\n\npublic static class {USER_TYPE}\n{{\n    public static string {USER_METHOD}() => \"{USER_GREETING}\";\n}}\n"
        ),
    )
    .expect("write independent C# source");

    let build = Command::new("dotnet")
        .args(["build", "--configuration", "Release", "--nologo"])
        .current_dir(&project)
        .output()
        .expect("run dotnet build for independent C# assembly");
    assert!(
        build.status.success(),
        "independent C# fixture must compile:\n{}\n{}",
        String::from_utf8_lossy(&build.stdout),
        String::from_utf8_lossy(&build.stderr)
    );

    let user_assembly = project
        .join("bin")
        .join("Release")
        .join("net10.0")
        .join(format!("{USER_ASSEMBLY_NAME}.dll"));
    let user_image = fs::read(&user_assembly).expect("read independently compiled C# assembly");
    let method = resolve_public_static_method(&user_image, USER_NAMESPACE, USER_TYPE, USER_METHOD)
        .expect(
            "FerrumWeave should resolve the user-defined public static method from CLR metadata",
        );
    assert_eq!(method.namespace, USER_NAMESPACE);
    assert_eq!(method.type_name, USER_TYPE);
    assert_eq!(method.method_name, USER_METHOD);

    let probe_dir = root.join("probe");
    fs::create_dir_all(&probe_dir).expect("create probe directory");
    fs::write(
        probe_dir.join(PROBE_ASSEMBLY_FILE),
        emit_probe_assembly_with_external_static_call(
            USER_ASSEMBLY_NAME,
            USER_NAMESPACE,
            USER_TYPE,
            USER_METHOD,
        ),
    )
    .expect("write Rust-emitted managed consumer probe");
    fs::write(
        probe_dir.join(PROBE_RUNTIME_CONFIG_FILE),
        probe_runtime_config(),
    )
    .expect("write probe runtime config");
    fs::copy(
        &user_assembly,
        probe_dir.join(format!("{USER_ASSEMBLY_NAME}.dll")),
    )
    .expect("place independently compiled C# assembly beside consumer probe");

    let run = Command::new("dotnet")
        .arg(PROBE_ASSEMBLY_FILE)
        .current_dir(&probe_dir)
        .output()
        .expect("execute Rust-emitted consumer probe on CoreCLR");
    assert!(
        run.status.success(),
        "managed consumer probe must execute:\n{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert_eq!(String::from_utf8_lossy(&run.stdout).trim(), USER_GREETING);

    let _ = fs::remove_dir_all(root);
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r05-user-assembly-{}-{nonce}",
        std::process::id()
    ))
}
