use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::fs;

mod lexer;
mod ast;
mod parser;
mod checker;
mod c_generator;
mod errors;

fn main() {
  let source = "
    fnc main() int ->
      let x = 5
      let y = x + 2
      print(y)
      ret y
    end
  ";

  match lexer::tokenize(source) {
    Ok(mut tokens) => {
      match parser::parse(&mut tokens) {
        Ok(ast) => {
          let mut file = File::create("output.c").expect("Failed to create a C file");
          file.write_all(c_generator::generate_c_code(&ast).as_bytes())
            .expect("Failed to write to C file");
        }
        Err(e) => eprintln!("Parsing Error: {}", e),
      }
    }
    Err(e) => eprintln!("Lexical Error: {}", e),
  }

  Command::new("gcc")
    .args(&["output.c", "-o", "output"])
    .status()
    .expect("Failed to compile C code");

  fs::remove_file("output.c").expect("Failed to delete C file");
}
