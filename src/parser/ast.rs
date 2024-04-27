use crate::source::{ConstType, Ident};
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
}

impl Literal {
    pub fn get_ty(&self) -> ConstType {
        match self {
            &Self::Int(_) => ConstType::Int,
            &Self::Float(_) => ConstType::Float,
            &Self::Str(_) => ConstType::Str,
            &Self::Bool(_) => ConstType::Bool,
        }
    }
}

pub fn get_operator_level(op: &str) -> u8 {
    match op {
        "&" | "|" => 1,
        "==" => 2,
        "<" | ">" | ">=" | "<=" => 3,
        "+" | "-" => 4,
        "*" | "/" | "%" => 5,
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
        name: Box<Expr>,
        val: Box<Expr>,
    },
    // fn declare ast is genereated in parser.functions
    FnCall {
        name: Box<Expr>,
        args: Vec<Expr>,
    },

    IfExpr {
        condition: Box<Expr>,
        body: Vec<Expr>,
        alt: Option<Box<Expr>>,
    },

    WhileExpr {
        condition: Box<Expr>,
        body: Vec<Expr>,
    },

    MemberExpr {
        parent: Box<Expr>,
        child: Box<Expr>,
    },
    Discard(Box<Expr>),
    Block(Vec<Expr>),
    PosInfo(String, u32, u32), // debugging
    RetExpr(Box<Expr>),
}
