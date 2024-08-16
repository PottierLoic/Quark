use std::process::Command;
use std::fs::File;
use std::io::Write;
use std::fs;

mod lexer;
mod ast;
mod gen;

fn main() {
  let quark_code = "
    fnc main() int ->
      let x = 5
      let y = x + 2
      ret y
    end
  ";

  let mut tokens = lexer::tokenize(quark_code);
  for i in 0..tokens.len() {
    println!("{:?}", tokens[i]);
  }

  let output_ast = ast::parse(&mut tokens);
  println!("{:#?}", output_ast);

  let c_code = gen::generate_c_code(&output_ast);
  println!("{}", c_code);

  let file_name = "output.c";

  let mut file = File::create(file_name).expect("Failed to create a C file");
  file.write_all(c_code.as_bytes()).expect("Failed to write to C file");

  Command::new("gcc")
    .args(&[file_name, "-o", "output"])
    .status()
    .expect("Failed to compile C code");

  fs::remove_file(file_name).expect("Failed to delete C file");
}
