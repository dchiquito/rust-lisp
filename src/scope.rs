use super::*;
use crate::evaluate::{EvaluationError, EvaluationResult};
use std::collections::HashMap;

pub struct Scope {
  mapping: HashMap<String, Expression>,
}

impl Scope {
  pub fn new() -> Scope {
    Scope {
      mapping: HashMap::new(),
    }
  }
  pub fn define(&mut self, symbol: String, expression: Expression) {
    self.mapping.insert(symbol, expression);
  }
  pub fn lookup(&self, symbol: &String) -> EvaluationResult {
    match self.mapping.get(symbol) {
      Some(expression) => Ok(expression.clone()),
      None => Err(EvaluationError::UndefinedSymbol),
    }
  }
}
