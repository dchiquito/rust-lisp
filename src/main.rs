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
    fn r#true() -> Atom {
        Atom::new("true")
    }
    fn r#false() -> Atom {
        Atom::new("false")
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

#[derive(Debug, Eq, PartialEq)]
enum EvaluationError {
    UnknownFunctionName,
    WrongNumberOfArguments,
}
type EvaluationResult = Result<Expression, EvaluationError>;

// macro_rules! unwrap_cons_list {
//     ($expression:expr, $length:expr) => {
//         unwrap_cons_list!($expression, $length, ())
//     };
//     ($expression:expr, $length:expr, ($($already_unwrapped:expr)*)) => {{
//         if $length == 0 {
//             if $expression == &Expression::Atom(Atom::nil()) {
//                 Ok(($($already_unwrapped),*))
//             } else {
//                 Err(EvaluationError::WrongNumberOfArguments)
//             }
//         } else {
//             if let Expression::Cons(cons) = $expression {

//             }
//         }
//         // match ($expression, $length) {
//         //     (Expression::Atom(Atom::nil()), 0) => ()
//         // }
//     }};
// }

fn _evaluate_eq(expression: &Expression) -> EvaluationResult {
    // let (a, b) = unwrap_cons_list!(expression, 2);
    if let Expression::Cons(cons) = expression {
        let a = evaluate(cons.car.as_ref())?;
        if let Expression::Cons(cons) = cons.cdr.as_ref() {
            let b = evaluate(cons.car.as_ref())?;
            if cons.cdr.as_ref() != &Expression::Atom(Atom::nil()) {
                return Err(EvaluationError::WrongNumberOfArguments);
            }
            if a == b {
                return Ok(Expression::Atom(Atom::r#true()));
            } else {
                return Ok(Expression::Atom(Atom::r#false()));
            }
        }
    }
    Err(EvaluationError::WrongNumberOfArguments)
}

fn _evaluate(function_name: &Atom, expression: &Expression) -> EvaluationResult {
    match &function_name.string as &str {
        "eq?" => _evaluate_eq(expression),
        _ => Err(EvaluationError::UnknownFunctionName),
    }
}

fn evaluate(expression: &Expression) -> EvaluationResult {
    match expression {
        Expression::Atom(atom) => Ok(Expression::Atom(atom.clone())),
        Expression::Cons(cons) => {
            match cons.car.as_ref() {
                Expression::Cons(_) => Err(EvaluationError::WrongNumberOfArguments),
                // Expression::Atom(function_name) => Ok(Expression::Atom(Atom::new(&format!(
                //     "calling {}",
                //     function_name.string
                // )))),
                Expression::Atom(function_name) => _evaluate(function_name, cons.cdr.as_ref()),
            }
            // Ok(Expression::Atom(Atom::new("foo")))
        }
    }
}

#[test]
fn test_evaluate() {
    assert_eq!(
        evaluate(&parse("(eq? 1 1)").unwrap()),
        Ok(Expression::Atom(Atom::r#true()))
    );
    assert_eq!(
        evaluate(&parse("(eq? foo foo)").unwrap()),
        Ok(Expression::Atom(Atom::r#true()))
    );
    assert_eq!(
        evaluate(&parse("(eq? foo bar)").unwrap()),
        Ok(Expression::Atom(Atom::r#false()))
    );
    assert_eq!(
        evaluate(&parse("(eq? (eq? 1 1) true)").unwrap()),
        Ok(Expression::Atom(Atom::r#true()))
    );
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
