use super::*;

fn _define(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let symbol = args.get(0).unwrap();
  let expression = evaluate(args.get(1).unwrap(), scope.clone())?;
  if let Expression::Symbol(symbol) = symbol {
    scope.borrow_mut().define(symbol, expression.clone());
    Ok(ProcedureValue::Expression(expression))
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}
pub const DEFINE: Expression =
  Expression::Procedure(Procedure::BuiltinFixedArgumentForm(_define, 2));

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_evaluate_define() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(define foo 1)").unwrap(), scope.clone()),
      Ok(int!(1))
    );
    assert_eq!(scope.borrow().lookup(&String::from("foo")), Ok(int!(1)));
    assert_eq!(evaluate(&parse("foo").unwrap(), scope.clone()), Ok(int!(1)))
  }
  #[test]
  fn test_evaluate_define_non_symbols() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(define '() 1)").unwrap(), scope.clone()),
      Err(EvaluationError::InvalidArgument)
    );
    assert_eq!(
      evaluate(&parse("(define () 1)").unwrap(), scope.clone()),
      Err(EvaluationError::InvalidArgument)
    );
    assert_eq!(
      evaluate(&parse("(define 6 1)").unwrap(), scope.clone()),
      Err(EvaluationError::InvalidArgument)
    );
    assert_eq!(
      evaluate(&parse("(define #t 1)").unwrap(), scope.clone()),
      Err(EvaluationError::InvalidArgument)
    );
    assert_eq!(
      evaluate(&parse("(define #f 1)").unwrap(), scope.clone()),
      Err(EvaluationError::InvalidArgument)
    );
  }
}
