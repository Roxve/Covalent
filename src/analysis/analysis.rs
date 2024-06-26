use std::fs;

use crate::parser::parse::Parse;
use crate::types::{mangle_types, type_mangle, AtomType};

use crate::err;
use crate::err::{ATErr, ErrKind};

use crate::parser::ast::{Blueprint, Expr, Ident, Node};

use super::*;

impl Analyzer {
    #[inline]
    pub fn analyz_body(&mut self, body: Vec<Node>, top: bool) -> Result<Vec<Node>, ErrKind> {
        let mut analyzed_body = vec![];
        if !top {
            self.env.child();
        }

        for node in body {
            if let &Expr::Use(ref path) = &node.expr {
                let abs = format!("{}/{}", self.workdir.clone(), path);
                let read = fs::read_to_string(abs)
                    .expect(format!("failed to open path {} to use", path).as_str());

                use crate::parser::Parser;
                let mut parser = Parser::new(read);

                let ast = parser.parse_prog();

                self.blueprints(parser.functions)?;
                let mut ast = self.analyz_body(ast, true)?;

                analyzed_body.append(&mut ast);
                continue;
            }
            analyzed_body.push(self.analyz(node)?);
        }

        if !top {
            self.env.parent();
        }
        Ok(analyzed_body)
    }

    #[inline]
    pub fn analyz_items(&mut self, items: Vec<Node>) -> Result<Vec<Node>, ErrKind> {
        let mut analyzed_items = vec![];
        for node in items {
            analyzed_items.push(self.analyz(node)?);
        }
        Ok(analyzed_items)
    }

    pub fn analyz_prog(
        exprs: Vec<Node>,
        functions: Vec<Blueprint>,
        workdir: String,
    ) -> Result<Vec<Node>, ErrKind> {
        let mut analyzer = Analyzer::new(workdir);
        let mut analyzed_prog = Vec::new();

        analyzer.import(
            &mut analyzed_prog,
            AtomType {
                kind: AtomKind::Basic(BasicType::Void),
                details: None,
            },
            "std",
            "writeln",
            vec![AtomType {
                kind: AtomKind::Dynamic,
                details: None,
            }],
        );

        // setting our env blueprints to our uncompiled functions (blueprints are then compiled pased on call arguments)
        analyzer.blueprints(functions)?;
        analyzed_prog.append(&mut analyzer.analyz_body(exprs, true)?);

        analyzed_prog = [
            analyzer.imports.clone(),
            analyzer.functions.clone(),
            analyzed_prog,
        ]
        .concat();
        Ok(analyzed_prog)
    }

