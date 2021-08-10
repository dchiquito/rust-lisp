pub fn pop_token(string: &str) -> (Option<String>, String) {
  if string.is_empty() {
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
      let mut symbol = String::from(first_char);
      while !remainder.is_empty() {
        let next_char = remainder.chars().next().unwrap();
        if (next_char == '(') | (next_char == ')') | next_char.is_whitespace() {
          return (Some(symbol), String::from(remainder));
        }
        symbol.push(next_char);
        remainder = remainder.get(1..).unwrap()
      }
      (Some(symbol), String::from(remainder))
    }
  }
}

#[cfg(test)]
mod test {
  use super::pop_token;

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
}
