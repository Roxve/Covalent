pub mod analysis;

use std::vec;

use crate::enviroment::{Enviroment, Symbol};

use crate::err::ErrKind;

use crate::parser::ast::{Blueprint, Expr, Ident, Node};
use crate::types::{self, type_mangle, AtomType, BasicType, BlueprintType, FunctionType};

pub struct Analyzer {
    workdir: String,
    pub env: Enviroment,
    pub imports: Vec<Node>,   // Import nodes
    pub functions: Vec<Node>, // Func nodes
    line: u16,
    column: u16,
}

const COMPARE_OP: &[&str] = &["==", "<", ">", "<=", ">="];
const LOGIC_OP: &[&str] = &["&&", "||"];

const MATH_OP: &[&str] = &["+", "-", "*", "/", "%"];

impl AtomType {
    pub fn get_op(&self) -> Vec<&str> {
        match self {
            &Self::Basic(BasicType::Bool) => [LOGIC_OP, &["=="]].concat(),
            &Self::Basic(BasicType::Float) | &Self::Basic(BasicType::Int) => [MATH_OP, COMPARE_OP].concat(),
            &Self::Atom(ref atom) if atom == &*types::Str || &atom.name == &*types::List.name => [COMPARE_OP, &["+"]].concat(),
            &Self::Dynamic | &Self::Unknown(_) => [LOGIC_OP, COMPARE_OP, MATH_OP].concat(),
            _ => Vec::new(),
        }
    }
}

#[inline]
pub fn ty_as(ty: &AtomType, expr: Node) -> Node {
    Node {
        expr: Expr::As(Box::new(expr)),
        ty: ty.clone(),
    }
}

#[inline]
pub fn supports_op(ty: &AtomType, op: &String) -> bool {
    let ops = ty.get_op();
    ops.contains(&op.as_str())
}
#[inline]
pub fn replace_body_ty(body: &mut Vec<Node>, old: &AtomType, new: &AtomType) {
    for node in &mut *body {
        replace_ty(node, old, new)
    }
}

pub fn replace_ty(node: &mut Node, old: &AtomType, new: &AtomType) {
    if let &AtomType::Unknown(None) = &node.ty {
        // If the new type is a function type, update the node's type to the function's return type, because we set any ref to our func to unknown
        if let &AtomType::Function(func) = &new {
            node.ty = *func.return_type.clone();
        }
    } else if &node.ty == old {
        node.ty = new.to_owned()
    }

    // replacing the insides of a node
    match &mut (*node).expr {
        &mut Expr::RetExpr(ref mut ret) => {
            replace_ty(&mut *ret, old, new);

            // convert the return to the return type if its not already (if we are replacing with a function type)
            if let &AtomType::Function(ref func) = new {
                if &*func.return_type != &ret.ty {
                    **ret = ty_as(&func.return_type, (**ret).clone());
                    node.ty = (*func.return_type).clone()
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

            if let &AtomType::Function(ref func) = new {
                // if the call results is Unknown and expected of a type
                if let &AtomType::Unknown(Some(ref ty)) = &node.ty {
                    if &func.return_type == ty {
                        node.ty = (*func.return_type).clone();
                    } else {
                        // if it doesnt return what is expected then convert it to that
                        let ty = ty.to_owned();
                        node.ty = (*func.return_type).clone();
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

fn get_ret_ty(node: &Node) -> Vec<AtomType> {
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

            if let &AtomType::Unknown(Some(ref ty)) = &node.ty {
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

pub fn get_fn_type(body: &Vec<Node>) -> AtomType {
    let possible = get_body_types(body);

    if possible.len() == 0 {
        return AtomType::Basic(BasicType::Void);
    }

    if possible.len() > 1 {
        // int | float -> float
        // otherwise -> dynamic

        if possible.len() == 2
            && possible.contains(&AtomType::Basic(BasicType::Int))
            && possible.contains(&AtomType::Basic(BasicType::Float))
        {
            return AtomType::Basic(BasicType::Float);
        }

        return AtomType::Unknown(None);
    }

    possible[0].clone()
}
impl Analyzer {
    pub fn new(workdir: String) -> Self {
        Self {
            env: Enviroment::init(),
            functions: Vec::new(),
            imports: Vec::new(),
            line: 0,
            column: 0,
            workdir,
        }
    }

    #[inline]
    fn import(
        &mut self,
        body: &mut Vec<Node>,
        ty: AtomType,
        module: &str,
        name: &str,
        params: Vec<AtomType>,
    ) {
        self.env
            .push_function(name.to_string(), FunctionType { params: params.clone(), return_type: Box::new(ty.clone()) });
        body.push(Node {
            expr: Expr::Import {
                module: module.to_string(),
                name: name.to_string(),
                params,
            },
            ty,
        })
    }

    // pub fn typed_id(&mut self, id: Ident) -> Result<Ident, ErrKind> {
    //     if let Ident::Tagged(tag, name) = id {
    //         if let Some(AtomKind::Type(ty)) = self.env.get_ty(&tag) {
    //             Ok(Ident::Typed(*ty, name))
    //         } else {
    //             err!(
    //                 self,
    //                 ErrKind::InvaildType,
    //                 format!("{} is not an Atom", tag)
    //             );
    //         }
    //     } else {
    //         panic!()
    //     }
    // }

    pub fn expect(&mut self, name: &Ident) -> Result<(), ErrKind> {
        self.expect_as(name.val(), name)
    }

    // expects a name as an ident tag if it has a tag
    pub fn expect_as(&mut self, name: &String, from: &Ident) -> Result<(), ErrKind> {
        let id = self.analyz_unknown_id(from.clone())?;
        self.env.expect(name, id.ty().clone());
        Ok(())
    }

    pub fn blueprints(&mut self, blueprints: Vec<Blueprint>) -> Result<(), ErrKind> {
        let blueprints = &mut blueprints.clone();

        for blueprint in &mut *blueprints {
            let mut params = Vec::new();
            let mut types = Vec::new();

            for arg in blueprint.args.clone() {
                let id = self.analyz_unknown_id(arg)?;
                params.push(id.clone());
                types.push(id.ty().clone());
            }

            blueprint.args = params;
            let ref_name = blueprint.name.val().clone();

            *blueprint.name.val_mut() = type_mangle(blueprint.name.val().clone(), types);

            let blueprint_ty = {
                let get = self.env.get_ty(&ref_name);
                
                let name = blueprint.name.val().clone();

                if get.is_none() {
                    // If the type is not found, create a new Blueprint type with the name
                    AtomType::Blueprint(BlueprintType { name: name.clone(), overlords: vec![name] })
                } else {
                    // If the type is found and is a Blueprint, add the overload to the list of overloads
                    match get.unwrap().clone() {
                        AtomType::Blueprint(mut blueprint) => {
                            blueprint.overlords.push(name);
                            AtomType::Blueprint(blueprint)
                        }
            
                        _ => panic!(),
                    }
                }
            };

            self.env.add(Symbol { name: ref_name, ty: blueprint_ty, refers_to_atom: false, value: None, expected: None });
        }

        self.env.blueprints.append(blueprints);

        // for blueprint in blueprints {
        //     if blueprint.args.len() == 0 {
        //         self.analyz_blueprint(blueprint, Vec::new())?;
        //     }
        // }
        Ok(())
    }
}
