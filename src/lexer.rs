use crate::errors::{Error, Result};

#[derive(Debug, PartialEq)]
pub enum Token {
  Fnc,                    // "fnc"
  Let,                    // "let"
  Return,                 // "ret"
  If,                     // "if"
  Else,                   // "else"
  While,                  // "while"
  For,                    // "for"
  Match,                  // "match"
  End,                    // "end"
  Identifier(String),     // Variable names and function names
  Number(i32),            // Numeric literals
  StringLiteral(String),  // String literals
  Operator(String),       // Operators like +, -, *, /, =
  OpenParen,              // "("
  CloseParen,             // ")"
  Arrow,                  // "->"
  Comma,                  // ","
  EOF,                    // End of file/input
}

pub fn tokenize(input: &str) -> Result<Vec<Token>> {
  let mut tokens = Vec::new();
  let mut chars = input.chars().peekable();

  while let Some(&ch) = chars.peek() {
    match ch {
      ' ' | '\n' | '\t' | '\r' => { chars.next(); } // Skip whitespace / newlines
      '(' => {
        tokens.push(Token::OpenParen);
        chars.next();
      }
      ')' => {
        tokens.push(Token::CloseParen);
        chars.next();
      }
      '-' => {
        chars.next();
        if let Some('>') = chars.peek() {
          tokens.push(Token::Arrow);
          chars.next();
        } else {
          tokens.push(Token::Operator("-".to_string()));
        }
      }
      '0'..='9' => {
        let mut num_str = String::new();
        while let Some(&ch) = chars.peek() {
          if ch.is_digit(10) {
            num_str.push(ch);
            chars.next();
          } else {
            break;
          }
        }
        let number = num_str.parse::<i32>().unwrap();
        tokens.push(Token::Number(number));
      }
      'a'..='z' | 'A'..='Z' | '_' => {
        let mut ident_str = String::new();
        while let Some(&ch) = chars.peek() {
          if ch.is_alphanumeric() || ch == '_' {
            ident_str.push(ch);
            chars.next();
          } else {
            break;
          }
        }
        match ident_str.as_str() {
          "fnc" => tokens.push(Token::Fnc),
          "let" => tokens.push(Token::Let),
          "ret" => tokens.push(Token::Return),
          "if" => tokens.push(Token::If),
          "else" => tokens.push(Token::Else),
          "while" => tokens.push(Token::While),
          "for" => tokens.push(Token::For),
          "match" => tokens.push(Token::Match),
          "end" => tokens.push(Token::End),
          _ => tokens.push(Token::Identifier(ident_str)),
        }
      }
      '+' | '*' | '/' | '=' => {
        tokens.push(Token::Operator(ch.to_string()));
        chars.next();
      }
      '"' => {
        chars.next(); // Skip the opening quote
        let mut string_lit = String::new();
        while let Some(&ch) = chars.peek() {
          if ch == '"' {
            break;
          }
          string_lit.push(ch);
          chars.next();
        }
        chars.next(); // Skip the closing quote
        tokens.push(Token::StringLiteral(string_lit));
      }
      ',' => {
        tokens.push(Token::Comma);
        chars.next();
      }
      _ => {
        return Err(Error::LexicalError(format!("Unexpected character '{}' at position {}", ch, input.len() - chars.clone().count())));
      }
    }
  }

  tokens.push(Token::EOF);
  Ok(tokens)
}