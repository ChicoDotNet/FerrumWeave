#![forbid(unsafe_code)]

use std::error::Error;
use std::fs;
use std::path::PathBuf;

use ferrumweave_cil::emit_r06_static_api_assembly;

fn main() -> Result<(), Box<dyn Error>> {
    let output = std::env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .ok_or("expected output assembly path")?;

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(output, emit_r06_static_api_assembly())?;
    Ok(())
}
