use std::fs;

fn workflow(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn historical_upstream_workflows_are_explicitly_oracle_characterization() {
    let cases = [
        (
            ".github/workflows/r02-integration.yml",
            "name: R02 Oracle Characterization — Rust to CLR",
            "name: characterize upstream Rust to CLR",
            "Certify R02 contracts",
        ),
        (
            ".github/workflows/r03-integration.yml",
            "name: R03 Oracle Characterization — Core Rust Semantics",
            "name: characterize upstream semantic slice",
            "Certify R03 semantic slice",
        ),
    ];

    for (path, workflow_name, characterization_step, forbidden_certification_step) in cases {
        let text = workflow(path);

        assert!(
            text.contains("repository: FractalFir/rustc_codegen_clr"),
            "{path} must remain an explicit pinned rustc_codegen_clr oracle lane"
        );
        assert!(
            text.contains(workflow_name),
            "{path} must identify itself as Oracle Characterization in GitHub Actions UI"
        );
        assert!(
            text.contains(characterization_step),
            "{path} must describe upstream execution as characterization, not product certification"
        );
        assert!(
            !text.contains(forbidden_certification_step),
            "{path} must not label rustc_codegen_clr execution as FerrumWeave certification"
        );
    }
}
