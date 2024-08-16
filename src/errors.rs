#[derive(Debug)]
pub enum Error {
  LexicalError(String),  // Errors during tokenization
  SyntaxError(String),   // Errors during parsing
  TypeError(String),     // Errors during type checking
  CodeGenError(String),  // Errors during code generation
}

impl std::fmt::Display for Error {
  fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
    match self {
      Error::LexicalError(msg) => write!(f, "Lexical error: {}", msg),
      Error::SyntaxError(msg) => write!(f, "Syntax error: {}", msg),
      Error::TypeError(msg) => write!(f, "Type error: {}", msg),
      Error::CodeGenError(msg) => write!(f, "Code generation error: {}", msg),
    }
  }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;
