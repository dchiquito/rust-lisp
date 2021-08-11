use crate::{Scope, define_builtins, parse, evaluate, Expression};
use std::cell::RefCell;
use std::io::{self, Write};
use std::rc::Rc;

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
                println!("Error evaluating input: {:?}", err);
                continue;
            }
        };
        if evaluation != Expression::Void {
            println!("{}", evaluation.outer_representation());
        }
    }
}