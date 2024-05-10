pub mod analysis;
pub mod correct;
use crate::parser::ast::{Expr, Node};
use crate::source::{ConstType, Enviroment};

pub struct Analyzer {
    pub env: Enviroment,
    line: u32,
    column: u32,
}

const COMPARE_OP: Vec<&'static str> = vec!["==", "<", ">", "<=", ">=", "+"];
const LOGIC_OP: Vec<&'static str> = vec!["&&", "||"];

const MATH_OP: Vec<&'static str> = vec!["+", "-", "*", "/", "%"];
const BOOL_OP: Vec<&'static str> = vec![vec!["=="], LOGIC_OP].concat();

const STROP: Vec<&'static str> = vec![COMPARE_OP, vec!["+", "-"]].concat();
const ALLOPS: Vec<&'static str> = vec![COMPARE_OP, LOGIC_OP, MATH_OP].concat();

impl ConstType {
    pub fn get_op(&self) -> Vec<&str> {
        match self {
            &ConstType::Bool => BOOL_OP,
            &ConstType::Float | &ConstType::Int => MATH_OP,
            &ConstType::Str | &ConstType::List(_) => STROP,
            &ConstType::Dynamic => ALLOPS,
            &ConstType::Void | &ConstType::Obj(_) | &ConstType::Func(_, _) => Vec::new(),
        }
    }
}

#[inline]
pub fn supports_op(ty: &ConstType, op: &String) -> bool {
    ty.get_op().contains(&op.as_str())
}

fn get_ret_ty(node: &Node, prev: ConstType) -> ConstType {
    match node.expr.clone() {
        Expr::RetExpr(_) => {
            if prev == ConstType::Void {
                node.ty
            } else if prev != node.ty {
                ConstType::Dynamic
            } else {
                prev
            }
        }

        Expr::If { body, alt, .. } => {
            let mut ty = get_fn_type(&body, prev);
            if alt.is_some() {
                ty = get_ret_ty(&*alt.unwrap(), ty);
            }
            ty
        }

        Expr::While { body, .. } | Expr::Block(body) => get_fn_type(&body, prev),
        // get fn ty => Block , ifBody
        _ => prev,
    }
}

pub fn get_fn_type(body: &Vec<Node>, prev: ConstType) -> ConstType {
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