    pub fn analyz(&mut self, node: Node) -> Result<Node, ErrKind> {
        match node.expr.clone() {
            Expr::Literal(literal) => {
                let ty = literal.get_ty();
                Ok(Node {
                    expr: Expr::Literal(literal),
                    ty,
                })
            }

            Expr::ListExpr(items) => {
                let items = self.analyz_items(items)?;

                let item_ty = if items.len() > 0 {
                    (&items.first().unwrap()).ty.clone()
                } else {
                    AtomType {
                        kind: AtomKind::Unknown,
                        details: None,
                    } // empty list unknown type figure out type on push
                };

                for (i, item) in (&items).iter().enumerate() {
                    if let &AtomKind::Unknown = &item_ty.kind {
                        break;
                    }

                    if &item.ty != &item_ty {
                        err!(self, ErrKind::InvaildType, format!("list items have to be of the same type, item {} is of an invaild type", i-1));
                    }
                }
                let ty = AtomType {
                    kind: AtomKind::Atom(types::List.spec(&[item_ty])),
                    details: None,
                };
                let expr = Expr::ListExpr(items);
                Ok(Node { expr, ty })
            }

            Expr::BinaryExpr { op, left, right } => self.analyz_binary_expr(*left, *right, op),
            Expr::Ident(id) => self.analyz_id(id),

            Expr::VarDeclare { name, val } => self.analyz_var_declare(name, *val),
            Expr::VarAssign { name, val } => self.analyz_var_assign(*name, *val),

            Expr::Discard(expr) => {
                let ty = AtomType {
                    kind: AtomKind::Basic(BasicType::Void),
                    details: None,
                };

                let expr = self.analyz(*expr)?;
                let expr = Expr::Discard(Box::new(expr));

                Ok(Node { expr, ty })
            }

            Expr::PosInfo(x, line, column) => {
                self.line = line;
                self.column = column;
                Ok(Node {
                    expr: Expr::PosInfo(x, line, column),
                    ty: AtomType {
                        kind: AtomKind::Basic(BasicType::Void),
                        details: None,
                    },
                })
            }

            Expr::RetExpr(expr) => {
                let expr = self.analyz(*expr)?;
                let ty = expr.ty.clone();

                let expr = Expr::RetExpr(Box::new(expr));
                Ok(Node { expr, ty })
            }

            Expr::FnCall { name, args } => self.analyz_call(*name, args),
            Expr::Extern { name, params } => self.analyz_extern(name, params),

            Expr::IfExpr {
                condition,
                body,
                alt,
            } => self.analyz_if_expr(
                *condition,
                body,
                match alt {
                    Some(alt) => Some(*alt),
                    None => None,
                },
            ),

            Expr::Block(block) => {
                let block = self.analyz_body(block, false)?;

                let last = block.last();
                let ty = if last.is_none() {
                    AtomType {
                        kind: AtomKind::Basic(BasicType::Void),
                        details: None,
                    }
                } else {
                    last.unwrap().ty.clone()
                };
                let expr = Expr::Block(block);

                Ok(Node { expr, ty })
            }

            Expr::WhileExpr { condition, body } => self.analyz_while_expr(*condition, body),

            Expr::MemberExpr { parent, child } => self.analyz_member(*parent, child),
            Expr::IndexExpr { parent, index } => self.analyz_index(*parent, *index),

            Expr::SpecExpr { parent, spec } => {
                let parent = Box::new(self.analyz(*parent)?);
                let spec = self.analyz_items(spec)?;

                if !parent.ty.is_type() {
                    err!(
                        self,
                        ErrKind::InvaildType,
                        format!("{} is not a type", parent.ty)
                    );
                }

                if spec.len() as i32 != parent.ty.generics() {
                    err!(
                        self,
                        ErrKind::InvaildType,
                        format!(
                            "expected {} generics got {}, for type {}",
                            parent.ty.generics(),
                            spec.len(),
                            parent.ty
                        )
                    );
                }

                let spec_types: Vec<AtomType> = spec.iter().map(|x| x.ty.clone()).collect();

                let ty = if let &AtomKind::Atom(ref atom) = &parent.ty.kind {
                    AtomType {
                        kind: AtomKind::Atom(atom.spec(&spec_types)),
                        details: Some(AtomDetails::Type),
                    }
                } else {
                    panic!("type {} is not an atom", parent.ty);
                };

                Ok(Node {
                    expr: Expr::SpecExpr { parent, spec },
                    ty,
                })
            }
            _ => todo!("node {:#?}", node),
        }
    }

    pub fn analyz_extern(
        &mut self,
        name: Ident,
        untyped_params: Vec<Ident>,
    ) -> Result<Node, ErrKind> {
        let name = self.analyz_unknown_id(name)?;

        let mut params: Vec<Ident> = Vec::new();
        for param in untyped_params {
            params.push(self.analyz_unknown_id(param)?);
        }

        let params_types = params.iter().map(|x| x.ty().clone()).collect();

        let ty = FunctionType {
            return_type: Box::new(name.ty().clone()),
            params: params_types,
        };

        let ty = AtomType {
            kind: AtomKind::Function(ty),
            details: None,
        };

        self.env.add(Symbol {
            name: name.val().clone(),
            ty: ty.clone(),
            value: None,
            expected: None,
        });

        let expr = Expr::Extern { name, params };

        Ok(Node { expr, ty })
    }

