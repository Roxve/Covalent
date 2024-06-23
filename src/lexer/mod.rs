use crate::err::{ATErr, ErrKind};

pub mod lex;
pub mod token;

use self::token::Token;
#[derive(Debug, Clone)]
pub struct Lexer {
    line: u16,
    column: u16,
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

    fn at(&mut self) -> char {
        let c = (&self.code).as_bytes()[self.pos].clone() as char;
        if &c == &'\n' {
            self.line += 1;
            self.column = 0;
        }

        c.clone()
    }

    fn eat(&mut self) -> char {
        self.pos += 1;
        self.column += 1;
        (&self.code).as_bytes()[self.pos - 1].clone() as char
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
