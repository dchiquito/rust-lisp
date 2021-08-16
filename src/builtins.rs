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

/// Plop all of the builtins into the given scope
pub fn define_builtins(scope: Rc<RefCell<Scope>>) {
  let mut scope = scope.borrow_mut();
  scope.define("+", arithmetic::ADD);
  scope.define("*", arithmetic::MULTIPLY);
  scope.define("-", arithmetic::SUBTRACT);
  scope.define("/", arithmetic::DIVIDE);
  scope.define("eq?", equality::EQ);
  scope.define("=", comparison::EQUALS);
  scope.define("<", comparison::LESS_THAN);
  scope.define(">", comparison::GREATER_THAN);
  scope.define("<=", comparison::LESS_THAN_OR_EQUAL);
  scope.define(">=", comparison::GREATER_THAN_OR_EQUAL);
  scope.define("quote", quote::QUOTE);
  scope.define("cons", pair::CONS);
  scope.define("car", pair::CAR);
  scope.define("cdr", pair::CDR);
  scope.define("define", define::DEFINE);
  scope.define("lambda", lambda::LAMBDA);
  scope.define("cond", conditional::COND);
}
