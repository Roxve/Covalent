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



// frontend generation -> feed into backend
