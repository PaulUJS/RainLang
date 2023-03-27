use crate::LiteralVal::*;
use crate::TokenType::*;

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralVal {
    IntVal(i64),
    FloatVal(f64),
    BoolVal(bool),
    StringVal(String),
    Identifier(String),
    Function,
    Terminator,
    Comparison,
    Operator,
    Grouping,
    LOOP,
    NullVal,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    PRINT,
    Fn,
    Var,
    Start,
    End,
    IF,
    ELSE,
    ELSEIF,
    FOR,
    WHILE,
    In,
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
    TRUE,
    FALSE,

    StringLiteral,
    NumLiteral,

    Return,
    NULL,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    pub tokentype: TokenType,
    pub lexeme: String,
    pub literal: LiteralVal,
    pub line: u64,
}

#[derive(Debug)]
pub struct Tokenizer {
    source: String,
    tokens: Vec<Token>,
    line: u64,
    writing: bool,
    current: u64,
}

impl Tokenizer {
    pub fn new(_source: &str) -> Self {
        Self {
            source: _source.to_string(),
            tokens: vec![],
            line: 1,
            writing: false,
            current: 1,
        }
    }

    pub fn split_tokens(self: &mut Self) -> &Vec<Token> {
        let start = self
            .source
            .replace("(", " ( ")
            .replace(")", " ) ")
            .replace("'", " ' ")
            .replace(";", " ; ");

        let mut end: Vec<&str> = start.split_whitespace().collect();
        self.scan_tokens(end.as_mut_slice())
    }

    fn scan_tokens(self: &mut Self, arr: &mut [&str]) -> &Vec<Token> {
        println!("{:?}", arr);
        let mut index = 0;
        while index < arr.len() {
            if self.current < self.line {
                self.current += 1;
            }
            match arr[index] {
                "fn" => self.add_token(Fn, "", Function),
                "if" => self.add_token(IF, "", Comparison),
                "else" => self.add_token(ELSE, "", Comparison),
                "else if" => self.add_token(ELSEIF, "", Comparison),
                "start" => self.add_token(Start, "", Terminator),
                "end" => self.add_token(End, "", Terminator),
                "then" => self.add_token(Then, "", Terminator),
                "do" => self.add_token(Do, "", Terminator),
                "true" => self.add_token(TRUE, "", BoolVal(true)),
                "false" => self.add_token(FALSE, "", BoolVal(false)),
                "return" => self.add_token(Return, "", Terminator),
                "var" => self.add_token(Var, "", Identifier("var".to_string())),
                "for" => self.add_token(FOR, "", LOOP),
                "while" => self.add_token(WHILE, "", LOOP),
                "in" => self.add_token(In, "", Comparison),
                "+" => self.add_token(Plus, "+", Operator),
                "-" => self.add_token(Minus, "-", Operator),
                "*" => self.add_token(Star, "*", Operator),
                "/" => self.add_token(Slash, "/", Operator),
                ";" => self.add_token(SemiColon, ";", Terminator),
                "=" => self.add_token(Equal, "=", Comparison),
                "==" => self.add_token(EqualEqual, "==", Comparison),
                "!" => self.add_token(Bang, "!", Comparison),
                "!=" => self.add_token(BangEqual, "!=", Comparison),
                ">" => self.add_token(Greater, ">", Comparison),
                ">=" => self.add_token(GreaterEqual, ">=", Comparison),
                "<" => self.add_token(Less, "<", Comparison),
                "<=" => self.add_token(LessEqual, "<=", Comparison),
                ")" => self.add_token(LeftParen, ")", Grouping),
                "(" => self.add_token(RightParen, "(", Grouping),
                "'" => {
                    index += 1;
                    self.add_token(Quote, "", Grouping);
                    let mut finalstr = "".to_owned();

                    while arr[index] != "'" {
                        finalstr.push_str(arr[index]);
                        index += 1;
                    }
                    index += 1;
                    self.add_token(StringLiteral, &finalstr, StringVal(finalstr.to_string()));
                    self.add_token(Quote, "", Grouping) 
                },
                "print" => {
                    if arr[index+1] == "'" {
                        index += 1;
                    }

                    let mut finalstr = "".to_owned();
                    while arr[index + 1] != "'" {
                        if finalstr != "" {
                            finalstr.push_str(" ");
                            finalstr.push_str(arr[index + 1]);
                            index += 1;
                        } else {
                            finalstr.push_str(arr[index + 1]);
                            index += 1;
                        }
                    }
                    index += 1;                        
                    self.add_token(PRINT, &finalstr, StringVal(finalstr.to_string()));
                },
                "Null" => self.add_token(NULL, "", NullVal),
                _ => { 
                    if self.is_num(arr[index]) {
                        let val = arr[index].parse::<i64>();
                        match val {
                            Ok(value) => self.add_token(NumLiteral, arr[index], IntVal(value)),
                            Err(_) => println!("Unable to validate type of {}", arr[index].to_string()),
                        };
                    };
                },
            };
            index += 1;
        }
        self.add_token(Eof, "", Terminator);
       //  println!("{:#?}", self.tokens);
        return &self.tokens;
    }
    
    fn is_num(self: &mut Self, num: &str) -> bool {
        for i in num.chars() {
            if !i.is_numeric() {
                return false;
            }
        }
        return true;
    }

    fn add_token(self: &mut Self, token: TokenType, lexeme: &str, literal: LiteralVal) {
        if literal == Terminator {
            self.line += 1
        };
        self.tokens.push(Token {
            tokentype: token,
            lexeme: lexeme.to_string(),
            literal: literal,
            line: self.current,
        });
    }
}