    pub fn analyz_blueprint(
        &mut self,
        blueprint: Blueprint,
        types: Vec<AtomType>,
    ) -> Result<String, ErrKind> {
        let mangle = type_mangle(blueprint.name.val().clone(), types.clone());
        if self.env.has(&mangle) {
            if let AtomKind::Function(_) = self.env.get_ty(&mangle).unwrap().kind {
                return Ok(mangle);
            }
        }

        self.env.child();
        self.expect_as(&mangle, &blueprint.name)?;
        // allows for the function to call itself
        let placeholder = FunctionType {
            return_type: Box::new(AtomType {
                kind: AtomKind::Unknown,
                details: None,
            }),
            params: Vec::new(),
        };

        self.env.push_function(mangle.clone(), placeholder.clone());

        let mut typed_params = Vec::new();
        for (i, arg) in (&blueprint.args).into_iter().enumerate() {
            self.env.add(Symbol {
                name: arg.val().clone(),
                ty: types[i].clone(),

                value: None,
                expected: None,
            });

            typed_params.push(Ident::Typed(types[i].clone(), arg.val().clone()))
        }

        let mut body = self.analyz_body(blueprint.body, false)?;
        let ty = get_fn_type(&body);

        if !self.env.is_expected(&mangle, &ty) {
            err!(
                self,
                ErrKind::InvaildType,
                format!(
                    "invaild return type for function {}, expected {} got {}",
                    mangle,
                    self.env.get(&mangle).unwrap().expected.as_ref().unwrap(),
                    ty
                )
            );
        }

        let placeholder = AtomType {
            kind: AtomKind::Function(placeholder),
            details: None,
        };

        let func_type = FunctionType {
            return_type: Box::new(ty.clone()),
            params: types.clone(),
        };

        replace_body_ty(
            &mut body,
            &placeholder,
            &AtomType {
                kind: AtomKind::Function(func_type.clone()),
                details: None,
            },
        );

        self.env.parent();

        self.env.push_function(mangle.clone(), func_type);

        let func = Expr::Func {
            ret: ty.clone(),
            name: mangle.clone(),
            args: typed_params,
            body,
        };

        self.functions.push(Node {
            ty: ty.clone(),
            expr: func,
        });

        Ok(mangle)
    }

    pub fn analyz_binary_expr(
        &mut self,
        left: Node,
        right: Node,
        op: String,
    ) -> Result<Node, ErrKind> {
        let mut lhs = self.analyz(left)?;
        let mut rhs = self.analyz(right)?;

        if &rhs.ty.kind == &AtomKind::Unknown && &lhs.ty.kind == &AtomKind::Unknown {
        } else if let &AtomKind::Unknown = &rhs.ty.kind {
            rhs.ty = AtomType {
                kind: AtomKind::Unknown,
                details: Some(AtomDetails::Unknown(Box::new(lhs.ty.clone()))),
            }; // assume the unknown side as the other side
        } else if let &AtomKind::Unknown = &lhs.ty.kind {
            lhs.ty = AtomType {
                kind: AtomKind::Unknown,
                details: Some(AtomDetails::Unknown(Box::new(rhs.ty.clone()))),
            };
        }

        (lhs, rhs) = self.type_conv(lhs, rhs)?;
        let ty = match op.as_str() {
            "==" | ">" | "<" | ">=" | "<=" => AtomType {
                kind: AtomKind::Basic(BasicType::Bool),
                details: None,
            },

            _ => {
                if let &Some(AtomDetails::Unknown(ref ty)) = &lhs.ty.details {
                    (**ty).clone()
                } else {
                    lhs.ty.clone()
                }
            }
        };

        if !supports_op(&lhs.ty, &op) {
            err!(self,ErrKind::OperationNotGranted, format!("one of possible types [{:?}] does not support operator {}, use the do keyword to do it anyways", lhs.ty, op));
        }
        let left = Box::new(lhs);
        let right = Box::new(rhs);

        let expr = Expr::BinaryExpr { op, left, right };
        Ok(Node { expr, ty })
    }

