use super::*;

fn _cons(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let left = evaluate(args.get(0).unwrap(), scope.clone())?;
  let right = evaluate(args.get(1).unwrap(), scope)?;
  Ok(ProcedureValue::Expression(cons!(&left, &right)))
}
pub const CONS: Procedure = Procedure::BuiltinFixedArgumentForm("cons", _cons, 2);

fn _car(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let arg = evaluate(args.get(0).unwrap(), scope)?;
  if let Expression::Cons(cons) = arg {
    Ok(ProcedureValue::Expression(cons.car.as_ref().clone()))
  } else {
    Err(EvaluationError::invalid_argument("car", "list", &arg))
  }
}
pub const CAR: Procedure = Procedure::BuiltinFixedArgumentForm("car", _car, 1);

fn _cdr(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let arg = evaluate(args.get(0).unwrap(), scope)?;
  if let Expression::Cons(cons) = arg {
    Ok(ProcedureValue::Expression(cons.cdr.as_ref().clone()))
  } else {
    Err(EvaluationError::invalid_argument("cdr", "list", &arg))
  }
}
pub const CDR: Procedure = Procedure::BuiltinFixedArgumentForm("cdr", _cdr, 1);

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::TestContext;

  #[test]
  fn test_cons() {
    let ctx = TestContext::new();
    ctx.assert_eq("(cons 1 2)", cons!(&int!(1), &int!(2)));
    ctx.assert_eq("(cons 1 2)", cons!(&int!(1), &int!(2)));
    ctx.assert_eq(
      "(cons (eq? 1 1) (eq? 1 2))",
      cons!(&boolean!(true), &boolean!(false)),
    );
    ctx.assert_eq("(cons 'foo '())", list!(symbol!("foo")));
    ctx.assert_eq("(eq? (cons 'foo '()) '(foo))", boolean!(true));
  }

  #[test]
  fn test_car() {
    let ctx = TestContext::new();
    ctx.assert_eq("(car '(1))", int!(1));
    ctx.assert_eq("(car '(1 2 3))", int!(1));
    ctx.assert_eq("(car (cons 'foo 'bar))", symbol!("foo"));
  }
  #[test]
  fn test_cdr() {
    let ctx = TestContext::new();
    ctx.assert_eq("(cdr '(1))", null!());
    ctx.assert_eq("(cdr '(1 2 3))", list!(int!(2), int!(3)));
    ctx.assert_eq("(cdr (cons 'foo 'bar))", symbol!("bar"));
  }
}
