pub mod analysis;
use crate::{
    ir::Enviroment,
    parser::ast::Literal,
    source::{ConstType, Ident},
};

pub struct Analyzer {
    env: Enviroment,
    line: u32,
    column: u32,
}

#[derive(Debug, Clone)]
pub enum AnalyzedExpr {
    Id(String, u16),
    Literal(Literal),
    BinaryExpr {
        op: String,
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    RetExpr(Box<TypedExpr>),

    VarDeclare {
        name: String,
        val: Box<TypedExpr>,
    },

    VarAssign {
        name: String,
        val: Box<TypedExpr>,
    },

    FnCall {
        name: String,
        args: Vec<TypedExpr>,
    },

    Func {
        ret: ConstType,
        name: String,
        args: Vec<Ident>,
        body: Vec<TypedExpr>,
    },
    Debug(String, u32, u32),
    Discard(Box<TypedExpr>),
    As(Box<TypedExpr>), // change an expr type if possible
}

#[derive(Debug, Clone)]
pub struct TypedExpr {
    pub expr: AnalyzedExpr,
    pub ty: ConstType,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            env: Enviroment::new(None),
            line: 0,
            column: 0,
        }
    }
}