    pub fn analyz_call(&mut self, name: Node, args: Vec<Node>) -> Result<Node, ErrKind> {
        let name = Box::new(self.analyz(name)?);

        let mut args = self.analyz_items(args)?;

        let args_types: Vec<AtomType> = args.iter().map(|arg| arg.ty.clone()).collect();
        match name.ty.clone().kind {
            AtomKind::Blueprint(blueprint_t) => {
                let mangle = type_mangle(blueprint_t.name.clone(), args_types.clone());
                // TODO! if its a member call pass parent as first arg and call the child instead
                // if &argc != &(args.len() as u32) {
                //     self.err(ErrKind::UndeclaredVar, format!("not enough arguments got {} arguments, expected {} arguments for function {:?}", args.len(), argc, name));
                //     return Err(ErrKind::UndeclaredVar);
                // }

                if self.env.has(&mangle) {
                    let id_ty = self.env.get_ty(&mangle).unwrap();

                    match id_ty.kind {
                        AtomKind::Function(ref func) => {
                            return Ok(Node {
                                expr: Expr::FnCall {
                                    name: Box::new(Node {
                                        expr: Expr::Ident(Ident::UnTagged(mangle)),
                                        ty: id_ty.clone(),
                                    }),
                                    args,
                                },
                                ty: *func.return_type.clone(),
                            })
                        }

                        AtomKind::Blueprint(_) => (), // continue building blueprint
                        _ => panic!(),
                    };
                }

                let mut blueprint = None;
                let mut possible = Vec::new();

                for overload in blueprint_t.overloads {
                    // if we got an exact overload no need to check for the best possible one to use
                    if overload == mangle {
                        blueprint = Some(self.env.get_blueprint(&overload).unwrap());

                        possible.clear(); // uneeded memory
                        break;
                    }

                    // make a list of possible overload that mangle could be from
                    let mangle = mangle_types(mangle.clone());

                    let mut found = true;
                    for (i, ty) in mangle_types(overload.clone()).iter().enumerate() {
                        let m_ty = &mangle[i];

                        if !(ty == m_ty || ty == &String::from("any")) {
                            found = false;
                            break;
                        }
                    }

                    if found {
                        possible.push(overload);
                    }
                }

                if blueprint.is_none() {
                    // choose the name with least possible any
                    let mut least_count = args_types.len(); // least possible count where all types are any;

                    let mut choosen = String::new();
                    for name in possible {
                        let types = mangle_types(name.clone());
                        let mut count = 0;

                        types.iter().for_each(|ty| {
                            if ty == &String::from("any") {
                                count += 1
                            }
                        });

                        if count <= least_count {
                            least_count = count;
                            choosen = name;
                        }
                    }
                    blueprint = Some(self.env.get_blueprint(&choosen).unwrap());
                }
                // building a function from blueprint
                let blueprint = blueprint.unwrap();

                let fun = self.analyz_blueprint(blueprint, args_types)?;
                let ty = self.env.get_ty(&fun).unwrap(); // calling the built function

                let ret = if let AtomKind::Function(func) = ty.clone().kind {
                    *func.return_type
                } else {
                    panic!()
                };

                let expr = Expr::FnCall {
                    name: Box::new(Node {
                        ty: ty.clone(),
                        expr: Expr::Ident(Ident::UnTagged(fun)),
                    }),
                    args,
                };

                Ok(Node { expr, ty: ret })
            }

            AtomKind::Function(func) => {
                if func.params.len() != args.len() {
                    err!(self, ErrKind::UndeclaredVar, format!("not enough arguments got {} arguments, expected {} arguments for function {:?}", args.len(), args_types.len(), name));
                }

                for (i, arg) in (&mut args).iter_mut().enumerate() {
                    if &arg.ty != &func.params[i] {
                        let attempt = self.type_cast(arg.clone(), func.params[i].clone());

                        if attempt.is_ok() {
                            *arg = attempt.unwrap();
                        } else {
                            err!(
                                self,
                                ErrKind::UnexceptedArgs,
                                format!(
                                    "unexpected argument type, at arg {}, expected {:?}, got {:?}",
                                    i, func.params[i], arg.ty
                                )
                            );
                        }
                    }
                }

                let expr = Expr::FnCall { name, args };

                Ok(Node {
                    expr,
                    ty: *func.return_type,
                })
            }

            _ => {
                err!(
                    self,
                    ErrKind::UnexceptedTokenE,
                    format!("expected symbol to call got {:?}", name.expr)
                );
            }
        }
    }

    pub fn analyz_index(&mut self, parent: Node, index: Node) -> Result<Node, ErrKind> {
        let parent = self.analyz(parent)?;
        let index = self.analyz(index)?;

        if index.ty.kind != AtomKind::Basic(BasicType::Int) {
            err!(self, ErrKind::InvaildType, format!("index is not an int"));
        }

        let ty = match parent.ty.clone().kind {
            // until i add interfaces(traits) i have to do it manually
            AtomKind::Atom(ref atom) if atom.name == types::Str.name => parent.ty.clone(), // str indexs into str not char for now
            AtomKind::Atom(ref atom) if atom.name == types::List.name => atom.generics[0].clone(),
            _ => {
                err!(
                    self,
                    ErrKind::InvaildType,
                    format!("cannot index {:?}", parent.ty)
                );
            }
        };

        let expr = Expr::IndexExpr {
            parent: Box::new(parent),
            index: Box::new(index),
        };
        Ok(Node { expr, ty })
    }

