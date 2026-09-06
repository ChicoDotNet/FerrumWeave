use ferrumweave_projection_types::{RustScalar, direct_cts_mapping, utf16_to_rust_string};

#[test]
fn rust_char_is_rejected_as_a_direct_system_char_projection_with_a_stable_diagnostic() {
    let error = direct_cts_mapping(RustScalar::Char)
        .expect_err("Rust char must not be silently narrowed to System.Char");

    assert_eq!(error.rust, RustScalar::Char);
    assert_eq!(
        error.reason,
        "Rust char is a Unicode scalar value while System.Char is one UTF-16 code unit; a direct mapping would be lossy"
    );
}

#[test]
fn invalid_clr_utf16_is_rejected_instead_of_being_lossily_replaced() {
    let error = utf16_to_rust_string(&[0xD800])
        .expect_err("an unpaired CLR UTF-16 surrogate must fail explicitly");

    assert_eq!(
        error.reason,
        "System.String contains an unpaired UTF-16 surrogate that cannot be represented by Rust String"
    );
}
