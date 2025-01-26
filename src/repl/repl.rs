use crate::interpreter::interpreter::eval;
use crate::interpreter::interpreter::execute;
use crate::ir::ast::Expression;
use crate::parser::parser::*;
use std::collections::HashMap;
use std::io::{self, Write};
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
        let mut output: Result<String, String> = Ok(format!(""));

        // Parsing of expressions
        match expression(input) {
            Ok(("", _expr)) => {
                // Evaluate the expression
                output = repl_parse_expression(input, &current_env);
            }
            Ok((_, _)) => {
                // Try to parse statements in the input
                match repl_parse_statements(input, current_env.clone()) {
                    Ok(new_env) => current_env = new_env,
                    Err(e) => output = Err(e),
                };
            }
            Err(e) => output = Err(e.to_string()),
        }
        match output {
            // Prints the output -> if no output -> continue the loop
            Ok(result) => {
                if !result.is_empty() {
                    println!("{}", result);
                } else {
                    continue;
                }
            }
            Err(e) => println!("Sintax Error: {}", e),
        }
    }
    Ok(())
}

fn repl_parse_expression(
    input: &str,
    current_env: &HashMap<String, Expression>,
) -> Result<String, String> {
    // Parse the input as an expression
    match expression(input) {
        Ok(("", expr)) => {
            // Evaluate the expression
            match eval(expr, &current_env.clone()) {
                Ok(evaluated_expression) => match evaluated_expression {
                    Expression::CInt(val) => Ok(val.to_string()),
                    Expression::CReal(val) => Ok(val.to_string()),
                    Expression::CString(string) => Ok(string),
                    Expression::CTrue => Ok(String::from("True")),
                    Expression::CFalse => Ok(String::from("False")),
                    _ => Err(format!("NonExistent Type")),
                },
                Err(e) => Err(format!("Evaluation Error: {}", e)),
            }
        }
        Ok((_, _)) => Err(format!("Parsing Expression Error")),
        Err(_) => Err(format!("Parsing Expression Error")),
    }
}

