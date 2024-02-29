use std::collections::HashMap;

use crate::ast::*;
use crate::ir::ConstType;

#[derive(Debug, Clone, PartialEq)]
// open file as current -> tokenize
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
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    IfKw,
    ElseKw,
    SetKw,
    RetKw,
    EOF,
}

#[derive(Debug, Clone)]
pub enum ErrKind {
    UnknownCharE = 0,
    UnexceptedTokenE = 1,
    UndeclaredVar = 2,
    VarAlreadyDeclared = 3,
    CannotConvertRight = 4, // in binary expressions right is always coverted to left
    UnexceptedArgs = 5,
}

#[derive(Debug, Clone)]
pub struct ATErr {
    pub kind: ErrKind,
    pub msg: String,
    pub line: u32,
    pub column: u32,
}

impl ATErr {
    pub fn get_error(&self) -> String {
        format!(
            "code:AT00{}\n{}\nat line:{}, column:{}",
            self.kind.clone() as u8,
            self.msg,
            self.line,
            self.column
        )
    }

    // customize later
    pub fn out_error(&self) {
        println!("{}", self.get_error());
    }
}

#[derive(Debug, Clone)]

pub struct Function {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub body: Vec<Expr>,
}

impl Function {
    pub fn get_name(&self) -> String {
        self.name.0.clone()
    }
}

// frontend generation -> feed into backend
#[derive(Debug, Clone)]
pub struct Source {
    pub code: String,
    pub line: u32,
    pub column: u32,
    pub current_tok: Option<Token>,
    pub next_tok: Option<Token>,
    pub functions: Vec<Function>,
    pub vars: HashMap<String, ConstType>,
    pub errors: Vec<ATErr>,
    pub warnings: Vec<ATErr>, // program can continue error
}

impl Source {
    pub fn new(code: String) -> Self {
        let src = Source {
            code,
            line: 1,
            column: 0,
            current_tok: None,
            next_tok: None,
            functions: vec![],
            vars: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        return src;
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
}
