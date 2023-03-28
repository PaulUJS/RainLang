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

    pub fn get_var(self: &mut Self, name: String) -> Expression {
        if self.values.contains_key(&name) {
            return self.values[&name].clone();
        } else {
            panic!();
        }
    }
}
