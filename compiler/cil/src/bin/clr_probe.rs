#![forbid(unsafe_code)]

use std::env;
use std::path::PathBuf;
use std::process::ExitCode;

fn main() -> ExitCode {
    let output_dir = env::args_os()
        .nth(1)
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("target/ferrumweave/r01"));

    match ferrumweave_cil::write_probe_artifacts(&output_dir) {
        Ok(paths) => {
            println!("{}", paths.assembly.display());
            ExitCode::SUCCESS
        }
        Err(error) => {
            eprintln!("failed to emit R01 CLR probe: {error}");
            ExitCode::FAILURE
        }
    }
}
