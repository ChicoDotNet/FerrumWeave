use std::fs;

#[test]
fn backend_convergence_tracks_all_backend_evidence_inputs() {
    let path = ".github/workflows/codegen-backend-convergence.yml";
    let text =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"));

    for required_path in [
        "compiler/codegen-backend/**",
        "compiler/cil/**",
        "projection/**",
        "sdk/**",
        "eng/verification/**",
        "tests/r05/**",
        "tests/**",
        "docs/architecture/**",
    ] {
        let quoted = format!("\"{required_path}\"");
        assert!(
            text.contains(&quoted),
            "{path} must trigger for {required_path} so backend evidence cannot change without convergence replay"
        );
    }
}
