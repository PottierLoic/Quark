use crate::ast::Statement;
use crate::ast::Expr;

pub fn generate_c_code(ast: &Vec<Statement>) -> String {
  let mut c_code = String::new();
  for statement in ast {
    c_code.push_str(&translate_statement(statement));
  }
  c_code
}

fn translate_statement(statement: &Statement) -> String {
  match statement {
    Statement::Function(name, params, body) => {
      let mut func_code = format!("int {}(", name);
      for (i, (param_name, param_type)) in params.iter().enumerate() {
        if i > 0 {
          func_code.push_str(", ");
        }
        func_code.push_str(&format!("{} {}", param_type, param_name));
      }
      func_code.push_str(") {\n");
      for stmt in body {
        func_code.push_str(&translate_statement(stmt));
      }
      func_code.push_str("}\n");
      func_code
    }
    Statement::Let(name, expr) => {
      format!("int {} = {};\n", name, translate_expr(expr))
    }
    Statement::Return(expr) => {
      format!("return {};\n", translate_expr(expr))
    }
    _ => panic!("Unhandled statement type"),
  }
}

fn translate_expr(expr: &Expr) -> String {
  match expr {
    Expr::Number(n) => n.to_string(),
    Expr::Identifier(name) => name.clone(),
    Expr::BinaryOp(left, op, right) => {
      format!("{} {} {}", translate_expr(left), op, translate_expr(right))
    }
    _ => panic!("Unhandled expression type"),
  }
}