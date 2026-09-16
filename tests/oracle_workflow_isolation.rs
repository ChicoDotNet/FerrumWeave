use std::fs;
use std::path::PathBuf;

#[test]
fn historical_upstream_oracle_lanes_are_retired_from_active_ci() {
    let repo = PathBuf::from(env!("CARGO_MANIFEST_DIR"));

    for retired_path in [
        ".github/workflows/r02-integration.yml",
        ".github/workflows/r03-integration.yml",
        ".github/workflows/managed-consumption-causality.yml",
        "eng/r02/verify.py",
        "eng/r03/verify.py",
        "eng/verification/verify_managed_static_call_causality.py",
    ] {
        assert!(
            !repo.join(retired_path).exists(),
            "retired upstream oracle lane must not return to active CI without an explicit evidence-lifecycle decision: {retired_path}"
        );
    }

    let upstream = fs::read_to_string(repo.join("docs/upstream/rustc_codegen_clr.md"))
        .expect("upstream oracle history must remain documented after executable retirement");
    assert!(upstream.contains("historical **characterization oracle / differential reference**"));
    assert!(upstream.contains("retired from active CI"));
    assert!(upstream.contains("fixtures and contract ledgers remain"));

    let inventory = fs::read_to_string(repo.join("docs/quality/r09-python-to-rust-inventory.md"))
        .expect("R09 migration inventory must remain readable");
    for retired_entry in [
        "retired `eng/r02/verify.py`",
        "retired `eng/r03/verify.py`",
        "retired `eng/verification/verify_managed_static_call_causality.py`",
    ] {
        assert!(
            inventory.contains(retired_entry),
            "migration inventory must preserve the retired oracle history: {retired_entry}"
        );
    }
}
