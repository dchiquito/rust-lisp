mod arithmetic;
mod comparison;
mod conditional;
mod define;
mod equality;
mod lambda;
mod pair;
mod quote;

use crate::evaluate::*;
use crate::*;
use std::cell::RefCell;
use std::rc::Rc;

fn define_builtin(scope: &mut Scope, procedure: Procedure) {
  match procedure {
    Procedure::BuiltinFixedArgumentForm(procedure_name, _, _) => {
      scope.define(procedure_name, Expression::Procedure(procedure))
    }
    Procedure::BuiltinVariableArgumentForm(procedure_name, _, _) => {
      scope.define(procedure_name, Expression::Procedure(procedure))
    }
    _ => panic!("Cannot define non-builtin procedure as builtin"),
  }
}

/// Plop all of the builtins into the given scope
pub fn define_builtins(scope: Rc<RefCell<Scope>>) {
  let scope = &mut scope.borrow_mut();
  define_builtin(scope, arithmetic::ADD);
  define_builtin(scope, arithmetic::MULTIPLY);
  define_builtin(scope, arithmetic::SUBTRACT);
  define_builtin(scope, arithmetic::DIVIDE);
  define_builtin(scope, equality::EQ);
  define_builtin(scope, comparison::EQUALS);
  define_builtin(scope, comparison::LESS_THAN);
  define_builtin(scope, comparison::GREATER_THAN);
  define_builtin(scope, comparison::LESS_THAN_OR_EQUAL);
  define_builtin(scope, comparison::GREATER_THAN_OR_EQUAL);
  define_builtin(scope, quote::QUOTE);
  define_builtin(scope, pair::CONS);
  define_builtin(scope, pair::CAR);
  define_builtin(scope, pair::CDR);
  define_builtin(scope, define::DEFINE);
  define_builtin(scope, lambda::LAMBDA);
  define_builtin(scope, conditional::COND);
}
