use crate::repl::repl::repl;
use crate::cli::cli::cli;
use std::io;

pub mod interpreter;
pub mod ir;
pub mod parser;
pub mod repl;
pub mod tc;
pub mod cli;

fn main() -> io::Result<()> {
    cli()
}
