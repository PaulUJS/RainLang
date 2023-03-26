use std::fs;

pub mod tokenizer;
use crate::tokenizer::*;

fn main() -> Result<(), String> {
    println!("Currently running rain program!");
    
    read_file();

    Ok(println!("Rain finished!"))
}

fn read_file() -> Result<(), String> {
    match fs::read_to_string("text.ra") {
        Ok(contents) => Ok(tokenize_file(&contents)),
        Err(_msg) => Err("Unable to read file".to_string()),
    }
}

fn tokenize_file(contents: &str) {
    let mut scanner  = Tokenizer::new(contents);
    scanner.split_tokens();
}
fn parse_file(contents: Vec<Token>) -> Result<(), String> {
    todo!()
}
