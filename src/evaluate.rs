use crate::*;

#[derive(Debug, Eq, PartialEq)]
pub enum EvaluationError {
  UnknownFunctionName,
  WrongNumberOfArguments,
  InvalidArgument,
}
pub type EvaluationResult = Result<Expression, EvaluationError>;

// Some helpers to cut down on the boilerplate
impl Expression {
  fn car(&self) -> EvaluationResult {
    if let Expression::Cons(cons) = self {
      return Ok(cons.car.as_ref().clone());
    } else {
      return Err(EvaluationError::WrongNumberOfArguments);
    }
  }
  fn cdr(&self) -> EvaluationResult {
    if let Expression::Cons(cons) = self {
      return Ok(cons.cdr.as_ref().clone());
    } else {
      return Err(EvaluationError::WrongNumberOfArguments);
    }
  }
  fn assert_empty(&self) -> Result<(), EvaluationError> {
    if self == &atom!("nil") {
      return Ok(());
    }
    return Err(EvaluationError::WrongNumberOfArguments);
  }
}

fn _evaluate_eq(expression: &Expression) -> EvaluationResult {
  let a = evaluate(&expression.car()?)?;
  let b = evaluate(&expression.cdr()?.car()?)?;
  expression.cdr()?.cdr()?.assert_empty()?;

  if a == b {
    Ok(atom!("true"))
  } else {
    Ok(atom!("false"))
  }
}

fn _evaluate_quote(expression: &Expression) -> EvaluationResult {
  if let Expression::Cons(cons) = expression {
    if cons.cdr.as_ref() == &atom!("nil") {
      return Ok(cons.car.as_ref().clone());
    }
  }
  Err(EvaluationError::WrongNumberOfArguments)
}

fn _evaluate_cons(expression: &Expression) -> EvaluationResult {
  let a = evaluate(&expression.car()?)?;
  let b = evaluate(&expression.cdr()?.car()?)?;
  expression.cdr()?.cdr()?.assert_empty()?;

  Ok(cons!(&a, &b))
}

fn _evaluate_car(expression: &Expression) -> EvaluationResult {
  let cons = evaluate(&expression.car()?)?;
  expression.cdr()?.assert_empty()?;
  if let Expression::Cons(cons) = cons {
    Ok(cons.car.as_ref().clone())
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

fn _evaluate_cdr(expression: &Expression) -> EvaluationResult {
  let cons = evaluate(&expression.car()?)?;
  expression.cdr()?.assert_empty()?;
  if let Expression::Cons(cons) = cons {
    Ok(cons.cdr.as_ref().clone())
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

fn _evaluate(function_name: &Atom, expression: &Expression) -> EvaluationResult {
  match &function_name.string as &str {
    "eq?" => _evaluate_eq(expression),
    "quote" => _evaluate_quote(expression),
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
  fn test_evaluate_eq() {
    assert_eq!(evaluate(&parse("(eq? 1 1)").unwrap()), Ok(atom!("true")));
    assert_eq!(
      evaluate(&parse("(eq? foo foo)").unwrap()),
      Ok(atom!("true"))
    );
    assert_eq!(
      evaluate(&parse("(eq? foo bar)").unwrap()),
      Ok(atom!("false"))
    );
    assert_eq!(
      evaluate(&parse("(eq? (eq? 1 1) true)").unwrap()),
      Ok(atom!("true"))
    );
  }

  #[test]
  fn test_quote() {
    assert_eq!(evaluate(&parse("'foo").unwrap()), Ok(atom!("foo"),));
    assert_eq!(evaluate(&parse("'(foo)").unwrap()), Ok(list!(atom!("foo"))));
    assert_eq!(
      evaluate(&parse("(eq? (eq? 1 1) (eq? 1 1))").unwrap()),
      Ok(atom!("true"))
    );
    assert_eq!(
      evaluate(&parse("(eq? '(eq? 1 1) (eq? 1 1))").unwrap()),
      Ok(atom!("false"))
    );
    assert_eq!(
      evaluate(&parse("(eq? '(a b c) (quote (a b c)))").unwrap()),
      Ok(atom!("true"))
    );
  }

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
