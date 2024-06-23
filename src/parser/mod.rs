pub mod ast;
pub mod parse;
use self::ast::{Blueprint, Ident};
use crate::err::{ATErr, ErrKind};
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use crate::scope::Scope;
use ast::Node;

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    line: u16,
    column: u16,
    current_tok: Option<Token>,
    pub functions: Vec<Blueprint>,
    current_scope: Scope,
    pub errors: Vec<ATErr>,
}

impl Parser {
    pub fn new(code: String) -> Self {
        Self {
            lexer: Lexer::new(code),
            line: 1,
            column: 0,
            current_tok: None,
            functions: vec![],
            current_scope: Scope::Top,
            errors: Vec::new(),
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

    pub fn push_function(&mut self, name: Ident, args: Vec<Ident>, body: Vec<Node>) {
        self.functions.push(Blueprint { name, args, body });
    }
    fn current(&mut self) -> Token {
        if self.current_tok.is_none() {
            self.next();
        }
        self.current_tok.clone().unwrap()
    }
    fn next(&mut self) -> Token {
        let next = self.lexer.tokenize();
        self.current_tok = Some(next.clone());
        next
    }
    pub fn except(&mut self, tok: Token) -> Token {
        if self.current() != tok {
            let t = self.current();
            self.next();

            self.err(
                ErrKind::UnexceptedTokenE,
                format!("unexcepted token [{:?}] excepted [{:?}]", t, tok),
            );
            Token::Err("unexcepted token".to_string())
        } else {
            self.next()
        }
    }
}
