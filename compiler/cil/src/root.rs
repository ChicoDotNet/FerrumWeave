#![forbid(unsafe_code)]

#[path = "wrapper.rs"]
mod legacy;
pub use legacy::*;

mod scalar_export;
pub use scalar_export::*;

mod argument_export;
pub use argument_export::*;

mod arithmetic_export;
pub use arithmetic_export::*;

mod control_flow_export;
pub use control_flow_export::*;

mod direct_call_export;
pub use direct_call_export::*;

mod managed_static_export;
pub use managed_static_export::*;

mod r06;
pub use r06::*;

mod r07;
pub use r07::*;
