use super::*;
use crate::evaluate::{EvaluationError, EvaluationResult};
use std::collections::HashMap;

pub struct Scope {
  parent: Option<Box<Scope>>,
  mapping: HashMap<String, Expression>,
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      parent: None,
      mapping: HashMap::new(),
    }
  }
  pub fn define(&mut self, symbol: &str, expression: Expression) {
    self.mapping.insert(String::from(symbol), expression);
  }
  pub fn lookup(&self, symbol: &String) -> EvaluationResult {
    match self.mapping.get(symbol) {
      Some(expression) => Ok(expression.clone()),
      None => {
        if let Some(parent) = &self.parent {
          parent.lookup(symbol)
        } else {
          Err(EvaluationError::UndefinedSymbol)
        }
      }
    }
  }
}
