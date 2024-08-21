use crate::lexer::Token;
use crate::ast::{Expr, Statement};
use crate::errors::Error;

fn parse_let(tokens: &mut Vec<Token>) -> Result<Statement, Error> {
  tokens.remove(0); // assuming the first token is "let"
  if let Token::Identifier(name) = tokens.remove(0) {
    if let Token::Operator(op) = tokens.remove(0) {
      if op == "=" {
        let expr = parse_expr(tokens)?;
        return Ok(Statement::Let(name, expr));
      }
    }
  }
  Err(Error::Syntax("Syntax error in let statement".into()))
}

fn parse_return(tokens: &mut Vec<Token>) -> Result<Statement, Error> {
  tokens.remove(0); // assuming the first token is "ret"
  let expr = parse_expr(tokens)?;
  Ok(Statement::Return(expr))
}

fn parse_expr(tokens: &mut Vec<Token>) -> Result<Expr, Error> {
  // parsing primary expression
  let mut left = match tokens.remove(0) {
    Token::Number(n) => Expr::Number(n),
    Token::Identifier(name) => {
      // Check if this identifier is a function call
      if let Some(Token::OpenParen) = tokens.first() {
        tokens.remove(0); // Remove the '('
        let mut args = Vec::new();
        // Parse function arguments
        while let Some(token) = tokens.first() {
          if let Token::CloseParen = token {
            tokens.remove(0); // Remove the ')'
            break;
          }
          if let Token::Comma = token {
            tokens.remove(0); // Remove the ','
            continue;
          }
          args.push(parse_expr(tokens)?);
        }
        Expr::Call(name, args)
      } else {
        Expr::Identifier(name)
      }
    },
    _ => return Err(Error::Syntax("Unexpected token in expression".into())),
  };
  // Checking binary operator following the expression
  while let Some(Token::Operator(op)) = tokens.first() {
    let op = op.clone();
    tokens.remove(0);
    let right = parse_expr(tokens)?;
    left = Expr::BinaryOp(Box::new(left), op, Box::new(right));
  }
  Ok(left)
}

fn parse_fnc(tokens: &mut Vec<Token>) -> Result<Statement, Error> {
  tokens.remove(0); // Assuming the first token is "fnc"

  let name = match tokens.remove(0) {
    Token::Identifier(name) => name,
    _ => return Err(Error::Syntax("Expected function name".into())),
  };

  if let Token::OpenParen = tokens.remove(0) {
  } else {
    return Err(Error::Syntax("Expected '(' after function name".into()));
  }

  let mut params = Vec::new();
  while let Token::Identifier(param_name) = tokens.remove(0) {
    if let Token::Identifier(param_type) = tokens.remove(0) {
      params.push((param_name, param_type));
    } else {
      return Err(Error::Syntax("Expected parameter type".into()));
    }

    match tokens.remove(0) {
      Token::CloseParen => break,
      Token::Comma => continue,
      _ => return Err(Error::Syntax("Expected ',' or ')' after parameter".into())),
    }
  }

  // TODO: use this
  let _return_type = match tokens.remove(0) {
    Token::Identifier(ret_type) => ret_type,
    _ => return Err(Error::Syntax("Expected return type".into())),
  };

  if let Token::Arrow = tokens.remove(0) {
  } else {
    return Err(Error::Syntax("Expected '->' before function body".into()));
  }

  let mut body = Vec::new();
  while let Some(token) = tokens.first() {
    match token {
      Token::End => {
        tokens.remove(0);
        break;
      }
      Token::Eof => return Err(Error::Syntax("Unexpected EOF in function body".into())),
      Token::Let => {
        body.push(parse_let(tokens)?);
      }
      Token::Return => {
        body.push(parse_return(tokens)?);
      }
      _ => {
        body.push(Statement::Expr(parse_expr(tokens)?));
      }
    }
  }

  Ok(Statement::Function(name, params, body))
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Vec<Statement>, Error> {
  let mut statements = Vec::new();
  while !tokens.is_empty() {
    match tokens[0] {
      Token::Let => statements.push(parse_let(tokens)?),
      Token::Return => statements.push(parse_return(tokens)?),
      Token::Fnc => statements.push(parse_fnc(tokens)?),
      Token::Eof => break,
      _ => return Err(Error::Syntax(format!("Unexpected token: {:?}", tokens[0]))),
    }
  }
  Ok(statements)
}