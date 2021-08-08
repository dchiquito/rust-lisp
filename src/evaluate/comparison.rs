use super::*;

fn _eq(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let a = evaluate(args.get(0).unwrap(), scope.clone())?;
  let b = evaluate(args.get(1).unwrap(), scope.clone())?;
  Ok(ProcedureValue::Expression(boolean!(a == b)))
}

pub const EQ: Expression = Expression::Procedure(Procedure::BuiltinFixedArgumentForm(_eq, 2));

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_evaluate_eq() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(eq? 1 1)").unwrap(), scope.clone()),
      Ok(boolean!(true))
    );
    assert_eq!(
      evaluate(&parse("(eq? 'foo 'foo)").unwrap(), scope.clone()),
      Ok(boolean!(true))
    );
    assert_eq!(
      evaluate(&parse("(eq? 'foo 'bar)").unwrap(), scope.clone()),
      Ok(boolean!(false))
    );
    assert_eq!(
      evaluate(&parse("(eq? (eq? 1 1) #t)").unwrap(), scope.clone()),
      Ok(boolean!(true))
    );
    assert_eq!(
      evaluate(&parse("(eq? (eq? 1 1) #true)").unwrap(), scope.clone()),
      Ok(boolean!(true))
    );
  }
}
