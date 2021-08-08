use super::*;
use std::cell::RefCell;
use std::rc::Rc;

fn _add(
  args: Vec<Expression>,
  varargs: Vec<Expression>,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  if let Expression::Number(Number::Integer(mut sum)) =
    evaluate(args.get(0).unwrap(), scope.clone())?
  {
    for arg in varargs {
      if let Expression::Number(Number::Integer(integer)) = evaluate(&arg, scope.clone())? {
        sum += integer;
      } else {
        return Err(EvaluationError::InvalidArgument);
      }
    }
    Ok(ProcedureValue::Expression(int!(sum)))
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

pub const ADD: Expression = Expression::Procedure(Procedure::BuiltinVariableArgumentForm(_add, 1));

pub fn _multiply(
  args: Vec<Expression>,
  varargs: Vec<Expression>,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  if let Expression::Number(Number::Integer(mut product)) =
    evaluate(args.get(0).unwrap(), scope.clone())?
  {
    for arg in varargs {
      if let Expression::Number(Number::Integer(integer)) = evaluate(&arg, scope.clone())? {
        product *= integer;
      } else {
        return Err(EvaluationError::InvalidArgument);
      }
    }
    Ok(ProcedureValue::Expression(int!(product)))
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

pub const MULTIPLY: Expression =
  Expression::Procedure(Procedure::BuiltinVariableArgumentForm(_multiply, 1));

pub fn _subtract(
  args: Vec<Expression>,
  varargs: Vec<Expression>,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  if let Expression::Number(Number::Integer(mut negation)) =
    evaluate(args.get(0).unwrap(), scope.clone())?
  {
    if varargs.len() == 0 {
      Ok(ProcedureValue::Expression(int!(-negation)))
    } else {
      for arg in varargs {
        if let Expression::Number(Number::Integer(integer)) = evaluate(&arg, scope.clone())? {
          negation -= integer;
        } else {
          return Err(EvaluationError::InvalidArgument);
        }
      }
      Ok(ProcedureValue::Expression(int!(negation)))
    }
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

pub const SUBTRACT: Expression =
  Expression::Procedure(Procedure::BuiltinVariableArgumentForm(_subtract, 1));

pub fn _divide(
  args: Vec<Expression>,
  varargs: Vec<Expression>,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  if let Expression::Number(Number::Integer(mut quotient)) =
    evaluate(args.get(0).unwrap(), scope.clone())?
  {
    if varargs.len() == 0 {
      if quotient == 0 {
        Err(EvaluationError::DivideByZero)
      } else {
        Ok(ProcedureValue::Expression(int!(1 / quotient)))
      }
    } else {
      for arg in varargs {
        if let Expression::Number(Number::Integer(integer)) = evaluate(&arg, scope.clone())? {
          if integer == 0 {
            return Err(EvaluationError::DivideByZero);
          }
          quotient /= integer;
        } else {
          return Err(EvaluationError::InvalidArgument);
        }
      }
      Ok(ProcedureValue::Expression(int!(quotient)))
    }
  } else {
    Err(EvaluationError::InvalidArgument)
  }
}

pub const DIVIDE: Expression =
  Expression::Procedure(Procedure::BuiltinVariableArgumentForm(_divide, 1));

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_evaluate_add() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(+ 1 2)").unwrap(), scope.clone()),
      Ok(int!(3))
    );
    assert_eq!(
      evaluate(&parse("(+ 1 2 3)").unwrap(), scope.clone()),
      Ok(int!(6))
    );
    assert_eq!(
      evaluate(&parse("(+ 1 -2)").unwrap(), scope.clone()),
      Ok(int!(-1))
    );
    assert_eq!(
      evaluate(&parse("(+ 1)").unwrap(), scope.clone()),
      Ok(int!(1))
    );
    assert_eq!(
      evaluate(&parse("(+ 0 0 0 0)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(+ (+ 1 2) (+ 3 4))").unwrap(), scope.clone()),
      Ok(int!(10))
    );
    assert_eq!(
      evaluate(&parse("(+)").unwrap(), scope.clone()),
      Err(EvaluationError::WrongNumberOfArguments)
    );
    assert_eq!(
      evaluate(&parse("(+ ())").unwrap(), scope.clone()),
      Err(EvaluationError::InvalidArgument)
    );
    // Test an improper list
    assert_eq!(
      evaluate(
        &cons!(&symbol!("+"), &cons!(&int!(1), &int!(2))),
        scope.clone()
      ),
      Err(EvaluationError::InvalidArgument)
    );
  }

  #[test]
  fn test_evaluate_multiply() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(* 1 2)").unwrap(), scope.clone()),
      Ok(int!(2))
    );
    assert_eq!(
      evaluate(&parse("(* 1 2 3)").unwrap(), scope.clone()),
      Ok(int!(6))
    );
    assert_eq!(
      evaluate(&parse("(* 1 -2)").unwrap(), scope.clone()),
      Ok(int!(-2))
    );
    assert_eq!(
      evaluate(&parse("(* 1)").unwrap(), scope.clone()),
      Ok(int!(1))
    );
    assert_eq!(
      evaluate(&parse("(* 0 0 0 0)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(* (* 1 2) (* 3 4))").unwrap(), scope.clone()),
      Ok(int!(24))
    );
    assert_eq!(
      evaluate(&parse("(*)").unwrap(), scope.clone()),
      Err(EvaluationError::WrongNumberOfArguments)
    );
    assert_eq!(
      evaluate(&parse("(* ())").unwrap(), scope.clone()),
      Err(EvaluationError::InvalidArgument)
    );
    // Test an improper list
    assert_eq!(
      evaluate(
        &cons!(&symbol!("*"), &cons!(&int!(1), &int!(2))),
        scope.clone()
      ),
      Err(EvaluationError::InvalidArgument)
    );
  }

  #[test]
  fn test_evaluate_subtract() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(- 2 1)").unwrap(), scope.clone()),
      Ok(int!(1))
    );
    assert_eq!(
      evaluate(&parse("(- 3 2 1)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(- 1 2)").unwrap(), scope.clone()),
      Ok(int!(-1))
    );
    assert_eq!(
      evaluate(&parse("(- 1)").unwrap(), scope.clone()),
      Ok(int!(-1))
    );
    assert_eq!(
      evaluate(&parse("(- 0 0 0 0)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(- (- 1 2) (- 3 4))").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(-)").unwrap(), scope.clone()),
      Err(EvaluationError::WrongNumberOfArguments)
    );
    assert_eq!(
      evaluate(&parse("(- ())").unwrap(), scope.clone()),
      Err(EvaluationError::InvalidArgument)
    );
    // Test an improper list
    assert_eq!(
      evaluate(
        &cons!(&symbol!("-"), &cons!(&int!(1), &int!(2))),
        scope.clone()
      ),
      Err(EvaluationError::InvalidArgument)
    );
  }

  #[test]
  fn test_evaluate_divide() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("(/ 20 2)").unwrap(), scope.clone()),
      Ok(int!(10))
    );
    assert_eq!(
      evaluate(&parse("(/ 12 2 3)").unwrap(), scope.clone()),
      Ok(int!(2))
    );
    assert_eq!(
      evaluate(&parse("(/ 1 2)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(/ 0 2)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(/ 1)").unwrap(), scope.clone()),
      Ok(int!(1))
    );
    assert_eq!(
      evaluate(&parse("(/ 2)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(/ -1)").unwrap(), scope.clone()),
      Ok(int!(-1))
    );
    assert_eq!(
      evaluate(&parse("(/ -2)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(/ 0)").unwrap(), scope.clone()),
      Err(EvaluationError::DivideByZero)
    );
    assert_eq!(
      evaluate(&parse("(/ 1 0)").unwrap(), scope.clone()),
      Err(EvaluationError::DivideByZero)
    );
    assert_eq!(
      evaluate(&parse("(/ (/ 100 2) (/ 15 3))").unwrap(), scope.clone()),
      Ok(int!(10))
    );
    assert_eq!(
      evaluate(&parse("(-)").unwrap(), scope.clone()),
      Err(EvaluationError::WrongNumberOfArguments)
    );
    assert_eq!(
      evaluate(&parse("(- ())").unwrap(), scope.clone()),
      Err(EvaluationError::InvalidArgument)
    );
    // Test an improper list
    assert_eq!(
      evaluate(
        &cons!(&symbol!("-"), &cons!(&int!(1), &int!(2))),
        scope.clone()
      ),
      Err(EvaluationError::InvalidArgument)
    );
  }
}
