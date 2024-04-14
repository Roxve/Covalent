pub mod ast;
pub mod parse;
use crate::lexer::Tokenize;
use crate::source::Token;
use crate::source::{ATErr, ErrKind, Ident};
use ast::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Value,
    _Func(String),
    Top,
}

impl Scope {
    pub fn is_used(&self) -> bool {
        let owned = self.to_owned();

        owned == Scope::Value
    }
}

#[derive(Debug, Clone)]

pub struct Function {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub body: Vec<Expr>,
}

impl Function {
    /*pub fn get_name(&self) -> String {
        self.name.val.clone()
    }*/
}

#[derive(Debug, Clone)]
pub struct Parser {
    code: String,
    pub line: u32,
    pub column: u32,
    current_tok: Option<Token>,
    pub functions: Vec<Function>,
    pub current_scope: Scope,
    errors: Vec<ATErr>,
    _warnings: Vec<ATErr>, // program can continue error
}

impl Parser {
    pub fn new(code: String) -> Self {
        Self {
            code,
            line: 1,
            column: 0,
            current_tok: None,
            functions: vec![],
            current_scope: Scope::Top,
            errors: Vec::new(),
            _warnings: Vec::new(),
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

    /*pub fn get_function(&self, name: String) -> Option<Function> {
        for fun in self.functions.clone().into_iter() {
            if fun.get_name() == name {
                return Some(fun);
            }
        }
        return None;
    }*/

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
        self.current_tok = Some(tok.clone());
        return tok;
    }

    pub fn current(&mut self) -> Token {
        if self.current_tok.is_none() {
            self.tokenize();
        }
        return self.current_tok.clone().expect("None");
    }

    pub fn except(&mut self, tok: Token) -> Token {
        dbg!(&tok);
        if self.current() != tok {
            let t = self.current();
            self.tokenize();

            self.err(
                ErrKind::UnexceptedTokenE,
                format!("unexcepted token [{:?}] excepted [{:?}]", t, tok),
            );
            return Token::Err("unexcepted token".to_string());
        }
        dbg!(&self.current());
        dbg!(&self.code);

        return self.tokenize();
    }
}
