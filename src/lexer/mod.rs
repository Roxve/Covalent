use crate::source::{ATErr, ErrKind};

pub mod lex;
#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(String),
    // convert these into literal
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),

    Ident(String),
    Tag(String),
    Err(String), // error code and msg
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Exec,
    Colon,
    Comma,
    Dot,
    Access,
    IfKw,
    ElseKw,
    WhileKw,
    BreakKw,
    Continuekw,
    SetKw,
    RetKw,
    EOF,
}
#[derive(Debug, Clone)]
pub struct Lexer {
    line: u32,
    column: u32,
    code: String,
    pos: usize,
    pub errors: Vec<ATErr>,
}

impl Lexer {
    pub fn new(code: String) -> Self {
        Self {
            line: 0,
            column: 0,
            pos: 0,
            code,
            errors: Vec::new(),
        }
    }

    fn at(&self) -> char {
        self.code.clone().chars().nth(self.pos).unwrap()
    }

    fn eat(&mut self) -> char {
        self.pos += 1;
        self.column += 1;
        self.code.clone().chars().nth(self.pos - 1).unwrap()
    }

    fn not_eof(&self) -> bool {
        self.code.len() - 1 >= self.pos
    }

    fn err(&mut self, msg: String, kind: ErrKind) -> Token {
        let err = ATErr {
            kind,
            msg,
            line: self.line,
            column: self.column,
        };
        self.errors.push(err.clone());
        err.out_error();
        Token::Err(err.get_error())
    }
}
