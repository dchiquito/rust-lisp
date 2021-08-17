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

pub const ADD: Procedure = Procedure::BuiltinVariableArgumentForm("+", _add, 1);

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

pub const MULTIPLY: Procedure = Procedure::BuiltinVariableArgumentForm("*", _multiply, 1);

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

pub const SUBTRACT: Procedure = Procedure::BuiltinVariableArgumentForm("-", _subtract, 1);

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

pub const DIVIDE: Procedure = Procedure::BuiltinVariableArgumentForm("/", _divide, 1);

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::TestContext;

  #[test]
  fn test_evaluate_add() {
    let ctx = TestContext::new();
    ctx.assert_eq("(+ 1 2)", int!(3));
    ctx.assert_eq("(+ 1 2 3)", int!(6));
    ctx.assert_eq("(+ 1 -2)", int!(-1));
    ctx.assert_eq("(+ 1)", int!(1));
    ctx.assert_eq("(+ 0 0 0 0)", int!(0));
    ctx.assert_eq("(+ (+ 1 2) (+ 3 4))", int!(10));
    ctx.assert_err(
      "(+)",
      EvaluationError::WrongNumberOfVariableArguments("+".to_string(), 1, 0),
    );
    ctx.assert_err(
      "(+ ())",
      EvaluationError::invalid_argument("+", "number", &null!()),
    );
    // Test an improper list
    ctx.assert_err(
      "(+ 1 . 2)",
      EvaluationError::invalid_argument("+", "list", &cons!(&int!(1), &int!(2))),
    );
  }

  #[test]
  fn test_evaluate_multiply() {
    let ctx = TestContext::new();
    ctx.assert_eq("(* 1 2)", int!(2));
    ctx.assert_eq("(* 1 2 3)", int!(6));
    ctx.assert_eq("(* 1 -2)", int!(-2));
    ctx.assert_eq("(* 1)", int!(1));
    ctx.assert_eq("(* 0 0 0 0)", int!(0));
    ctx.assert_eq("(* (* 1 2) (* 3 4))", int!(24));
    ctx.assert_err(
      "(*)",
      EvaluationError::WrongNumberOfVariableArguments("*".to_string(), 1, 0),
    );
    ctx.assert_err(
      "(* ())",
      EvaluationError::invalid_argument("*", "number", &null!()),
    );
    // Test an improper list
    ctx.assert_err(
      "(* 1 . 2)",
      EvaluationError::invalid_argument("*", "list", &cons!(&int!(1), &int!(2))),
    );
  }

  #[test]
  fn test_evaluate_subtract() {
    let ctx = TestContext::new();
    ctx.assert_eq("(- 2 1)", int!(1));
    ctx.assert_eq("(- 3 2 1)", int!(0));
    ctx.assert_eq("(- 1 2)", int!(-1));
    ctx.assert_eq("(- 1)", int!(-1));
    ctx.assert_eq("(- 0 0 0 0)", int!(0));
    ctx.assert_eq("(- (- 1 2) (- 3 4))", int!(0));
    ctx.assert_err(
      "(-)",
      EvaluationError::WrongNumberOfVariableArguments("-".to_string(), 1, 0),
    );
    ctx.assert_err(
      "(- ())",
      EvaluationError::invalid_argument("-", "number", &null!()),
    );
    // Test an improper list
    ctx.assert_err(
      "(- 1 . 2)",
      EvaluationError::invalid_argument("-", "list", &cons!(&int!(1), &int!(2))),
    );
  }

  #[test]
  fn test_evaluate_divide() {
    let ctx = TestContext::new();
    ctx.assert_eq("(/ 20 2)", int!(10));
    ctx.assert_eq("(/ 12 2 3)", int!(2));
    ctx.assert_eq("(/ 1 2)", int!(0));
    ctx.assert_eq("(/ 0 2)", int!(0));
    ctx.assert_eq("(/ 1)", int!(1));
    ctx.assert_eq("(/ 2)", int!(0));
    ctx.assert_eq("(/ -1)", int!(-1));
    ctx.assert_eq("(/ -2)", int!(0));
    ctx.assert_err("(/ 0)", EvaluationError::DivideByZero(Number::Integer(1)));
    ctx.assert_err("(/ 3 0)", EvaluationError::DivideByZero(Number::Integer(3)));
    ctx.assert_eq("(/ (/ 100 2) (/ 15 3))", int!(10));
    ctx.assert_err(
      "(/)",
      EvaluationError::WrongNumberOfVariableArguments("/".to_string(), 1, 0),
    );
    ctx.assert_err(
      "(/ ())",
      EvaluationError::invalid_argument("/", "number", &null!()),
    );
    // Test an improper list
    ctx.assert_err(
      "(/ 1 . 2)",
      EvaluationError::invalid_argument("/", "list", &cons!(&int!(1), &int!(2))),
    );
  }
}
