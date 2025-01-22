use std::io::{self, Write};
use std::collections::HashMap;
use crate::interpreter::interpreter::execute;
use crate::parser::parser::*;
use crate::interpreter::interpreter::eval;
use crate::ir::ast::Expression;
use std::process::Command;

pub fn repl() -> io::Result<()> {
    // Print welcome message
    println!("R-Python REPL");
    println!("Type 'exit()' to quit\n");
    let mut current_env = HashMap::new();

    loop {
        // Display prompt
        print!("R-Python >>> ");
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
        
        // Reset the output
        let mut output: Result::<String, String> = Ok(format!(""));

        // Parsing of expressions
        match expression(input) {
            Ok(("", _expr)) =>{
                // Evaluate the expression
                output = repl_parse_expression(input, &current_env);
            },
            Ok((_, _)) => {
                // Try to parse statements in the input
                match repl_parse_statements(input, current_env.clone()){
                    Ok(new_env) => current_env = new_env,
                    Err(e) => output = Err(e),
                };
            },
            Err(e) => output = Err(e.to_string()),
            
        }
        match output{
            // Prints the output -> if no output -> continue the loop
            Ok(result) => {
                if !result.is_empty() {
                    println!("{}", result);
                }
                else {
                    continue
                }
            }
            Err(e) => println!("Sintax Error: {}", e),
        }
    }
    Ok(())
}

fn repl_parse_expression(input: &str, current_env: &HashMap<String, Expression>) -> Result<String, String>{
    // Parse the input as an expression
    match expression(input) {
        Ok(("", expr)) =>{
            // Evaluate the expression
            match eval(expr, &current_env.clone()) {
                Ok(evaluated_expression) => {
                    match evaluated_expression{
                        Expression::CInt(val) => Ok(val.to_string()),
                        Expression::CReal(val) => Ok(val.to_string()),
                        Expression::CString(string) => Ok(string),
                        Expression::CTrue => Ok(String::from("True")),
                        Expression::CFalse => Ok(String::from("False")),
                        _ => Err(format!("NonExistent Type")),
                    }
                },
                Err(e) => Err(format!("Evaluation Error: {}", e)),
            }
        },
        Ok((_, _)) => Err(format!("Parsing Expression Error")),
        Err(_)=> Err(format!("Parsing Expression Error"))
    }
}

fn repl_parse_statements(input: &str, mut current_env: HashMap<String, Expression>) -> Result<HashMap<String, Expression>, String> {
    // Parse the input as a statement
    match parse(input) {
        Ok((remaining, statements)) => {
            if !remaining.is_empty() {
                return Err(format!("Warning: Unparsed input remains: {:?}\n", remaining));
            }

            for stmt in statements {
                match execute(stmt, current_env.clone()) {
                    Ok(new_env) => current_env = new_env,
                    Err(e) => return Err(format!("Execution Error: {}", e)),
                }    
            }
            Ok(current_env.clone())
        }
        Err(e) => Err(format!("Statement Parse Error: {}", e)),
    }
}

