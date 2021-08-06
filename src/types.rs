use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq)]
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
      Expression::Symbol(symbol) => {
        if symbol == "nil" {
          // We have reached the nil terminator
          write!(f, "{})", self.car)?;
        } else {
          // There is no nil terminator, so this isn't actually a list!
          // Format the final symbol with the special cons cell .
          write!(f, "{} . {})", self.car, self.cdr)?;
        }
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

#[derive(Clone, Debug, Eq, PartialEq)]
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

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Procedure {
  SingleArgumentForm(String, Vec<Expression>),
  FixedArgumentForm(Vec<String>, Vec<Expression>),
  VariableArgumentForm(Vec<String>, String, Vec<Expression>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
  Symbol(String),
  Cons(Cons),
  Number(Number),
  Boolean(bool),
  Procedure(Procedure),
  Null,
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
      Expression::Procedure(_) => write!(f, "#<procedure>"),
      Expression::Null => write!(f, "'()"),
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

#[macro_export]
macro_rules! procedure {
  ($arg:expr, $body:expr) => {
    Expression::Procedure(Procedure::SingleArgumentForm($arg, $body))
  };
  (fixed $arg:expr, $body:expr) => {
    Expression::Procedure(Procedure::FixedArgumentForm($arg, $body))
  };
  ($arg:expr , $vararg:expr, $body:expr) => {
    Expression::Procedure(Procedure::VariableArgumentForm($arg, $vararg, $body))
  };
}

#[macro_export]
macro_rules! null {
  () => {
    Expression::Null
  };
}
