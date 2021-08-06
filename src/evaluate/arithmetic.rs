use super::*;

pub fn evaluate_add(mut expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  // This will implicitly test that there is at least one argument
  arg_get(expression, 0)?;
  let mut sum = 0;
  while let Expression::Cons(cons) = expression {
    if let Expression::Number(Number::Integer(integer)) = evaluate(cons.car.as_ref(), scope)? {
      sum += integer;
    } else {
      return Err(EvaluationError::InvalidArgument);
    }
    expression = cons.cdr.as_ref();
  }
  if expression != &null!() {
    return Err(EvaluationError::InvalidArgument);
  }
  Ok(int!(sum))
}

pub fn evaluate_multiply(mut expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  // This will implicitly test that there is at least one argument
  arg_get(expression, 0)?;
  let mut product = 1;
  while let Expression::Cons(cons) = expression {
    if let Expression::Number(Number::Integer(integer)) = evaluate(cons.car.as_ref(), scope)? {
      product *= integer;
    } else {
      return Err(EvaluationError::InvalidArgument);
    }
    expression = cons.cdr.as_ref();
  }
  if expression != &null!() {
    return Err(EvaluationError::InvalidArgument);
  }
  Ok(int!(product))
}

pub fn evaluate_subtract(expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  if let Expression::Cons(cons) = expression {
    if let Expression::Number(Number::Integer(mut negation)) = evaluate(cons.car.as_ref(), scope)? {
      if cons.cdr.as_ref() == &null!() {
        // When a single argument is passed, it is negated
        return Ok(int!(-negation));
      }

      let mut expression = cons.cdr.as_ref();
      while let Expression::Cons(cons) = expression {
        if let Expression::Number(Number::Integer(integer)) = evaluate(cons.car.as_ref(), scope)? {
          negation -= integer;
        } else {
          return Err(EvaluationError::InvalidArgument);
        }
        expression = cons.cdr.as_ref();
      }
      if expression != &null!() {
        return Err(EvaluationError::InvalidArgument);
      }
      return Ok(int!(negation));
    }
    return Err(EvaluationError::InvalidArgument);
  }
  Err(EvaluationError::WrongNumberOfArguments)
}
pub fn evaluate_divide(expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  if let Expression::Cons(cons) = expression {
    if let Expression::Number(Number::Integer(mut quotient)) = evaluate(cons.car.as_ref(), scope)? {
      if cons.cdr.as_ref() == &null!() {
        // When a single argument is passed, it is negated
        if quotient == 0 {
          return Err(EvaluationError::DivideByZero);
        }
        return Ok(int!(1 / quotient));
      }

      let mut expression = cons.cdr.as_ref();
      while let Expression::Cons(cons) = expression {
        if let Expression::Number(Number::Integer(integer)) = evaluate(cons.car.as_ref(), scope)? {
          if integer == 0 {
            return Err(EvaluationError::DivideByZero);
          }
          quotient /= integer;
        } else {
          return Err(EvaluationError::InvalidArgument);
        }
        expression = cons.cdr.as_ref();
      }
      if expression != &null!() {
        return Err(EvaluationError::InvalidArgument);
      }
      return Ok(int!(quotient));
    }
  }
  Err(EvaluationError::InvalidArgument)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_evaluate_add() {
    let scope = &mut Scope::new();
    assert_eq!(evaluate(&parse("(+ 1 2)").unwrap(), scope), Ok(int!(3)));
    assert_eq!(evaluate(&parse("(+ 1 2 3)").unwrap(), scope), Ok(int!(6)));
    assert_eq!(evaluate(&parse("(+ 1 -2)").unwrap(), scope), Ok(int!(-1)));
    assert_eq!(evaluate(&parse("(+ 1)").unwrap(), scope), Ok(int!(1)));
    assert_eq!(evaluate(&parse("(+ 0 0 0 0)").unwrap(), scope), Ok(int!(0)));
    assert_eq!(
      evaluate(&parse("(+ (+ 1 2) (+ 3 4))").unwrap(), scope),
      Ok(int!(10))
    );
    assert_eq!(
      evaluate(&parse("(+)").unwrap(), scope),
      Err(EvaluationError::WrongNumberOfArguments)
    );
    assert_eq!(
      evaluate(&parse("(+ ())").unwrap(), scope),
      Err(EvaluationError::InvalidArgument)
    );
    // Test an improper list
    assert_eq!(
      evaluate(&cons!(&atom!("+"), &cons!(&int!(1), &int!(2))), scope),
      Err(EvaluationError::InvalidArgument)
    );
  }

  #[test]
  fn test_evaluate_multiply() {
    let scope = &mut Scope::new();
    assert_eq!(evaluate(&parse("(* 1 2)").unwrap(), scope), Ok(int!(2)));
    assert_eq!(evaluate(&parse("(* 1 2 3)").unwrap(), scope), Ok(int!(6)));
    assert_eq!(evaluate(&parse("(* 1 -2)").unwrap(), scope), Ok(int!(-2)));
    assert_eq!(evaluate(&parse("(* 1)").unwrap(), scope), Ok(int!(1)));
    assert_eq!(evaluate(&parse("(* 0 0 0 0)").unwrap(), scope), Ok(int!(0)));
    assert_eq!(
      evaluate(&parse("(* (* 1 2) (* 3 4))").unwrap(), scope),
      Ok(int!(24))
    );
    assert_eq!(
      evaluate(&parse("(*)").unwrap(), scope),
      Err(EvaluationError::WrongNumberOfArguments)
    );
    assert_eq!(
      evaluate(&parse("(* ())").unwrap(), scope),
      Err(EvaluationError::InvalidArgument)
    );
    // Test an improper list
    assert_eq!(
      evaluate(&cons!(&atom!("*"), &cons!(&int!(1), &int!(2))), scope),
      Err(EvaluationError::InvalidArgument)
    );
  }

  #[test]
  fn test_evaluate_subtract() {
    let scope = &mut Scope::new();
    assert_eq!(evaluate(&parse("(- 2 1)").unwrap(), scope), Ok(int!(1)));
    assert_eq!(evaluate(&parse("(- 3 2 1)").unwrap(), scope), Ok(int!(0)));
    assert_eq!(evaluate(&parse("(- 1 2)").unwrap(), scope), Ok(int!(-1)));
    assert_eq!(evaluate(&parse("(- 1)").unwrap(), scope), Ok(int!(-1)));
    assert_eq!(evaluate(&parse("(- 0 0 0 0)").unwrap(), scope), Ok(int!(0)));
    assert_eq!(
      evaluate(&parse("(- (- 1 2) (- 3 4))").unwrap(), scope),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(-)").unwrap(), scope),
      Err(EvaluationError::WrongNumberOfArguments)
    );
    assert_eq!(
      evaluate(&parse("(- ())").unwrap(), scope),
      Err(EvaluationError::InvalidArgument)
    );
    // Test an improper list
    assert_eq!(
      evaluate(&cons!(&atom!("-"), &cons!(&int!(1), &int!(2))), scope),
      Err(EvaluationError::InvalidArgument)
    );
  }

  #[test]
  fn test_evaluate_divide() {
    let scope = &mut Scope::new();
    assert_eq!(evaluate(&parse("(/ 20 2)").unwrap(), scope), Ok(int!(10)));
    assert_eq!(evaluate(&parse("(/ 12 2 3)").unwrap(), scope), Ok(int!(2)));
    assert_eq!(evaluate(&parse("(/ 1 2)").unwrap(), scope), Ok(int!(0)));
    assert_eq!(evaluate(&parse("(/ 0 2)").unwrap(), scope), Ok(int!(0)));
    assert_eq!(evaluate(&parse("(/ 1)").unwrap(), scope), Ok(int!(1)));
    assert_eq!(evaluate(&parse("(/ 2)").unwrap(), scope), Ok(int!(0)));
    assert_eq!(evaluate(&parse("(/ -1)").unwrap(), scope), Ok(int!(-1)));
    assert_eq!(evaluate(&parse("(/ -2)").unwrap(), scope), Ok(int!(0)));
    assert_eq!(
      evaluate(&parse("(/ 0)").unwrap(), scope),
      Err(EvaluationError::DivideByZero)
    );
    assert_eq!(
      evaluate(&parse("(/ 1 0)").unwrap(), scope),
      Err(EvaluationError::DivideByZero)
    );
    assert_eq!(
      evaluate(&parse("(/ (/ 100 2) (/ 15 3))").unwrap(), scope),
      Ok(int!(10))
    );
    assert_eq!(
      evaluate(&parse("(-)").unwrap(), scope),
      Err(EvaluationError::WrongNumberOfArguments)
    );
    assert_eq!(
      evaluate(&parse("(- ())").unwrap(), scope),
      Err(EvaluationError::InvalidArgument)
    );
    // Test an improper list
    assert_eq!(
      evaluate(&cons!(&atom!("-"), &cons!(&int!(1), &int!(2))), scope),
      Err(EvaluationError::InvalidArgument)
    );
  }
}
