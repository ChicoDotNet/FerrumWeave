fn main() {
    let mut value = 41;
    let shared = &value;
    let mutable = &mut value;

    *mutable += 1;
    let _ = shared;
}
