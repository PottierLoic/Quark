use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::process::Command;

mod lexer;
mod ast;
mod parser;
mod checker;
mod c_generator;
mod errors;

fn main() {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    eprintln!("Usage: quark <source_file.quark>");
    std::process::exit(1);
  }

  let source_file = &args[1];
  let mut source = String::new();
  match File::open(source_file) {
    Ok(mut file) => {
      file.read_to_string(&mut source)
        .expect("Failed to read source file");
    }
    Err(e) => {
      eprintln!("Failed to open source file: {}", e);
      std::process::exit(1);
    }
  }

  match lexer::tokenize(&source) {
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
