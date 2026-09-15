use std::fs;

#[test]
fn rust_export_selection_is_not_owned_by_a_fixture_literal() {
    let lowering = fs::read_to_string("compiler/codegen-backend/src/lowering.rs")
        .expect("read FerrumWeave MIR lowering");

    assert!(
        lowering.contains("tcx.symbol_name(instance).name"),
        "FerrumWeave must derive export identity from rustc-owned symbol information"
    );
    assert!(
        !lowering.contains("const EXPORT_SYMBOL: &str = \"answer\""),
        "FerrumWeave product lowering still selects the Rust export through the fixture-specific `answer` literal"
    );
}
