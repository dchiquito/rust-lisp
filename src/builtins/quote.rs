use super::*;

fn _quote(args: Vec<Expression>, _scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  Ok(ProcedureValue::Expression(args.get(0).unwrap().clone()))
}

pub const QUOTE: Expression =
  Expression::Procedure(Procedure::BuiltinFixedArgumentForm("quote", _quote, 1));

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_quote() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("'foo").unwrap(), scope.clone()),
      Ok(symbol!("foo"))
    );
    assert_eq!(
      evaluate(&parse("'(foo)").unwrap(), scope.clone()),
      Ok(list!(symbol!("foo")))
    );
    assert_eq!(
      evaluate(&parse("(eq? (eq? 1 1) (eq? 1 1))").unwrap(), scope.clone()),
      Ok(boolean!(true))
    );
    assert_eq!(
      evaluate(&parse("(eq? '(eq? 1 1) (eq? 1 1))").unwrap(), scope.clone()),
      Ok(boolean!(false))
    );
    assert_eq!(
      evaluate(
        &parse("(eq? '(a b c) (quote (a b c)))").unwrap(),
        scope.clone()
      ),
      Ok(boolean!(true))
    );
    assert_eq!(
      evaluate(
        &parse("(eq? '((a b) (c d)) (quote ((a b) (c d)) ))").unwrap(),
        scope.clone()
      ),
      Ok(boolean!(true))
    );
  }
}
