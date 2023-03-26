
pub enum LiteralVal {
    IntVal(i64),
    FloatVal(f64),
    BoolVal(bool),
    StringVal(String),
}

pub enum TokenType {
    Fn,
    Start,
    End,
    IF,
    ELSE,
    WHILE,
    Then,
    Do,

    SemiColon,
    LeftParen,
    RightParen,
    Quote,
    
    Plus,
    Minus,
    Star,
    Slash,
    Equal,
    EqualEqual,
    Bang,
    BangEqual,
    Less,
    LessEqual,
    Greater,
    GreaterEqual,

    Return,
    Eof
}

pub struct Token {
    tokentype: TokenType,
    lexeme: String,
    literal: LiteralVal,
    line: u64,
}

pub struct Tokenizer {
    source: String,
    tokens: Vec<Token>,
    line: u64,
    current: u64,
}

impl Tokenizer {
    pub fn new(_source: &str) -> Self {
        Self {
            source: _source.to_string(),
            tokens: vec![],
            line: 1,
            current: 1,
        }
    }

    pub fn split_tokens(self: &mut Self) -> Result<(), String> {
       let start = self.source
           .replace("(", " ( ")
           .replace(")", " ) ")
           .replace("'", " ' ")
           .replace(";", " ; ");

        let mut end: Vec<&str> = start.split_whitespace().collect();
        self.scan_tokens(end.as_mut_slice())
    }

    fn scan_tokens(self: &mut Self, arr: &mut [&str]) -> Result<(), String> {
        for i in arr {
            if self.current < self.line {
                self.current += 1;
            }
            match i as &str {
                "fn" => todo!(),
                "if" => todo!(),
                "else" => todo!(),
                "else if" => todo!(),
                "start" => todo!(),
                "end" => todo!(),
                "then" => todo!(),
                "do" => todo!(),
                "return" => todo!(),
                _ => todo!(),
            };
        }
        Ok(())
    }

    fn add_token(self: &mut Self, token: TokenType, lexeme: &str, literal: LiteralVal) {
        self.tokens.push(Token { tokentype: token, lexeme: lexeme.to_string(), literal: literal, line: self.current });
    }

    fn add_token_literal(self: &mut Self) -> Token {
        todo!()
    }
}
