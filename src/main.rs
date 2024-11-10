use std::env;
use std::error::Error;
use std::fs::File;
use std::io::Read;

pub mod lexer;

mod ast;
mod checker;
mod errors;
mod parser;

fn main() -> Result<(), Box<dyn Error>> {
  let args: Vec<String> = env::args().collect();
  if args.len() != 2 {
    eprintln!("Usage: quark <source_file.quark>");
    std::process::exit(1);
  }

  let source_file = &args[1];
  let mut source = String::new();
  File::open(source_file)?.read_to_string(&mut source)?;

  let tokens = lexer::tokenize(&source)?;
  println!("Tokens:");
  for token in &tokens {
    println!("{:?}", token);
  }

  // let ast = parser::parse(&mut tokens.clone())?;
  // println!("\nAST:");
  // println!("{:#?}", ast);
  //
  // let c_code = c_generator::generate_c_code(&ast)?;
  //
  // let mut file = File::create("output.c")?;
  // file.write_all(c_code.as_bytes())?;
  //
  // Command::new("gcc")
  //   .args(["output.c", "-o", "output"])
  //   .status()?
  //   .success()
  //   .then_some(())
  //   .ok_or("Failed to compile C code")?;
  //
  // fs::remove_file("output.c")?;
  //
  Ok(())
}
