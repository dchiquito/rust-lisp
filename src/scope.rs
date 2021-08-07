use super::*;
use crate::evaluate::{define_builtins, EvaluationError, EvaluationResult};
use std::collections::HashMap;

pub struct Scope {
  parent: Option<Rc<RefCell<Scope>>>,
  mapping: HashMap<String, Expression>,
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      parent: None,
      mapping: HashMap::new(),
    }
  }
  pub fn child(parent: Rc<RefCell<Scope>>) -> Rc<RefCell<Scope>> {
    let mut scope = Scope::new();
    scope.parent = Some(parent);
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
  pub fn lookup(&self, symbol: &String) -> EvaluationResult {
    match self.mapping.get(symbol) {
      Some(expression) => Ok(expression.clone()),
      None => {
        if let Some(parent) = &self.parent {
          parent.borrow().lookup(symbol)
        } else {
          Err(EvaluationError::UndefinedSymbol)
        }
      }
    }
  }
}
