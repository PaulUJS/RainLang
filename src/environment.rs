use std::collections::HashMap;

use crate::tokenizer::*;
use crate::LiteralVal::*;

use crate::syntaxtree::*;
use crate::Expression;

pub struct Environment {
    values: HashMap<String, Expression>,
}

impl Environment {
    pub fn new() -> Self {
        Self {
            values: HashMap::new(),
        }
    }
    pub fn define(self: &mut Self, name: String, val: Expression) {
        self.values.insert(name, val);
    }

    pub fn get_var(self: &mut Self, name: &String) -> Result<&Expression, bool> {
        println!("checking name {:#?}", name);
        if self.values.contains_key(name) {
            return Ok(&self.values[name]);
        } else {
            return Err(false);
        }
    }

    pub fn check(self: &mut Self, name: &String) -> bool {
        return self.values.contains_key(name);
    }

    pub fn print_env(self: &mut Self) {
        println!("                              ");
        println!("ALL VARIABLES IN THE RAIN FILE");
        println!("                              ");
        for (k, v) in &self.values {
            println!("{} / {:#?}", k, v);
        }
    }
}
