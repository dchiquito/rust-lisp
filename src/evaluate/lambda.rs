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
    Expression::Cons(_) => {
      let mut args = vec![];
      while let Expression::Cons(cons) = formals {
        if let Expression::Symbol(symbol) = cons.car.as_ref() {
          args.push(symbol.clone());
          formals = cons.cdr.as_ref();
        } else {
          return Err(EvaluationError::InvalidArgument);
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
          Err(EvaluationError::InvalidArgument)
        }
      } else {
        Ok(ProcedureValue::Expression(procedure!(args, body)))
      }
    }
    _ => Err(EvaluationError::InvalidArgument),
  }
}
pub const LAMBDA: Expression =
  Expression::Procedure(Procedure::BuiltinVariableArgumentForm(_lambda, 2));

#[cfg(test)]
mod test {
  use super::*;
  use crate::parse::parse;

  #[test]
  fn test_lambda_inline() {
    let scope = Scope::builtins();
    assert_eq!(
      evaluate(&parse("((lambda (x) (+ x 1)) 5)").unwrap(), scope.clone()),
      Ok(int!(6))
    );
    assert_eq!(
      evaluate(
        &parse("((lambda x (cons 0 x)) 1 2)").unwrap(),
        scope.clone()
      ),
      Ok(list!(&int!(0), &int!(1), &int!(2)))
    );
    assert_eq!(
      evaluate(
        &parse("((lambda (x . y) (cons x y)) 1 2 3)").unwrap(),
        scope.clone()
      ),
      Ok(list!(&int!(1), &int!(2), &int!(3)))
    );
  }
  #[test]
  fn test_lambda_define() {
    let scope = Scope::builtins();
    evaluate(
      &parse("(define square (lambda (a) (* a a)))").unwrap(),
      scope.clone(),
    )
    .unwrap();
    for i in -20..20 {
      assert_eq!(
        evaluate(&parse(&format!("(square {})", i)).unwrap(), scope.clone()),
        Ok(int!(i * i))
      );
    }
  }
  #[test]
  fn test_lambda_fibonacci_naive() {
    let scope = Scope::builtins();
    evaluate(
      &parse(
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
      )
      .unwrap(),
      scope.clone(),
    )
    .unwrap();
    assert_eq!(
      evaluate(&parse("(fibonacci 0)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(fibonacci 1)").unwrap(), scope.clone()),
      Ok(int!(1))
    );
    assert_eq!(
      evaluate(&parse("(fibonacci 2)").unwrap(), scope.clone()),
      Ok(int!(1))
    );
    assert_eq!(
      evaluate(&parse("(fibonacci 3)").unwrap(), scope.clone()),
      Ok(int!(2))
    );
    assert_eq!(
      evaluate(&parse("(fibonacci 4)").unwrap(), scope.clone()),
      Ok(int!(3))
    );
    assert_eq!(
      evaluate(&parse("(fibonacci 5)").unwrap(), scope.clone()),
      Ok(int!(5))
    );
    assert_eq!(
      evaluate(&parse("(fibonacci 6)").unwrap(), scope.clone()),
      Ok(int!(8))
    );
  }
  #[test]
  fn test_lambda_tail_call_recursion() {
    let scope = Scope::builtins();
    evaluate(
      &parse(
        "
(define loopy (lambda (index)
  (cond
    ((eq? index 0) 0)
    (else (loopy (- index 1)))
  )
))",
      )
      .unwrap(),
      scope.clone(),
    )
    .unwrap();
    assert_eq!(
      evaluate(&parse("(loopy 0)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(loopy 1)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    assert_eq!(
      evaluate(&parse("(loopy 2)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
    // This will stack overflow unless tail call recursion is working correctly
    assert_eq!(
      evaluate(&parse("(loopy 10000)").unwrap(), scope.clone()),
      Ok(int!(0))
    );
  }
}
