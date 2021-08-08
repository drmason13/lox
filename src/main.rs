use anyhow::Result;

use lox::Interpreter;

fn show_usage() {
    eprintln!("Usage: lox [script]");
    std::process::exit(64);
}

fn main() -> Result<()> {
    let mut args = std::env::args().skip(1);
    let mut lox = Interpreter::new();

    if let Some(path) = args.next() {
        if args.count() > 0 {
            show_usage();
        };
        lox.run_file(path)?;
    } else {
        lox.run_prompt()?;
    }
    Ok(())
}
