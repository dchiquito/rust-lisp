// use crate::evaluate::ProcedureResult;
// use crate::Scope;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Eq, PartialEq)]
pub struct Cons {
  pub car: Box<Expression>,
  pub cdr: Box<Expression>,
}

impl Cons {
  pub fn new(car: &Expression, cdr: &Expression) -> Cons {
    Cons {
      car: Box::new(car.clone()),
      cdr: Box::new(cdr.clone()),
    }
  }
  /// Format this Cons cell as if it were an interior item in a list.
  /// When rendering the outermost Cons, fmt is called, which writes the opening '('.
  /// The subsequent Cons need to avoid writing the '(' again, hence this alternative method.
  fn fmt_as_inner_element(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self.cdr.as_ref().clone() {
      Expression::Cons(cons) => {
        // There are more Cons in the chain
        // Format this car and continue recursing
        write!(f, "{} ", self.car)?;
        cons.fmt_as_inner_element(f)?;
      }
      Expression::Null => {
        // We have reached the nil terminator
        write!(f, "{})", self.car)?;
      }
      _ => {
        // There is no nil terminator, so this isn't actually a list!
        // Format the final symbol with the special cons cell .
        write!(f, "{} . {})", self.car, self.cdr)?;
      }
    };
    Ok(())
  }
}

