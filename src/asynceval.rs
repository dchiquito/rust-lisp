use crate::*;
use std::collections::VecDeque;
use std::fmt;

/*
Planning
Basic types
BindingLayer - just a HashMap<String, Expression>
Bindings - a global BindingLayer and a stack of BindingLayers. Each procedure call has a new scope, represented by a new BindingLayer.
Procedure - enum of lambdas and builtins. Lambdas always take evaluated arguments, but some builtins take in the arguments without evaluating them (like quote).

The VM is a state machine that mutates in one tick increments.
It is built around a stack of Frames (computations in progress) and a Bindings stack (scoped variables).
It is sadly more complex than a simple call stack, since computations must be performed to determine the arguments to a procedure call.
When a frame performs its last tick, it is removed from the stack and passes an Expression value up for the parent Frame to consume.
When the final frame finishes computation, the resulting Expression is stored in the State as the result of the evaluation.
EvaluateFrame - an expression to be evaluated. tick behavior depends on type:
    If it's a primitive, it is pushed into the parent frame, because the parent frame asked for it to be evaluated.
    If it's a procedure call, it is converted into an ArgParseFrame.
ArgParseFrame - a Procedure, a Bindings that is being generated, a list of argument names, and a list of provided arguments.
        If argnames+args are non-empty, then the first arg is popped off.
            If it is a primitive, it is assigned to the argname in the inner binding and the tick completes.
            If it is procedure call, then a new Frame is pushed calling that procedure.
                When that procedure completes in a future tick, the argname is popped off and the return value assigned to the Bindings.
        If argnames+args are both empty, then the completed Bindings is pushed on to the binding stack and a new *CallFrame invoking the procedure replaces this Frame.
BuiltinCallFrame - a BuiltinProcedure, a Bindings, and a tick counter
    Builtins are easy, they just idle until their tick timer lapses, then return the result of calling the rust function.
LambdaCallFrame - a cons list of expressions to evaluate and a Bindings.
    The expression list is just the program from the LambdaProcedure, but cloned.
    Ticking involves popping off the first expression from the list and pushing a new EvaluateFrame onto the stack.
Frame - an enum of all *Frame types.
State - a global Bindings and a stack (vec) of Frames. Every tick
*/

#[derive(Debug, Eq, PartialEq, Clone)]
pub enum EvaluationErrorCause {
    WrongNumberOfArguments(String, usize, usize),
    WrongNumberOfVariableArguments(String, usize, usize),
    InvalidArgument(String, String, Expression),
    UndefinedSymbol(String),
    DivideByZero(Number),
    NotAProcedure(Expression),
}
impl fmt::Display for EvaluationErrorCause {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EvaluationErrorCause::WrongNumberOfArguments(procedure_name, expected, actual) => {
                write!(
                    fmt,
                    "wrong number of arguments for {}: expected {}, got {}",
                    procedure_name, expected, actual
                )
            }
            EvaluationErrorCause::WrongNumberOfVariableArguments(
                procedure_name,
                expected,
                actual,
            ) => {
                write!(
                    fmt,
                    "wrong number of arguments for {}: expected {} or more, got {}",
                    procedure_name, expected, actual
                )
            }
            EvaluationErrorCause::InvalidArgument(procedure_name, expected, actual) => {
                write!(
                    fmt,
                    "invalid argument for {}: expected {}, got {}",
                    procedure_name, expected, actual
                )
            }
            EvaluationErrorCause::UndefinedSymbol(symbol) => {
                write!(fmt, "undefined symbol {}", symbol)
            }
            EvaluationErrorCause::DivideByZero(quotient) => {
                write!(fmt, "attempted to divide {} by 0", quotient)
            }
            EvaluationErrorCause::NotAProcedure(non_procedure) => {
                write!(
                    fmt,
                    "expected a procedure, given {}",
                    non_procedure.outer_representation()
                )
            }
        }
    }
}
#[derive(Debug, Eq, PartialEq, Clone)]
pub struct EvaluationError {
    cause: EvaluationErrorCause,
}
impl EvaluationError {
    pub fn invalid_argument(
        procedure_name: &str,
        expected: &str,
        actual: &Expression,
    ) -> EvaluationError {
        EvaluationError {
            cause: EvaluationErrorCause::InvalidArgument(
                procedure_name.to_string(),
                expected.to_string(),
                actual.clone(),
            ),
        }
    }
}
impl fmt::Display for EvaluationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        self.cause.fmt(fmt)
    }
}
// pub type EvaluationResult = Result<Expression, EvaluationError>;

