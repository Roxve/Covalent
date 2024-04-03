pub mod analysis;
use crate::{ir::Enviroment, parser::ast::Expr};

pub struct Analyzer {
    env: Enviroment
}

pub struct TypedExpr {
    pub expr: Expr,
    pub rc: i16
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            env: Enviroment::new(None)
        }
    }
}