    pub fn analyz_member(&mut self, parent: Node, child: String) -> Result<Node, ErrKind> {
        let parent = self.analyz(parent)?;

        let ty = parent.ty.get(&child);
        let ty = if ty.is_none() {
            let func = self.env.ty_parent_fn(&parent.ty, &child);
            if func.is_none() {
                return Err(ErrKind::UndeclaredVar);
            }

            func.unwrap()
        } else {
            ty.unwrap().clone()
        };

        let expr = Expr::MemberExpr {
            parent: Box::new(parent),
            child,
        };

        Ok(Node { ty, expr })
    }

    pub fn analyz_unknown_id(&mut self, id: Ident) -> Result<Ident, ErrKind> {
        match &id {
            &Ident::Tagged(ref tag, ref id) => {
                let tag = self.analyz(*tag.clone())?;
                let expr = tag.expr;
                let tag = tag.ty;

                // if tag has type details then it is a type, return tag type without the type details
                if tag.is_type() {
                    return Ok(Ident::Typed(
                        AtomType {
                            kind: tag.kind,
                            details: None,
                        },
                        id.clone(),
                    ));
                } else {
                    err!(
                        self,
                        ErrKind::InvaildType,
                        format!("{:?} is not an Atom", expr)
                    );
                }
            }
            &Ident::Typed(_, _) | &Ident::UnTagged(_) => Ok(id),
        }
    }

    pub fn analyz_id(&mut self, id: Ident) -> Result<Node, ErrKind> {
        if let &Ident::Tagged(_, _) = &id {
            err!(
                self,
                ErrKind::InvaildType,
                format!(
                    "invaild id {:?} you cannot use @ outside of declaration",
                    &id
                )
            );
        }

        if !self.env.has(&id.val()) {
            dbg!(id);
            return Err(ErrKind::UndeclaredVar);
        }

        let ty = self.env.get_ty(&id.val()).unwrap();

        let expr = Expr::Ident(id);
        Ok(Node { expr, ty })
    }

    pub fn analyz_var_declare(&mut self, name: Ident, val: Node) -> Result<Node, ErrKind> {
        let val = self.analyz(val)?;

        if self.env.has(&name.val()) {
            return Err(ErrKind::VarAlreadyDeclared);
        }
        self.env.add(Symbol {
            name: name.val().clone(),
            ty: AtomType {
                kind: AtomKind::Unknown,
                details: None,
            },
            value: None,
            expected: None,
        });

        self.expect(&name)?;
        let ty = val.ty.clone();

        if !self.env.is_expected(&name.val(), &ty) {
            err!(
                self,
                ErrKind::InvaildType,
                format!(
                    "unexpected type {ty}, for id {}, expected {}",
                    name.val(),
                    self.env.get_expected(name.val())
                )
            );
        }
        self.env.modify(
            name.val(),
            Symbol {
                name: name.val().clone(),
                ty: ty.clone(),
                value: None,
                expected: None,
            },
        );

        let expr = Expr::VarDeclare {
            name,
            val: Box::new(val),
        };
        Ok(Node { expr, ty })
    }

    pub fn analyz_var_assign(&mut self, id: Node, val: Node) -> Result<Node, ErrKind> {
        let val = self.analyz(val)?;
        let name = self.analyz(id)?;
        let mut ty = val.ty.clone();

        if let Expr::Ident(ref name) = name.expr {
            self.env.modify_ty(&name.val(), ty.clone());
        } else if val.ty != name.ty {
            if name.ty.kind == AtomKind::Unknown {
                ty = name.ty.clone();
            } else {
                err!(self, ErrKind::InvaildType, format!("cannot set the value of an Obj property to a value of different type, got type {:?} expected {:?}, in expr {:?} = {:?}", val.ty, name.ty, name, val));
            }
        }

        let expr = Expr::VarAssign {
            name: Box::new(name),
            val: Box::new(val),
        };
        Ok(Node { expr, ty })
    }

