use crate::ast::{Statement, Expr, Type};
use crate::errors::Error;

pub fn generate_c_code(ast: &Vec<Statement>) -> Result<String, Error> {
  let mut c_code = String::new();
  let mut needs_stdio = false; // TODO: Use a more general thing
  for statement in ast {
    c_code.push_str(&translate_statement(statement, &mut needs_stdio)?);
  }
  let mut final_code = String::new();
  if needs_stdio {
    final_code.push_str("#include <stdio.h>\n\n");
  }
  final_code.push_str(&c_code);
  Ok(final_code)
}

fn translate_statement(statement: &Statement, needs_stdio: &mut bool) -> Result<String, Error> {
    match statement {
        Statement::Function(name, params, return_type, body) => {
            let mut func_code = format!("{} {}(", translate_type(return_type)?, name);
            for (i, (param_name, param_type)) in params.iter().enumerate() {
                if i > 0 {
                    func_code.push_str(", ");
                }
                func_code.push_str(&format!("{} {}", translate_type(param_type)?, param_name));
            }
            func_code.push_str(") {\n");
            for stmt in body {
                func_code.push_str(&translate_statement(stmt, needs_stdio)?);
            }
            func_code.push_str("}\n");
            Ok(func_code)
        }
        Statement::Let(name, type_annotation, expr) => {
            let type_str = match type_annotation {
                Some(t) => translate_type(t)?,
                None => "auto".to_string(),
            };
            Ok(format!("{} {} = {};\n", type_str, name, translate_expr(expr, needs_stdio)?))
        }
        Statement::Return(expr) => {
            match expr {
                Some(e) => Ok(format!("return {};\n", translate_expr(e, needs_stdio)?)),
                None => Ok("return;\n".to_string()),
            }
        },
        Statement::Expr(expr) => {
            Ok(format!("{};\n", translate_expr(expr, needs_stdio)?))
        },
        _ => Err(Error::CodeGen("Unhandled statement type".into())),
    }
}

fn translate_type(t: &Type) -> Result<String, Error> {
    match t {
        Type::Int => Ok("int".to_string()),
        Type::Float => Ok("float".to_string()),
        Type::String => Ok("char*".to_string()),
        Type::Bool => Ok("int".to_string()),
        Type::Void => Ok("void".to_string()),
        Type::Array(inner) => Ok(format!("{}*", translate_type(inner)?)),
        Type::Unknown => Err(Error::CodeGen("Unknown type".into())),
    }
}

fn translate_expr(expr: &Expr, needs_stdio: &mut bool) -> Result<String, Error> {
    match expr {
        Expr::Number(n) => Ok(n.to_string()),
        Expr::String(s) => Ok(format!("\"{}\"", s)),
        Expr::Boolean(b) => Ok(if *b { "1" } else { "0" }.to_string()),
        Expr::Identifier(name) => Ok(name.clone()),
        Expr::BinaryOp(left, op, right) => {
            Ok(format!("({} {} {})", translate_expr(left, needs_stdio)?, op, translate_expr(right, needs_stdio)?))
        },
        Expr::UnaryOp(op, expr) => {
            Ok(format!("{}({})", op, translate_expr(expr, needs_stdio)?))
        },
        Expr::Call(func_name, args) => {
            if func_name == "print" {
                *needs_stdio = true;
                let mut print_code = String::from("printf(");
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        print_code.push_str(", ");
                    }
                    print_code.push_str(&format!("\"%d\", {}", translate_expr(arg, needs_stdio)?));
                }
                print_code.push(')');
                Ok(print_code)
            } else {
                let mut call_code = format!("{}(", func_name);
                for (i, arg) in args.iter().enumerate() {
                    if i > 0 {
                        call_code.push_str(", ");
                    }
                    call_code.push_str(&translate_expr(arg, needs_stdio)?);
                }
                call_code.push(')');
                Ok(call_code)
            }
        },
        Expr::ArrayLiteral(elements) => {
            let mut array_code = String::from("{");
            for (i, elem) in elements.iter().enumerate() {
                if i > 0 {
                    array_code.push_str(", ");
                }
                array_code.push_str(&translate_expr(elem, needs_stdio)?);
            }
            array_code.push('}');
            Ok(array_code)
        },
        Expr::ArrayAccess(array, index) => {
            Ok(format!("{}[{}]", translate_expr(array, needs_stdio)?, translate_expr(index, needs_stdio)?))
        },
        Expr::Ternary(cond, true_expr, false_expr) => {
            Ok(format!("({}) ? ({}) : ({})",
                translate_expr(cond, needs_stdio)?,
                translate_expr(true_expr, needs_stdio)?,
                translate_expr(false_expr, needs_stdio)?))
        },
    }
}