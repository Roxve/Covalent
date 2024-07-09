pub mod analysis;

use std::vec;

use crate::enviroment::{Enviroment, Symbol};

use crate::err::ErrKind;

use crate::parser::ast::{Blueprint, Expr, Ident, Node};
use crate::types::{
    self, type_mangle, AtomDetails, AtomKind, AtomType, BasicType, BlueprintType, FunctionType,
};

pub struct Analyzer {
    workdir: String,
    pub env: Enviroment,
    pub unknown_c: u32,
    line: u16,
    column: u16,
}

impl AtomType {
    pub fn get_op(&self) -> &[&str] {
        match &self.kind {
            &AtomKind::Basic(BasicType::Bool) => &["==", "||", "&&"],
            &AtomKind::Basic(BasicType::Float) | &AtomKind::Basic(BasicType::Int) => {
                &["+", "-", "*", "/", "%", "<", ">", "<=", ">=", "=="]
            }
            &AtomKind::Atom(ref atom)
                if atom == &*types::Str || &atom.name == &*types::List.name =>
            {
                &["<", ">", "==", "<=", ">=", "+", "-"]
            }
            &AtomKind::Dynamic | &AtomKind::Unknown(_) => &[
                "&&", "||", "==", "<", ">", "<=", ">=", "+", "-", "*", "/", "%",
            ],
            _ => &[],
        }
    }
}

#[inline]
pub fn _supports_op(ty: &AtomType, op: &String) -> bool {
    let ops = ty.get_op();
    ops.contains(&op.as_str())
}

fn get_ret_ty(node: &Node) -> Vec<AtomType> {
    match node.expr.clone() {
        Expr::RetExpr(node) => {
            // if let &Some(AtomDetails::Unknown(ref ty)) = &node.ty.details {
            //     return vec![(**ty).clone()];
            // }
            return vec![node.ty.clone()];
        }

        Expr::IfExpr { body, alt, .. } => {
            let mut ty = get_ret_ty(&body);
            if alt.is_some() {
                ty = get_ret_ty(&alt.unwrap());
            }
            ty
        }

        Expr::WhileExpr { body, .. } => get_ret_ty(&body),

        Expr::Block(body) => get_body_types(&body),
        // get fn ty => Block , ifBody
        _ => Vec::new(),
    }
}

pub fn get_body_types(body: &Vec<Node>) -> Vec<AtomType> {
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

pub fn _get_fn_type(body: &Vec<Node>) -> AtomType {
    let possible = get_body_types(body);

    if possible.len() == 0 {
        return AtomType {
            kind: AtomKind::Basic(BasicType::Void),
            details: None,
        };
    }

    if possible.len() > 1 {
        // int | float -> float
        // otherwise -> dynamic

        if possible.len() == 2
            && possible.contains(&AtomType {
                kind: AtomKind::Basic(BasicType::Int),
                details: None,
            })
            && possible.contains(&AtomType {
                kind: AtomKind::Basic(BasicType::Float),
                details: None,
            })
        {
            return AtomType {
                kind: AtomKind::Basic(BasicType::Float),
                details: None,
            };
        }

        return AtomType {
            kind: AtomKind::Unknown(0),
            details: None,
        };
    }

    possible[0].clone()
}
impl Analyzer {
    pub fn new(workdir: String) -> Self {
        Self {
            env: Enviroment::init(),
            unknown_c: 0,
            line: 0,
            column: 0,
            workdir,
        }
    }

    // pub fn blueprints(&mut self, blueprints: Vec<Blueprint>) -> Result<(), ErrKind> {
    //     let blueprints = &mut blueprints.clone();

    //     for blueprint in &mut *blueprints {
    //         let mut params = Vec::new();
    //         let mut types = Vec::new();

    //         for arg in blueprint.args.clone() {
    //             let id = self.analyz_unknown_id(arg)?;
    //             params.push(id.clone());
    //             types.push(id.ty().clone());
    //         }

    //         blueprint.args = params;
    //         let ref_name = blueprint.name.val().clone();

    //         *blueprint.name.val_mut() = type_mangle(blueprint.name.val().clone(), types);

    //         let blueprint_ty = {
    //             let get = self.env.get_ty(&ref_name);

    //             let name = blueprint.name.val().clone();

    //             if get.is_none() {
    //                 // If the type is not found, create a new Blueprint type with the name
    //                 AtomType {
    //                     kind: AtomKind::Blueprint(BlueprintType {
    //                         name: name.clone(),
    //                         overloads: vec![name],
    //                     }),
    //                     details: None,
    //                 }
    //             } else {
    //                 // If the type is found and is a Blueprint, add the overload to the list of overloads
    //                 match get.unwrap().clone().kind {
    //                     AtomKind::Blueprint(mut blueprint) => {
    //                         blueprint.overloads.push(name);

    //                         AtomType {
    //                             kind: AtomKind::Blueprint(blueprint),
    //                             details: None,
    //                         }
    //                     }

    //                     _ => panic!(),
    //                 }
    //             }
    //         };

    //         self.env.add(Symbol {
    //             name: ref_name,
    //             ty: blueprint_ty,
    //             value: None,
    //             expected: None,
    //         });
    //     }

    //     self.env.blueprints.append(blueprints);

    //     // for blueprint in blueprints {
    //     //     if blueprint.args.len() == 0 {
    //     //         self.analyz_blueprint(blueprint, Vec::new())?;
    //     //     }
    //     // }
    //     Ok(())
    // }
}
