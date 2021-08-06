use super::*;

pub fn evaluate_quote(expression: &Expression, _scope: &mut Scope) -> EvaluationResult {
  assert_arg_length(expression, 1)?;
  Ok(arg_get(expression, 0)?)
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_quote() {
    let scope = &mut Scope::new();
    assert_eq!(evaluate(&parse("'foo").unwrap(), scope), Ok(symbol!("foo")));
    assert_eq!(
      evaluate(&parse("'(foo)").unwrap(), scope),
      Ok(list!(symbol!("foo")))
    );
    assert_eq!(
      evaluate(&parse("(eq? (eq? 1 1) (eq? 1 1))").unwrap(), scope),
      Ok(boolean!(true))
    );
    assert_eq!(
      evaluate(&parse("(eq? '(eq? 1 1) (eq? 1 1))").unwrap(), scope),
      Ok(boolean!(false))
    );
    assert_eq!(
      evaluate(&parse("(eq? '(a b c) (quote (a b c)))").unwrap(), scope),
      Ok(boolean!(true))
    );
    assert_eq!(
      evaluate(
        &parse("(eq? '((a b) (c d)) (quote ((a b) (c d)) ))").unwrap(),
        scope
      ),
      Ok(boolean!(true))
    );
  }
}
