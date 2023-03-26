use crate::tokenizer::*;
use crate::syntaxtree::*;
use crate::TokenType::*;
use crate::LiteralVal::*;
use crate::Expression::*;

macro_rules! matchtokens {
    ($parser:ident, $($token:ident),+) => {{
        let mut result = false;
        $(
            result |= $parser.match_token($token);
        )*
        result
    }}
}

pub struct Parser {
    tokens: Vec<Token>,
    index: usize,
}

impl Parser {
    pub fn new(_token: Vec<Token>) -> Self {
        Self {
            tokens: _token,
            index: 0,
        }
    }

    fn expression(self: &mut Self) -> Expression {
        self.equality()
    }

    fn equality(self: &mut Self) -> Expression {
        let mut expr = self.comparison();
        while matchtokens!(self, Bang, BangEqual) {
            let op = self.previous();
            let right = self.comparison();
            expr = Binary { left: Box::from(expr), operator: op, right: Box::from(right) };
        }
        expr
    }

    fn comparison(self: &mut Self) -> Expression {
        let mut expr = self.term();
        while matchtokens!(self, Greater, GreaterEqual, Less, LessEqual) {
            let op = self.previous();
            let right = self.term();
            expr = Binary { left: Box::from(expr), operator: op, right: Box::from(right) };
        }
        expr
    }

    fn term(self: &mut Self) -> Expression {
        let mut expr = self.factor();
        while matchtokens!(self, Minus, Plus) {
            let op = self.previous();
            let right = self.factor();
            expr = Binary { left: Box::from(expr), operator: op, right: Box::from(right) };
        }
        expr
    }

    fn factor(self: &mut Self) -> Expression {
        let mut expr = self.unary();
        while matchtokens!(self, Slash, Star) {
            let op = self.previous();
            let right = self.unary();
            expr = Binary { left: Box::from(expr), operator: op, right: Box::from(right) };
        }
        expr
    }

    fn unary(self: &mut Self) -> Expression {
        if matchtokens!(self, Bang, Minus) {
            let op = self.previous();
            let right = self.unary();
            return Unary { operator: op, right: Box::from(right) };
        };
        self.primary()
    }

    fn primary(self: &mut Self) -> Expression {
        todo!()
    }

    pub fn match_token(self: &mut Self, token: TokenType) -> bool {
        if self.check(token) {
            self.advance();
            true
        } else {
            false
        }
    }
    
    fn check(self: &mut Self, token: TokenType) -> bool {
        if self.is_at_end() {
            false
        } else {
            self.peek().tokentype == token 
        }
    }

    fn previous(self: &mut Self) -> Token {
        self.tokens[self.index - 1].clone()
    }

    fn advance(self: &mut Self) -> Token {
        if !self.is_at_end() {
            self.index += 1;
        }
        self.previous()
    }

    fn is_at_end(self: &mut Self) -> bool {
        self.peek().tokentype == Eof
    }

    fn peek(self: &mut Self) -> Token {
        self.tokens[self.index].clone()
    }

    pub fn parse(self: &mut Self) {
        todo!()
    }
}