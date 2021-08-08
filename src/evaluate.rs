mod arithmetic;
mod comparison;
mod conditional;
mod define;
mod lambda;
mod pair;
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

#[derive(Clone, Debug)]
pub enum ProcedureValue {
  Expression(Expression),
  TailCall(Procedure, Expression, Rc<RefCell<Scope>>),
}
impl ProcedureValue {
  pub fn resolve(&self) -> EvaluationResult {
    match self {
      ProcedureValue::Expression(expression) => Ok(expression.clone()),
      ProcedureValue::TailCall(procedure, expression, arg_scope) => {
        evaluate_procedure(procedure, expression, arg_scope.clone())
      }
    }
  }
}
pub type ProcedureResult = Result<ProcedureValue, EvaluationError>;

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

fn vec_arg(mut args: Vec<Expression>) -> EvaluationResult {
  let mut list = null!();
  while let Some(arg) = args.pop() {
    list = cons!(&arg, &list);
  }
  Ok(list)
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
  scope.define("cons", pair::CONS);
  scope.define("car", pair::CAR);
  scope.define("cdr", pair::CDR);
  scope.define("define", define::DEFINE);
  scope.define("lambda", lambda::LAMBDA);
  scope.define("cond", conditional::COND);
}

/// Evaluate all the lines in the body, and return the last result
fn _evaluate_procedure_body(body: &Vec<Expression>, scope: Rc<RefCell<Scope>>) -> ProcedureResult {
  // let last_line = body.pop().unwrap();

  let mut body = body.iter();
  let mut return_value = evaluate_in_tail_position(body.next().unwrap(), scope.clone())?;
  for line in body {
    return_value = evaluate_in_tail_position(line, scope.clone())?;
  }
  Ok(return_value)
}

fn _evaluate_procedure(
  procedure: &Procedure,
  args: &Expression,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  let args = arg_vec(args)?;
  match &procedure {
    Procedure::FixedArgumentForm(arg_names, body) => {
      if args.len() != arg_names.len() {
        return Err(EvaluationError::WrongNumberOfArguments);
      }
      // Bind the arguments to their symbols
      let inner_scope = Scope::child(scope.clone());
      for (arg_name, arg) in arg_names.iter().zip(args.iter()) {
        inner_scope
          .borrow_mut()
          .define(arg_name, evaluate(&arg.clone(), scope.clone())?);
      }
      _evaluate_procedure_body(body, inner_scope)
    }
    Procedure::VariableArgumentForm(arg_names, vararg_name, body) => {
      if args.len() < arg_names.len() {
        return Err(EvaluationError::WrongNumberOfArguments);
      }
      // Bind the arguments to their symbols
      let inner_scope = Scope::child(scope.clone());
      // Bind the named arguments
      for (arg_name, arg) in arg_names.iter().zip(args.iter()) {
        inner_scope
          .borrow_mut()
          .define(arg_name, evaluate(&arg.clone(), scope.clone())?);
      }
      let varargs = args[arg_names.len()..].to_vec();
      let mut evaluated_varargs = vec![];
      for vararg in &varargs {
        evaluated_varargs.push(evaluate(vararg, scope.clone())?);
      }
      inner_scope
        .borrow_mut()
        .define(&vararg_name, vec_arg(evaluated_varargs)?);
      _evaluate_procedure_body(body, inner_scope)
    }
    Procedure::BuiltinFixedArgumentForm(builtin, argc) => {
      if args.len() != *argc {
        return Err(EvaluationError::WrongNumberOfArguments);
      }
      builtin(args, scope)
    }
    Procedure::BuiltinVariableArgumentForm(builtin, argc) => {
      if args.len() < *argc {
        return Err(EvaluationError::WrongNumberOfArguments);
      }
      let varargs = args[*argc..].to_vec();
      let args = args[0..*argc].to_vec();
      builtin(args, varargs, scope)
    }
  }
}
fn evaluate_procedure(
  procedure: &Procedure,
  args: &Expression,
  scope: Rc<RefCell<Scope>>,
) -> EvaluationResult {
  let mut procedure_value = _evaluate_procedure(procedure, args, scope)?;
  while let ProcedureValue::TailCall(child_procedure, args, arg_scope) = procedure_value {
    // if &child_procedure == procedure {
    procedure_value = _evaluate_procedure(&child_procedure, &args, arg_scope)?
    // } else {
    //   return evaluate_procedure(&child_procedure, &args, arg_scope);
    // }
  }
  match procedure_value {
    ProcedureValue::Expression(expression) => Ok(expression),
    ProcedureValue::TailCall(_, _, _) => panic!(),
  }
}

/// This should behave identically to evaluate, but return a wrapper around a procedure call
/// instead of calling it immediately. This allows evaluate_procedure to call the procedure in a
/// loop instead of evaluating it recursively.
/// This function should always be used to evaluate any expression in a tail position.
pub fn evaluate_in_tail_position(
  expression: &Expression,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  match expression {
    Expression::Symbol(symbol) => Ok(ProcedureValue::Expression(scope.borrow().lookup(symbol)?)),
    Expression::Cons(cons) => match evaluate(cons.car.as_ref(), scope.clone())? {
      Expression::Procedure(procedure) => {
        // Save the procedure call in a TailCall rather than executing it immediately
        Ok(ProcedureValue::TailCall(
          procedure,
          cons.cdr.as_ref().clone(),
          scope.clone(),
        ))
      }
      _ => Err(EvaluationError::NotAProcedure),
    },
    _ => Ok(ProcedureValue::Expression(expression.clone())),
  }
}

pub fn evaluate(expression: &Expression, scope: Rc<RefCell<Scope>>) -> EvaluationResult {
  evaluate_in_tail_position(expression, scope.clone())?.resolve()
}
