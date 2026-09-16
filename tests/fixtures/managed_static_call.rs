#![feature(adt_const_params, core_intrinsics, unsized_const_params)]
#![allow(incomplete_features, internal_features)]

// This source is compiled only through the pinned CLR backend in the
// managed-consumption causality certifier. The marker name and const-generic
// shape are part of rustc_codegen_clr's experimental managed-call contract.
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

const PROBE_VALUE: i32 = 137;

fn main() {
    rustc_clr_interop_managed_call1_::<
        "System.Console",
        "System.Console",
        false,
        "WriteLine",
        true,
        (),
        i32,
    >(PROBE_VALUE);
}
