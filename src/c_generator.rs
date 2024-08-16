use crate::ast::{Statement, Expr};

pub fn generate_c_code(ast: &Vec<Statement>) -> String {
  let mut c_code = String::new();
  let mut needs_stdio = false; // TODO: Use a more general thing
  for statement in ast {
    c_code.push_str(&translate_statement(statement, &mut needs_stdio));
  }
  let mut final_code = String::new();
  if needs_stdio {
    final_code.push_str("#include <stdio.h>\n\n");
  }
  final_code.push_str(&c_code);
  final_code
}

fn translate_statement(statement: &Statement, needs_stdio: &mut bool) -> String {
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
        func_code.push_str(&translate_statement(stmt, needs_stdio));
      }
      func_code.push_str("}\n");
      func_code
    }
    Statement::Let(name, expr) => {
      format!("int {} = {};\n", name, translate_expr(expr, needs_stdio))
    }
    Statement::Return(expr) => {
      format!("return {};\n", translate_expr(expr, needs_stdio))
    },
    Statement::Expr(expr) => {
      format!("{};\n", translate_expr(expr, needs_stdio))
    },
    _ => panic!("Unhandled statement type"),
  }
}

fn translate_expr(expr: &Expr, needs_stdio: &mut bool) -> String {
  match expr {
    Expr::Number(n) => n.to_string(),
    Expr::Identifier(name) => name.clone(),
    Expr::BinaryOp(left, op, right) => {
      format!("{} {} {}", translate_expr(left, needs_stdio), op, translate_expr(right, needs_stdio))
    },
    Expr::Call(func_name, args) => {
      if func_name == "print" {
        *needs_stdio = true;
        let mut print_code = String::from("printf(");
        for (i, arg) in args.iter().enumerate() {
          if i > 0 {
            print_code.push_str(", ");
          }
          // TODO handle string etc
          print_code.push_str(&format!("\"%d\", {}", translate_expr(arg, needs_stdio)));
        }
        print_code.push_str(")");
        print_code
      } else {
        // Handle other function calls normally
        let mut call_code = format!("{}(", func_name);
        for (i, arg) in args.iter().enumerate() {
          if i > 0 {
            call_code.push_str(", ");
          }
          call_code.push_str(&translate_expr(arg, needs_stdio));
        }
        call_code.push_str(")");
        call_code
      }
    }
  }
}