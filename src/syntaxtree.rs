use crate::tokenizer::*;
use crate::LiteralVal::*;
use crate::TokenType::*;

#[derive(Debug)]
pub enum Statement {
    PrintStatement { expr: Expression },
    ExprStatement { expr: Expression },
    VarStatement { name: Token, init: Expression },
}

impl Statement {
    pub fn evaluate(self: &mut Self) -> LiteralVal {
        match self {
            Statement::ExprStatement { expr } => {
                expr.evaluate()
            },
            Statement::PrintStatement { expr } => {
                expr.evaluate()
            },
            Statement::VarStatement { name, init } => todo!(),
        }
    }

    pub fn interpret(self: &mut Self) {
        println!("{:#?}", self.evaluate());
    }
}

#[derive(Debug)]
pub enum Expression {
    Binary { left: Box<Expression>, operator: Token, right: Box<Expression> },
    Unary { operator: Token, right: Box<Expression> },
    Literal { value: LiteralVal },
    Grouping { expr: Box<Expression> },
}

impl Expression {
    pub fn evaluate(self: &mut Self) -> LiteralVal {
        match self {
            Self::Binary { left, operator, right } => {
                let left = (*left).evaluate();
                let right = (*right).evaluate();
                match (left, right, &operator.tokentype) {
                    (IntVal(x), IntVal(y), Plus) => IntVal(x+y),
                    (IntVal(x), IntVal(y), Minus) => IntVal(x-y),
                    (IntVal(x), IntVal(y), Star) => IntVal(x*y),
                    (IntVal(x), IntVal(y), Slash) => IntVal(x/y),
                    (IntVal(x), IntVal(y), Equal) => BoolVal(x==y),
                    (IntVal(x), IntVal(y), EqualEqual) => BoolVal(x==y),
                    (IntVal(x), IntVal(y), BangEqual) => BoolVal(x!=y),
                    (IntVal(x), IntVal(y), Less) => BoolVal(x<y),
                    (IntVal(x), IntVal(y), LessEqual) => BoolVal(x<=y),
                    (IntVal(x), IntVal(y), Greater) => BoolVal(x>y),
                    (IntVal(x), IntVal(y), GreaterEqual) => BoolVal(x>=y),
                    _ => todo!(), 
                }
            },
            Self::Unary { operator, right } => {
                let right = (*right).evaluate();
                match (right, &operator.tokentype) {
                    (IntVal(x), Minus) => IntVal(-x),
                    (FloatVal(x), Minus) => FloatVal(-x),
                    _ => todo!(),
                }
            },
            Self::Literal { value } => {
                match value {
                    IntVal(x) => IntVal(*x),
                    FloatVal(x) => FloatVal(*x),
                    BoolVal(x) => BoolVal(*x),
                    NullVal => NullVal,
                    StringVal(x) => StringVal(x.to_string()),
                    Terminator => Terminator,
                    Identifier(x) => Identifier(x.to_string()),
                    Grouping => Grouping,
                    _ => todo!(),
                }
            },
            Self::Grouping { expr } => expr.evaluate(),
        }
    }

}
