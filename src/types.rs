#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Atom {
  pub string: String,
}

impl Atom {
  pub fn new(string: &str) -> Atom {
    Atom {
      string: String::from(string),
    }
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
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Expression {
  Atom(Atom),
  Cons(Cons),
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
