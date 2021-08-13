use super::*;

macro_rules! comparison_operator {
    ($comparison:tt, $args:expr, $varargs:expr, $scope:expr) => {{
        // Evaluate the arguments and verify that all results are numbers
        // fn to_number(expression: &Expression) -> Result<Number, EvaluationError> {
        // }
        let to_number = |expression| match evaluate(expression, $scope.clone())? {
            Expression::Number(number) => Ok(number),
            _ => Err(EvaluationError::InvalidArgument),
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
    comparison_operator!(==, args, varargs, scope)
}

pub const EQUALS: Expression =
    Expression::Procedure(Procedure::BuiltinVariableArgumentForm(_equals, 1));

#[cfg(test)]
mod test {
    use super::*;
    use crate::parse::parse;

    #[test]
    fn test_evaluate_equals() {
        let scope = Scope::builtins();
        assert_eq!(
            evaluate(&parse("(= 1)").unwrap(), scope.clone()),
            Ok(boolean!(true))
        );
        assert_eq!(
            evaluate(&parse("(= 1 1)").unwrap(), scope.clone()),
            Ok(boolean!(true))
        );
        assert_eq!(
            evaluate(&parse("(= 1 1 1 1 1 1 1)").unwrap(), scope.clone()),
            Ok(boolean!(true))
        );
        assert_eq!(
            evaluate(&parse("(= (+ 2 2) (* 2 2))").unwrap(), scope.clone()),
            Ok(boolean!(true))
        );
        assert_eq!(
            evaluate(
                &parse("(= (+ 2 2) (* 2 2) (- (* 2 2 2) (+ 1 1 1 1)))").unwrap(),
                scope.clone()
            ),
            Ok(boolean!(true))
        );
        assert_eq!(
            evaluate(&parse("(= 'foo)").unwrap(), scope.clone()),
            Err(EvaluationError::InvalidArgument)
        );
    }
}
