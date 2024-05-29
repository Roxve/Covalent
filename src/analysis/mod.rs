pub mod analysis;
pub mod correct;

use crate::parser::ast::{Expr, Node};
use crate::source::{ConstType, Enviroment};

pub struct Analyzer {
    pub env: Enviroment,
    pub imports: Vec<Node>,   // Import nodes
    pub functions: Vec<Node>, // Func nodes
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
            &ConstType::Float | &ConstType::Int => [MATH_OP, COMPARE_OP].concat(),
            &ConstType::Str | &ConstType::List(_) => [COMPARE_OP, &["+"]].concat(),
            &ConstType::Dynamic | &ConstType::Unknown => [LOGIC_OP, COMPARE_OP, MATH_OP].concat(),
            &ConstType::Void
            | &ConstType::Obj(_)
            | &ConstType::Func(_, _)
            | &ConstType::Blueprint { .. } => Vec::new(),
        }
    }
}

#[inline]
pub fn supports_op(ty: &ConstType, op: &String) -> bool {
    let ops = ty.get_op();
    ops.contains(&op.as_str())
}
#[inline]
pub fn replace_body_ty(body: &mut Vec<Node>, old: &ConstType, new: &ConstType) {
    for node in &mut *body {
        replace_ty(node, old, new)
    }
}

pub fn replace_ty(node: &mut Node, old: &ConstType, new: &ConstType) {
    if &node.ty == old {
        node.ty = new.to_owned()
    }

    // replacing the insides of a node
    match &mut (*node).expr {
        &mut Expr::RetExpr(ref mut ret) => replace_ty(&mut *ret, old, new),
        &mut Expr::BinaryExpr {
            ref mut left,
            ref mut right,
            ..
        } => {
            replace_ty(&mut *left, old, new);
            replace_ty(&mut *right, old, new);
        }

        &mut Expr::IfExpr {
            ref mut condition,
            ref mut body,
            ref mut alt,
        } => {
            replace_ty(&mut *condition, old, new);

            if alt.is_some() {
                replace_ty(alt.as_mut().unwrap(), old, new);
            }
            replace_body_ty(&mut *body, old, new);
        }

        &mut Expr::WhileExpr {
            ref mut condition,
            ref mut body,
        } => {
            replace_ty(&mut *condition, old, new);
            replace_body_ty(&mut *body, old, new);
        }

        &mut Expr::Block(ref mut body) => replace_body_ty(&mut *body, old, new),

        &mut Expr::FnCall {
            ref mut name,
            ref mut args,
        } => {
            replace_ty(&mut *name, old, new);
            replace_body_ty(&mut *args, old, new);

            // if the call results is unknown and our new type has the results
            if &node.ty == &ConstType::Unknown {
                if let &ConstType::Func(ret, _) = &new {
                    node.ty = *ret.clone();
                }
            }
        }

        &mut Expr::As(ref mut thing) | &mut Expr::Discard(ref mut thing) => {
            replace_ty(&mut *thing, old, new)
        }
        _ => (),
    }
}

fn get_ret_ty(node: &Node, prev: ConstType) -> ConstType {
    match node.expr.clone() {
        Expr::RetExpr(node) => {
            if prev == ConstType::Void {
                if &node.ty != &ConstType::Unknown
                    && &node.ty != &ConstType::Func(Box::new(ConstType::Unknown), Vec::new())
                {
                    return node.ty.clone();
                }
            } else if prev != node.ty {
                return ConstType::Dynamic;
            }
            prev
        }

        Expr::IfExpr { body, alt, .. } => {
            let mut ty = get_fn_type(&body, prev);
            if alt.is_some() {
                ty = get_ret_ty(&alt.unwrap(), ty);
            }
            ty
        }

        Expr::WhileExpr { body, .. } | Expr::Block(body) => get_fn_type(&body, prev),
        // get fn ty => Block , ifBody
        _ => prev,
    }
}

pub fn get_fn_type(body: &Vec<Node>, prev: ConstType) -> ConstType {
    let mut ty = prev;
    for node in body {
        ty = get_ret_ty(node, ty);
    }
    ty
}

impl Analyzer {
    pub fn new() -> Self {
        Self {
            env: Enviroment::new(None),
            functions: Vec::new(),
            imports: Vec::new(),
            line: 0,
            column: 0,
        }
    }
}
