use crate::tokenizer::*;

#[derive(Debug)]
pub enum Expression {
    Binary { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Unary { operator: Token, right: Box<Expression> },
    Literal { value: LiteralVal },
    Grouping { expr: Box<Expression> },
}
