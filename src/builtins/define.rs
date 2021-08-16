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
pub const DEFINE: Expression =
  Expression::Procedure(Procedure::BuiltinFixedArgumentForm("define", _define, 2));

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_evaluate_define() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(define foo (+ 1 1))").unwrap(), scope.clone()),
      Ok(void!())
    );
    assert_eq!(scope.borrow().lookup(&String::from("foo")), Ok(int!(2)));
    assert_eq!(evaluate(&parse("foo").unwrap(), scope.clone()), Ok(int!(2)));
  }
  #[test]
  fn test_evaluate_define_arguments() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(define)").unwrap(), scope.clone()),
      Err(EvaluationError::WrongNumberOfArguments(
        "define".to_string(),
        2,
        0
      ))
    );
    assert_eq!(
      evaluate(&parse("(define foo)").unwrap(), scope.clone()),
      Err(EvaluationError::WrongNumberOfArguments(
        "define".to_string(),
        2,
        1
      ))
    );
  }
  #[test]
  fn test_evaluate_define_non_symbols() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(define 6 1)").unwrap(), scope.clone()),
      Err(EvaluationError::invalid_argument(
        "define",
        "symbol",
        &int!(6)
      ))
    );
    assert_eq!(
      evaluate(&parse("(define #t 1)").unwrap(), scope.clone()),
      Err(EvaluationError::invalid_argument(
        "define",
        "symbol",
        &boolean!(true)
      ))
    );
    assert_eq!(
      evaluate(&parse("(define #f 1)").unwrap(), scope.clone()),
      Err(EvaluationError::invalid_argument(
        "define",
        "symbol",
        &boolean!(false)
      ))
    );
  }
}
