use std::fs;
use std::path::PathBuf;

fn repository_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn console_template_keeps_real_rust_main_and_executable_project_shape() {
    let root = repository_root();
    let source = fs::read_to_string(root.join("sdk/templates/console/src/main.rs"))
        .expect("console Rust source should be readable");
    let project = fs::read_to_string(root.join("sdk/templates/console/HelloFerrum.rsproj"))
        .expect("console project should be readable");
    let targets = fs::read_to_string(root.join("sdk/FerrumWeave.Sdk/Sdk/Sdk.targets"))
        .expect("FerrumWeave SDK targets should be readable");

    assert!(source.contains("fn main()"), "console source must keep an ordinary Rust main");
    assert!(project.contains("<OutputType>Exe</OutputType>"));
    assert!(
        targets.contains("<FerrumWeaveRustCrateType Condition=\"'$(OutputType)' == 'Exe'\">bin</FerrumWeaveRustCrateType>"),
        "executable .rsproj must reach rustc as a bin crate"
    );
    assert!(
        targets.contains("--crate-type $(FerrumWeaveRustCrateType)"),
        "CoreCompile must use the SDK-selected Rust crate type"
    );
}