    pub fn analyz_if_expr(
        &mut self,
        condition: Node,
        body: Vec<Node>,
        alt: Option<Node>,
    ) -> Result<Node, ErrKind> {
        let condition = Box::new(self.analyz(condition)?);

        if condition.ty.kind != AtomKind::Basic(BasicType::Bool) {
            err!(
                self,
                ErrKind::InvaildType,
                format!(
                    "invaild condition for while loop expected Bool got {:?}",
                    condition.ty
                )
            );
        }
        let body = self.analyz_body(body, false)?;

        let analyzed_alt = if alt.is_none() {
            None
        } else {
            Some(Box::new(self.analyz(alt.unwrap())?))
        };

        let last = body.last();

        let ty = if last.is_none() {
            AtomType {
                kind: AtomKind::Basic(BasicType::Void),
                details: None,
            }
        } else {
            last.unwrap().ty.clone()
        };

        let expr = Expr::IfExpr {
            condition,
            body,
            alt: analyzed_alt,
        };

        Ok(Node { expr, ty })
    }

    pub fn analyz_while_expr(&mut self, condition: Node, body: Vec<Node>) -> Result<Node, ErrKind> {
        let condition = Box::new(self.analyz(condition)?);

        if condition.ty.kind != AtomKind::Basic(BasicType::Bool) {
            err!(
                self,
                ErrKind::InvaildType,
                format!(
                    "invaild condition for while loop expected Bool got {:?}",
                    condition.ty
                )
            );
        }
        let body = self.analyz_body(body, false)?;

        let expr = Expr::WhileExpr { condition, body };
        let ty = AtomType {
            kind: AtomKind::Basic(BasicType::Void),
            details: None,
        };

        Ok(Node { expr, ty })
    }

    // who tf wrote this code
    pub fn type_cast(&mut self, from: Node, into: AtomType) -> Result<Node, ErrKind> {
        if into.kind == AtomKind::Dynamic || into.kind == AtomKind::Atom(types::Str.clone()) {
            Ok(ty_as(&into, from))
        } else {
            let _tmp = Expr::Ident(Ident::UnTagged("temp".to_string()));
            let _tmp = Node {
                expr: _tmp,
                ty: into,
            };
            let (try_conv, unchanged) = self.type_conv(from.clone(), _tmp.clone())?;
            if &unchanged != &_tmp {
                err!(
                    self,
                    ErrKind::InvaildType,
                    format!("cannot convert from {:?} into {:?}", from.ty, _tmp.ty)
                );
            }
            Ok(try_conv)
        }
    }

    pub fn type_conv(&mut self, mut lhs: Node, mut rhs: Node) -> Result<(Node, Node), ErrKind> {
        // this code is not good rn
        if lhs.ty != rhs.ty {
            if let &Some(AtomDetails::Unknown(ref ty)) = &lhs.ty.details {
                if &**ty == &rhs.ty {
                    return Ok((lhs, rhs));
                }
            }

            if let &Some(AtomDetails::Unknown(ref ty)) = &rhs.ty.details {
                if &**ty == &lhs.ty {
                    return Ok((lhs, rhs));
                }
            }

            if lhs.ty.kind == AtomKind::Basic(BasicType::Float)
                && rhs.ty.kind == AtomKind::Basic(BasicType::Int)
            {
                rhs = ty_as(&lhs.ty, rhs);
            } else if lhs.ty.kind == AtomKind::Basic(BasicType::Int)
                && rhs.ty.kind == AtomKind::Basic(BasicType::Float)
            {
                lhs = ty_as(&rhs.ty, lhs);
            } else if lhs.ty.kind == AtomKind::Atom(types::Str.clone()) {
                rhs = ty_as(&lhs.ty, rhs);
            } else if rhs.ty.kind == AtomKind::Atom(types::Str.clone()) {
                lhs = ty_as(&rhs.ty, lhs);
            } else if lhs.ty.kind == AtomKind::Dynamic {
                rhs = ty_as(&lhs.ty, rhs);
            } else if rhs.ty.kind == AtomKind::Dynamic {
                lhs = ty_as(&rhs.ty, lhs);
            } else {
                err!(
                    self,
                    ErrKind::InvaildType,
                    format!("cannot make {:?} and {:?} as the same type", lhs.ty, rhs.ty)
                );
            }
        }

        Ok((lhs, rhs))
    }
}
