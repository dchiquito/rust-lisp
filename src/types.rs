use std::fmt;

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Atom {
  pub string: String,
}

impl Atom {
  pub fn new(string: &str) -> Atom {
    Atom {
      string: String::from(string),
    }
  }
  pub fn is_nil(&self) -> bool {
    self.string == "nil"
  }
  pub fn is_bool(&self) -> bool {
    self.string == "true" || self.string == "false"
  }
  pub fn is_symbol(&self) -> bool {
    // it's not a symbol if it is a primitive type
    !self.is_nil() && !self.is_bool()
  }
}

impl fmt::Display for Atom {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.string)
  }
}

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
      Expression::Atom(atom) => {
        if atom.string == "nil" {
          // We have reached the nil terminator
          write!(f, "{})", self.car)?;
        } else {
          // There is no nil terminator, so this isn't actually a list!
          // Format the final atom with the special cons cell .
          write!(f, "{} . {})", self.car, self.cdr)?;
        }
      }
      _ => {
        // There is no nil terminator, so this isn't actually a list!
        // Format the final atom with the special cons cell .
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
pub enum Expression {
  Atom(Atom),
  Cons(Cons),
  Number(Number),
  Boolean(bool),
}

impl fmt::Display for Expression {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      Expression::Atom(atom) => write!(f, "{}", atom),
      Expression::Cons(cons) => write!(f, "{}", cons),
      Expression::Number(number) => write!(f, "{}", number),
      Expression::Boolean(boolean) => {
        if *boolean {
          write!(f, "#t")
        } else {
          write!(f, "#f")
        }
      }
    }
  }
}

#[macro_export]
macro_rules! atom {
  ($atom:expr) => {
    Expression::Atom(Atom::new($atom))
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
        atom!("nil")
    };
    ($car:expr) => {
        cons!(&$car, &atom!("nil"))
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
