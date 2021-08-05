use super::*;

pub fn evaluate_define(expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  assert_arg_length(expression, 2)?;
  let symbol = arg_get(expression, 0)?;
  let expression = evaluate(&arg_get(expression, 1)?, scope)?;

  if let Expression::Atom(symbol) = symbol {
    scope.define(symbol, expression.clone());
    Ok(expression)
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_evaluate_define() {
    let scope = &mut Scope::new();
    assert_eq!(
      evaluate(&parse("(define foo 1)").unwrap(), scope),
      Ok(atom!("1"))
    );
    assert_eq!(scope.lookup(&Atom::new("foo")), Ok(atom!("1")));
    assert_eq!(evaluate(&parse("foo").unwrap(), scope), Ok(atom!("1")))
  }
}
