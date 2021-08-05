use super::*;

pub fn evaluate_cons(expression: &Expression, scope: &mut Scope) -> EvaluationResult {
  assert_arg_length(expression, 2)?;
  let a = evaluate(&arg_get(expression, 0)?, scope)?;
  let b = evaluate(&arg_get(expression, 1)?, scope)?;

  Ok(cons!(&a, &b))
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_cons() {
    let scope = &mut Scope::new();
    assert_eq!(
      evaluate(&parse("(cons 1 2)").unwrap(), scope),
      Ok(cons!(&int!(1), &int!(2)))
    );
    assert_eq!(
      evaluate(&parse("(cons 1 2)").unwrap(), scope),
      Ok(cons!(&int!(1), &int!(2)))
    );
    assert_eq!(
      evaluate(&parse("(cons (eq? 1 1) (eq? 1 2))").unwrap(), scope),
      Ok(cons!(&atom!("true"), &atom!("false")))
    );
    assert_eq!(
      evaluate(&parse("(cons 'foo nil)").unwrap(), scope),
      Ok(list!(atom!("foo")))
    );
    assert_eq!(
      evaluate(&parse("(eq? (cons 'foo nil) '(foo))").unwrap(), scope),
      Ok(atom!("true"))
    );
  }
}