trait FrameTrait {
    fn tick(self, state: &mut State);
}

#[derive(Debug)]
struct EvaluateFrame {
    expression: Expression,
}
impl FrameTrait for EvaluateFrame {
    fn tick(self, state: &mut State) {
        match self.expression {
            Expression::Cons(cons) => {
                let expr = cons.car.as_ref().clone();
                let args = cons.cdr.as_ref().clone();
                match expr {
                    Expression::Symbol(procedure_name) => {
                        if let Some(procedure) = state.bindings.get(&procedure_name) {
                            match procedure {
                                Expression::Procedure(procedure) => {
                                    state.parse_args(procedure.clone(), args)
                                }
                                _ => state.panic(EvaluationErrorCause::NotAProcedure(procedure)),
                            }
                        } else {
                            state.panic(EvaluationErrorCause::UndefinedSymbol(procedure_name))
                        }
                    }
                    Expression::Procedure(procedure) => state.parse_args(procedure, args),
                    _ => state.panic(EvaluationErrorCause::NotAProcedure(expr)),
                }
            }
            expr => state.pass_value_up(expr),
        }
    }
}

pub fn arg_vec(list: &Expression) -> VecDeque<Expression> {
    let mut args = VecDeque::from([]);
    let mut sublist = list;
    while let Expression::Cons(cons) = sublist {
        args.push_back(cons.car.as_ref().clone());
        sublist = cons.cdr.as_ref();
    }
    if sublist != &null!() {
        // TODO wrap this in a result
        panic!("aag can't parse args")
    }
    args
}

#[derive(Debug)]
struct ArgParseFrame {
    procedure: Procedure,
    new_bindings: BindingLayer,
    argnames: VecDeque<String>,
    arguments: VecDeque<Expression>,
}
impl ArgParseFrame {
    fn new(procedure: Procedure, arguments: Expression) -> ArgParseFrame {
        ArgParseFrame {
            argnames: VecDeque::from(procedure.argnames()),
            procedure,
            new_bindings: BindingLayer::new(),
            arguments: arg_vec(&arguments),
        }
    }
    fn take_value(&mut self, value: Expression) {
        let argname = self.argnames.pop_front().unwrap();
        println!("EGADS {} := {}", argname, value);
        self.new_bindings.insert(argname, value);
    }
}
impl FrameTrait for ArgParseFrame {
    fn tick(mut self, state: &mut State) {
        if self.arguments.is_empty() {
            if self.argnames.is_empty() {
                state.invoke(self.procedure, self.new_bindings)
            } else {
                // TODO check this before ticking into the arguments
                state.panic(EvaluationErrorCause::WrongNumberOfArguments(
                    "#<procedure>".to_string(),
                    666,
                    666,
                ))
            }
        } else {
            let arg_expression = self.arguments.pop_front().unwrap();
            state.push_frame(Frame::ArgParseFrame(self));
            state.push_frame(Frame::EvaluateFrame(EvaluateFrame {
                expression: arg_expression,
            }))
        }
    }
}

#[derive(Debug)]
struct BuiltinCallFrame {
    procedure: BuiltinProcedure,
    ticks: i32,
}
impl BuiltinCallFrame {
    fn new(procedure: BuiltinProcedure) -> Frame {
        Frame::BuiltinCallFrame(BuiltinCallFrame {
            procedure,
            ticks: 0,
        })
    }
}
impl FrameTrait for BuiltinCallFrame {
    fn tick(mut self, state: &mut State) {
        println!("ticky {} {}", self.ticks, self.procedure.ticks);
        self.ticks += 1;
        if self.ticks < self.procedure.ticks {
            println!("incr");
            state.push_frame(Frame::BuiltinCallFrame(self))
        } else {
            println!("returno");
            let program = self.procedure.program;
            let value = program(&mut state.bindings);
            state.pass_value_up(value)
        }
    }
}

