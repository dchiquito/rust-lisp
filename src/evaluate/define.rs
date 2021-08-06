use super::*;

pub fn evaluate_define(expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  assert_arg_length(expression, 2)?;
  let symbol = arg_get(expression, 0)?;
  let expression = evaluate(&arg_get(expression, 1)?, scope)?;

  if let Expression::Symbol(symbol) = symbol {
    scope.define(symbol, expression.clone());
    return Ok(expression);
  }
  Err(EvaluationError::InvalidArgument)
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
      Ok(int!(1))
    );
    assert_eq!(scope.lookup(&String::from("foo")), Ok(int!(1)));
    assert_eq!(evaluate(&parse("foo").unwrap(), scope), Ok(int!(1)))
  }
  #[test]
  fn test_evaluate_define_non_symbols() {
    let scope = &mut Scope::new();
    assert_eq!(
      evaluate(&parse("(define '() 1)").unwrap(), scope),
      Err(EvaluationError::InvalidArgument)
    );
    assert_eq!(
      evaluate(&parse("(define () 1)").unwrap(), scope),
      Err(EvaluationError::InvalidArgument)
    );
    assert_eq!(
      evaluate(&parse("(define 6 1)").unwrap(), scope),
      Err(EvaluationError::InvalidArgument)
    );
    assert_eq!(
      evaluate(&parse("(define #t 1)").unwrap(), scope),
      Err(EvaluationError::InvalidArgument)
    );
    assert_eq!(
      evaluate(&parse("(define #f 1)").unwrap(), scope),
      Err(EvaluationError::InvalidArgument)
    );
  }
}
