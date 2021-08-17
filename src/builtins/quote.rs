use super::*;

fn _quote(args: Vec<Expression>, _scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  Ok(ProcedureValue::Expression(args.get(0).unwrap().clone()))
}

pub const QUOTE: Procedure = Procedure::BuiltinFixedArgumentForm("quote", _quote, 1);

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::TestContext;

  #[test]
  fn test_quote() {
    let ctx = TestContext::new();
    ctx.assert_eq("'foo", symbol!("foo"));
    ctx.assert_eq("'(foo)", list!(symbol!("foo")));
    ctx.assert_eq("(eq? (eq? 1 1) (eq? 1 1))", boolean!(true));
    ctx.assert_eq("(eq? '(eq? 1 1) (eq? 1 1))", boolean!(false));
    ctx.assert_eq("(eq? '(a b c) (quote (a b c)))", boolean!(true));
    ctx.assert_eq(
      "(eq? '((a b) (c d)) (quote ((a b) (c d)) ))",
      boolean!(true),
    );
  }
}
