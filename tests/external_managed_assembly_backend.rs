mod support;

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use support::{
    assert_success, build_codegen_backend, compile_rust_source, create_work_dir,
    make_csharp_consumer, remove_work_dir, run_managed_consumer,
};

const EXTERNAL_ASSEMBLY: &str = "External.Managed.dll";

#[test]
fn external_managed_assembly_is_source_causal_through_the_ferrumweave_backend() {
    let backend = build_codegen_backend();
    let work = create_work_dir("external-managed-assembly");
    let dependency = build_external_dependency(&work);

    let payload_137 = compile_variant(&backend, &work, "payload_137", 137, "answer");
    let payload_211 = compile_variant(&backend, &work, "payload_211", 211, "answer");
    let renamed = compile_variant(&backend, &work, "renamed_export", 137, "compute_result");

    assert_external_reference(&payload_137, "Answer");
    assert_external_reference(&payload_211, "Answer");
    assert_external_reference(&renamed, "ComputeResult");

    assert_eq!(
        run_external_consumer(&payload_137, &dependency, &work, "payload_137", "Answer"),
        1137
    );
    assert_eq!(
        run_external_consumer(&payload_211, &dependency, &work, "payload_211", "Answer"),
        1211
    );
    assert_eq!(
        run_external_consumer(
            &renamed,
            &dependency,
            &work,
            "renamed_export",
            "ComputeResult",
        ),
        1137
    );

    let baseline =
        fs::read(&payload_137).expect("baseline external-managed artifact should be readable");
    assert_ne!(
        baseline,
        fs::read(&payload_211)
            .expect("payload-mutated external-managed artifact should be readable"),
        "changing only Rust payload 137 -> 211 must change the external-managed artifact"
    );
    assert_ne!(
        baseline,
        fs::read(&renamed).expect("renamed external-managed artifact should be readable"),
        "changing only Rust export answer -> compute_result must change managed metadata"
    );

    remove_work_dir(&work);
}

fn rust_source(value: i32, export_name: &str) -> String {
    format!(
        "#[inline(never)]\nfn ferrumweave_external_managed_transform(value: i32) -> i32 {{ value }}\n\n#[no_mangle]\npub extern \"C\" fn {export_name}() -> i32 {{ ferrumweave_external_managed_transform({value}) }}\n"
    )
}

fn compile_variant(
    backend: &Path,
    work: &Path,
    name: &str,
    value: i32,
    export_name: &str,
) -> PathBuf {
    compile_rust_source(
        backend,
        work,
        &format!("{name}.rs"),
        &format!("{name}.dll"),
        &rust_source(value, export_name),
        &format!("FerrumWeave should lower external-managed variant {name}"),
    )
}

fn build_external_dependency(root: &Path) -> PathBuf {
    let project = root.join("external");
    fs::create_dir(&project).expect("external dependency project directory should be created");
    fs::write(
        project.join("External.Managed.csproj"),
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup>\n    <TargetFramework>net10.0</TargetFramework>\n    <AssemblyName>External.Managed</AssemblyName>\n    <RootNamespace>ExternalManaged</RootNamespace>\n  </PropertyGroup>\n</Project>\n",
    )
    .expect("external dependency project should be written");
    fs::write(
        project.join("ExternalApi.cs"),
        "namespace ExternalManaged;\n\npublic static class ExternalApi\n{\n    public static int Transform(int value) => value + 1000;\n}\n",
    )
    .expect("external dependency source should be written");

    let output = Command::new("dotnet")
        .args(["build"])
        .arg(project.join("External.Managed.csproj"))
        .args(["-c", "Release", "--nologo"])
        .env("DOTNET_NOLOGO", "1")
        .env("DOTNET_CLI_TELEMETRY_OPTOUT", "1")
        .env("DOTNET_SKIP_FIRST_TIME_EXPERIENCE", "1")
        .output()
        .expect("dotnet should start the independent external dependency build");
    assert_success(
        &output,
        "independent external managed assembly should build",
    );

    let assembly = project
        .join("bin")
        .join("Release")
        .join("net10.0")
        .join(EXTERNAL_ASSEMBLY);
    assert!(
        assembly.is_file(),
        "independent external managed assembly should exist at {}",
        assembly.display()
    );
    assembly
}

fn assert_external_reference(artifact: &Path, method_name: &str) {
    let image = fs::read(artifact).expect("external-managed artifact should be readable");
    for expected in [
        "MZ",
        "BSJB",
        "FerrumWeave.Generated",
        "RustApi",
        method_name,
        "External.Managed",
        "ExternalManaged",
        "ExternalApi",
        "Transform",
    ] {
        assert!(
            image
                .windows(expected.len())
                .any(|window| window == expected.as_bytes()),
            "external-managed artifact is missing {expected:?}"
        );
    }
}

fn run_external_consumer(
    artifact: &Path,
    dependency: &Path,
    root: &Path,
    label: &str,
    method_name: &str,
) -> i32 {
    let program = format!(
        "using System.Reflection;\nvar method = typeof(FerrumWeave.RustApi).GetMethod(\"{method_name}\", BindingFlags.Public | BindingFlags.Static)!;\nvar il = method.GetMethodBody()!.GetILAsByteArray()!;\nif (System.Array.IndexOf(il, (byte)0x28) < 0) throw new Exception(\"{method_name} must contain a managed call opcode\");\nConsole.WriteLine(FerrumWeave.RustApi.{method_name}());\n"
    );
    let project = make_csharp_consumer(artifact, root, label, &program);
    let consumer = project
        .parent()
        .expect("generated consumer project should have a parent directory");
    fs::copy(dependency, consumer.join(EXTERNAL_ASSEMBLY))
        .expect("external dependency should be copied into the consumer");
    fs::write(
        &project,
        "<Project Sdk=\"Microsoft.NET.Sdk\">\n  <PropertyGroup><OutputType>Exe</OutputType><TargetFramework>net10.0</TargetFramework><ImplicitUsings>enable</ImplicitUsings></PropertyGroup>\n  <ItemGroup>\n    <Reference Include=\"FerrumWeave.Generated\"><HintPath>FerrumWeave.Generated.dll</HintPath><Private>true</Private></Reference>\n    <Reference Include=\"External.Managed\"><HintPath>External.Managed.dll</HintPath><Private>true</Private></Reference>\n  </ItemGroup>\n</Project>\n",
    )
    .expect("external-managed consumer project should include the external dependency");

    let output = run_managed_consumer(&project, &format!("external-managed consumer for {label}"));
    assert_success(
        &output,
        &format!("external-managed consumer should pass for {label}"),
    );

    let stdout = String::from_utf8(output.stdout).expect("consumer stdout should be UTF-8");
    let observed = stdout
        .lines()
        .rfind(|line| !line.trim().is_empty())
        .expect("external-managed consumer should produce an observable")
        .trim();
    observed.parse::<i32>().unwrap_or_else(|error| {
        panic!("unexpected external-managed observable for {label}: {observed:?}: {error}")
    })
}
