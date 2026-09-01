#![cfg_attr(
    ferrumweave_clr,
    feature(adt_const_params, core_intrinsics, unsized_const_params)
)]
#![cfg_attr(
    ferrumweave_clr,
    allow(incomplete_features, internal_features)
)]

// R03 S01 semantic fixture.
//
// The computation below is ordinary safe Rust. Only the observation function
// differs by backend: native Rust prints with std, while the CLR build uses the
// pinned rustc_codegen_clr managed-call marker. The verifier requires both
// builds of this same source file to produce identical output.

#[cfg(ferrumweave_clr)]
#[inline(never)]
fn rustc_clr_interop_managed_call1_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
    const METHOD: &'static str,
    const IS_STATIC: bool,
    Ret,
    Arg1,
>(arg1: Arg1) -> Ret {
    let _ = arg1;
    core::intrinsics::abort();
}

#[cfg(ferrumweave_clr)]
fn observe(value: i32) {
    rustc_clr_interop_managed_call1_::<
        "System.Console",
        "System.Console",
        false,
        "WriteLine",
        true,
        (),
        i32,
    >(value);
}

#[cfg(not(ferrumweave_clr))]
fn observe(value: i32) {
    println!("{value}");
}

fn primitives_and_locals() -> i32 {
    let base: i32 = 40;
    let enabled: bool = true;
    if enabled { base + 2 } else { 0 }
}

fn assignment() -> i32 {
    let mut value: i32 = 10;
    value = value + 5;
    value *= 2;
    value
}

fn arithmetic_and_comparison() -> i32 {
    let answer = 6 * 7;
    if answer == 42 && answer > 40 { 1 } else { 0 }
}

fn add(left: i32, right: i32) -> i32 {
    left + right
}

fn function_call() -> i32 {
    add(19, 23)
}

fn conditional(score: i32) -> i32 {
    if score >= 42 { 7 } else { 3 }
}

fn main() {
    observe(primitives_and_locals());
    observe(assignment());
    observe(arithmetic_and_comparison());
    observe(function_call());
    observe(conditional(function_call()));
}
