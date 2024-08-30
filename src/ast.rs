#[derive(Debug, Clone, PartialEq)]
pub enum Type {
  Int,
  Float,
  String,
  Bool,
  Void,
  Array(Box<Type>),
  Unknown,
}

#[derive(Debug)]
pub enum Expr {
  Number(f64),
  String(String),
  Boolean(bool),
  Identifier(String),
  BinaryOp(Box<Expr>, String, Box<Expr>),
  UnaryOp(String, Box<Expr>),
  Call(String, Vec<Expr>),
  ArrayLiteral(Vec<Expr>),
  ArrayAccess(Box<Expr>, Box<Expr>),
  Ternary(Box<Expr>, Box<Expr>, Box<Expr>),
}

#[derive(Debug)]
pub enum Statement {
  Let(String, Option<Type>, Expr),
  Return(Option<Expr>),
  If(Expr, Vec<Statement>, Option<Vec<Statement>>),
  While(Expr, Vec<Statement>),
  For(String, Expr, Vec<Statement>),
  Function(String, Vec<(String, Type)>, Type, Vec<Statement>),
  Block(Vec<Statement>),
  Expr(Expr),
}