use std::collections::HashMap;
use std::env;
use std::fs;
use std::process;

use crate::io;

use crate::interpreter::interpreter::execute;
use crate::parser::parser::*;

use crate::interpreter::interpreter::ControlFlow;
use crate::ir::ast::{EnvValue, Environment, Expression, Statement};
pub fn print_env(env: &Environment) {
    for (key, value) in env {
        println!("{} = {:?}", key, value);
    }
}

pub fn cli(file_path: &String) -> io::Result<Environment> {
    let file_content = fs::read_to_string(file_path).unwrap_or_else(|err| {
        eprintln!("Error reading file {}: {}", file_path, err);
        process::exit(1);
    });

    let (remaining_input, parsed_statements) = parse(&file_content).unwrap_or_else(|err| {
        eprintln!("Error parsing file content: {}", err);
        process::exit(1);
    });

    let mut env: Environment = HashMap::new();

    println!("Environment before execution:");
    print_env(&env);

    for stmt in parsed_statements {
        match execute(stmt, &env, true) {
            Ok(ControlFlow::Continue(new_env)) => {
                env = new_env;
            }
            Ok(ControlFlow::Return(value)) => {
                println!("Execution returned: {:?}", value);
                break;
            }
            Err(err) => {
                eprintln!("Error during execution: {}", err);
                process::exit(1);
            }
        }
    }

    println!("\nEnvironment after execution:");
    print_env(&env);
    Ok(env)
}
