use super::*;

macro_rules! comparison_operator {
    ($operator_name:expr, $comparison:tt, $args:expr, $varargs:expr, $scope:expr) => {{
        // Evaluate the arguments and verify that all results are numbers
        // fn to_number(expression: &Expression) -> Result<Number, EvaluationError> {
        // }
        let to_number = |expression| match evaluate(expression, $scope.clone())? {
            Expression::Number(number) => Ok(number),
            non_number => Err(EvaluationError::invalid_argument($operator_name, "number", &non_number)),
        };
        let mut previous_arg = &to_number($args.get(0).unwrap())?;
        let varargs = $varargs
            .iter()
            .map(to_number)
            .collect::<Result<Vec<Number>, EvaluationError>>()?;
        for arg in varargs.iter() {
            if !(previous_arg $comparison arg) {
                return Ok(ProcedureValue::Expression(boolean!(false)));
            }
            previous_arg = arg;
        }
        Ok(ProcedureValue::Expression(boolean!(true)))
    }};
}

fn _equals(
    args: Vec<Expression>,
    varargs: Vec<Expression>,
    scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
    comparison_operator!("=", ==, args, varargs, scope)
}
pub const EQUALS: Procedure = Procedure::BuiltinVariableArgumentForm("=", _equals, 1);

fn _less_than(
    args: Vec<Expression>,
    varargs: Vec<Expression>,
    scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
    comparison_operator!("<", <, args, varargs, scope)
}
pub const LESS_THAN: Procedure = Procedure::BuiltinVariableArgumentForm("<", _less_than, 1);

fn _greater_than(
    args: Vec<Expression>,
    varargs: Vec<Expression>,
    scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
    comparison_operator!(">",>, args, varargs, scope)
}
pub const GREATER_THAN: Procedure = Procedure::BuiltinVariableArgumentForm(">", _greater_than, 1);

fn _less_than_or_equal(
    args: Vec<Expression>,
    varargs: Vec<Expression>,
    scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
    comparison_operator!("<=",<=, args, varargs, scope)
}
pub const LESS_THAN_OR_EQUAL: Procedure =
    Procedure::BuiltinVariableArgumentForm("<=", _less_than_or_equal, 1);

fn _greater_than_or_equal(
    args: Vec<Expression>,
    varargs: Vec<Expression>,
    scope: Rc<RefCell<Scope>>,
) -> ProcedureResult {
    comparison_operator!(">=", >=, args, varargs, scope)
}
pub const GREATER_THAN_OR_EQUAL: Procedure =
    Procedure::BuiltinVariableArgumentForm(">=", _greater_than_or_equal, 1);

#[cfg(test)]
mod test {
    use super::*;
    use crate::test::TestContext;

    #[test]
    fn test_evaluate_equals() {
        let ctx = TestContext::new();
        ctx.assert_eq("(= 1)", boolean!(true));
        ctx.assert_eq("(= 1 1)", boolean!(true));
        ctx.assert_eq("(= 1 1 1 1 1 1 1)", boolean!(true));
        ctx.assert_eq("(= 1 0)", boolean!(false));
        ctx.assert_eq("(= (+ 2 2) (* 2 2))", boolean!(true));
        ctx.assert_eq("(= (+ 1 2) (* 2 2))", boolean!(false));
        ctx.assert_eq(
            "(= (+ 2 2) (* 2 2) (- (* 2 2 2) (+ 1 1 1 1)))",
            boolean!(true),
        );
        ctx.assert_err(
            "(=)",
            EvaluationError::WrongNumberOfVariableArguments("=".to_string(), 1, 0),
        );
        ctx.assert_err(
            "(= 'foo)",
            EvaluationError::invalid_argument("=", "number", &symbol!("foo")),
        );
    }

