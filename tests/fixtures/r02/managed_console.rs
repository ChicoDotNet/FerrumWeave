#![feature(adt_const_params, core_intrinsics, unsized_const_params)]
#![allow(incomplete_features, internal_features)]

// R02 compatibility fixture.
//
// The rustc_clr_interop_* marker names and RustcCLRInteropManagedChar type name
// are part of rustc_codegen_clr's experimental compiler contract. FerrumWeave
// keeps only the minimum surface required for this vertical slice; provenance
// and the pinned upstream revision are documented in docs/upstream/.

#[derive(Clone, Copy)]
struct RustcCLRInteropManagedChar {
    size: u16,
}

#[inline(never)]
fn rustc_clr_interop_managed_call0_<
    const ASSEMBLY: &'static str,
    const CLASS_PATH: &'static str,
    const IS_VALUETYPE: bool,
    const METHOD: &'static str,
    Ret,
>() -> Ret {
    core::intrinsics::abort();
}

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

fn write_char(value: u16) {
    let managed = unsafe { core::mem::transmute::<u16, RustcCLRInteropManagedChar>(value) };
    rustc_clr_interop_managed_call1_::<
        "System.Console",
        "System.Console",
        false,
        "Write",
        true,
        (),
        RustcCLRInteropManagedChar,
    >(managed);
}

fn main() {
    for value in [
        'H', 'e', 'l', 'l', 'o', ' ', 'F', 'e', 'r', 'r', 'u', 'm', 'W', 'e', 'a', 'v', 'e',
    ] {
        write_char(value as u16);
    }

    rustc_clr_interop_managed_call0_::<
        "System.Console",
        "System.Console",
        false,
        "WriteLine",
        (),
    >();
}
