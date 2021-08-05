mod evaluate;
mod parse;
mod scope;
mod token;
mod types;

pub use crate::evaluate::evaluate;
pub use crate::parse::parse;
pub use crate::scope::Scope;
pub use crate::types::*;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let mut scope = Scope::new();
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

        let evaluation = match evaluate(&input_expression, &mut scope) {
            Ok(expression) => expression,
            Err(err) => {
                println!("Error evaluating input: {:?}", err);
                continue;
            }
        };
        println!("{}", evaluation);
    }
}
