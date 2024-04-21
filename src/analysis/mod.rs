pub mod analysis;
use crate::{
    ir::Enviroment,
    parser::ast::Literal,
    source::{ConstType, Ident},
};

pub struct Analyzer {
    pub env: Enviroment,
    line: u32,
    column: u32,
}

#[derive(Debug, Clone)]
pub enum AnalyzedExpr {
    Import {
        module: String,
        name: String,
        args: Vec<ConstType>,
    },

    Id(String),
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
    If {
        cond: Box<TypedExpr>,
        body: Vec<TypedExpr>,
        alt: Option<Box<TypedExpr>>,
    },
    Block(Vec<TypedExpr>),
    Debug(String, u32, u32),
    Discard(Box<TypedExpr>),
    As(Box<TypedExpr>), // change an expr type if possible
}

#[derive(Debug, Clone)]
pub struct TypedExpr {
    pub expr: AnalyzedExpr,
    pub ty: ConstType,
}

#[inline]
pub fn supports_op(ty: &ConstType, op: &String) -> bool {
    match ty {
        &ConstType::Int | &ConstType::Float | &ConstType::Dynamic => true,
        &ConstType::Str => match op.as_str() {
            "+" => true,
            "==" | ">" | "<" | ">=" | "<=" => true,
            _ => false,
        },
        &ConstType::Void | &ConstType::Bool => false,
    }
}

fn get_ret_ty(expr: &TypedExpr, prev: ConstType) -> ConstType {
    match expr.expr {
        AnalyzedExpr::RetExpr(_) => {
            if prev == ConstType::Void {
                expr.ty
            } else if prev != expr.ty {
                ConstType::Dynamic
            } else {
                prev
            }
        }
        // get fn ty => Block , ifBody
        _ => prev,
    }
}

pub fn get_fn_type(body: &Vec<TypedExpr>) -> ConstType {
    let mut ty = ConstType::Void;
    for expr in body {
        ty = get_ret_ty(expr, ty);
    }
    ty
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
