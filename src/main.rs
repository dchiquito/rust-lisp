#[derive(Clone, Debug, Eq, PartialEq)]
struct Atom {
    string: String,
}

impl Atom {
    fn new(string: &str) -> Atom {
        Atom {
            string: String::from(string),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Cons {
    car: Box<Expression>,
    cdr: Box<Expression>,
}

impl Cons {
    fn new(car: &Expression, cdr: &Expression) -> Cons {
        Cons {
            car: Box::new(car.clone()),
            cdr: Box::new(cdr.clone()),
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expression {
    Atom(Atom),
    Cons(Cons),
}

macro_rules! atom {
    ($atom:expr) => {
        Expression::Atom(Atom::new($atom))
    };
}

macro_rules! cons {
    ($left:expr, $right:expr) => {
        Expression::Cons(Cons::new($left, $right))
    };
}

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

fn pop_token(string: &str) -> (Option<String>, String) {
    if string.len() <= 0 {
        return (None, String::new());
    }
    let first_char = string.chars().next().unwrap();
    let mut remainder = string.get(1..).unwrap();
    // Recursively trim whitespace off until we reach a real token
    if first_char.is_whitespace() {
        return pop_token(remainder);
    }
    match first_char {
        '(' | ')' | '\'' => (Some(String::from(first_char)), String::from(remainder)),
        _ => {
            let mut atom = String::from(first_char);
            while remainder.len() > 0 {
                let next_char = remainder.chars().next().unwrap();
                if (next_char == '(') | (next_char == ')') | next_char.is_whitespace() {
                    return (Some(atom), String::from(remainder));
                }
                atom.push(next_char);
                remainder = remainder.get(1..).unwrap()
            }
            (Some(atom), String::from(remainder))
        }
    }
}

#[test]
fn test_pop_token() {
    assert_eq!(pop_token(""), (None, String::new()));
    assert_eq!(pop_token("("), (Some(String::from("(")), String::new()));
    assert_eq!(pop_token(")"), (Some(String::from(")")), String::new()));
    assert_eq!(pop_token("'"), (Some(String::from("'")), String::new()));
    assert_eq!(
        pop_token("aaaa"),
        (Some(String::from("aaaa")), String::new())
    );
}

#[test]
fn test_pop_token_trim_whitespace() {
    assert_eq!(pop_token(" "), (None, String::new()));
    assert_eq!(pop_token(" \n("), (Some(String::from("(")), String::new()));
    assert_eq!(pop_token(" \t )"), (Some(String::from(")")), String::new()));
    assert_eq!(
        pop_token("  ' "),
        (Some(String::from("'")), String::from(" "))
    );
    assert_eq!(
        pop_token("    aaaa"),
        (Some(String::from("aaaa")), String::new())
    );
}

#[test]
fn test_pop_token_multiple_tokens() {
    assert_eq!(
        pop_token("()"),
        (Some(String::from("(")), String::from(")"))
    );
    assert_eq!(
        pop_token(")("),
        (Some(String::from(")")), String::from("("))
    );
    assert_eq!(
        pop_token("(a123"),
        (Some(String::from("(")), String::from("a123"))
    );
    assert_eq!(
        pop_token("+++)"),
        (Some(String::from("+++")), String::from(")"))
    );
    assert_eq!(
        pop_token("'()"),
        (Some(String::from("'")), String::from("()"))
    );
}

#[derive(Debug, Eq, PartialEq)]
enum ParseError {
    UnexpectedEOF,
    UnmatchedClosingParen,
}
type ParseResult = Result<Expression, ParseError>;

fn parse_expression(string: &str) -> (ParseResult, String) {
    match pop_token(string) {
        (None, remainder) => (Err(ParseError::UnexpectedEOF), remainder),
        (Some(token), remainder) => match &token as &str {
            "(" => parse_list(&remainder),
            ")" => (Err(ParseError::UnmatchedClosingParen), remainder),
            "'" => match parse_expression(&remainder) {
                (Err(err), remainder) => (Err(err), remainder),
                (Ok(quoted_value), remainder) => {
                    (Ok(list!(atom!("quote"), quoted_value)), remainder)
                }
            },
            token => (Ok(atom!(token)), remainder),
        },
    }
}

/// Parses a list, starting from the first element
fn parse_list(string: &str) -> (ParseResult, String) {
    match parse_expression(string) {
        // An UnmatchedClosingParen actually means we encountered the end of the list
        // Lists are terminated with a nil atom, so just return that
        (Err(ParseError::UnmatchedClosingParen), remainder) => (Ok(atom!("nil")), remainder),
        (Ok(car), remainder) => {
            let (cdr, remainder) = parse_list(&remainder);
            if let Ok(cdr) = cdr {
                (Ok(cons!(&car, &cdr)), remainder)
            } else {
                // This will only happen if cdr is an Err
                (cdr, remainder)
            }
        }
        (Err(err), remainder) => (Err(err), remainder),
    }
}

fn parse(string: &str) -> ParseResult {
    let (result, _) = parse_expression(string);
    result
}

#[test]
fn test_parse() {
    assert_eq!(parse("aaa"), Ok(atom!("aaa")));
    assert_eq!(parse("()"), Ok(atom!("nil")));
    assert_eq!(parse("(aaa)"), Ok(list!(atom!("aaa"))));
    assert_eq!(
        parse("  (  aaa   bbb  )  "),
        Ok(list!(atom!("aaa"), atom!("bbb")))
    );
    assert_eq!(parse("("), Err(ParseError::UnexpectedEOF));
    assert_eq!(parse(")"), Err(ParseError::UnmatchedClosingParen));
    assert_eq!(parse("'aaa"), Ok(list!(atom!("quote"), atom!("aaa"))));
    assert_eq!(
        parse("'(aaa)"),
        Ok(list!(atom!("quote"), list!(atom!("aaa"))))
    );
}

#[derive(Debug, Eq, PartialEq)]
enum EvaluationError {
    UnknownFunctionName,
    WrongNumberOfArguments,
    InvalidArgument,
}
type EvaluationResult = Result<Expression, EvaluationError>;

// Some helpers to cut down on the boilerplate
impl Expression {
    fn car(&self) -> EvaluationResult {
        if let Expression::Cons(cons) = self {
            return Ok(cons.car.as_ref().clone());
        } else {
            return Err(EvaluationError::WrongNumberOfArguments);
        }
    }
    fn cdr(&self) -> EvaluationResult {
        if let Expression::Cons(cons) = self {
            return Ok(cons.cdr.as_ref().clone());
        } else {
            return Err(EvaluationError::WrongNumberOfArguments);
        }
    }
    fn assert_empty(&self) -> Result<(), EvaluationError> {
        if self == &atom!("nil") {
            return Ok(());
        }
        return Err(EvaluationError::WrongNumberOfArguments);
    }
}

fn _evaluate_eq(expression: &Expression) -> EvaluationResult {
    let a = evaluate(&expression.car()?)?;
    let b = evaluate(&expression.cdr()?.car()?)?;
    expression.cdr()?.cdr()?.assert_empty()?;

    if a == b {
        Ok(atom!("true"))
    } else {
        Ok(atom!("false"))
    }
}

fn _evaluate_quote(expression: &Expression) -> EvaluationResult {
    if let Expression::Cons(cons) = expression {
        if cons.cdr.as_ref() == &atom!("nil") {
            return Ok(cons.car.as_ref().clone());
        }
    }
    Err(EvaluationError::WrongNumberOfArguments)
}

fn _evaluate_cons(expression: &Expression) -> EvaluationResult {
    let a = evaluate(&expression.car()?)?;
    let b = evaluate(&expression.cdr()?.car()?)?;
    expression.cdr()?.cdr()?.assert_empty()?;

    Ok(cons!(&a, &b))
}

fn _evaluate_car(expression: &Expression) -> EvaluationResult {
    let cons = evaluate(&expression.car()?)?;
    expression.cdr()?.assert_empty()?;
    if let Expression::Cons(cons) = cons {
        Ok(cons.car.as_ref().clone())
    } else {
        Err(EvaluationError::InvalidArgument)
    }
}

fn _evaluate_cdr(expression: &Expression) -> EvaluationResult {
    let cons = evaluate(&expression.car()?)?;
    expression.cdr()?.assert_empty()?;
    if let Expression::Cons(cons) = cons {
        Ok(cons.cdr.as_ref().clone())
    } else {
        Err(EvaluationError::InvalidArgument)
    }
}

fn _evaluate(function_name: &Atom, expression: &Expression) -> EvaluationResult {
    match &function_name.string as &str {
        "eq?" => _evaluate_eq(expression),
        "quote" => _evaluate_quote(expression),
        "cons" => _evaluate_cons(expression),
        "car" => _evaluate_car(expression),
        "cdr" => _evaluate_cdr(expression),
        _ => Err(EvaluationError::UnknownFunctionName),
    }
}

fn evaluate(expression: &Expression) -> EvaluationResult {
    match expression {
        Expression::Atom(atom) => Ok(Expression::Atom(atom.clone())),
        Expression::Cons(cons) => match cons.car.as_ref() {
            Expression::Cons(_) => Err(EvaluationError::WrongNumberOfArguments),
            Expression::Atom(function_name) => _evaluate(function_name, cons.cdr.as_ref()),
        },
    }
}

#[test]
fn test_evaluate_eq() {
    assert_eq!(evaluate(&parse("(eq? 1 1)").unwrap()), Ok(atom!("true")));
    assert_eq!(
        evaluate(&parse("(eq? foo foo)").unwrap()),
        Ok(atom!("true"))
    );
    assert_eq!(
        evaluate(&parse("(eq? foo bar)").unwrap()),
        Ok(atom!("false"))
    );
    assert_eq!(
        evaluate(&parse("(eq? (eq? 1 1) true)").unwrap()),
        Ok(atom!("true"))
    );
}

#[test]
fn test_quote() {
    assert_eq!(evaluate(&parse("'foo").unwrap()), Ok(atom!("foo"),));
    assert_eq!(evaluate(&parse("'(foo)").unwrap()), Ok(list!(atom!("foo"))));
    assert_eq!(
        evaluate(&parse("(eq? (eq? 1 1) (eq? 1 1))").unwrap()),
        Ok(atom!("true"))
    );
    assert_eq!(
        evaluate(&parse("(eq? '(eq? 1 1) (eq? 1 1))").unwrap()),
        Ok(atom!("false"))
    );
    assert_eq!(
        evaluate(&parse("(eq? '(a b c) (quote (a b c)))").unwrap()),
        Ok(atom!("true"))
    );
}

#[test]
fn test_cons() {
    assert_eq!(
        evaluate(&parse("(cons 1 2)").unwrap()),
        Ok(cons!(&atom!("1"), &atom!("2")))
    );
    assert_eq!(
        evaluate(&parse("(cons '1 '2)").unwrap()),
        Ok(cons!(&atom!("1"), &atom!("2")))
    );
    assert_eq!(
        evaluate(&parse("(cons (eq? 1 1) (eq? 1 2))").unwrap()),
        Ok(cons!(&atom!("true"), &atom!("false")))
    );
    assert_eq!(
        evaluate(&parse("(cons foo nil)").unwrap()),
        Ok(list!(atom!("foo")))
    );
    assert_eq!(
        evaluate(&parse("(eq? (cons foo nil) '(foo))").unwrap()),
        Ok(atom!("true"))
    );
}

#[test]
fn test_car() {
    assert_eq!(evaluate(&parse("(car '(1))").unwrap()), Ok(atom!("1")));
    assert_eq!(evaluate(&parse("(car '(1 2 3))").unwrap()), Ok(atom!("1")));
    assert_eq!(
        evaluate(&parse("(car (cons foo bar))").unwrap()),
        Ok(atom!("foo"))
    );
}

#[test]
fn test_cdr() {
    assert_eq!(evaluate(&parse("(cdr '(1))").unwrap()), Ok(atom!("nil")));
    assert_eq!(
        evaluate(&parse("(cdr '(1 2 3))").unwrap()),
        Ok(list!(atom!("2"), atom!("3")))
    );
    assert_eq!(
        evaluate(&parse("(cdr (cons foo bar))").unwrap()),
        Ok(atom!("bar"))
    );
}

fn main() {
    let e: Expression = atom!("yes");
    println!("Hello, world! {:?}", e);
    println!("Hello, world! {:?}", evaluate(&e));
    let e: Expression = cons!(&atom!("A"), &atom!("B"));
    println!("Hello, world! {:?}", e);
    println!("Hello, world! {:?}", evaluate(&e));
    println!("{:?}", pop_token("aa"));
    println!("{:?}", pop_token("(("));
}
