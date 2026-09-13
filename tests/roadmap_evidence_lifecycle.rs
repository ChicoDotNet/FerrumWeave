use std::fs;

#[test]
fn roadmap_separates_oracle_characterization_from_product_backend_evidence() {
    let path = "docs/roadmap/README.md";
    let roadmap =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"));

    for marker in [
        "R02/R03 historical evidence is `oracle-proven`",
        "`rustc_codegen_clr` is not a FerrumWeave runtime, SDK, or product dependency",
        "product capability claims require `FerrumWeave-backend-proven` evidence",
    ] {
        assert!(
            roadmap.contains(marker),
            "roadmap must state evidence lifecycle marker: {marker}"
        );
    }
}
