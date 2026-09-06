#![forbid(unsafe_code)]

#[path = "wrapper.rs"]
mod legacy;
pub use legacy::*;

mod r06;
pub use r06::*;
