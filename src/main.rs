use crate::interpreter::interpreter::eval;
use crate::interpreter::interpreter::execute;
use crate::ir::ast::Expression;
use crate::parser::parser::*;
use std::collections::HashMap;
use std::io::{self, Write};
use std::process::Command;

pub mod interpreter;
pub mod ir;
pub mod parser;
pub mod tc;

fn main() -> io::Result<()> {
    // Print welcome message
    println!("R-Python REPL");
    println!("Type 'exit()' to quit\n");
    let mut current_env = HashMap::new();

    loop {
        // Display prompt
        print!("RP >>> ");
        io::stdout().flush()?;

        // Read input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();

        // Handle exit condition
        if input == "exit()" {
            break;
        }
        
        // Clear the terminal screen
        if input == "clear" {
            if cfg!(target_os = "windows") {
                Command::new("cmd").arg("/c").arg("cls").status()?;
            } else {
                Command::new("clear").status()?;
            }
            continue;
        }

        // If just enter or spaces continue the loop
        if input == "" {
            continue;
        }

        // Parsing of expressions
        match expression(input) {
            Ok(("", expr)) => {
                // Evaluate the expression
                match eval(expr, &current_env.clone()) {
                    Ok(evaluated_expression) => match evaluated_expression {
                        Expression::CInt(val) => println!("{:?}", val),
                        Expression::CReal(val) => println!("{:?}", val),
                        Expression::CString(string) => println!("{:?}", string),
                        Expression::CTrue => println!("True"),
                        Expression::CFalse => println!("False"),
                        _ => panic!(),
                    },
                    Err(e) => {
                        println!("Execution error: {}", e);
                    }
                }
            }
            // If not expression -> test if statements
            Ok((_, _expr)) => match parse(input) {
                Ok((remaining, statements)) => {
                    if !remaining.is_empty() {
                        println!("Warning: Unparsed input remains: {:?}\n", remaining);
                    }

                    for stmt in statements {
                        match execute(stmt, current_env.clone()) {
                            Ok(new_env) => {
                                current_env = new_env;
                            }
                            Err(e) => {
                                println!("Execution error: {}", e);
                            }
                        }
                    }
                }
                Err(e) => println!("Parse error: {:?}", e),
            },
            Err(_) => println!("Parse error"),
        }
    }
    Ok(())
}
