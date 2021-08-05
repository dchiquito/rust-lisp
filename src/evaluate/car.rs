use super::*;

pub fn evaluate_car(expression: &Expression) -> EvaluationResult {
  assert_arg_length(expression, 1)?;
  let cons = evaluate(&arg_get(expression, 0)?)?;
  if let Expression::Cons(cons) = cons {
    Ok(cons.car.as_ref().clone())
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_car() {
    assert_eq!(evaluate(&parse("(car '(1))").unwrap()), Ok(atom!("1")));
    assert_eq!(evaluate(&parse("(car '(1 2 3))").unwrap()), Ok(atom!("1")));
    assert_eq!(
      evaluate(&parse("(car (cons foo bar))").unwrap()),
      Ok(atom!("foo"))
    );
  }
}
