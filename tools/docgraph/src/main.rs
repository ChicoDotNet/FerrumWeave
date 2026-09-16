use std::env;
use std::path::PathBuf;

use ferrumweave_docgraph::{Mode, process_repository};

fn main() {
    let mut args = env::args().skip(1);
    let mode = match args.next().as_deref() {
        Some("check") => Mode::Check,
        Some("render") => Mode::Render,
        Some(other) => {
            eprintln!(
                "unknown command: {other}\nusage: ferrumweave-docgraph <check|render> [repository-root]"
            );
            std::process::exit(2);
        }
        None => {
            eprintln!("usage: ferrumweave-docgraph <check|render> [repository-root]");
            std::process::exit(2);
        }
    };

    let root = args
        .next()
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    if args.next().is_some() {
        eprintln!(
            "too many arguments\nusage: ferrumweave-docgraph <check|render> [repository-root]"
        );
        std::process::exit(2);
    }

    match process_repository(&root, mode) {
        Ok(stale) if mode == Mode::Check && !stale.is_empty() => {
            eprintln!("docgraph-managed files are stale:");
            for path in stale {
                eprintln!("  {}", path.display());
            }
            eprintln!("run: cargo run -p ferrumweave-docgraph -- render");
            std::process::exit(1);
        }
        Ok(stale) if mode == Mode::Render => {
            for path in stale {
                println!("rendered {}", path.display());
            }
        }
        Ok(_) => println!("docgraph is current"),
        Err(error) => {
            eprintln!("docgraph error: {error}");
            std::process::exit(1);
        }
    }
}
