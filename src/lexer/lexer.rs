use crate::errors::Error;

#[derive(Debug, PartialEq, Clone)]
pub enum Token {
  Let,                   // "let"
  Return,                // "ret"
  If,                    // "if"
  Else,                  // "else"
  While,                 // "while"
  For,                   // "for"
  Match,                 // "match"
  Identifier(String),    // Variable names and function names
  Number(f64),           // Numeric literals
  StringLiteral(String), // String literals
  Operator(String),      // Operators like +, -, *, /, =
  OpenBracket,           // "["
  CloseBracket,          // "]"
  Arrow,                 // "->"
  PipeArrow,             // "|>" for function chaining
  Colon,                 // ":" for pattern matching
  Semicolon,             // ";" as end of block marker
  Eof,                   // End of file/input
}

pub fn tokenize(input: &str) -> Result<Vec<Token>, Error> {
  let mut tokens = Vec::new();
  let mut chars = input.chars().peekable();

  while let Some(&ch) = chars.peek() {
    match ch {
      ' ' | '\n' | '\t' | '\r' => {
        chars.next();
      }
      '[' => {
        tokens.push(Token::OpenBracket);
        chars.next();
      }
      ']' => {
        tokens.push(Token::CloseBracket);
        chars.next();
      }
      ':' => {
        tokens.push(Token::Colon);
        chars.next();
      }
      '-' => {
        chars.next();
        if let Some('>') = chars.peek() {
          tokens.push(Token::Arrow);
          chars.next();
        } else if let Some('=') = chars.peek() {
          tokens.push(Token::Operator("-=".to_string()));
          chars.next();
        } else {
          tokens.push(Token::Operator("-".to_string()));
        }
      }
      '>' => {
        chars.next();
        if let Some('=') = chars.peek() {
          tokens.push(Token::Operator(">=".to_string()));
          chars.next();
        } else {
          tokens.push(Token::Operator(">".to_string()));
        }
      }
      '<' => {
        chars.next();
        if let Some('=') = chars.peek() {
          tokens.push(Token::Operator("<=".to_string()));
          chars.next();
        } else {
          tokens.push(Token::Operator("<".to_string()));
        }
      }
      '=' => {
        chars.next();
        if let Some('=') = chars.peek() {
          tokens.push(Token::Operator("==".to_string()));
          chars.next();
        } else {
          tokens.push(Token::Operator("=".to_string()));
        }
      }
      ';' => {
        tokens.push(Token::Semicolon);
        chars.next();
      }
      '0'..='9' => {
        let mut num_str = String::new();
        while let Some(&ch) = chars.peek() {
          if ch.is_ascii_digit() {
            num_str.push(ch);
            chars.next();
          } else {
            break;
          }
        }
        let number = num_str.parse::<f64>().unwrap();
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
          "let" => tokens.push(Token::Let),
          "ret" => tokens.push(Token::Return),
          "if" => tokens.push(Token::If),
          "else" => tokens.push(Token::Else),
          "while" => tokens.push(Token::While),
          "for" => tokens.push(Token::For),
          "match" => tokens.push(Token::Match),
          _ => tokens.push(Token::Identifier(ident_str)),
        }
      }
      '+' => {
        chars.next();
        if let Some('=') = chars.peek() {
          tokens.push(Token::Operator("+=".to_string()));
          chars.next();
        } else {
          tokens.push(Token::Operator("+".to_string()));
        }
      }
      '*' => {
        chars.next();
        if let Some('=') = chars.peek() {
          tokens.push(Token::Operator("*=".to_string()));
          chars.next();
        } else {
          tokens.push(Token::Operator("*".to_string()));
        }
      }
      '/' => {
        chars.next();
        if let Some('=') = chars.peek() {
          tokens.push(Token::Operator("/=".to_string()));
          chars.next();
        } else {
          tokens.push(Token::Operator("/".to_string()));
        }
      }
      '"' => {
        chars.next();
        let mut string_lit = String::new();
        while let Some(&ch) = chars.peek() {
          if ch == '"' {
            break;
          }
          string_lit.push(ch);
          chars.next();
        }
        chars.next();
        tokens.push(Token::StringLiteral(string_lit));
      }
      _ => {
        return Err(Error::Lexical(format!(
          "Unexpected character '{}' at position {}",
          ch,
          input.len() - chars.clone().count()
        )));
      }
    }
  }

  tokens.push(Token::Eof);
  Ok(tokens)
}
