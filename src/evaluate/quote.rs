use super::*;

pub fn evaluate_quote(expression: &Expression) -> EvaluationResult {
  assert_arg_length(expression, 1)?;
  Ok(arg_get(expression, 0)?)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

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
}
