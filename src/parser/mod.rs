pub mod parse;
use crate::source::{ATErr, ErrKind};
use crate::source::Token;
use crate::ast::{Ident, Expr};
use crate::lexer::Tokenize;
#[derive(Debug, Clone)]

pub struct Function {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub body: Vec<Expr>,
}

impl Function {
    pub fn get_name(&self) -> String {
        self.name.val.clone()
    }
}


#[derive(Debug, Clone)]
pub struct Parser {
    code: String,
    pub line: u32,
    pub column: u32,
    current_tok: Option<Token>,
    next_tok: Option<Token>,
    pub functions: Vec<Function>,
    errors: Vec<ATErr>,
    warnings: Vec<ATErr>, // program can continue error
}

impl Parser {
    pub fn new(code: String) -> Self {
        Self {
            code,
            line: 1,
            column: 0,
            current_tok: None,
            next_tok: None,
            functions: vec![],
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn err(&mut self, kind: ErrKind, msg: String) {
        let err = ATErr {
            kind,
            msg,
            line: self.line,
            column: self.column,
        };
        self.errors.push(err.clone());
        err.out_error();
    }

    // TODO gen funcs for each arg type making args less dynamic for faster exe

    pub fn get_function(&self, name: String) -> Option<Function> {
        for fun in self.functions.clone().into_iter() {
            if fun.get_name() == name {
                return Some(fun);
            }
        }
        return None;
    }

    pub fn push_function(&mut self, name: Ident, args: Vec<Ident>, body: Vec<Expr>) {
        self.functions.push(Function { name, args, body });
    }
    pub fn not_eof(&self) -> bool {
        self.code.len() > 0
    }
    pub fn eat(&mut self) -> char {
        let p = self.at();
        self.code.remove(0);
        self.column += 1;
        return p;
    }

    pub fn at(&self) -> char {
        return self.code.as_bytes()[0] as char;
    }

    pub fn set(&mut self, tok: Token) -> Token {
        self.current_tok = self.next_tok.clone();
        self.next_tok = Some(tok.clone());
        return tok;
    } 

    pub fn next(&mut self) -> Token {
        if self.next_tok.is_none() {
            if self.current_tok.is_none() {
                self.tokenize();
            }
            self.current_tok = self.next_tok.clone();
        }

        return self.next_tok.clone().expect("None");
    }

    pub fn current(&mut self) -> Token {
        if self.current_tok.is_none() {
            self.tokenize();
            self.current_tok = self.next_tok.clone();
            self.tokenize();
        }
        return self.current_tok.clone().expect("None");
    }

    pub fn except(&mut self, tok: Token) -> Token {
        if self.current() != tok {
            let t = self.current();
            self.tokenize();

            self.err(
                ErrKind::UnexceptedTokenE,
                format!("unexcepted token [{:?}] excepted [{:?}]", t, tok),
            );
            return Token::Err("unexcepted token".to_string());
        }

        return self.tokenize();
    }
}
