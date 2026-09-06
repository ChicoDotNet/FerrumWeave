use std::fs;
use std::path::PathBuf;
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

use ferrumweave_cil::r06::write_r06_static_api_artifact;

#[test]
fn clr_reflection_observes_coherent_rust_defined_public_api() {
    let root = unique_temp_dir();
    fs::create_dir_all(&root).expect("create R06 reflection fixture directory");

    let assembly = write_r06_static_api_artifact(root.join("managed"))
        .expect("emit Rust-defined R06 managed assembly");

    let verifier = root.join("reflection-verifier");
    fs::create_dir_all(&verifier).expect("create CLR reflection verifier directory");
    fs::write(
        verifier.join("ReflectionVerifier.csproj"),
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
    .expect("write CLR reflection verifier project");

    fs::write(
        verifier.join("Program.cs"),
        r#"using System.Reflection;
using System.Runtime.Loader;

var assemblyPath = Path.GetFullPath(args.Single());
var assembly = AssemblyLoadContext.Default.LoadFromAssemblyPath(assemblyPath);

var rustApi = assembly.GetType("FerrumWeave.RustApi", throwOnError: true)!;
if (!rustApi.IsPublic || !rustApi.IsAbstract || !rustApi.IsSealed)
    throw new InvalidOperationException("RustApi must be a public static-class shape");

var staticAnswer = rustApi.GetMethod(
    "Answer",
    BindingFlags.Public | BindingFlags.Static,
    binder: null,
    types: Type.EmptyTypes,
    modifiers: null) ?? throw new InvalidOperationException("missing public static RustApi.Answer()");
if (staticAnswer.ReturnType != typeof(int))
    throw new InvalidOperationException($"RustApi.Answer return type was {staticAnswer.ReturnType}");

var rustValue = assembly.GetType("FerrumWeave.RustValue", throwOnError: true)!;
if (!rustValue.IsPublic || rustValue.IsAbstract)
    throw new InvalidOperationException("RustValue must be a public constructible type");

var ctor = rustValue.GetConstructor(Type.EmptyTypes)
    ?? throw new InvalidOperationException("missing public RustValue.ctor()");
if (!ctor.IsPublic)
    throw new InvalidOperationException("RustValue.ctor must be public");

var instanceAnswer = rustValue.GetMethod(
    "Answer",
    BindingFlags.Public | BindingFlags.Instance,
    binder: null,
    types: Type.EmptyTypes,
    modifiers: null) ?? throw new InvalidOperationException("missing public instance RustValue.Answer()");
if (instanceAnswer.ReturnType != typeof(int))
    throw new InvalidOperationException($"RustValue.Answer return type was {instanceAnswer.ReturnType}");

Console.WriteLine("R06 CLR reflection contract verified");
"#,
    )
    .expect("write CLR reflection verifier source");

    let run = Command::new("dotnet")
        .args([
            "run",
            "--project",
            verifier
                .join("ReflectionVerifier.csproj")
                .to_str()
                .expect("verifier path is UTF-8"),
            "--configuration",
            "Release",
            "--",
            assembly.to_str().expect("assembly path is UTF-8"),
        ])
        .output()
        .expect("run independent CLR reflection verifier");

    assert!(
        run.status.success(),
        "CLR reflection verifier must accept the Rust-defined API:\n{}\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr)
    );
    assert!(String::from_utf8_lossy(&run.stdout).contains("R06 CLR reflection contract verified"));

    let _ = fs::remove_dir_all(root);
}

fn unique_temp_dir() -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("system clock must be after Unix epoch")
        .as_nanos();
    std::env::temp_dir().join(format!(
        "ferrumweave-r06-reflection-{}-{nonce}",
        std::process::id()
    ))
}
