use super::*;

fn _eq(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let a = evaluate(args.get(0).unwrap(), scope.clone())?;
  let b = evaluate(args.get(1).unwrap(), scope)?;
  Ok(ProcedureValue::Expression(boolean!(a == b)))
}

pub const EQ: Procedure = Procedure::BuiltinFixedArgumentForm("eq?", _eq, 2);

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::TestContext;

  #[test]
  fn test_evaluate_eq() {
    let ctx = TestContext::new();
    ctx.assert_eq("(eq? 1 1)", boolean!(true));
    ctx.assert_eq("(eq? 'foo 'foo)", boolean!(true));
    ctx.assert_eq("(eq? 'foo 'bar)", boolean!(false));
    ctx.assert_eq("(eq? (eq? 1 1) #t)", boolean!(true));
    ctx.assert_eq("(eq? (eq? 1 1) #true)", boolean!(true));
  }
}
