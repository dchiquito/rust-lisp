use super::*;
use std::cell::RefCell;
use std::rc::Rc;

fn _lambda(
  args: Vec<Expression>,
  varargs: Vec<Expression>,
  _scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
  let mut formals = args.get(0).unwrap();
  // lambda requires two arguments: the formals, and at least one statement in the body.
  // We will graft that required statement onto the beginning of varargs to build to the statement list.
  let first_statement = args.get(1).unwrap();
  let mut body = varargs;
  body.insert(0, first_statement.clone());
  match formals {
    Expression::Symbol(symbol) => Ok(ProcedureValue::Expression(procedure!(
      vec![],
      symbol.clone(),
      body
    ))),
    Expression::Cons(_) | Expression::Null => {
      let mut args = vec![];
      while let Expression::Cons(cons) = formals {
        if let Expression::Symbol(symbol) = cons.car.as_ref() {
          args.push(symbol.clone());
          formals = cons.cdr.as_ref();
        } else {
          return Err(EvaluationError::invalid_argument(
            "lambda",
            "list of symbols",
            formals,
          ));
        }
      }
      if formals != &null!() {
        // Variable argument forms are encoded using an improper list as the lambda arguments
        if let Expression::Symbol(symbol) = formals {
          Ok(ProcedureValue::Expression(procedure!(
            args,
            symbol.clone(),
            body
          )))
        } else {
          Err(EvaluationError::invalid_argument(
            "lambda", "symbol", formals,
          ))
        }
      } else {
        Ok(ProcedureValue::Expression(procedure!(args, body)))
      }
    }
    _ => Err(EvaluationError::invalid_argument("lambda", "list", formals)),
  }
}
pub const LAMBDA: Procedure = Procedure::BuiltinVariableArgumentForm("lambda", _lambda, 2);

#[cfg(test)]
mod test {
  use super::*;
  use crate::test::TestContext;

  #[test]
  fn test_lambda_inline() {
    let ctx = TestContext::new();
    ctx.assert_eq("((lambda (x) (+ x 1)) 5)", int!(6));
    ctx.assert_eq(
      "((lambda x (cons 0 x)) 1 2)",
      list!(&int!(0), &int!(1), &int!(2)),
    );
    ctx.assert_eq(
      "((lambda (x . y) (cons x y)) 1 2 3)",
      list!(&int!(1), &int!(2), &int!(3)),
    );
    ctx.assert_eq("((lambda () 1 2 3))", int!(3));
    ctx.assert_err(
      "((lambda (x) x))",
      EvaluationError::WrongNumberOfArguments("#<procedure>".to_string(), 1, 0),
    );
    ctx.assert_err(
      "((lambda (x) x) 1 2)",
      EvaluationError::WrongNumberOfArguments("#<procedure>".to_string(), 1, 2),
    );
    ctx.assert_err(
      "((lambda (x . y) x))",
      EvaluationError::WrongNumberOfVariableArguments("#<procedure>".to_string(), 1, 0),
    );
    ctx.assert_err(
      "(lambda 1 1)",
      EvaluationError::invalid_argument("lambda", "list", &int!(1)),
    );
    ctx.assert_err(
      "(lambda (1) 1)",
      EvaluationError::invalid_argument("lambda", "list of symbols", &list!(&int!(1))),
    );
    ctx.assert_err(
      "(lambda (x . 2) 2)",
      EvaluationError::invalid_argument("lambda", "symbol", &int!(2)),
    );
  }
  #[test]
  fn test_lambda_define() {
    let ctx = TestContext::new();
    ctx.exec("(define square (lambda (a) (* a a)))");
    for i in -20..20 {
      ctx.assert_eq(&format!("(square {})", i), int!(i * i));
    }
  }
  #[test]
  fn test_lambda_fibonacci_naive() {
    let ctx = TestContext::new();
    ctx.exec(
      "
(define fibonacci (lambda (index)
  (cond
    ((eq? index 0) 0)
    ((eq? index 1) 1)
    (else (+
      (fibonacci (- index 1))
      (fibonacci (- index 2))
    ))
  )
))",
    );
    ctx.assert_eq("(fibonacci 0)", int!(0));
    ctx.assert_eq("(fibonacci 1)", int!(1));
    ctx.assert_eq("(fibonacci 2)", int!(1));
    ctx.assert_eq("(fibonacci 3)", int!(2));
    ctx.assert_eq("(fibonacci 4)", int!(3));
    ctx.assert_eq("(fibonacci 5)", int!(5));
    ctx.assert_eq("(fibonacci 6)", int!(8));
  }
  #[test]
  fn test_lambda_tail_call_recursion() {
    let ctx = TestContext::new();
    ctx.exec(
      "
(define loopy (lambda (index)
  (cond
    ((eq? index 0) 0)
    (else (loopy (- index 1)))
  )
))",
    );
    ctx.assert_eq("(loopy 0)", int!(0));
    ctx.assert_eq("(loopy 1)", int!(0));
    ctx.assert_eq("(loopy 2)", int!(0));
    // This will stack overflow unless tail call recursion is working correctly
    ctx.assert_eq("(loopy 10000)", int!(0));
  }

  #[test]
  fn test_lambda_not_a_procedure() {
    let ctx = TestContext::new();
    ctx.exec("(define foo 1)");
    ctx.assert_err("(foo)", EvaluationError::NotAProcedure(int!(1)));
  }
}
