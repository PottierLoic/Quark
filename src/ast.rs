use crate::lexer::Token;

#[derive(Debug)]
pub enum Expr {
  Number(i32),
  Identifier(String),
  BinaryOp(Box<Expr>, String, Box<Expr>), // x + y
  Call(String, Vec<Expr>),                // func(a, b) TODO
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

fn parse_let(tokens: &mut Vec<Token>) -> Statement {
  tokens.remove(0); // assuming the first token is "let"
  if let Token::Identifier(name) = tokens.remove(0) {
    if let Token::Operator(op) = tokens.remove(0) {
      if op == "=" {
        let expr = parse_expr(tokens);
        return Statement::Let(name, expr);
      }
    }
  }
  panic!("Syntax error in let statement");
}

fn parse_return(tokens: &mut Vec<Token>) -> Statement {
  tokens.remove(0); // assuming the first token is "ret"
  let expr = parse_expr(tokens);
  Statement::Return(expr)
}

fn parse_expr(tokens: &mut Vec<Token>) -> Expr {
  // parsing primary expression
  let mut left = match tokens.remove(0) {
    Token::Number(n) => Expr::Number(n),
    Token::Identifier(name) => {
      // Check if this identifier is a function call
      if let Some(Token::OpenParen) = tokens.get(0) {
        tokens.remove(0); // Remove the '('
        let mut args = Vec::new();
        // Parse function arguments
        while let Some(token) = tokens.get(0) {
          if let Token::CloseParen = token {
            tokens.remove(0); // Remove the ')'
            break;
          }
          if let Token::Comma = token {
            tokens.remove(0); // Remove the ','
            continue;
          }
          args.push(parse_expr(tokens));
        }
        Expr::Call(name, args)
      } else {
        Expr::Identifier(name)
      }
    },
    _ => panic!("Unexpected token in expression"),
  };
  // Checking binary operator following the expression
  while let Some(Token::Operator(op)) = tokens.get(0) {
    let op = op.clone();
    tokens.remove(0);
    let right = parse_expr(tokens);
    left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
  }
  left
}

fn parse_fnc(tokens: &mut Vec<Token>) -> Statement {
  tokens.remove(0); // Assuming the first token is "fnc"

  let name = match tokens.remove(0) {
    Token::Identifier(name) => name,
    _ => panic!("Expected function name"),
  };

  if let Token::OpenParen = tokens.remove(0) {
  } else {
    panic!("Expected '(' after function name");
  }

  let mut params = Vec::new();
  while let Token::Identifier(param_name) = tokens.remove(0) {
    if let Token::Identifier(param_type) = tokens.remove(0) {
      params.push((param_name, param_type));
    } else {
      panic!("Expected parameter type");
    }

    match tokens.remove(0) {
      Token::CloseParen => break,
      Token::Comma => continue,
      _ => panic!("Expected ',' or ')' after parameter"),
    }
  }

  // TODO: use this
  let _return_type = match tokens.remove(0) {
    Token::Identifier(ret_type) => ret_type,
    _ => panic!("Expected return type"),
  };

  if let Token::Arrow = tokens.remove(0) {
  } else {
    panic!("Expected '->' before function body");
  }

  let mut body = Vec::new();
  while let Some(token) = tokens.get(0) {
    match token {
      Token::End => {
        tokens.remove(0);
        break;
      }
      Token::EOF => panic!("Unexpected EOF in function body"),
      Token::Let => {
        body.push(parse_let(tokens));
      }
      Token::Return => {
        body.push(parse_return(tokens));
      }
      _ => {
        body.push(Statement::Expr(parse_expr(tokens)));
      }
    }
  }

  Statement::Function(name, params, body)
}

pub fn parse(tokens: &mut Vec<Token>) -> Vec<Statement> {
  let mut statements = Vec::new();
  while !tokens.is_empty() {
    match tokens[0] {
      Token::Let => statements.push(parse_let(tokens)),
      Token::Return => statements.push(parse_return(tokens)),
      Token::Fnc => statements.push(parse_fnc(tokens)),
      Token::EOF => break,
      _ => panic!("Unexpected token: {:?}", tokens[0]),
    }
  }
  statements
}