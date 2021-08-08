use super::*;
use crate::evaluate::{define_builtins, EvaluationError, EvaluationResult};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Scope {
  global: Option<Rc<RefCell<Scope>>>,
  mapping: HashMap<String, Expression>,
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      global: None,
      mapping: HashMap::new(),
    }
  }
  pub fn child(parent: Rc<RefCell<Scope>>) -> Rc<RefCell<Scope>> {
    let mut scope = Scope::new();
    scope.global = if let Some(global) = &parent.clone().borrow().global {
      Some(global.clone())
    } else {
      Some(parent)
    };
    Rc::new(RefCell::new(scope))
  }
  pub fn builtins() -> Rc<RefCell<Scope>> {
    let scope = Rc::new(RefCell::new(Scope::new()));
    define_builtins(scope.clone());
    scope
  }
  pub fn define(&mut self, symbol: &str, expression: Expression) {
    self.mapping.insert(String::from(symbol), expression);
  }
  pub fn lookup(&self, symbol: &str) -> EvaluationResult {
    match self.mapping.get(symbol) {
      Some(expression) => Ok(expression.clone()),
      None => {
        if let Some(global) = &self.global {
          global.borrow().lookup(symbol)
        } else {
          Err(EvaluationError::UndefinedSymbol)
        }
      }
    }
  }
}
