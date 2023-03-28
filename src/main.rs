use std::fs;

pub mod tokenizer;
use crate::tokenizer::*;

pub mod parser;
use crate::parser::*;

pub mod syntaxtree;
use crate::syntaxtree::*;

pub mod environment;
use crate::environment::*;

fn main() -> Result<(), String> {
    println!("Currently running rain program!");
    
    read_file();

    Ok(println!("Rain finished!"))
}

fn read_file() -> Result<(), String> {
    match fs::read_to_string("test.ra") {
        Ok(contents) => Ok(tokenize_file(&contents)),
        Err(_msg) => Err("Unable to read file".to_string()),
    }
}

fn tokenize_file(contents: &str) {
    let mut scanner  = Tokenizer::new(contents);
    let tokens = scanner.split_tokens();
    parse_file(tokens.to_vec());
}
fn parse_file(contents: Vec<Token>) {
   let mut parser = Parser::new(contents);
   for mut i in parser.parse() {
       i.interpret();
   }
}
