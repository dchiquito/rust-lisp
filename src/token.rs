fn consume_non_tokens(string: &str) -> (Option<char>, String) {
  // Trim whitespace off until we reach a real token
  let mut string = string.chars();
  let mut first_token_char = None;
  let mut in_comment = false;
  while let Some(c) = string.next() {
    if !in_comment {
      if c == ';' {
        in_comment = true;
      } else if !c.is_whitespace() {
        first_token_char = Some(c);
        break;
      }
    } else {
      if c == '\n' {
        in_comment = false;
      }
    }
  }
  (first_token_char, string.collect())
}

pub fn pop_token(string: &str) -> (Option<String>, String) {
  // let first_char = string.chars().next().unwrap();
  // let mut remainder = string.get(1..).unwrap();
  // if first_char.is_whitespace() {
  //   return pop_token(remainder);
  // }
  // let mut string = consume_non_tokens(string).chars();
  // let mut first_char = string.next().unwrap();
  // let mut remainder: &str = &string.collect::<String>();
  if let (Some(first_char), mut remainder) = consume_non_tokens(string) {
    match first_char {
      '(' | ')' | '\'' => (Some(String::from(first_char)), remainder),
      _ => {
        let mut symbol = String::from(first_char);
        while !remainder.is_empty() {
          let next_char = remainder.chars().next().unwrap();
          if (next_char == '(') | (next_char == ')') | next_char.is_whitespace() {
            return (Some(symbol), String::from(remainder));
          }
          symbol.push(next_char);
          remainder = remainder.get(1..).unwrap().to_string();
        }
        (Some(symbol), remainder)
      }
    }
  } else {
    (None, String::new())
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_consume_non_tokens() {
    assert_eq!(consume_non_tokens(""), (None, String::new()));
    assert_eq!(consume_non_tokens("   "), (None, String::new()));
    assert_eq!(consume_non_tokens(" \n \n "), (None, String::new()));
    assert_eq!(consume_non_tokens("; no newline"), (None, String::new()));
    assert_eq!(consume_non_tokens("; newline\n"), (None, String::new()));
    assert_eq!(consume_non_tokens("("), (Some('('), String::new()));
    assert_eq!(consume_non_tokens(")"), (Some(')'), String::new()));
    assert_eq!(consume_non_tokens("'"), (Some('\''), String::new()));
    assert_eq!(consume_non_tokens("aaaa"), (Some('a'), String::from("aaa")));
    assert_eq!(
      consume_non_tokens("  ; bbb \n  aaaa"),
      (Some('a'), String::from("aaa"))
    );
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

  #[test]
  fn test_pop_token_comments() {
    assert_eq!(pop_token(";"), (None, String::new()));
    assert_eq!(pop_token(" ;  "), (None, String::new()));
    assert_eq!(pop_token(" ; foobar "), (None, String::new()));
    assert_eq!(pop_token(";\n"), (None, String::new()));
    assert_eq!(pop_token(";\n\n\n"), (None, String::new()));
    assert_eq!(
      pop_token("; comment \n("),
      (Some(String::from("(")), String::new())
    );
  }
}