impl fmt::Display for Cons {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "(")?;
    self.fmt_as_inner_element(f)
  }
}
impl fmt::Debug for Cons {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

#[derive(Clone, Eq, PartialEq, Ord, PartialOrd)]
pub enum Number {
  Integer(i32),
}

impl fmt::Display for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Number::Integer(number) => write!(f, "{}", number),
    }
  }
}
impl fmt::Debug for Number {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

// #[derive(Clone, Eq, PartialEq)]
// pub enum Procedure {
//   FixedArgumentForm(Vec<String>, Vec<Expression>),
//   VariableArgumentForm(Vec<String>, String, Vec<Expression>),
//   BuiltinFixedArgumentForm(
//     &'static str,
//     fn(Vec<Expression>, Rc<RefCell<Scope>>) -> ProcedureResult,
//     usize,
//   ),
//   #[allow(clippy::type_complexity)]
//   BuiltinVariableArgumentForm(
//     &'static str,
//     fn(Vec<Expression>, Vec<Expression>, Rc<RefCell<Scope>>) -> ProcedureResult,
//     usize,
//   ),
// }
// impl Procedure {
//   pub fn name(&self) -> String {
//     match self {
//       Procedure::FixedArgumentForm(_, _) => "#<procedure>".to_string(),
//       Procedure::VariableArgumentForm(_, _, _) => "#<procedure>".to_string(),
//       Procedure::BuiltinFixedArgumentForm(procedure_name, _, _) => procedure_name.to_string(),
//       Procedure::BuiltinVariableArgumentForm(procedure_name, _, _) => procedure_name.to_string(),
//     }
//   }
// }
// impl fmt::Display for Procedure {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     match self {
//       Procedure::FixedArgumentForm(_, _) => write!(f, "#<procedure>"),
//       Procedure::VariableArgumentForm(_, _, _) => write!(f, "#<procedure>"),
//       Procedure::BuiltinFixedArgumentForm(procedure_name, _, _) => {
//         write!(f, "#<procedure:{}>", procedure_name)
//       }
//       Procedure::BuiltinVariableArgumentForm(procedure_name, _, _) => {
//         write!(f, "#<procedure:{}>", procedure_name)
//       }
//     }
//   }
// }
// impl fmt::Debug for Procedure {
//   fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//     write!(f, "{}", self)
//   }
// }

pub type BindingLayer = HashMap<String, Expression>;
#[derive(Debug)]
pub struct Bindings {
  globals: BindingLayer,
  stack: Vec<BindingLayer>,
}
impl Bindings {
  pub fn new() -> Bindings {
    Bindings {
      globals: BindingLayer::new(),
      stack: vec![],
    }
  }
  pub fn bind(&mut self, variable: &str, value: Expression) {
    let bindopt = self.stack.last_mut().unwrap_or(&mut self.globals);
    bindopt.insert(String::from(variable), value);
  }
  pub fn bind_builtin(&mut self, expr: Expression) {
    match expr {
      Expression::Procedure(Procedure::BuiltinProcedure(builtin)) => {
        self.bind(&builtin.name.clone(), Expression::Procedure(Procedure::BuiltinProcedure(builtin)));
      },
      _ => {}
    }
  }
  pub fn get(&self, variable: &str) -> Option<Expression> {
    self
      .stack
      .last()
      .unwrap_or(&self.globals)
      .get(variable)
      .map(|value| value.clone())
  }
  pub fn push(&mut self, bindings: BindingLayer) {
    self.stack.push(bindings);
  }
  pub fn pop(&mut self) {
    self.stack.pop();
  }
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct LambdaProcedure {
  program: Box<Expression>,
  argnames: Vec<String>,
}
#[derive(Clone, Eq, PartialEq, Debug)]
pub struct BuiltinProcedure {
  pub name: String,
  pub program: fn(Bindings) -> (Expression, Bindings),
  pub argnames: Vec<String>,
  pub ticks: i32,
}
#[derive(Clone, Eq, PartialEq)]
pub enum Procedure {
  LambdaProcedure(LambdaProcedure),
  BuiltinProcedure(BuiltinProcedure),
}
impl Procedure {
  pub fn argnames(&self) -> Vec<String> {
    match self {
      Procedure::LambdaProcedure(lambda) => lambda.argnames.clone(),
      Procedure::BuiltinProcedure(builtin) => builtin.argnames.clone(),
    }
  }
}
impl fmt::Display for Procedure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Procedure::LambdaProcedure(_) => write!(f, "#<procedure>"),
      Procedure::BuiltinProcedure(builtin) => {
        write!(f, "#<procedure:{}>", "builtin!!!!!") // TODO builtin names
      }
    }
  }
}
impl fmt::Debug for Procedure {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

#[derive(Clone, Eq, PartialEq)]
pub enum Expression {
  Symbol(String),
  Cons(Cons),
  Number(Number),
  Boolean(bool),
  Procedure(Procedure),
  Null,
  Void,
  // MyProcedure(MyProcedure),
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Symbol(symbol) => write!(f, "{}", symbol),
      Expression::Cons(cons) => write!(f, "{}", cons),
      Expression::Number(number) => write!(f, "{}", number),
      Expression::Boolean(boolean) => {
        if *boolean {
          write!(f, "#t")
        } else {
          write!(f, "#f")
        }
      }
      Expression::Procedure(procedure) => write!(f, "{}", procedure),
      Expression::Null => write!(f, "'()"),
      Expression::Void => write!(f, "#<void>"),
      // Expression::MyProcedure(procedure) => write!(f, "MYSTERIOUS myprocedure"),
    }
  }
}
impl fmt::Debug for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self)
  }
}

impl Expression {
  pub fn outer_representation(&self) -> String {
    match self {
      Expression::Symbol(symbol) => format!("'{}", symbol),
      Expression::Cons(cons) => format!("'{}", cons),
      Expression::Number(number) => format!("{}", number),
      Expression::Boolean(boolean) => {
        if *boolean {
          "#t".to_string()
        } else {
          "#f".to_string()
        }
      }
      Expression::Procedure(procedure) => format!("{}", procedure),
      Expression::Null => "'()".to_string(),
      Expression::Void => "#<void>".to_string(),
      // Expression::MyProcedure(procedure) => "mysterious myprocedure".to_string(),
    }
  }
}

#[macro_export]
macro_rules! symbol {
  ($symbol:expr) => {
    Expression::Symbol(String::from($symbol))
  };
}

#[macro_export]
macro_rules! cons {
  ($left:expr, $right:expr) => {
    Expression::Cons(Cons::new($left, $right))
  };
}

#[macro_export]
macro_rules! list {
    () => {
        null!()
    };
    ($car:expr) => {
        cons!(&$car, &null!())
    };
    ($car:expr, $($cdr:expr),*) => {
        cons!(&$car, &list!($($cdr),*))
    };
}

