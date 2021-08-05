use crate::token::pop_token;
use crate::*;

#[derive(Debug, Eq, PartialEq)]
pub enum ParseError {
  UnexpectedEOF,
  UnmatchedClosingParen,
}
pub type ParseResult = Result<Expression, ParseError>;

fn parse_expression(string: &str) -> (ParseResult, String) {
  match pop_token(string) {
    (None, remainder) => (Err(ParseError::UnexpectedEOF), remainder),
    (Some(token), remainder) => match &token as &str {
      "(" => parse_list(&remainder),
      ")" => (Err(ParseError::UnmatchedClosingParen), remainder),
      "'" => match parse_expression(&remainder) {
        (Err(err), remainder) => (Err(err), remainder),
        (Ok(quoted_value), remainder) => (Ok(list!(atom!("quote"), quoted_value)), remainder),
      },
      token => {
        fn is_digit(c: char) -> bool {
          c.is_digit(10)
        }
        if token.chars().all(is_digit) {
          (Ok(int!(token.parse().unwrap())), remainder)
        } else {
          (Ok(atom!(token)), remainder)
        }
      }
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

pub fn parse(string: &str) -> ParseResult {
  let (result, _) = parse_expression(string);
  result
}

#[cfg(test)]
mod test {
  use super::*;

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
}
