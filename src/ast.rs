// #[derive(Debug, Clone, PartialEq)]
// pub enum Type {
//   Int,
//   Float,
//   String,
//   Bool,
//   Void, // Non return type
//   Unknown, // Should never happend
// }

#[derive(Debug)]
pub enum Expr {
  Number(i32),
  Identifier(String),
  BinaryOp(Box<Expr>, String, Box<Expr>), // x + y
  Call(String, Vec<Expr>),                // func(a, b)
}

#[derive(Debug)]
pub enum Statement {
  Let(String, Expr),       // let x = expr;
  Return(Expr),            // ret expr;
  If(Expr, Vec<Statement>, Option<Vec<Statement>>),  // if expr -> ... else ...
  While(Expr, Vec<Statement>),                       // while expr -> ...
  Function(String, Vec<(String, String)>, Vec<Statement>),     // fnc name(args) ->
  Block(Vec<Statement>),                             // General block of statements
  Expr(Expr)                                         // Standalone expression as a statement
}