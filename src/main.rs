use cli::cli::cli;
use repl::repl::execute_inline_command;
use std::{io, process};
mod cli;
mod interpreter;
mod ir;
mod parser;
mod repl;
mod tc;
use crate::repl::repl::repl;

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();

    if args.len() > 1 {
        match args[1].as_ref() {
            "-c" => {
                let command = args[2..].join(" ");
                execute_inline_command(&command)?;
            }
            "--exec" => {
                if args.len() >= 3 && args[2].ends_with(".rpy") {
                    let _ = cli(&args[2]);
                } else {
                    eprintln!("Usage: {} {} <file_path>", args[0], args[1]);
                    process::exit(1);
                };
            }
            _ => {}
        };
    } else {
        repl()?;
    }

    Ok(())
}
