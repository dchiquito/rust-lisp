use super::*;

pub fn evaluate_cons(expression: &Expression) -> EvaluationResult {
  assert_arg_length(expression, 2)?;
  let a = evaluate(&arg_get(expression, 0)?)?;
  let b = evaluate(&arg_get(expression, 1)?)?;

  Ok(cons!(&a, &b))
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
}
