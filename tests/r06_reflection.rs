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
        "ferrumweave-r06-reflection-{}-{nonce}",
        std::process::id(),
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

fn run_reflection_verifier(verifier: &Path, assembly: &Path, shape: &str) -> Output {
    Command::new("dotnet")
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
            shape,
        ])
        .output()
        .expect("run independent CLR reflection verifier")
}

#[test]
fn clr_reflection_observes_rust_source_causal_public_shapes() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let template = repo.join("sdk/templates/rust/HelloFerrum.rsproj");
    let root = unique_temp_dir();
    let rust_project = root.join("rust-source");
    let source_dir = rust_project.join("src");
    let verifier = root.join("reflection-verifier");
    fs::create_dir_all(&source_dir).expect("create R06 Rust reflection source directory");
    fs::create_dir_all(&verifier).expect("create CLR reflection verifier directory");
    fs::copy(template, rust_project.join("RustLibrary.rsproj")).expect("copy canonical rsproj");

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

var assemblyPath = Path.GetFullPath(args[0]);
var shape = args[1];
var assembly = AssemblyLoadContext.Default.LoadFromAssemblyPath(assemblyPath);

if (shape == "static")
{
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
}
else if (shape == "instance")
{
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
}
else
{
    throw new InvalidOperationException($"unknown reflection shape: {shape}");
}

Console.WriteLine($"R06 CLR reflection {shape} contract verified");
"#,
    )
    .expect("write CLR reflection verifier source");

    fs::write(
        source_dir.join("main.rs"),
        "#[no_mangle]\npub extern \"C\" fn answer() -> i32 { 137 }\n",
    )
    .expect("write R06 Rust static reflection source");
    let static_build = dotnet_build(&repo, &rust_project);
    assert!(
        static_build.status.success(),
        "R06 static reflection source must build through .rsproj -> rustc -> FerrumWeave:\n{}\n{}",
        String::from_utf8_lossy(&static_build.stdout),
        String::from_utf8_lossy(&static_build.stderr),
    );
    let assembly = rust_project.join("bin/Debug/net10.0/RustLibrary.dll");
    assert!(assembly.is_file(), "R06 static reflection build did not produce managed DLL");
    let static_bytes = fs::read(&assembly).expect("read Rust-source causal static artifact");
    let static_run = run_reflection_verifier(&verifier, &assembly, "static");
    assert!(
        static_run.status.success(),
        "CLR reflection must accept the Rust-source causal static API:\n{}\n{}",
        String::from_utf8_lossy(&static_run.stdout),
        String::from_utf8_lossy(&static_run.stderr),
    );

    fs::write(
        source_dir.join("main.rs"),
        "pub struct RustValue;\n\nimpl RustValue {\n    pub fn answer(&self) -> i32 { 211 }\n}\n\n#[no_mangle]\npub extern \"C\" fn answer(value: &RustValue) -> i32 { value.answer() }\n",
    )
    .expect("write R06 Rust instance reflection source");
    let instance_build = dotnet_build(&repo, &rust_project);
    assert!(
        instance_build.status.success(),
        "R06 instance reflection source must build through .rsproj -> rustc -> FerrumWeave:\n{}\n{}",
        String::from_utf8_lossy(&instance_build.stdout),
        String::from_utf8_lossy(&instance_build.stderr),
    );
    assert!(assembly.is_file(), "R06 instance reflection build did not produce managed DLL");
    let instance_bytes = fs::read(&assembly).expect("read Rust-source causal instance artifact");
    assert_ne!(
        static_bytes, instance_bytes,
        "changing Rust source from static to instance public shape must change the managed artifact",
    );
    let instance_run = run_reflection_verifier(&verifier, &assembly, "instance");
    assert!(
        instance_run.status.success(),
        "CLR reflection must accept the Rust-source causal instance API:\n{}\n{}",
        String::from_utf8_lossy(&instance_run.stdout),
        String::from_utf8_lossy(&instance_run.stderr),
    );

    let stdout = format!(
        "{}{}",
        String::from_utf8_lossy(&static_run.stdout),
        String::from_utf8_lossy(&instance_run.stdout),
    );
    assert!(stdout.contains("R06 CLR reflection static contract verified"));
    assert!(stdout.contains("R06 CLR reflection instance contract verified"));

    let _ = fs::remove_dir_all(root);
}
