use crate::*;
use std::collections::VecDeque;

/*
Planning
Basic types
Bindings - just a HashMap<String, Expression>
Procedure - enum of lambdas and builtins. Lambdas always take evaluated arguments, but some builtins take in the arguments without evaluating them (like quote).
EvaluateFrame - an expression to be evaluated. tick behavior depends on type:
    If it's a primitive, it is pushed into the parent frame, because the parent frame asked for it to be evaluated.
    If it's a procedure call, it is converted into an ArgParseFrame.
ArgParseFrame - a Procedure, a Bindings that is being generated, a list of argument names, and a cons list of provided arguments.
        If argnames+args are non-empty, then the first arg is popped off.
            If it is a primitive, it is assigned to the argname in the inner binding and the tick completes.
            If it is procedure call, then a new Frame is pushed calling that procedure.
                When that procedure completes in a future tick, the argname is popped off and the return value assigned to the Bindings.
        If argnames+args are both empty, then the completed Bindings is pushed on to the binding stack and a new *CallFrame invoking the procedure replaces this Frame.
BuiltinCallFrame - a BuiltinProcedure, a Bindings, and a tick counter
    Builtins are easy, they just idle until their tick timer lapses, then return the result of calling the rust function.
LambdaCallFrame - a cons list of expressions to evaluate and a Bindings.
    The expression list is just the program from the LambdaProcedure, but cloned.
    Ticking involves popping off the first expression from the list and evaluating it onto the stack.
Frame - an enum of all *Frame types.
State - a global Bindings and a stack (vec) of Frames. Every tick
*/

// (add (sub 6 5) 2) => 3

trait FrameTrait {
    fn tick(self, state: State) -> State;
}

#[derive(Debug)]
struct EvaluateFrame {
    expression: Expression,
}
impl FrameTrait for EvaluateFrame {
    fn tick(self, state: State) -> State {
        match self.expression {
            Expression::Cons(cons) => {
                let expr = cons.car.as_ref().clone();
                let args = cons.cdr.as_ref().clone();
                match expr {
                    Expression::Symbol(procedure_name) => {
                        let procedure = state.bindings.get(&procedure_name).unwrap();
                        match procedure {
                            Expression::Procedure(procedure) => {
                                state.parse_args(procedure.clone(), args)
                            }
                            _ => panic!("non-procedure"),
                        }
                    }
                    Expression::Procedure(procedure) => state.parse_args(procedure, args),
                    _ => panic!("non-symbol"),
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
    fn tick(mut self, mut state: State) -> State {
        if self.arguments.is_empty() {
            if self.argnames.is_empty() {
                state.invoke(self.procedure, self.new_bindings)
            } else {
                panic!("not enough args")
            }
        } else {
            let arg_expression = self.arguments.pop_front().unwrap();
            state = state.push_frame(Frame::ArgParseFrame(self));
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
    fn tick(mut self, mut state: State) -> State {
        println!("ticky {} {}", self.ticks, self.procedure.ticks);
        self.ticks += 1;
        if self.ticks < self.procedure.ticks {
            println!("incr");
            state.push_frame(Frame::BuiltinCallFrame(self))
        } else {
            println!("returno");
            let program = self.procedure.program;
            let (value, new_bindings) = program(state.bindings);
            state.bindings = new_bindings;
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
    fn tick(self, state: State) -> State {
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
struct State {
    bindings: Bindings,
    frames: Vec<Frame>,
    value: Option<Expression>,
}
impl State {
    pub fn empty() -> State {
        State {
            bindings: Bindings::new(),
            frames: vec![],
            value: None,
        }
    }
    pub fn begin(mut self, expression: Expression) -> State {
        self.push_frame(Frame::EvaluateFrame(EvaluateFrame {expression}))
    }
    pub fn tick(mut self) -> State {
        if self.value.is_none() {
            let frame = self.frames.pop().unwrap();
            frame.tick(self)
        } else {
            self
        }
    }
    pub fn run_to_completion(mut self) -> State {
        while self.value == None {
            self = self.tick();
        }
        self
    }
    fn push_frame(mut self, frame: Frame) -> State {
        self.frames.push(frame);
        self
    }
    fn parse_args(mut self, procedure: Procedure, arguments: Expression) -> State {
        self.push_frame(Frame::ArgParseFrame(ArgParseFrame::new(
            procedure, arguments,
        )))
    }
    fn pass_value_up(mut self, value: Expression) -> State {
        if let Some(frame) = self.frames.last_mut() {
            frame.take_value(value);
        } else {
            self.value = Some(value);
        }
        self
    }
    fn invoke(mut self, procedure: Procedure, bindings: BindingLayer) -> State {
        self.bindings.push(bindings);
        let frame = match procedure {
            Procedure::BuiltinProcedure(builtin) => BuiltinCallFrame::new(builtin),
            Procedure::LambdaProcedure(lambda) => LambdaCallFrame::new(lambda),
        };
        self.push_frame(frame)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_foo() {
        let mut state = State::empty();
        // state.bindings.bind("foo", Expression::Procedure(Procedure::BuiltinProcedure(BuiltinProcedure {
        //     program: |bindings| (bindings.get("a").unwrap().clone(), bindings),
        //     argnames: vec!["a".to_string()],
        //     ticks: 5,
        // })));
        state.bindings.bind("pass", builtin!{
            fn add (a:Number, b:Number) => int!(a+b)
        });
        state = state.begin(parse("(pass 7)").unwrap());
        println!("{:?}\n", state);
        state = state.run_to_completion();
        println!("{:?}\n", state);
        assert_eq!(state.value, Some(int!(12)))
    }
}
