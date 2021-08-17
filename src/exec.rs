use crate::parse::parse_expression;
use crate::token::pop_token;
use crate::*;
use crate::{define_builtins, evaluate, parse, Expression, Scope};
use std::cell::RefCell;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, Write};
use std::rc::Rc;

pub fn exec_file(filename: &str) -> io::Result<()> {
    let scope = Rc::new(RefCell::new(Scope::new()));
    define_builtins(scope.clone());

    let mut file = File::open(filename)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    while let (Some(_), _) = pop_token(&contents) {
        let (result, remainder) = parse_expression(&contents);
        contents = remainder;
        match result {
            Ok(input_expression) => {
                if input_expression == void!() {
                    return Ok(());
                }
                let evaluation = match evaluate(&input_expression, scope.clone()) {
                    Ok(expression) => expression,
                    Err(err) => {
                        println!("Error evaluating input: {}", err);
                        return Ok(());
                    }
                };
                if evaluation != Expression::Void {
                    println!("{}", evaluation.outer_representation());
                }
            }
            Err(err) => {
                println!("Error parsing input: {:?}", err);
                return Ok(());
            }
        }
    }
    Ok(())
}

pub fn repl() -> io::Result<()> {
    let scope = Rc::new(RefCell::new(Scope::new()));
    define_builtins(scope.clone());
    loop {
        let mut input = String::new();
        print!("> ");
        io::stdout().flush()?;
        if io::stdin().read_line(&mut input)? == 0 {
            return Ok(());
        }

        let input_expression = match parse(&input) {
            Ok(expression) => expression,
            Err(err) => {
                println!("Error parsing input: {:?}", err);
                continue;
            }
        };

        let evaluation = match evaluate(&input_expression, scope.clone()) {
            Ok(expression) => expression,
            Err(err) => {
                println!("Error evaluating input: {}", err);
                continue;
            }
        };
        if evaluation != Expression::Void {
            println!("{}", evaluation.outer_representation());
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_exec_file() {
        exec_file("test/src/mergesort.lisp").unwrap();
    }
}
