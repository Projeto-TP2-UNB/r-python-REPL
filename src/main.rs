use std::io;
use crate::repl::repl::repl;

pub mod interpreter;
pub mod ir;
pub mod parser;
pub mod tc;
pub mod repl;

fn main() -> io::Result<()> {
    repl()
}
