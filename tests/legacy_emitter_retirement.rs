#![forbid(unsafe_code)]

use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn legacy_source_parser_emitter_is_not_a_registered_product_binary() {
    let manifest_path = repo_root().join("compiler/cil/Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", manifest_path.display()));

    assert!(
        !manifest.contains("name = \"ferrumweave_emit\""),
        "legacy source-parser emitter must not remain a registered compiler/cil binary"
    );
}

#[test]
fn sdk_product_path_invokes_rustc_backend_and_not_legacy_emitter() {
    let targets_path = repo_root().join("sdk/FerrumWeave.Sdk/Sdk/Sdk.targets");
    let targets = fs::read_to_string(&targets_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", targets_path.display()));

    assert!(
        targets.contains("-Z codegen-backend=&quot;$(FerrumWeaveCodegenBackendPath)&quot;"),
        "FerrumWeave.Sdk must invoke rustc with the FerrumWeave CodegenBackend"
    );
    assert!(
        !targets.contains("ferrumweave_emit"),
        "FerrumWeave.Sdk must not invoke the legacy source-parser emitter"
    );
}
