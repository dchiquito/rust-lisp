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
        (Ok(quoted_value), remainder) => (Ok(list!(symbol!("quote"), quoted_value)), remainder),
      },
      token => {
        fn is_digit(c: char) -> bool {
          c.is_digit(10)
        }
        if token.chars().all(is_digit) {
          // non-negative integers are all digits
          (Ok(int!(token.parse().unwrap())), remainder)
        } else if token.chars().next() == Some('-')
          && token.len() > 1
          && token.chars().skip(1).all(is_digit)
        {
          // negative numbers are also allowed
          (Ok(int!(token.parse().unwrap())), remainder)
        } else if token == "#t" || token == "#true" {
          (Ok(boolean!(true)), remainder)
        } else if token == "#f" || token == "#false" {
          (Ok(boolean!(false)), remainder)
        } else {
          (Ok(symbol!(token)), remainder)
        }
      }
    },
  }
}

/// Parses a list, starting from the first element
fn parse_list(string: &str) -> (ParseResult, String) {
  match parse_expression(string) {
    // An UnmatchedClosingParen actually means we encountered the end of the list
    // Lists are terminated with a nil symbol, so just return that
    (Err(ParseError::UnmatchedClosingParen), remainder) => (Ok(null!()), remainder),
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
    assert_eq!(parse("aaa"), Ok(symbol!("aaa")));
    assert_eq!(parse("()"), Ok(null!()));
    assert_eq!(parse("(aaa)"), Ok(list!(symbol!("aaa"))));
    assert_eq!(
      parse("  (  aaa   bbb  )  "),
      Ok(list!(symbol!("aaa"), symbol!("bbb")))
    );
    assert_eq!(parse("1"), Ok(int!(1)));
    assert_eq!(parse("99999"), Ok(int!(99999)));
    assert_eq!(parse("0"), Ok(int!(0)));
    assert_eq!(parse("-1"), Ok(int!(-1)));
    assert_eq!(parse("-"), Ok(symbol!("-")));
    assert_eq!(parse("#t"), Ok(boolean!(true)));
    assert_eq!(parse("#true"), Ok(boolean!(true)));
    assert_eq!(parse("#f"), Ok(boolean!(false)));
    assert_eq!(parse("#false"), Ok(boolean!(false)));
    assert_eq!(parse("("), Err(ParseError::UnexpectedEOF));
    assert_eq!(parse(")"), Err(ParseError::UnmatchedClosingParen));
    assert_eq!(parse("'aaa"), Ok(list!(symbol!("quote"), symbol!("aaa"))));
    assert_eq!(
      parse("'(aaa)"),
      Ok(list!(symbol!("quote"), list!(symbol!("aaa"))))
    );
  }
}
