use super::*;

fn _cons(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let left = evaluate(args.get(0).unwrap(), scope.clone())?;
  let right = evaluate(args.get(1).unwrap(), scope)?;
  Ok(ProcedureValue::Expression(cons!(&left, &right)))
}
pub const CONS: Expression =
  Expression::Procedure(Procedure::BuiltinFixedArgumentForm("cons", _cons, 2));

fn _car(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let arg = evaluate(args.get(0).unwrap(), scope)?;
  if let Expression::Cons(cons) = arg {
    Ok(ProcedureValue::Expression(cons.car.as_ref().clone()))
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}
pub const CAR: Expression =
  Expression::Procedure(Procedure::BuiltinFixedArgumentForm("car", _car, 1));

fn _cdr(args: Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  let arg = evaluate(args.get(0).unwrap(), scope)?;
  if let Expression::Cons(cons) = arg {
    Ok(ProcedureValue::Expression(cons.cdr.as_ref().clone()))
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}
pub const CDR: Expression =
  Expression::Procedure(Procedure::BuiltinFixedArgumentForm("cdr", _cdr, 1));

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_cons() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(cons 1 2)").unwrap(), scope.clone()),
      Ok(cons!(&int!(1), &int!(2)))
    );
    assert_eq!(
      evaluate(&parse("(cons 1 2)").unwrap(), scope.clone()),
      Ok(cons!(&int!(1), &int!(2)))
    );
    assert_eq!(
      evaluate(&parse("(cons (eq? 1 1) (eq? 1 2))").unwrap(), scope.clone()),
      Ok(cons!(&boolean!(true), &boolean!(false)))
    );
    assert_eq!(
      evaluate(&parse("(cons 'foo '())").unwrap(), scope.clone()),
      Ok(list!(symbol!("foo")))
    );
    assert_eq!(
      evaluate(
        &parse("(eq? (cons 'foo '()) '(foo))").unwrap(),
        scope.clone()
      ),
      Ok(boolean!(true))
    );
  }

  #[test]
  fn test_car() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(car '(1))").unwrap(), scope.clone()),
      Ok(int!(1))
    );
    assert_eq!(
      evaluate(&parse("(car '(1 2 3))").unwrap(), scope.clone()),
      Ok(int!(1))
    );
    assert_eq!(
      evaluate(&parse("(car (cons 'foo 'bar))").unwrap(), scope.clone()),
      Ok(symbol!("foo"))
    );
  }
  #[test]
  fn test_cdr() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(cdr '(1))").unwrap(), scope.clone()),
      Ok(null!())
    );
    assert_eq!(
      evaluate(&parse("(cdr '(1 2 3))").unwrap(), scope.clone()),
      Ok(list!(int!(2), int!(3)))
    );
    assert_eq!(
      evaluate(&parse("(cdr (cons 'foo 'bar))").unwrap(), scope.clone()),
      Ok(symbol!("bar"))
    );
  }
}
