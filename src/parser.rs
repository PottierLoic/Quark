use crate::lexer::Token;
use crate::ast::{Expr, Statement, Type};
use crate::errors::Error;

fn parse_type(tokens: &mut Vec<Token>) -> Result<Type, Error> {
  match tokens.remove(0) {
    Token::Identifier(type_name) => match type_name.as_str() {
      "int" => Ok(Type::Int),
      "float" => Ok(Type::Float),
      "string" => Ok(Type::String),
      "bool" => Ok(Type::Bool),
      "void" => Ok(Type::Void),
      _ => Err(Error::Syntax(format!("Unknown type: {}", type_name))),
    },
    Token::OpenBracket => {
      tokens.remove(0); // Remove '['
      let inner_type = parse_type(tokens)?;
      if let Token::CloseBracket = tokens.remove(0) {
        Ok(Type::Array(Box::new(inner_type)))
      } else {
        Err(Error::Syntax("Expected ']' after array type".into()))
      }
    },
    _ => Err(Error::Syntax("Expected type".into())),
  }
}

fn parse_let(tokens: &mut Vec<Token>) -> Result<Statement, Error> {
  tokens.remove(0); // Remove 'let'
  let name = match tokens.remove(0) {
    Token::Identifier(name) => name,
    _ => return Err(Error::Syntax("Expected identifier after 'let'".into())),
  };
  
  let type_annotation = if let Some(Token::Colon) = tokens.first() {
    tokens.remove(0); // Remove ':'
    Some(parse_type(tokens)?)
  } else {
    None
  };

  if let Token::Operator(op) = tokens.remove(0) {
    if op == "=" {
      let expr = parse_expr(tokens)?;
      Ok(Statement::Let(name, type_annotation, expr))
    } else {
      Err(Error::Syntax("Expected '=' in let statement".into()))
    }
  } else {
    Err(Error::Syntax("Expected '=' in let statement".into()))
  }
}

fn parse_expr(tokens: &mut Vec<Token>) -> Result<Expr, Error> {
    match tokens.remove(0) {
        Token::Number(n) => Ok(Expr::Number(n)),
        Token::StringLiteral(s) => Ok(Expr::String(s)),
        Token::Identifier(name) => {
            if let Some(Token::OpenParen) = tokens.first() {
                tokens.remove(0); // Remove the '('
                let mut args = Vec::new();
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
                Ok(Expr::Call(name, args))
            } else if let Some(Token::OpenBracket) = tokens.first() {
                tokens.remove(0); // Remove '['
                let index = parse_expr(tokens)?;
                if let Token::CloseBracket = tokens.remove(0) {
                    Ok(Expr::ArrayAccess(Box::new(Expr::Identifier(name)), Box::new(index)))
                } else {
                    Err(Error::Syntax("Expected ']' after array index".into()))
                }
            } else {
                Ok(Expr::Identifier(name))
            }
        },
        Token::OpenBracket => {
            let mut elements = Vec::new();
            while let Some(token) = tokens.first() {
                if let Token::CloseBracket = token {
                    tokens.remove(0);
                    break;
                }
                elements.push(parse_expr(tokens)?);
                if let Some(Token::Comma) = tokens.first() {
                    tokens.remove(0);
                }
            }
            Ok(Expr::ArrayLiteral(elements))
        },
        _ => Err(Error::Syntax("Unexpected token in expression".into())),
    }
}

fn parse_for(tokens: &mut Vec<Token>) -> Result<Statement, Error> {
  tokens.remove(0); // Remove 'for'
  let iterator = match tokens.remove(0) {
    Token::Identifier(name) => name,
    _ => return Err(Error::Syntax("Expected identifier after 'for'".into())),
  };
  
  if let Token::Identifier(keyword) = tokens.remove(0) {
    if keyword != "in" {
      return Err(Error::Syntax("Expected 'in' after iterator in for loop".into()));
    }
  } else {
    return Err(Error::Syntax("Expected 'in' after iterator in for loop".into()));
  }
  
  let iterable = parse_expr(tokens)?;
  
  if let Token::Arrow = tokens.remove(0) {
  } else {
    return Err(Error::Syntax("Expected '->' before for loop body".into()));
  }
  
  let body = parse_block(tokens)?;
  
  Ok(Statement::For(iterator, iterable, body))
}

fn parse_block(tokens: &mut Vec<Token>) -> Result<Vec<Statement>, Error> {
  let mut statements = Vec::new();
  while let Some(token) = tokens.first() {
    match token {
      Token::End => {
        tokens.remove(0);
        break;
      }
      Token::Eof => return Err(Error::Syntax("Unexpected EOF in block".into())),
      _ => statements.push(parse_statement(tokens)?),
    }
  }
  Ok(statements)
}

fn parse_statement(tokens: &mut Vec<Token>) -> Result<Statement, Error> {
  match tokens[0] {
    Token::Let => parse_let(tokens),
    Token::Return => parse_return(tokens),
    Token::If => parse_if(tokens),
    Token::While => parse_while(tokens),
    Token::For => parse_for(tokens),
    Token::Fnc => parse_fnc(tokens),
    _ => Ok(Statement::Expr(parse_expr(tokens)?)),
  }
}

pub fn parse(tokens: &mut Vec<Token>) -> Result<Vec<Statement>, Error> {
  let mut statements = Vec::new();
  while !tokens.is_empty() {
    match tokens[0] {
      Token::Eof => break,
      _ => statements.push(parse_statement(tokens)?),
    }
  }
  Ok(statements)
}

fn parse_return(tokens: &mut Vec<Token>) -> Result<Statement, Error> {
    tokens.remove(0); // Remove 'ret'
    let expr = if tokens.first() != Some(&Token::End) {
        Some(parse_expr(tokens)?)
    } else {
        None
    };
    Ok(Statement::Return(expr))
}

fn parse_if(_tokens: &mut Vec<Token>) -> Result<Statement, Error> {
    // Implement if statement parsing
    todo!("Implement if statement parsing")
}

fn parse_while(_tokens: &mut Vec<Token>) -> Result<Statement, Error> {
    // Implement while loop parsing
    todo!("Implement while loop parsing")
}

fn parse_fnc(_tokens: &mut Vec<Token>) -> Result<Statement, Error> {
    // Implement function parsing
    todo!("Implement function parsing")
}