#[inline(never)]
fn ferrumweave_system_math_abs(value: i32) -> i32 {
    value
}

#[no_mangle]
pub extern "C" fn answer() -> i32 {
    ferrumweave_system_math_abs(42)
}
