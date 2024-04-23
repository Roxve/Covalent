pub mod analysis;
pub mod correct;
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
    While {
        cond: Box<TypedExpr>,
        body: Vec<TypedExpr>,
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

macro_rules! op {
    (Bool) => {
        vec!["==", ">", "<", "<=", ">="]
    };
    (Logical) => {
        vec!["&&", "||"]
    };
    (Math) => {
        vec!["+", "-", "*", "/", "%"]
    };
}

impl ConstType {
    pub fn get_op(&self) -> Vec<&str> {
        match self {
            &ConstType::Bool => [op!(Bool), op!(Logical)].concat(),
            &ConstType::Float | &ConstType::Int => [op!(Math), op!(Bool)].concat(),
            &ConstType::Str => {
                let mut ops = op!(Bool);
                ops.push("+");
                ops
            }
            &ConstType::Dynamic => [op!(Logical), op!(Bool), op!(Math)].concat(),
            &ConstType::Void => Vec::new(),
        }
    }
}

#[inline]
pub fn supports_op(ty: &ConstType, op: &String) -> bool {
    ty.get_op().contains(&op.as_str())
}

fn get_ret_ty(expr: &TypedExpr, prev: ConstType) -> ConstType {
    match expr.expr.clone() {
        AnalyzedExpr::RetExpr(_) => {
            if prev == ConstType::Void {
                expr.ty
            } else if prev != expr.ty {
                ConstType::Dynamic
            } else {
                prev
            }
        }
        AnalyzedExpr::Block(body) => get_fn_type(&body, prev),
        AnalyzedExpr::If { body, alt, .. } => {
            let mut ty = get_fn_type(&body, prev);
            if alt.is_some() {
                ty = get_ret_ty(&*alt.unwrap(), ty);
            }
            ty
        }
        // get fn ty => Block , ifBody
        _ => prev,
    }
}

pub fn get_fn_type(body: &Vec<TypedExpr>, prev: ConstType) -> ConstType {
    let mut ty = prev;
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
