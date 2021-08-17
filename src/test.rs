// We do this awkward module hoist so we don't have to label everything with #[cfg(test)]
#[cfg(test)]
mod _test {
    use crate::evaluate::EvaluationError;
    use crate::*;
    use std::cell::RefCell;
    use std::rc::Rc;

    /// A convenient helper for writing more concise tests
    pub struct TestContext {
        pub scope: Rc<RefCell<Scope>>,
    }

    impl TestContext {
        pub fn new() -> TestContext {
            TestContext {
                scope: Scope::builtins(),
            }
        }
        /// Make a state change to the test context
        pub fn exec(&self, string: &str) {
            evaluate(&parse(string).unwrap(), self.scope.clone()).unwrap();
        }
        /// Sugar around assert_eq!(evaluate(parse(...)), Ok(...))
        pub fn assert_eq(&self, string: &str, expected: Expression) {
            assert_eq!(
                evaluate(&parse(string).unwrap(), self.scope.clone()),
                Ok(expected)
            );
        }
        /// Sugar around assert_eq!(evaluate(parse(...)), Err(...))
        pub fn assert_err(&self, string: &str, error: EvaluationError) {
            assert_eq!(
                evaluate(&parse(string).unwrap(), self.scope.clone()),
                Err(error)
            );
        }
    }
}

#[cfg(test)]
pub use _test::*;
