use super::*;
use std::cell::RefCell;
use std::rc::Rc;

fn _add(
  args: Vec<Expression>,
  varargs: Vec<Expression>,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  let sum = evaluate(args.get(0).unwrap(), scope.clone())?;
  if let Expression::Number(Number::Integer(mut sum)) = sum {
    for arg in varargs {
      let term = evaluate(&arg, scope.clone())?;
      if let Expression::Number(Number::Integer(integer)) = term {
        sum += integer;
      } else {
        return Err(EvaluationError::invalid_argument("+", "number", &term));
      }
    }
    Ok(ProcedureValue::Expression(int!(sum)))
  } else {
    Err(EvaluationError::invalid_argument("+", "number", &sum))
  }
}

pub const ADD: Expression =
  Expression::Procedure(Procedure::BuiltinVariableArgumentForm("+", _add, 1));

pub fn _multiply(
  args: Vec<Expression>,
  varargs: Vec<Expression>,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  let product = evaluate(args.get(0).unwrap(), scope.clone())?;
  if let Expression::Number(Number::Integer(mut product)) = product {
    for arg in varargs {
      let term = evaluate(&arg, scope.clone())?;
      if let Expression::Number(Number::Integer(integer)) = term {
        product *= integer;
      } else {
        return Err(EvaluationError::invalid_argument("*", "number", &term));
      }
    }
    Ok(ProcedureValue::Expression(int!(product)))
  } else {
    Err(EvaluationError::invalid_argument("*", "number", &product))
  }
}

pub const MULTIPLY: Expression =
  Expression::Procedure(Procedure::BuiltinVariableArgumentForm("*", _multiply, 1));

pub fn _subtract(
  args: Vec<Expression>,
  varargs: Vec<Expression>,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  let subtraction = evaluate(args.get(0).unwrap(), scope.clone())?;
  if let Expression::Number(Number::Integer(mut subtraction)) = subtraction {
    if varargs.is_empty() {
      Ok(ProcedureValue::Expression(int!(-subtraction)))
    } else {
      for arg in varargs {
        let term = evaluate(&arg, scope.clone())?;
        if let Expression::Number(Number::Integer(integer)) = term {
          subtraction -= integer;
        } else {
          return Err(EvaluationError::invalid_argument("-", "number", &term));
        }
      }
      Ok(ProcedureValue::Expression(int!(subtraction)))
    }
  } else {
    Err(EvaluationError::invalid_argument(
      "-",
      "number",
      &subtraction,
    ))
  }
}

pub const SUBTRACT: Expression =
  Expression::Procedure(Procedure::BuiltinVariableArgumentForm("-", _subtract, 1));

pub fn _divide(
  args: Vec<Expression>,
  varargs: Vec<Expression>,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  let quotient = evaluate(args.get(0).unwrap(), scope.clone())?;
  if let Expression::Number(Number::Integer(mut quotient)) = quotient {
    if varargs.is_empty() {
      if quotient == 0 {
        Err(EvaluationError::DivideByZero(Number::Integer(1)))
      } else {
        Ok(ProcedureValue::Expression(int!(1 / quotient)))
      }
    } else {
      for arg in varargs {
        let term = evaluate(&arg, scope.clone())?;
        if let Expression::Number(Number::Integer(integer)) = term {
          if integer == 0 {
            return Err(EvaluationError::DivideByZero(Number::Integer(quotient)));
          }
          quotient /= integer;
        } else {
          return Err(EvaluationError::invalid_argument("/", "number", &term));
        }
      }
      Ok(ProcedureValue::Expression(int!(quotient)))
    }
  } else {
    Err(EvaluationError::invalid_argument("/", "number", &quotient))
  }
}

pub const DIVIDE: Expression =
  Expression::Procedure(Procedure::BuiltinVariableArgumentForm("/", _divide, 1));

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
      Err(EvaluationError::WrongNumberOfVariableArguments(
        "+".to_string(),
        1,
        0
      ))
    );
    assert_eq!(
      evaluate(&parse("(+ ())").unwrap(), scope.clone()),
      Err(EvaluationError::invalid_argument("+", "number", &null!()))
    );
    // Test an improper list
    assert_eq!(
      evaluate(
        &cons!(&symbol!("+"), &cons!(&int!(1), &int!(2))),
        scope.clone()
      ),
      Err(EvaluationError::invalid_argument(
        "+",
        "list",
        &cons!(&int!(1), &int!(2))
      ))
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
      Err(EvaluationError::WrongNumberOfVariableArguments(
        "*".to_string(),
        1,
        0
      ))
    );
    assert_eq!(
      evaluate(&parse("(* ())").unwrap(), scope.clone()),
      Err(EvaluationError::invalid_argument("*", "number", &null!()))
    );
    // Test an improper list
    assert_eq!(
      evaluate(
        &cons!(&symbol!("*"), &cons!(&int!(1), &int!(2))),
        scope.clone()
      ),
      Err(EvaluationError::invalid_argument(
        "*",
        "list",
        &cons!(&int!(1), &int!(2))
      ))
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
      Err(EvaluationError::WrongNumberOfVariableArguments(
        "-".to_string(),
        1,
        0
      ))
    );
    assert_eq!(
      evaluate(&parse("(- ())").unwrap(), scope.clone()),
      Err(EvaluationError::invalid_argument("-", "number", &null!()))
    );
    // Test an improper list
    assert_eq!(
      evaluate(
        &cons!(&symbol!("-"), &cons!(&int!(1), &int!(2))),
        scope.clone()
      ),
      Err(EvaluationError::invalid_argument(
        "-",
        "list",
        &cons!(&int!(1), &int!(2))
      ))
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
      Err(EvaluationError::DivideByZero(Number::Integer(1)))
    );
    assert_eq!(
      evaluate(&parse("(/ 3 0)").unwrap(), scope.clone()),
      Err(EvaluationError::DivideByZero(Number::Integer(3)))
    );
    assert_eq!(
      evaluate(&parse("(/ (/ 100 2) (/ 15 3))").unwrap(), scope.clone()),
      Ok(int!(10))
    );
    assert_eq!(
      evaluate(&parse("(/)").unwrap(), scope.clone()),
      Err(EvaluationError::WrongNumberOfVariableArguments(
        "/".to_string(),
        1,
        0
      ))
    );
    assert_eq!(
      evaluate(&parse("(/ ())").unwrap(), scope.clone()),
      Err(EvaluationError::invalid_argument("/", "number", &null!()))
    );
    // Test an improper list
    assert_eq!(
      evaluate(
        &cons!(&symbol!("/"), &cons!(&int!(1), &int!(2))),
        scope.clone()
      ),
      Err(EvaluationError::invalid_argument(
        "/",
        "list",
        &cons!(&int!(1), &int!(2))
      ))
    );
  }
}
