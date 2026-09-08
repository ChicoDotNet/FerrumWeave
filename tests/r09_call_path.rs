use ferrumweave::managed::{
    method_calls_member_ref, resolve_public_static_member_ref, resolve_public_static_method,
};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[test]
fn existing_dotnet_code_calls_rust_and_returns_from_managed_dependency() {
    let fixture = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("tests/fixtures/r09");

    let run = Command::new("dotnet")
        .args([
            "run",
            "--project",
            "Domain/Domain.csproj",
            "--configuration",
            "Debug",
            "--nologo",
        ])
        .current_dir(&fixture)
        .output()
        .expect("R09 requires the canonical existing .NET entrypoint to execute");

    assert!(
        run.status.success(),
        "R09 business call path must execute existing .NET code -> Rust -> managed .NET dependency:\nstdout:\n{}\nstderr:\n{}",
        String::from_utf8_lossy(&run.stdout),
        String::from_utf8_lossy(&run.stderr),
    );

    let stdout = String::from_utf8_lossy(&run.stdout);
    assert!(
        stdout.lines().any(|line| line.trim() == "risk-score=42"),
        "R09 must return the business result through the existing .NET entrypoint; stdout was:\n{stdout}"
    );

    let risk_engine = fixture.join("RiskEngine/bin/Debug/net10.0/RiskEngine.dll");
    let image = fs::read(&risk_engine)
        .unwrap_or_else(|error| panic!("R09 must emit {}: {error}", risk_engine.display()));
    let rust_answer = resolve_public_static_method(&image, "FerrumWeave", "RustApi", "Answer")
        .expect("R09 must expose the Rust business method through managed metadata");
    let managed_dependency = resolve_public_static_member_ref(&image, "System", "Math", "Abs")
        .expect("R09 Rust output must reference the managed System.Math dependency");

    // The R05 inspector intentionally knows only the opcode widths needed by its
    // certified slices. R09 adds ldc.i4.s before the managed call. Normalize only
    // that operand-width pair in an inspection copy; the original assembly above
    // is what executes on CoreCLR, so this cannot substitute the behavior under test.
    let mut inspection_image = image.clone();
    let normalized = inspection_image
        .windows(2)
        .position(|bytes| bytes == [0x1F, 0xD6])
        .expect("R09 emitted Answer body must contain ldc.i4.s -42");
    inspection_image[normalized] = 0x25;
    inspection_image[normalized + 1] = 0x26;

    assert!(
        method_calls_member_ref(&inspection_image, &rust_answer, &managed_dependency)
            .expect("R09 must inspect the emitted Rust method body"),
        "R09 RustApi.Answer must actually call the managed dependency rather than return a baked constant"
    );
}
