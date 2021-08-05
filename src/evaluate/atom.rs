use super::*;

pub fn evaluate_atom(expression: &Expression) -> EvaluationResult {
  assert_arg_length(expression, 1)?;
  let expression = evaluate(&arg_get(expression, 0)?)?;

  if let Expression::Atom(atom) = expression {
    if atom.string != "nil" {
      Ok(atom!("true"))
    } else {
      Ok(atom!("false"))
    }
  } else {
    Ok(atom!("false"))
  }
}

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_evaluate_atom() {
    assert_eq!(evaluate(&parse("(atom? 1)").unwrap()), Ok(atom!("true")));
    assert_eq!(evaluate(&parse("(atom? true)").unwrap()), Ok(atom!("true")));
    assert_eq!(
      evaluate(&parse("(atom? false)").unwrap()),
      Ok(atom!("true"))
    );

    // nil is considered the null value, not an atom
    assert_eq!(evaluate(&parse("(atom? nil)").unwrap()), Ok(atom!("false")));
    assert_eq!(evaluate(&parse("(atom? '())").unwrap()), Ok(atom!("false")));

    assert_eq!(
      evaluate(&parse("(atom? '(1 2))").unwrap()),
      Ok(atom!("false"))
    );
    assert_eq!(
      evaluate(&parse("(atom? (cons 1 2))").unwrap()),
      Ok(atom!("false"))
    );
  }
}
