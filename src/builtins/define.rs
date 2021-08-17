use super::*;

fn _define(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let symbol = args.get(0).unwrap();
  let expression = evaluate(args.get(1).unwrap(), scope.clone())?;
  if let Expression::Symbol(symbol) = symbol {
    scope.borrow_mut().define(symbol, expression);
    Ok(ProcedureValue::Expression(void!()))
  } else {
    Err(EvaluationError::invalid_argument(
      "define", "symbol", symbol,
    ))
  }
}
pub const DEFINE: Procedure = Procedure::BuiltinFixedArgumentForm("define", _define, 2);

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::TestContext;

  #[test]
  fn test_evaluate_define() {
    let ctx = TestContext::new();
    ctx.assert_eq("(define foo (+ 1 1))", void!());
    assert_eq!(ctx.scope.borrow().lookup(&String::from("foo")), Ok(int!(2)));
    ctx.assert_eq("foo", int!(2));
    ctx.assert_err("bar", EvaluationError::UndefinedSymbol("bar".to_string()));
  }
  #[test]
  fn test_evaluate_define_arguments() {
    let ctx = TestContext::new();
    ctx.assert_err(
      "(define)",
      EvaluationError::WrongNumberOfArguments("define".to_string(), 2, 0),
    );
    ctx.assert_err(
      "(define foo)",
      EvaluationError::WrongNumberOfArguments("define".to_string(), 2, 1),
    );
  }
  #[test]
  fn test_evaluate_define_non_symbols() {
    let ctx = TestContext::new();
    ctx.assert_err(
      "(define 6 1)",
      EvaluationError::invalid_argument("define", "symbol", &int!(6)),
    );
    ctx.assert_err(
      "(define #t 1)",
      EvaluationError::invalid_argument("define", "symbol", &boolean!(true)),
    );
    ctx.assert_err(
      "(define #f 1)",
      EvaluationError::invalid_argument("define", "symbol", &boolean!(false)),
    );
  }
}
