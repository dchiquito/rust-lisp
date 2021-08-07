mod arithmetic;
// mod car;
// mod cdr;
mod comparison;
// mod cons;
// mod define;
// mod lambda;
mod quote;

use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Eq, PartialEq)]
pub enum EvaluationError {
  UnknownFunctionName,
  WrongNumberOfArguments,
  InvalidArgument,
  UndefinedSymbol,
  DivideByZero,
  NotAProcedure,
}
pub type EvaluationResult = Result<Expression, EvaluationError>;

fn arg_vec(mut expression: &Expression) -> Result<Vec<Expression>, EvaluationError> {
  let mut args = vec![];
  while let Expression::Cons(cons) = expression {
    args.push(cons.car.as_ref().clone());
    expression = cons.cdr.as_ref();
  }
  if expression != &null!() {
    return Err(EvaluationError::InvalidArgument);
  }
  Ok(args)
}

/// Plop all of the builtins into the given scope
pub fn define_builtins(scope: Rc<RefCell<Scope>>) {
  let mut scope = scope.borrow_mut();
  scope.define("+", arithmetic::ADD);
  scope.define("*", arithmetic::MULTIPLY);
  scope.define("-", arithmetic::SUBTRACT);
  scope.define("/", arithmetic::DIVIDE);
  scope.define("eq?", comparison::EQ);
  scope.define("quote", quote::QUOTE);
}

// fn _evaluate(function_name: &str, expression: &Expression, scope: &mut Scope) -> EvaluationResult {
//   match function_name {
//     "+" => arithmetic::evaluate_add(expression, scope),
//     "*" => arithmetic::evaluate_multiply(expression, scope),
//     "-" => arithmetic::evaluate_subtract(expression, scope),
//     "/" => arithmetic::evaluate_divide(expression, scope),
//     "eq?" => comparison::evaluate_eq(expression, scope),
//     "quote" => quote::evaluate_quote(expression, scope),
//     "cons" => cons::evaluate_cons(expression, scope),
//     "car" => car::evaluate_car(expression, scope),
//     "cdr" => cdr::evaluate_cdr(expression, scope),
//     "define" => define::evaluate_define(expression, scope),
//     "lambda" => lambda::evaluate_lambda(expression, scope),
//     // TODO look up lambdas from the scope
//     Expression::Procedure(procedure) => {
//       lambda::evaluate_procedure(procedure, cons.cdr.as_ref(), scope)
//     }
//     _ => Err(EvaluationError::UnknownFunctionName),
//   }
// }

fn evaluate_procedure(
  procedure: Procedure,
  args: &Expression,
  scope: Rc<RefCell<Scope>>,
) -> EvaluationResult {
  match procedure {
    Procedure::BuiltinFixedArgumentForm(builtin, argc) => {
      let args = arg_vec(args)?;
      if args.len() != argc {
        return Err(EvaluationError::WrongNumberOfArguments);
      }
      builtin(args, scope)
    }
    Procedure::BuiltinVariableArgumentForm(builtin, argc) => {
      let args = arg_vec(args)?;
      if args.len() < argc {
        return Err(EvaluationError::WrongNumberOfArguments);
      }
      let varargs = args[argc..].to_vec();
      let args = args[0..argc].to_vec();
      builtin(args, varargs, scope)
    }
    _ => Err(EvaluationError::UnknownFunctionName),
  }
}

pub fn evaluate(expression: &Expression, scope: Rc<RefCell<Scope>>) -> EvaluationResult {
  match expression {
    Expression::Symbol(symbol) => scope.borrow().lookup(symbol),
    Expression::Cons(cons) => match evaluate(cons.car.as_ref(), scope.clone())? {
      Expression::Procedure(procedure) => evaluate_procedure(procedure, cons.cdr.as_ref(), scope),
      _ => Err(EvaluationError::NotAProcedure),
    },
    _ => Ok(expression.clone()),
  }
}
