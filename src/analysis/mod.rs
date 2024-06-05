pub mod analysis;

use crate::enviroment::Enviroment;
use crate::parser::ast::{Blueprint, Expr, Node};
use crate::types::AtomKind;

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

impl AtomKind {
    pub fn get_op(&self) -> Vec<&str> {
        match self {
            &Self::Bool => [LOGIC_OP, &["=="]].concat(),
            &Self::Float | &Self::Int => [MATH_OP, COMPARE_OP].concat(),
            &Self::Str | &Self::List(_) => [COMPARE_OP, &["+"]].concat(),
            &Self::Dynamic | &Self::Unknown(_) => [LOGIC_OP, COMPARE_OP, MATH_OP].concat(),
            &Self::Void | &Self::Obj(_) | &Self::Func(_, _, _) | &Self::Blueprint { .. } => {
                Vec::new()
            }
        }
    }
}

#[inline]
pub fn ty_as(ty: &AtomKind, expr: Node) -> Node {
    Node {
        expr: Expr::As(Box::new(expr)),
        ty: ty.clone(),
    }
}

#[inline]
pub fn supports_op(ty: &AtomKind, op: &String) -> bool {
    let ops = ty.get_op();
    ops.contains(&op.as_str())
}
#[inline]
pub fn replace_body_ty(body: &mut Vec<Node>, old: &AtomKind, new: &AtomKind) {
    for node in &mut *body {
        replace_ty(node, old, new)
    }
}

pub fn replace_ty(node: &mut Node, old: &AtomKind, new: &AtomKind) {
    if let &AtomKind::Unknown(None) = &node.ty {
        if let &AtomKind::Func(ret, _, _) = &new {
            node.ty = *ret.clone();
        }
    } else if &node.ty == old {
        node.ty = new.to_owned()
    }

    // replacing the insides of a node
    match &mut (*node).expr {
        &mut Expr::RetExpr(ref mut ret) => {
            replace_ty(&mut *ret, old, new);

            // convert the return to the return type if its not already (if we are replacing with a function type)
            if let &AtomKind::Func(ref ret_ty, _, _) = new {
                if &**ret_ty != &ret.ty {
                    **ret = ty_as(&ret_ty, (**ret).clone());
                    node.ty = (**ret_ty).clone()
                }
            }
        }
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

            if let &AtomKind::Func(ref ret, _, _) = new {
                // if the call results is Unknown and expected of a type
                if let &AtomKind::Unknown(Some(ref ty)) = &node.ty {
                    if ret == ty {
                        node.ty = (**ret).clone();
                    } else {
                        // if it doesnt return what is expected then convert it to that
                        let ty = ty.to_owned();
                        node.ty = (**ret).clone();
                        *node = ty_as(&ty, node.to_owned());
                    }
                }
            }
        }

        &mut Expr::As(ref mut thing) | &mut Expr::Discard(ref mut thing) => {
            replace_ty(&mut *thing, old, new)
        }
        _ => (),
    }
}

fn get_ret_ty(node: &Node) -> Vec<AtomKind> {
    match node.expr.clone() {
        Expr::RetExpr(node) => {
            // if let &AtomKind::Unknown(_) = &node.ty {
            //     return vec![prev];
            // } else if let &AtomKind::Func(ref ret, ref args, _) = &node.ty {
            //     if let &AtomKind::Unknown(_) = &**ret {
            //         return vec![prev];
            //     }
            // } else if &prev != &node.ty && &prev != &AtomKind::Void {
            //     return vec![AtomKind::Dynamic];
            // }

            if let &AtomKind::Unknown(Some(ref ty)) = &node.ty {
                return vec![(**ty).clone()];
            }
            return vec![node.ty.clone()];
        }

        Expr::IfExpr { body, alt, .. } => {
            let mut ty = get_body_types(&body);
            if alt.is_some() {
                ty = get_ret_ty(&alt.unwrap());
            }
            ty
        }

        Expr::WhileExpr { body, .. } | Expr::Block(body) => get_body_types(&body),
        // get fn ty => Block , ifBody
        _ => Vec::new(),
    }
}

pub fn get_body_types(body: &Vec<Node>) -> Vec<AtomKind> {
    let mut types = Vec::new();
    for node in body {
        for ty in get_ret_ty(node) {
            if !types.contains(&ty) {
                types.push(ty);
            }
        }
    }
    types
}

pub fn get_fn_type(body: &Vec<Node>) -> AtomKind {
    let possible = get_body_types(body);

    if possible.len() > 1 {
        // int | float -> float
        // otherwise -> dynamic

        if possible.len() == 2
            && possible.contains(&AtomKind::Int)
            && possible.contains(&AtomKind::Float)
        {
            return AtomKind::Float;
        }

        return AtomKind::Dynamic;
    }

    possible[0].clone()
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
    pub fn blueprints(&mut self, blueprints: Vec<Blueprint>) {
        self.env.blueprints = blueprints.clone();
        for blueprint in blueprints {
            self.env.add(
                &blueprint.name.val(),
                AtomKind::Blueprint {
                    argc: blueprint.args.len() as u32,
                    name: blueprint.name.val().clone(),
                },
            );
        }
    }
}