    #[test]
    fn test_evaluate_less_than() {
        let ctx = TestContext::new();
        ctx.assert_eq("(< 1)", boolean!(true));
        ctx.assert_eq("(< 1 2)", boolean!(true));
        ctx.assert_eq("(< 2 1)", boolean!(false));
        ctx.assert_eq("(< 1 1)", boolean!(false));
        ctx.assert_eq("(< 1 2 3 4 5 6 7)", boolean!(true));
        ctx.assert_eq("(< (+ 1 1) (* 2 2))", boolean!(true));
        ctx.assert_eq("(< (* 2 2) (+ 1 1))", boolean!(false));
        ctx.assert_err(
            "(<)",
            EvaluationError::WrongNumberOfVariableArguments("<".to_string(), 1, 0),
        );
        ctx.assert_err(
            "(< 'foo)",
            EvaluationError::invalid_argument("<", "number", &symbol!("foo")),
        );
    }

    #[test]
    fn test_evaluate_greater_than() {
        let ctx = TestContext::new();
        ctx.assert_eq("(> 1)", boolean!(true));
        ctx.assert_eq("(> 2 1)", boolean!(true));
        ctx.assert_eq("(> 1 2)", boolean!(false));
        ctx.assert_eq("(> 2 2)", boolean!(false));
        ctx.assert_eq("(> 7 6 5 4 3 2 1)", boolean!(true));
        ctx.assert_eq("(> (* 2 2) (+ 1 1))", boolean!(true));
        ctx.assert_eq("(> (+ 1 1) (* 2 2))", boolean!(false));
        ctx.assert_err(
            "(>)",
            EvaluationError::WrongNumberOfVariableArguments(">".to_string(), 1, 0),
        );
        ctx.assert_err(
            "(> 'foo)",
            EvaluationError::invalid_argument(">", "number", &symbol!("foo")),
        );
    }

    #[test]
    fn test_evaluate_less_than_or_equal() {
        let ctx = TestContext::new();
        ctx.assert_eq("(<= 1)", boolean!(true));
        ctx.assert_eq("(<= 1 2)", boolean!(true));
        ctx.assert_eq("(<= 2 1)", boolean!(false));
        ctx.assert_eq("(<= 1 1)", boolean!(true));
        ctx.assert_eq("(<= 1 2 3 4 5 6 7)", boolean!(true));
        ctx.assert_eq("(<= 1 1 2 2 3 3)", boolean!(true));
        ctx.assert_eq("(<= (+ 1 1) (* 2 2))", boolean!(true));
        ctx.assert_eq("(<= (* 2 2) (+ 1 1))", boolean!(false));
        ctx.assert_err(
            "(<=)",
            EvaluationError::WrongNumberOfVariableArguments("<=".to_string(), 1, 0),
        );
        ctx.assert_err(
            "(<= 'foo)",
            EvaluationError::invalid_argument("<=", "number", &symbol!("foo")),
        );
    }

    #[test]
    fn test_evaluate_greater_than_or_equal() {
        let ctx = TestContext::new();
        ctx.assert_eq("(>= 1)", boolean!(true));
        ctx.assert_eq("(>= 2 1)", boolean!(true));
        ctx.assert_eq("(>= 1 2)", boolean!(false));
        ctx.assert_eq("(>= 2 2)", boolean!(true));
        ctx.assert_eq("(>= 7 6 5 4 3 2 1)", boolean!(true));
        ctx.assert_eq("(>= 3 3 2 2 1 1)", boolean!(true));
        ctx.assert_eq("(>= (* 2 2) (+ 1 1))", boolean!(true));
        ctx.assert_eq("(>= (+ 1 1) (* 2 2))", boolean!(false));
        ctx.assert_err(
            "(>=)",
            EvaluationError::WrongNumberOfVariableArguments(">=".to_string(), 1, 0),
        );
        ctx.assert_err(
            "(>= 'foo)",
            EvaluationError::invalid_argument(">=", "number", &symbol!("foo")),
        );
    }
}
