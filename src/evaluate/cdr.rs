use super::*;

pub fn evaluate_cdr(expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  assert_arg_length(expression, 1)?;
  let cons = evaluate(&arg_get(expression, 0)?, scope)?;
  if let Expression::Cons(cons) = cons {
    Ok(cons.cdr.as_ref().clone())
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_cdr() {
    let scope = &mut Scope::new();
    assert_eq!(
      evaluate(&parse("(cdr '(1))").unwrap(), scope),
      Ok(atom!("nil"))
    );
    assert_eq!(
      evaluate(&parse("(cdr '(1 2 3))").unwrap(), scope),
      Ok(list!(atom!("2"), atom!("3")))
    );
    assert_eq!(
      evaluate(&parse("(cdr (cons 'foo 'bar))").unwrap(), scope),
      Ok(atom!("bar"))
    );
  }
}
