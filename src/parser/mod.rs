pub mod ast;
pub mod parse;
use crate::lexer::Lexer;
use crate::lexer::Token;
use crate::source::{ATErr, ErrKind, Ident, Scope};
use ast::Node;

#[derive(Debug, Clone)]

pub struct Function {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub body: Vec<Node>,
}

impl Function {
    /*pub fn get_name(&self) -> String {
        self.name.val.clone()
    }*/
}

#[derive(Debug, Clone)]
pub struct Parser {
    lexer: Lexer,
    line: u32,
    column: u32,
    current_tok: Option<Token>,
    pub functions: Vec<Function>,
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
        self.functions.push(Function { name, args, body });
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
