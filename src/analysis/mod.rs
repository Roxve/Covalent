pub mod analysis;
pub mod correct;
use crate::source::Ident;

use crate::parser::ast::{Expr, Node};
use crate::source::{ConstType, Enviroment};

pub struct Analyzer {
    pub env: Enviroment,
    line: u32,
    column: u32,
}

const COMPARE_OP: &[&str] = &["==", "<", ">", "<=", ">="];
const LOGIC_OP: &[&str] = &["&&", "||"];

const MATH_OP: &[&str] = &["+", "-", "*", "/", "%"];

impl ConstType {
    pub fn get_op(&self) -> Vec<&str> {
        match self {
            &ConstType::Bool => [LOGIC_OP, &["=="]].concat(),
            &ConstType::Float | &ConstType::Int => MATH_OP.to_vec(),
            &ConstType::Str | &ConstType::List(_) => [COMPARE_OP, &["+"]].concat(),
            &ConstType::Dynamic => [LOGIC_OP, COMPARE_OP, MATH_OP].concat(),
            &ConstType::Void
            | &ConstType::Obj(_)
            | &ConstType::Func(_, _)
            | &ConstType::Unknown => Vec::new(),
        }
    }
}

#[inline]
pub fn supports_op(ty: &ConstType, op: &String) -> bool {
    ty.get_op().contains(&op.as_str())
}

fn get_ret_ty(name: &String, node: &Node, prev: ConstType) -> ConstType {
    match node.expr.clone() {
        Expr::RetExpr(node) => {
            // if function calls itself
            if let Node {
                expr: Expr::FnCall { name: fn_name, .. },
                ty: ConstType::Unknown,
            } = *node.clone()
            {
                if let Expr::Ident(id) = fn_name.expr {
                    if &id.val == name {
                        return prev;
                    }
                }
            }

            if prev == ConstType::Void {
                node.ty.clone()
            } else if prev != node.ty {
                ConstType::Dynamic
            } else {
                prev
            }
        }

        Expr::IfExpr { body, alt, .. } => {
            let mut ty = get_fn_type(name, &body, prev);
            if alt.is_some() {
                ty = get_ret_ty(name, &*alt.unwrap(), ty);
            }
            ty
        }

        Expr::WhileExpr { body, .. } | Expr::Block(body) => get_fn_type(name, &body, prev),
        // get fn ty => Block , ifBody
        _ => prev,
    }
}

pub fn get_fn_type(name: &String, body: &Vec<Node>, prev: ConstType) -> ConstType {
    let mut ty = prev;
    for node in body {
        ty = get_ret_ty(name, node, ty);
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
