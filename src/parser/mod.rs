pub mod ast;
pub mod parse;
use self::ast::{Blueprint, Expr, Ident, Program};
use crate::err::{ATErr, ErrKind};
use crate::lexer::token::Token;
use crate::lexer::Lexer;
use crate::scope::Scope;
use crate::types::{AtomKind, AtomType};
use ast::Node;

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    line: u16,
    column: u16,
    current_tok: Option<Token>,
    pub program: Program,
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
            program: Program {
                body: Vec::new(),
                functions: Vec::new(),
                types: Vec::new(),
            },
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

    pub fn push_function(&mut self, name: Ident, params: Vec<Ident>, body: Vec<Node>) {
        self.program.functions.push(Blueprint { name,params, body });
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

    pub fn untyped(&self, expr: Expr) -> Node {
        Node {
            expr,
            ty: AtomType {
                kind: AtomKind::Unknown(0),
                details: None,
            },
            line: self.line,
            start: self.column,
            end: self.column,
        }
    }
}
