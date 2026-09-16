use std::fs;

const REPOSITORY_LAYOUT: &str = "docs/architecture/repository-layout.md";

#[test]
fn repository_layout_names_ferrumweave_owned_codegen_backend() {
    let layout = fs::read_to_string(REPOSITORY_LAYOUT)
        .expect("repository layout documentation must remain readable");

    assert!(
        layout.contains("codegen-backend"),
        "repository layout must name the FerrumWeave-owned rustc CodegenBackend boundary"
    );
    assert!(
        !layout.contains("compiler/\n│   ├── codegen-clr/"),
        "repository target structure must not retain the legacy codegen-clr product slot"
    );
}
