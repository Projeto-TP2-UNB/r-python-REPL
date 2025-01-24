use crate::repl::repl::repl;
use std::io;

pub mod interpreter;
pub mod ir;
pub mod parser;
pub mod repl;
pub mod tc;

fn main() -> io::Result<()> {
    repl()
}
