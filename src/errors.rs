#[derive(Debug)]
pub enum Error {
  Lexical(String),  // Errors during tokenization
  Syntax(String),   // Errors during parsing
  Type(String),     // Errors during type checking
  CodeGen(String),  // Errors during code generation
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::Lexical(msg) => write!(f, "Lexical error: {}", msg),
      Error::Syntax(msg) => write!(f, "Syntax error: {}", msg),
      Error::Type(msg) => write!(f, "Type error: {}", msg),
      Error::CodeGen(msg) => write!(f, "Code generation error: {}", msg),
    }
  }
}

impl std::error::Error for Error {}