#[derive(Debug)]
struct LambdaCallFrame {/* ... TODO ... */}
impl LambdaCallFrame {
    fn new(procedure: LambdaProcedure) -> Frame {
        Frame::LambdaCallFrame(LambdaCallFrame {})
    }
}

#[derive(Debug)]
enum Frame {
    EvaluateFrame(EvaluateFrame),
    ArgParseFrame(ArgParseFrame),
    BuiltinCallFrame(BuiltinCallFrame),
    LambdaCallFrame(LambdaCallFrame),
}
impl Frame {
    fn tick(self, state: &mut State) {
        match self {
            Frame::EvaluateFrame(frame) => frame.tick(state),
            Frame::ArgParseFrame(frame) => frame.tick(state),
            Frame::BuiltinCallFrame(frame) => frame.tick(state),
            _ => panic!("unsupported frame type"),
            // Frame::LambdaCallFrame(frame) => frame.tick(state),
        }
    }
    fn take_value(&mut self, value: Expression) {
        match self {
            Frame::ArgParseFrame(frame) => frame.take_value(value),
            _ => panic!("this frame can't take a value"),
            // Frame::BuiltinCallFrame(frame) => frame.tick(state),
            // Frame::LambdaCallFrame(frame) => frame.tick(state),
        }
    }
}

#[derive(Debug)]
pub struct State {
    bindings: Bindings,
    frames: Vec<Frame>,
    value: Option<Result<Expression, EvaluationError>>,
}
impl State {
    pub fn empty() -> State {
        State {
            bindings: Bindings::new(),
            frames: vec![],
            value: None,
        }
    }
    pub fn bind_builtin(&mut self, builtin: Expression) {
        self.bindings.bind_builtin(builtin);
    }
    pub fn begin(&mut self, expression: Expression) {
        self.push_frame(Frame::EvaluateFrame(EvaluateFrame { expression }))
    }
    pub fn tick(&mut self) {
        if self.value.is_none() {
            let frame = self.frames.pop().unwrap();
            frame.tick(self);
        }
    }
    pub fn get_value(&mut self) -> Option<Result<Expression, EvaluationError>> {
        if self.value.is_some() {
            self.value.clone()
        } else {
            None
        }
    }
    pub fn run_to_completion(&mut self) {
        while self.value == None {
            self.tick();
        }
    }
    fn push_frame(&mut self, frame: Frame) {
        self.frames.push(frame);
    }
    fn parse_args(&mut self, procedure: Procedure, arguments: Expression) {
        self.push_frame(Frame::ArgParseFrame(ArgParseFrame::new(
            procedure, arguments,
        )))
    }
    fn pass_value_up(&mut self, value: Expression) {
        if let Some(frame) = self.frames.last_mut() {
            frame.take_value(value);
        } else {
            self.value = Some(Ok(value));
        }
    }
    fn invoke(&mut self, procedure: Procedure, bindings: BindingLayer) {
        self.bindings.push(bindings);
        let frame = match procedure {
            Procedure::BuiltinProcedure(builtin) => BuiltinCallFrame::new(builtin),
            Procedure::LambdaProcedure(lambda) => LambdaCallFrame::new(lambda),
        };
        self.push_frame(frame)
    }
    fn panic(&mut self, cause: EvaluationErrorCause) {
        self.value = Some(Err(EvaluationError { cause }));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_call_builtin() {
        let mut state = State::empty();
        state.bind_builtin(builtin! {
            fn + (a:Number, b:Number) => int!(a+b)
        });
        state.bindings.bind("foo", int!(6));
        state.begin(parse("(+ 1 foo)").unwrap());
        state.run_to_completion();
        assert_eq!(state.value, Some(Ok(int!(7))))
    }

    #[test]
    fn test_foo() {
        let mut state = State::empty();
        state.bindings.bind("foo", int!(6));
        state.begin(parse("(- 1 2)").unwrap());
        println!("{:?}\n", state);
        state.run_to_completion();
        assert_eq!(
            state.value,
            Some(Err(EvaluationError {
                cause: EvaluationErrorCause::UndefinedSymbol("-".to_string())
            }))
        )
    }
}
