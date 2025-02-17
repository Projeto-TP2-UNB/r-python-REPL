use std::io;
use repl::repl::execute_inline_command;
mod interpreter;
mod ir;
mod parser;
mod repl;
mod tc;
use crate::repl::repl::repl;

fn main() -> io::Result<()> {
    // Get command-line arguments
    let args: Vec<String> = std::env::args().collect();

    // Check if the "-c" flag is provided
    let command = if args.len() > 1 && args[1] == "-c" {
        Some(args[2..].join(" ")) // Join all arguments after "-c" as the command
    } else {
        None
    };

    // Execute the inline command if provided, otherwise start the REPL
    if let Some(command) = command {
        execute_inline_command(&command)?;
    } else {
        repl()?;
    }

    Ok(())
}