#[no_mangle]
pub extern "C" fn answer() -> i32 {
    0
}

fn main() {
    std::process::exit(answer())
}
