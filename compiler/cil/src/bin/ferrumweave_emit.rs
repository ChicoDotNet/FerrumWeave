#![forbid(unsafe_code)]

mod r08_exec;

use std::error::Error;
use std::fs;
use std::path::PathBuf;

fn main() -> Result<(), Box<dyn Error>> {
    let mut args = std::env::args_os().skip(1);
    let output = args
        .next()
        .map(PathBuf::from)
        .ok_or("expected output assembly path")?;
    let source = args
        .next()
        .map(PathBuf::from)
        .ok_or("expected Rust source path")?;
    let assembly_name = args
        .next()
        .and_then(|value| value.into_string().ok())
        .ok_or("expected assembly name")?;

    let source_text = fs::read_to_string(&source)?;
    let message = parse_single_println(&source_text)
        .ok_or("R08 currently requires main to contain one println!(\"literal\") observable")?;

    if let Some(parent) = output.parent() {
        fs::create_dir_all(parent)?;
    }
    fs::write(
        output,
        r08_exec::emit_console_assembly(&assembly_name, message),
    )?;
    Ok(())
}

fn parse_single_println(source: &str) -> Option<&str> {
    let marker = "println!(\"";
    let start = source.find(marker)? + marker.len();
    let tail = &source[start..];
    let end = tail.find("\");")?;
    let message = &tail[..end];
    if message.contains('\\') || tail[end + 3..].contains(marker) {
        return None;
    }
    Some(message)
}

#[cfg(test)]
mod tests {
    use super::parse_single_println;

    #[test]
    fn extracts_literal_main_observable() {
        assert_eq!(
            parse_single_println("fn main() { println!(\"Hello from FerrumWeave!\"); }"),
            Some("Hello from FerrumWeave!"),
        );
    }

    #[test]
    fn rejects_escaped_or_missing_observable() {
        assert_eq!(parse_single_println("fn main() {}"), None);
        assert_eq!(
            parse_single_println("fn main() { println!(\"a\\nb\"); }"),
            None,
        );
    }
}
