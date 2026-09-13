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
fn legacy_source_parser_emitter_is_absent_from_product_source_tree() {
    let legacy_emitter = repo_root().join("compiler/cil/src/bin/ferrumweave_emit.rs");

    assert!(
        !legacy_emitter.exists(),
        "legacy source-parser emitter must not remain in the compiler/cil product source tree; Git history preserves its provenance"
    );
}

#[test]
fn legacy_hardcoded_r08_emitter_is_absent_from_product_source_tree() {
    let legacy_emitter = repo_root().join("compiler/cil/src/bin/r08_exec.rs");

    assert!(
        !legacy_emitter.exists(),
        "legacy hard-coded R08 PE/CIL emitter must not remain in the compiler/cil product source tree; Git history preserves its provenance"
    );
}

#[test]
fn clr_probe_remains_the_only_registered_compiler_cil_binary() {
    let manifest_path = repo_root().join("compiler/cil/Cargo.toml");
    let manifest = fs::read_to_string(&manifest_path)
        .unwrap_or_else(|error| panic!("failed to read {}: {error}", manifest_path.display()));

    assert!(
        manifest.contains("autobins = false"),
        "compiler/cil must keep automatic binary discovery disabled"
    );
    assert!(
        manifest.contains("name = \"clr_probe\""),
        "compiler/cil must preserve the intentional CLR probe binary"
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
