use std::env;
use std::fs::{self, File};
use std::io::{Read, Write};
use std::process::Command;
use std::error::Error;

mod lexer;
mod ast;
mod parser;
mod checker;
mod c_generator;
mod errors;

fn main() -> Result<(), Box<dyn Error>>{
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    eprintln!("Usage: quark <source_file.quark>");
    std::process::exit(1);
  }

  let source_file = &args[1];
  let mut source = String::new();
  File::open(source_file)?.read_to_string(&mut source)?;

  let mut tokens = lexer::tokenize(&source)?;
  let ast = parser::parse(&mut tokens)?;
  let c_code = c_generator::generate_c_code(&ast)?;

  let mut file = File::create("output.c")?;
  file.write_all(c_code.as_bytes())?;

  Command::new("gcc")
    .args(["output.c", "-o", "output"])
    .status()?
    .success()
    .then_some(())
    .ok_or("Failed to compile C code")?;

  fs::remove_file("output.c")?;

  Ok(())
}
