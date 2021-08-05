use super::*;

pub fn evaluate_eq(expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  assert_arg_length(expression, 2)?;
  let a = evaluate(&arg_get(expression, 0)?, scope)?;
  let b = evaluate(&arg_get(expression, 1)?, scope)?;

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
    let scope = &mut Scope::new();
    assert_eq!(
      evaluate(&parse("(eq? 1 1)").unwrap(), scope),
      Ok(atom!("true"))
    );
    assert_eq!(
      evaluate(&parse("(eq? 'foo 'foo)").unwrap(), scope),
      Ok(atom!("true"))
    );
    assert_eq!(
      evaluate(&parse("(eq? 'foo 'bar)").unwrap(), scope),
      Ok(atom!("false"))
    );
    assert_eq!(
      evaluate(&parse("(eq? (eq? 1 1) true)").unwrap(), scope),
      Ok(atom!("true"))
    );
  }
}
