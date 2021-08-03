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
    fn nil() -> Atom {
        Atom::new("nil")
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
        '(' | ')' => (Some(String::from(first_char)), String::from(remainder)),
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
            token => (Ok(Expression::Atom(Atom::new(token))), remainder),
        },
    }
}

/// Parses a list, starting from the first element
fn parse_list(string: &str) -> (ParseResult, String) {
    match parse_expression(string) {
        // An UnmatchedClosingParen actually means we encountered the end of the list
        // Lists are terminated with a nil atom, so just return that
        (Err(ParseError::UnmatchedClosingParen), remainder) => {
            (Ok(Expression::Atom(Atom::nil())), remainder)
        }
        (Ok(car), remainder) => {
            let (cdr, remainder) = parse_list(&remainder);
            if let Ok(cdr) = cdr {
                (Ok(Expression::Cons(Cons::new(&car, &cdr))), remainder)
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
    assert_eq!(parse("aaa"), Ok(Expression::Atom(Atom::new("aaa"))));
    assert_eq!(parse("()"), Ok(Expression::Atom(Atom::new("nil"))));
    assert_eq!(
        parse("(aaa)"),
        Ok(Expression::Cons(Cons::new(
            &Expression::Atom(Atom::new("aaa")),
            &Expression::Atom(Atom::new("nil"))
        )))
    );
    assert_eq!(
        parse("  (  aaa   bbb  )  "),
        Ok(Expression::Cons(Cons::new(
            &Expression::Atom(Atom::new("aaa")),
            &Expression::Cons(Cons::new(
                &Expression::Atom(Atom::new("bbb")),
                &Expression::Atom(Atom::new("nil"))
            ))
        )))
    );
    assert_eq!(parse("("), Err(ParseError::UnexpectedEOF));
    assert_eq!(parse(")"), Err(ParseError::UnmatchedClosingParen));
}

type EvaluationResult = Result<Expression, String>;

fn evaluate(expression: &Expression) -> EvaluationResult {
    match expression {
        Expression::Atom(atom) => Ok(Expression::Atom(atom.clone())),
        Expression::Cons(cons) => {
            match cons.car.as_ref() {
                Expression::Cons(_) => Err(String::from("")),
                Expression::Atom(function_name) => Ok(Expression::Atom(Atom::new(&format!(
                    "calling {}",
                    function_name.string
                )))),
            }
            // Ok(Expression::Atom(Atom::new("foo")))
        }
    }
}

fn main() {
    let e: Expression = Expression::Atom(Atom::new("yes"));
    println!("Hello, world! {:?}", e);
    println!("Hello, world! {:?}", evaluate(&e));
    let e: Expression = Expression::Cons(Cons::new(
        &Expression::Atom(Atom::new("A")),
        &Expression::Atom(Atom::new("B")),
    ));
    println!("Hello, world! {:?}", e);
    println!("Hello, world! {:?}", evaluate(&e));
    println!("{:?}", pop_token("aa"));
    println!("{:?}", pop_token("(("));
}
