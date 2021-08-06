use super::*;
use std::cell::RefCell;
use std::rc::Rc;

fn new_procedure(mut formals: &Expression, body: &Expression) -> EvaluationResult {
  let body = arg_vec(body)?;
  match formals {
    Expression::Symbol(symbol) => Ok(procedure!(symbol.clone(), body)),
    Expression::Cons(cons) => {
      let mut args = vec![];
      while let Expression::Cons(cons) = formals {
        if let Expression::Symbol(symbol) = cons.car.as_ref() {
          args.push(symbol.clone());
          formals = cons.cdr.as_ref();
        } else {
          return Err(EvaluationError::InvalidArgument);
        }
      }
      if formals != &null!() {
        // Variable argument forms are encoded using an improper list as the lambda arguments
        if let Expression::Symbol(symbol) = formals {
          Ok(procedure!(args, symbol.clone(), body))
        } else {
          Err(EvaluationError::InvalidArgument)
        }
      } else {
        Ok(procedure!(fixed args, body))
      }
    }
    _ => Err(EvaluationError::InvalidArgument),
  }
}

pub fn evaluate_lambda(expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  if let Expression::Cons(cons) = expression {
    let formals = cons.car.as_ref();
    let body = cons.cdr.as_ref();
    new_procedure(formals, body)
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

pub fn evaluate_procedure(
  procedure: &Procedure,
  arguments: &Expression,
  scope: &mut Scope,
) -> EvaluationResult {
  todo!()
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_lamda() {
    let scope = &mut Scope::new();
    assert_eq!(evaluate(&parse("(car '(1))").unwrap(), scope), Ok(int!(1)));
  }
}
