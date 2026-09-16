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

#[test]
fn repository_layout_keeps_rustc_codegen_clr_out_of_product_upstream_lifecycle() {
    let layout = fs::read_to_string(REPOSITORY_LAYOUT)
        .expect("repository layout documentation must remain readable");

    assert!(
        layout.contains("characterization oracle") || layout.contains("differential oracle"),
        "repository layout must classify rustc_codegen_clr as an oracle rather than product upstream"
    );
    assert!(
        !layout.contains("local divergence from `rustc`, `rust-analyzer`, `rustc_codegen_clr`"),
        "rustc_codegen_clr must not remain in the product upstream-divergence lifecycle"
    );
}