fn repl_parse_statements(
    input: &str,
    mut current_env: HashMap<String, Expression>,
) -> Result<HashMap<String, Expression>, String> {
    // Parse the input as a statement
    match parse(input) {
        Ok((remaining, statements)) => {
            if !remaining.is_empty() {
                return Err(format!(
                    "Warning: Unparsed input remains: {:?}\n",
                    remaining
                ));
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

mod tests {

    use super::*;

    #[test]
    fn test_simple_repl_parse_expression1() {
        let input = "10 + 10";
        let env = HashMap::new();
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("20", result),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_simple_repl_parse_expression2() {
        let input = "(10+100) * (500/100)";
        let env = HashMap::new();
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("550", result),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_simple_repl_parse_expression3() {
        let input = "10-100";
        let env = HashMap::new();
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("-90", result),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_simple_repl_parse_expression4() {
        let input = "90 -                                                           (100)";
        let env = HashMap::new();
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("-10", result),
            Err(e) => panic!("Error: {}", e),
        }
    }


    #[test]
    fn test_simple_repl_parse_expression5() {
        let input = "(-90)*(-20) - (100)";
        let env = HashMap::new();
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("1700", result),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_simple_repl_parse_expression6() {
        let input = "- 90*(- 20) - (200)";
        let env = HashMap::new();
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("1600", result),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_sad_path_repl_parse_expression1() {
        let input = "a + b";
        let env = HashMap::new();
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(_) => panic!("Error was expected"),
            Err(e) => assert_eq!("Evaluation Error: Variable a not found", e),
        }
    }

    #[test]
    fn test_sad_path_repl_parse_expression2() {
        let input = "exit";
        let env = HashMap::new();
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(_) => panic!("Error was expected"),
            Err(e) => assert_eq!("Evaluation Error: Variable exit not found", e),
        }
    }

    #[test]
    fn test_happy_path_repl_parse_expression1() {
        let input = "a + b";
        let mut env = HashMap::new();
        env.insert(String::from("a"), Expression::CInt(10));
        env.insert(String::from("b"), Expression::CInt(20));
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("30", result),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_happy_path_repl_parse_expression2() {
        let input = "a + b";
        let mut env = HashMap::new();
        env.insert(String::from("a"), Expression::CReal(10.0));
        env.insert(String::from("b"), Expression::CInt(20));
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("30", result),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_happy_path_repl_parse_expression3() {
        let input = "a * 500";
        let mut env = HashMap::new();
        env.insert(String::from("a"), Expression::CReal(10.0));
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("5000", result),
            Err(e) => panic!("Error: {}", e),
        }
    }

    #[test]
    fn test_typechecker_sad_path_repl_parse_expression() {
        let input = "a + b";
        let mut env = HashMap::new();
        env.insert(String::from("a"), Expression::CTrue);
        env.insert(String::from("b"), Expression::CInt(20));
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(_) => panic!("Error was expected"),
            Err(e) => assert_eq!(
                "Evaluation Error: addition '(+)' is only defined for numbers (integers and real).",
                e
            ),
        }
    }

    #[test]
    fn test_negative_first_argument() {
        let input = "-10-100";
        let env = HashMap::new();
        let output = repl_parse_expression(input, &env);
        match output {
            Ok(result) => assert_eq!("-110", result),
            Err(e) => panic!("Error: {}", e),
        }
    }


    #[test]
    fn test_repl_parse_assigment1() {
        let input = "a = 10";
        let env = HashMap::new();
        let mut env_expected = HashMap::new();
        env_expected.insert(String::from("a"), Expression::CInt(10));
        let env_output = repl_parse_statements(input, env);
        match env_output {
            Ok(new_env) => assert_eq!(new_env, env_expected),
            Err(_) => panic!("New enviroment was expected"),
        }
    }

    #[test]
    fn test_repl_parse_assigment2() {
        let input = "a = 10 + 30";
        let env = HashMap::new();
        let mut env_expected = HashMap::new();
        env_expected.insert(String::from("a"), Expression::CInt(40));
        let env_output = repl_parse_statements(input, env);
        match env_output {
            Ok(new_env) => assert_eq!(new_env, env_expected),
            Err(_) => panic!("New enviroment was expected"),
        }
    }

    #[test]
    fn test_repl_parse_assigment3() {
        let input = "a = 10 * 30 + 400";
        let env = HashMap::new();
        let mut env_expected = HashMap::new();
        env_expected.insert(String::from("a"), Expression::CInt(700));
        let env_output = repl_parse_statements(input, env);
        match env_output {
            Ok(new_env) => assert_eq!(new_env, env_expected),
            Err(_) => panic!("New enviroment was expected"),
        }
    }

    #[test]
    fn test_repl_parse_assigment4() {
        let input = "a = 10 > 10";
        let env = HashMap::new();
        let mut env_expected = HashMap::new();
        env_expected.insert(String::from("a"), Expression::CFalse);
        let env_output = repl_parse_statements(input, env);
        match env_output {
            Ok(new_env) => assert_eq!(new_env, env_expected),
            Err(_) => panic!("New enviroment was expected"),
        }
    }

    #[test]
    fn test_repl_parse_assigment5() {
        let input = "a = 10 == 10";
        let env = HashMap::new();
        let mut env_expected = HashMap::new();
        env_expected.insert(String::from("a"), Expression::CTrue);
        let env_output = repl_parse_statements(input, env);
        match env_output {
            Ok(new_env) => assert_eq!(new_env, env_expected),
            Err(_) => panic!("New enviroment was expected"),
        }
    }

    #[test]
    fn test_complex_repl_parse_assigment1() {
        // R-Python >> a = 10
        // R-Python >> b = a

        let input = "a = 10";
        let mut env = HashMap::new();
        let mut env_expected = HashMap::new();
        env_expected.insert(String::from("a"), Expression::CInt(10));
        match repl_parse_statements(input, env) {
            Ok(new_env) => {
                assert_eq!(new_env, env_expected);
                env = new_env;
            }
            Err(_) => panic!("New enviroment was expected"),
        }

        let input = "b = a";
        env_expected.insert(String::from("b"), Expression::CInt(10));
        let result = repl_parse_statements(input, env);

        match result {
            Ok(new_env) => assert_eq!(new_env, env_expected),
            Err(_) => panic!("New enviroment was expected"),
        }
    }

    #[test]
    fn test_complex_repl_parse_assigment2() {
        // R-Python >> a = 10
        // R-Python >> b = a

        let input = "a = 10";
        let mut env = HashMap::new();
        let mut env_expected = HashMap::new();
        env_expected.insert(String::from("a"), Expression::CInt(10));
        match repl_parse_statements(input, env) {
            Ok(new_env) => {
                assert_eq!(new_env, env_expected);
                env = new_env;
            }
            Err(_) => panic!("New enviroment was expected"),
        }

        let input = "b = a";
        env_expected.insert(String::from("b"), Expression::CInt(10));
        let result = repl_parse_statements(input, env);

        match result {
            Ok(new_env) => assert_eq!(new_env, env_expected),
            Err(_) => panic!("New enviroment was expected"),
        }
    }
}
