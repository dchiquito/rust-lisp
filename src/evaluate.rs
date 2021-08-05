mod comparison;
mod quote;

use crate::*;

#[derive(Debug, Eq, PartialEq)]
pub enum EvaluationError {
  UnknownFunctionName,
  WrongNumberOfArguments,
  InvalidArgument,
}
pub type EvaluationResult = Result<Expression, EvaluationError>;

// Some helpers to cut down on the boilerplate
fn arg_length(mut expression: &Expression) -> Result<usize, EvaluationError> {
  let mut length = 0;
  while let Expression::Cons(cons) = expression {
    length += 1;
    expression = cons.cdr.as_ref();
  }
  if expression != &atom!("nil") {
    Err(EvaluationError::InvalidArgument)
  } else {
    Ok(length)
  }
}
fn assert_arg_length(
  expression: &Expression,
  expected_length: usize,
) -> Result<(), EvaluationError> {
  if arg_length(expression)? != expected_length {
    Err(EvaluationError::WrongNumberOfArguments)
  } else {
    Ok(())
  }
}
fn arg_get(expression: &Expression, index: usize) -> EvaluationResult {
  if let Expression::Cons(cons) = expression {
    if index == 0 {
      Ok(cons.car.as_ref().clone())
    } else {
      arg_get(cons.cdr.as_ref(), index - 1)
    }
  } else {
    Err(EvaluationError::WrongNumberOfArguments)
  }
}

fn _evaluate_cons(expression: &Expression) -> EvaluationResult {
  assert_arg_length(expression, 2)?;
  let a = evaluate(&arg_get(expression, 0)?)?;
  let b = evaluate(&arg_get(expression, 1)?)?;

  Ok(cons!(&a, &b))
}

fn _evaluate_car(expression: &Expression) -> EvaluationResult {
  assert_arg_length(expression, 1)?;
  let cons = evaluate(&arg_get(expression, 0)?)?;
  if let Expression::Cons(cons) = cons {
    Ok(cons.car.as_ref().clone())
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

fn _evaluate_cdr(expression: &Expression) -> EvaluationResult {
  assert_arg_length(expression, 1)?;
  let cons = evaluate(&arg_get(expression, 0)?)?;
  if let Expression::Cons(cons) = cons {
    Ok(cons.cdr.as_ref().clone())
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

fn _evaluate(function_name: &Atom, expression: &Expression) -> EvaluationResult {
  match &function_name.string as &str {
    "eq?" => comparison::evaluate_eq(expression),
    "quote" => quote::evaluate_quote(expression),
    "cons" => _evaluate_cons(expression),
    "car" => _evaluate_car(expression),
    "cdr" => _evaluate_cdr(expression),
    _ => Err(EvaluationError::UnknownFunctionName),
  }
}

pub fn evaluate(expression: &Expression) -> EvaluationResult {
  match expression {
    Expression::Atom(atom) => Ok(Expression::Atom(atom.clone())),
    Expression::Cons(cons) => match cons.car.as_ref() {
      Expression::Cons(_) => Err(EvaluationError::WrongNumberOfArguments),
      Expression::Atom(function_name) => _evaluate(function_name, cons.cdr.as_ref()),
    },
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_cons() {
    assert_eq!(
      evaluate(&parse("(cons 1 2)").unwrap()),
      Ok(cons!(&atom!("1"), &atom!("2")))
    );
    assert_eq!(
      evaluate(&parse("(cons '1 '2)").unwrap()),
      Ok(cons!(&atom!("1"), &atom!("2")))
    );
    assert_eq!(
      evaluate(&parse("(cons (eq? 1 1) (eq? 1 2))").unwrap()),
      Ok(cons!(&atom!("true"), &atom!("false")))
    );
    assert_eq!(
      evaluate(&parse("(cons foo nil)").unwrap()),
      Ok(list!(atom!("foo")))
    );
    assert_eq!(
      evaluate(&parse("(eq? (cons foo nil) '(foo))").unwrap()),
      Ok(atom!("true"))
    );
  }

  #[test]
  fn test_car() {
    assert_eq!(evaluate(&parse("(car '(1))").unwrap()), Ok(atom!("1")));
    assert_eq!(evaluate(&parse("(car '(1 2 3))").unwrap()), Ok(atom!("1")));
    assert_eq!(
      evaluate(&parse("(car (cons foo bar))").unwrap()),
      Ok(atom!("foo"))
    );
  }

  #[test]
  fn test_cdr() {
    assert_eq!(evaluate(&parse("(cdr '(1))").unwrap()), Ok(atom!("nil")));
    assert_eq!(
      evaluate(&parse("(cdr '(1 2 3))").unwrap()),
      Ok(list!(atom!("2"), atom!("3")))
    );
    assert_eq!(
      evaluate(&parse("(cdr (cons foo bar))").unwrap()),
      Ok(atom!("bar"))
    );
  }
}
