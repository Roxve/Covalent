use crate::source::Ident;
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
}

pub fn get_operator_level(op: &str) -> u8 {
    match op {
        "&" | "|" => 1,
        "==" => 2,
        "<" | ">" => 3,
        "+" | "-" => 4,
        "*" | "/" => 5,
        _ => todo!(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    BinaryExpr {
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    Ident(Ident),
    VarDeclare {
        name: Ident,
        val: Box<Expr>,
    },
    VarAssign {
        name: Ident,
        val: Box<Expr>,
    },
    // fn declare ast is genereated in a special Vec in Source
    FnCall {
        name: Ident,
        args: Vec<Expr>,
    },
    IfExpr {
        condition: Box<Expr>,
        body: Vec<Expr>,
        alts: Vec<Expr>,
    },
    Discard(Box<Expr>),
    Block(Vec<Expr>),
    PosInfo(String, u32, u32), // debugging
    RetExpr(Box<Expr>),
}
