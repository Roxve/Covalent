pub mod analysis;
use crate::{ir::Enviroment, parser::ast::Expr, source::ConstType};

pub struct Analyzer {
    env: Enviroment,
}
pub enum AnalyzedExpr {
    Id(String, u16),
    BinaryExpr {
        op: String,
        left: Box<TypedExpr>,
        right: Box<TypedExpr>,
    },
    VarDeclare {
        name: String,
        val: Box<TypedExpr>,
    },
    VarAssign {
        name: String,
        val: Box<TypedExpr>,
    },

    Discard(Box<TypedExpr>),
    As(Box<TypedExpr>), // change an expr type if possible
}
pub struct TypedExpr {
    pub expr: AnalyzedExpr,
    pub ty: ConstType,
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            env: Enviroment::new(None),
        }
    }
}