#[macro_export]
macro_rules! int {
  ($number:expr) => {
    Expression::Number(Number::Integer($number))
  };
}

#[macro_export]
macro_rules! boolean {
  ($boolean:expr) => {
    Expression::Boolean($boolean)
  };
}

// #[macro_export]
// macro_rules! procedure {
//   ($arg:expr, $body:expr) => {
//     Expression::Procedure(Procedure::FixedArgumentForm($arg, $body))
//   };
//   ($arg:expr , $vararg:expr, $body:expr) => {
//     Expression::Procedure(Procedure::VariableArgumentForm($arg, $vararg, $body))
//   };
// }
#[macro_export]
macro_rules! _builtin_arg_type {
    ($bindings:ident => $argname:ident : Number) => {
        let $argname = match $bindings.get(stringify!($argname)).unwrap().clone() {
            Expression::Number(Number::Integer(integer)) => integer,
            _ => panic!("Not an integer"),
        };
    };
    ($bindings:ident => $argname:ident : Any) => {
        let $argname = $bindings.get(stringify!($argname)).unwrap().clone();
    };
}

#[macro_export]
macro_rules! builtin {
    (fn $name:tt ($($argname:ident : $argtype:ident),*) => $return_line:expr) => {
      builtin!{
        fn $name ($($argname:$argtype),*) {
          ;$return_line
        }
      }
    };
    (fn $name:tt ($($argname:ident : $argtype:ident),*) {$(let $var:ident = $val:expr);* ; $return_line:expr}) => {
        Expression::Procedure(Procedure::BuiltinProcedure(BuiltinProcedure {
            name: stringify!($name).to_string(),
            program: |bindings| {
                $(
                    // let $argname = bindings.get(stringify!($argname)).unwrap().clone();
                    _builtin_arg_type!(bindings => $argname:$argtype);
                )*
                $(let $var = $val;)*
                ($return_line, bindings)
            },
            argnames: vec![$(stringify!($argname).to_string()),*],
            ticks: 5,
        }))
    };
}

#[macro_export]
macro_rules! null {
  () => {
    Expression::Null
  };
}

#[macro_export]
macro_rules! void {
  () => {
    Expression::Void
  };
}

#[cfg(test)]
mod test {
  use super::*;

  macro_rules! assert_expr_eq {
    ($expression:expr, $display:expr, $outer_representation:expr) => {{
      assert_eq!(format!("{}", $expression), $display);
      assert_eq!(format!("{:?}", $expression), $display);
      assert_eq!($expression.outer_representation(), $outer_representation);
    }};
  }

  #[test]
  fn test_fmt_symbol() {
    assert_expr_eq!(symbol!("foo"), "foo", "'foo");
  }

  #[test]
  fn test_fmt_cons() {
    assert_expr_eq!(cons!(&int!(1), &null!()), "(1)", "'(1)");
    assert_expr_eq!(cons!(&int!(1), &int!(2)), "(1 . 2)", "'(1 . 2)");
    assert_expr_eq!(list!(&int!(1), &int!(2), &int!(3)), "(1 2 3)", "'(1 2 3)");
  }

  #[test]
  fn test_fmt_number() {
    assert_expr_eq!(int!(1), "1", "1");
    assert_expr_eq!(int!(0), "0", "0");
    assert_expr_eq!(int!(2), "2", "2");
    assert_expr_eq!(int!(-3), "-3", "-3");
    assert_expr_eq!(int!(987654321), "987654321", "987654321");
  }

  #[test]
  fn test_fmt_boolean() {
    assert_expr_eq!(boolean!(true), "#t", "#t");
    assert_expr_eq!(boolean!(false), "#f", "#f");
  }

  #[test]
  fn test_fmt_null() {
    assert_expr_eq!(null!(), "'()", "'()");
  }

  #[test]
  fn test_fmt_void() {
    assert_expr_eq!(void!(), "#<void>", "#<void>");
  }
}
