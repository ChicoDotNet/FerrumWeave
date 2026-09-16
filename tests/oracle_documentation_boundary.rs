use std::fs;

#[test]
fn upstream_documentation_classifies_rustc_codegen_clr_as_oracle_not_product_dependency() {
    let path = "docs/upstream/rustc_codegen_clr.md";
    let upstream =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"));

    for marker in [
        "characterization oracle",
        "not a FerrumWeave runtime, SDK, or product dependency",
        "FerrumWeave `rustc` CodegenBackend",
    ] {
        assert!(
            upstream.contains(marker),
            "upstream documentation must state lifecycle marker: {marker}"
        );
    }

    for stale_claim in [
        "upstream dependency/contribution target",
        "Future milestones must refresh the upstream snapshot",
    ] {
        assert!(
            !upstream.contains(stale_claim),
            "upstream documentation must retire stale product-ownership claim: {stale_claim}"
        );
    }
}

#[test]
fn compatibility_documentation_separates_oracle_history_from_current_product_evidence() {
    let path = "docs/compatibility/README.md";
    let compatibility =
        fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"));

    for marker in [
        "R02/R03 historical evidence is `oracle-proven`",
        "FerrumWeave-backend-proven",
        ".rsproj -> rustc -> FerrumWeave",
        "R05-R09",
    ] {
        assert!(
            compatibility.contains(marker),
            "compatibility documentation must state current evidence marker: {marker}"
        );
    }

    for stale_claim in [
        "R02 proves the first real Rust-language path into the CLR",
        "R03 moves FerrumWeave from the ceremonial R02 slice",
        "- consuming .NET APIs from Rust;",
        "- consuming Rust APIs from C#/VB/F#;",
        "- `.rsproj` / MSBuild / `dotnet new rust`;",
    ] {
        assert!(
            !compatibility.contains(stale_claim),
            "compatibility documentation must retire stale claim: {stale_claim}"
        );
    }
}
