use super::*;

pub fn evaluate_eq(expression: &Expression) -> EvaluationResult {
  assert_arg_length(expression, 2)?;
  let a = evaluate(&arg_get(expression, 0)?)?;
  let b = evaluate(&arg_get(expression, 1)?)?;

  if a == b {
    Ok(atom!("true"))
  } else {
    Ok(atom!("false"))
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
}
