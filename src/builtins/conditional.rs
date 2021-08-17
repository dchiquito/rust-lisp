use super::*;

fn _cond(
  _args: Vec<Expression>,
  mut varargs: Vec<Expression>,
  scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  // Validate all of the clauses before evaluating anything
  // All clauses must be non-empty lists
  for clause in &varargs {
    let clause = arg_vec("cond", clause)?;
    if clause.is_empty() {
      return Err(EvaluationError::invalid_argument(
        "cond",
        "clause is not a test-value pair",
        &null!(),
      ));
    }
  }
  // The final else clause (if it exists) needs to be handled specially
  let else_clause = if let Some(last_clause) = varargs.pop() {
    let last_clause_vec = arg_vec("cond", &last_clause)?;
    if last_clause_vec.get(0) == Some(&symbol!("else")) {
      if last_clause_vec.len() < 2 {
        // the else clause needs at least one expression
        return Err(EvaluationError::invalid_argument(
          "cond",
          "missing expressions in else clause",
          &last_clause,
        ));
      }
      Some(last_clause)
    } else {
      // No else, put the last clause back
      varargs.push(last_clause);
      None
    }
  } else {
    None
  };
  for clause in varargs {
    let clause = arg_vec("cond", &clause)?;
    // This was verified as the first step of this function
    let test = clause.get(0).unwrap();
    let test = evaluate(test, scope.clone())?;
    if test != boolean!(false) {
      let mut expression = test;
      let expressions = if clause.get(1).unwrap_or(&void!()) == &symbol!("=>") {
        // case clauses may be of the form (test => expression ...)
        // If so, skip the test and the =>
        clause[2..].iter()
      } else {
        // case clauses may also be of the form (test expression ...)
        // If so, skip the test since we have already evaluated it
        clause[1..].iter()
      };
      for line in expressions {
        expression = evaluate(line, scope.clone())?;
      }
      return Ok(ProcedureValue::Expression(expression));
    }
  }
  if let Some(else_clause) = else_clause {
    let else_clause = arg_vec("cond", &else_clause)?;
    let mut else_body = else_clause[1..].iter();
    let mut expression = evaluate_in_tail_position(else_body.next().unwrap(), scope.clone())?;
    for next_expression in else_body {
      expression = evaluate_in_tail_position(next_expression, scope.clone())?;
    }
    Ok(expression)
  } else {
    Ok(ProcedureValue::Expression(void!()))
  }
}

pub const COND: Procedure = Procedure::BuiltinVariableArgumentForm("cond", _cond, 0);

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::TestContext;

  #[test]
  fn test_evaluate_cond() {
    let ctx = TestContext::new();
    ctx.assert_eq("(cond)", void!());
    ctx.assert_eq("(cond (#t 1))", int!(1));
    ctx.assert_eq("(cond (#t #f))", boolean!(false));
    ctx.assert_eq("(cond (1 1))", int!(1));
    ctx.assert_eq("(cond (2 1))", int!(1));
    ctx.assert_eq("(cond (2))", int!(2));
    ctx.assert_eq("(cond (#f))", void!());
    ctx.assert_eq("(cond (#f #t))", void!());
    ctx.assert_eq("(cond (#f) (2))", int!(2));
    ctx.assert_eq("(cond ((eq? 1 1) 2) (3))", int!(2));
    ctx.assert_eq("(cond (#f #f) (else #f #t))", boolean!(true));
    ctx.assert_err(
      "(cond 5)",
      EvaluationError::invalid_argument("cond", "list", &int!(5)),
    );
    ctx.assert_err(
      "(cond ())",
      EvaluationError::invalid_argument("cond", "clause is not a test-value pair", &list!()),
    );
    ctx.assert_err(
      "(cond (else))",
      EvaluationError::invalid_argument(
        "cond",
        "missing expressions in else clause",
        &list!(&symbol!("else")),
      ),
    )
  }

  #[test]
  fn test_evaluate_cond_equal_gt() {
    let ctx = TestContext::new();
    ctx.assert_eq("(cond (#t => 1))", int!(1));
    ctx.assert_eq("(cond (1 => 1))", int!(1));
    ctx.assert_eq("(cond (2 => 1))", int!(1));
    ctx.assert_eq("(cond (#f => #t))", void!());
    ctx.assert_eq("(cond (#f) (2 => 3))", int!(3));
  }
  #[test]
  fn test_evaluate_cond_else() {
    let ctx = TestContext::new();
    ctx.assert_eq("(cond (else 1))", int!(1));
    ctx.assert_eq("(cond (else #f))", boolean!(false));
    // TODO more else tests
    ctx.assert_eq("(cond (#f) (else ()))", list!());
    ctx.assert_eq(
      "(cond ((eq? 1 2) => 1) ((eq? 2 3) => 2) ((eq? 3 4) 3) (else 4))",
      int!(4),
    );
  }
}
