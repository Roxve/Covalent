use crate::source::type_mangle;

use crate::parser::ast::{Expr, Node};
// use crate::parser::Function;
use crate::source::{ATErr, Blueprint, ConstType, ErrKind, Ident};

use super::*;

#[inline]
fn ty_as(ty: &ConstType, expr: Node) -> Node {
    Node {
        expr: Expr::As(Box::new(expr)),
        ty: ty.clone(),
    }
}

impl Analyzer {
    pub fn err(&mut self, kind: ErrKind, msg: String) {
        let err = ATErr {
            kind,
            msg,
            line: self.line,
            column: self.column,
        };
        err.out_error();
    }
    #[inline]
    fn import(
        &mut self,
        body: &mut Vec<Node>,
        ty: ConstType,
        module: &str,
        name: &str,
        args: Vec<ConstType>,
    ) {
        self.env
            .push_function(name.to_string(), args.clone(), ty.clone());
        body.push(Node {
            expr: Expr::Import {
                module: module.to_string(),
                name: name.to_string(),
                args,
            },
            ty,
        })
    }

    // pub fn analyz_func(&mut self, func: Function) -> Result<Node, ErrKind> {
    //     self.env = self.env.child();

    //     let args = func.args;

    //     for arg in &args {
    //         self.env
    //             .add(&arg.val, arg.tag.clone().unwrap_or(ConstType::Dynamic));
    //     }

    //     let mut body = vec![];
    //     for expr in func.body {
    //         body.push(self.analyz(expr)?);
    //     }

    //     let ty = get_fn_type(&func.name.val, &body, ConstType::Void);
    //     self.env = self.env.parent().unwrap();
    //     self.env.modify(
    //         &func.name.val,
    //         ConstType::Func(
    //             Box::new(ty.clone()),
    //             args.clone()
    //                 .iter()
    //                 .map(|t| t.tag.clone().unwrap_or(ConstType::Dynamic))
    //                 .collect(),
    //         ),
    //     );

    //     let expr = Expr::Func {
    //         ret: ty.clone(),
    //         name: func.name.val,
    //         args,
    //         body,
    //     };

    //     Ok(Node { expr, ty })
    // }

    pub fn analyz_prog(exprs: Vec<Node>, functions: Vec<Blueprint>) -> Result<Vec<Node>, ErrKind> {
        let mut analyzer = Analyzer::new();
        let mut analyzed_prog = Vec::new();

        analyzer.import(
            &mut analyzed_prog,
            ConstType::Void,
            "std",
            "writeln",
            vec![ConstType::Dynamic],
        );

        // for func in functions.clone() {
        //     analyzer.env.push_function(
        //         func.name.val.clone(),
        //         func.args.clone(),
        //         ConstType::Unknown,
        //     );
        // }
        // for func in functions {
        //     analyzed_prog.push(analyzer.analyz_func(func)?);
        // }

        // setting our env blueprints to our uncompiled functions (blueprints are then compiled pased on call arguments)
        analyzer.env.blueprints(functions);

        for expr in exprs {
            let analyzed_expr = analyzer.analyz(expr)?;
            dbg!(&analyzed_expr);
            analyzed_prog.push(analyzed_expr);
        }
        analyzed_prog = [
            analyzer.imports.clone(),
            analyzer.functions.clone(),
            analyzed_prog,
        ]
        .concat();
        analyzed_prog = analyzer.correct_prog(analyzed_prog)?;
        Ok(analyzed_prog)
    }

    #[inline]
    pub fn analyz_body(&mut self, body: Vec<Node>) -> Result<Vec<Node>, ErrKind> {
        let mut analyzed_body = vec![];
        self.env = self.env.child();
        for expr in body {
            analyzed_body.push(self.analyz(expr)?);
        }
        self.env = self.env.parent().unwrap();
        Ok(analyzed_body)
    }

    #[inline]
    pub fn analyz_items(&mut self, items: Vec<Node>) -> Result<Vec<Node>, ErrKind> {
        let mut analyzed_items = vec![];
        for expr in items {
            analyzed_items.push(self.analyz(expr)?);
        }
        Ok(analyzed_items)
    }

    pub fn analyz(&mut self, node: Node) -> Result<Node, ErrKind> {
        match node.expr {
            Expr::Literal(literal) => {
                let ty = literal.get_ty();
                Ok(Node {
                    expr: Expr::Literal(literal),
                    ty,
                })
            }

            Expr::ListExpr(items) => {
                let items = self.analyz_items(items)?;

                let mut item_ty = if items.len() > 0 {
                    (&items.first().unwrap()).ty.clone()
                } else {
                    ConstType::Void // empty list unknown type figure out type on push
                };

                for (i, item) in (&items).iter().enumerate() {
                    if &item.ty == &ConstType::Unknown {
                        item_ty = ConstType::Unknown;
                        break;
                    }

                    if &item.ty != &item_ty {
                        self.err(ErrKind::InvaildType, format!("list items have to be of the same type, item {} is of an invaild type", i-1));
                        return Err(ErrKind::InvaildType);
                    }
                }
                let ty = ConstType::List(Box::new(item_ty));
                let expr = Expr::ListExpr(items);
                Ok(Node { expr, ty })
            }

            Expr::BinaryExpr { op, left, right } => self.analyz_binary_expr(*left, *right, op),
            Expr::Ident(id) => self.analyz_id(id),
            Expr::Id(id) => self.analyz_id(Ident { val: id, tag: None }),
            Expr::VarDeclare { name, val } => self.analyz_var_declare(name, *val),
            Expr::VarAssign { name, val } => self.analyz_var_assign(*name, *val),
            Expr::Discard(expr) => {
                let ty = ConstType::Void;
                let expr = self.analyz(*expr)?;
                let expr = Expr::Discard(Box::new(expr));

                Ok(Node { expr, ty })
            }
            Expr::PosInfo(x, line, column) => {
                self.line = line;
                self.column = column;
                Ok(Node {
                    expr: Expr::PosInfo(x, line, column),
                    ty: ConstType::Void,
                })
            }

            Expr::RetExpr(expr) => {
                let expr = self.analyz(*expr)?;
                let ty = expr.ty.clone();

                let expr = Expr::RetExpr(Box::new(expr));
                Ok(Node { expr, ty })
            }

            Expr::FnCall { name, args } => self.analyz_call(*name, args),

            Expr::IfExpr {
                condition,
                body,
                alt,
            } => {
                let condition = Box::new(self.analyz(*condition)?);
                if &condition.ty != &ConstType::Bool {
                    self.err(
                        ErrKind::InvaildType,
                        format!(
                            "invaild condition for while loop expected Bool got {:?}",
                            condition.ty
                        ),
                    );
                    return Err(ErrKind::InvaildType);
                }
                let body = self.analyz_body(body)?;

                let analyzed_alt = if alt.is_none() {
                    None
                } else {
                    Some(Box::new(self.analyz(*alt.unwrap())?))
                };

                let last = body.last();

                let ty = if last.is_none() {
                    ConstType::Void
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
            Expr::Block(block) => {
                let block = self.analyz_body(block)?;

                let last = block.last();
                let ty = if last.is_none() {
                    ConstType::Void
                } else {
                    last.unwrap().ty.clone()
                };
                let expr = Expr::Block(block);

                Ok(Node { expr, ty })
            }

            Expr::WhileExpr { condition, body } => {
                let condition = Box::new(self.analyz(*condition)?);
                if &condition.ty != &ConstType::Bool {
                    self.err(
                        ErrKind::InvaildType,
                        format!(
                            "invaild condition for while loop expected Bool got {:?}",
                            condition.ty
                        ),
                    );
                    return Err(ErrKind::InvaildType);
                }
                let body = self.analyz_body(body)?;

                let expr = Expr::WhileExpr { condition, body };
                let ty = ConstType::Void;

                Ok(Node { expr, ty })
            }

            Expr::MemberExpr { parent, child } => self.analyz_member(*parent, child),
            Expr::IndexExpr { parent, index } => self.analyz_index(*parent, *index),
            _ => todo!("node {:#?}", node),
        }
    }

    pub fn analyz_binary_expr(
        &mut self,
        left: Node,
        right: Node,
        op: String,
    ) -> Result<Node, ErrKind> {
        let mut lhs = self.analyz(left)?;
        let mut rhs = self.analyz(right)?;
        (lhs, rhs) = self.type_conv(lhs, rhs)?;

        let ty = match op.as_str() {
            "==" | ">" | "<" | ">=" | "<=" => ConstType::Bool,
            _ => lhs.ty.clone(),
        };

        if !supports_op(&lhs.ty, &op) {
            self.err(ErrKind::OperationNotGranted, format!("one of possible types [{:?}] does not support operator {}, use the do keyword to do it anyways", lhs.ty, op));
            return Err(ErrKind::OperationNotGranted);
        }
        let left = Box::new(lhs);
        let right = Box::new(rhs);

        let expr = Expr::BinaryExpr { op, left, right };
        Ok(Node { expr, ty })
    }

    pub fn analyz_call(&mut self, name: Node, args: Vec<Node>) -> Result<Node, ErrKind> {
        let name = Box::new(self.analyz(name)?);
        match name.ty.clone() {
            ConstType::Blueprint { argc, name } => {
                let args = self.analyz_items(args)?;

                // TODO! if its a member call pass parent as first arg and call the child instead
                // if let Expr::MemberExpr {
                //     parent: p,
                //     child: c,
                // } = (*name).clone().expr
                // {
                //     if self.env.ty_parent_fn(&p.ty, &c).is_some() {
                //         name = Box::new(Node {
                //             expr: Expr::Id(c),
                //             ty: name.ty,
                //         });
                //         let mut p = vec![*p];
                //         p.append(&mut args);
                //         args = p;
                //     }
                // }

                if &argc != &(args.len() as u32) {
                    self.err(ErrKind::UndeclaredVar, format!("not enough arguments got {} arguments, expected {} arguments for function {:?}", args.len(), argc, name));
                    return Err(ErrKind::UndeclaredVar);
                }
                let args_types: Vec<ConstType> = args.iter().map(|arg| arg.ty.clone()).collect();
                let mangle = type_mangle(name.clone(), args_types.clone().into());

                let (expr, ty) = if self.env.has(&mangle) {
                    let id_ty = self.env.get_ty(&mangle).unwrap();

                    let ty = if let ConstType::Func(ref ret, _) = id_ty {
                        *ret.clone()
                    } else {
                        panic!()
                    };

                    (
                        Expr::FnCall {
                            name: Box::new(Node {
                                expr: Expr::Id(mangle),
                                ty: id_ty,
                            }),
                            args,
                        },
                        ty,
                    )
                } else {
                    // building a function from blueprint
                    let blueprint = self.env.get_blueprint(&name).unwrap();
                    self.env = self.env.child();
                    // allows for the function to call itself
                    self.env
                        .push_function(mangle.clone(), Vec::new(), ConstType::Unknown);

                    let mut typed_params = Vec::new();
                    for (i, arg) in (&blueprint.args).into_iter().enumerate() {
                        self.env.add(&arg.val, args_types[i].clone());
                        typed_params.push(Ident {
                            val: arg.val.clone(),
                            tag: Some(args_types[i].clone()),
                        })
                    }

                    let mut body = self.analyz_items(blueprint.body)?;
                    // TODO: loop through the body check for unknown function calls and replace them with function type

                    let ty = get_fn_type(&body, ConstType::Void);

                    replace_body_ty(
                        &mut body,
                        &ConstType::Func(Box::new(ConstType::Unknown), Vec::new()),
                        &ConstType::Func(Box::new(ty.clone()), args_types.clone()),
                    );

                    self.env = self.env.parent().unwrap();

                    self.env
                        .push_function(mangle.clone(), args_types, ty.clone());

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

                    dbg!(&self.functions);

                    // calling the built function
                    let expr = Expr::FnCall {
                        name: Box::new(Node {
                            ty: self.env.get_ty(&mangle).unwrap(),
                            expr: Expr::Id(mangle),
                        }),
                        args,
                    };
                    (expr, ty)
                };

                Ok(Node { expr, ty })
            }

            ConstType::Func(ret, args_types) => {
                let mut args = self.analyz_items(args)?;

                if &args_types.len() != &args.len() {
                    self.err(ErrKind::UndeclaredVar, format!("not enough arguments got {} arguments, expected {} arguments for function {:?}", args.len(), args_types.len(), name));
                    return Err(ErrKind::UndeclaredVar);
                }

                for (i, arg) in (&mut args).iter_mut().enumerate() {
                    if &arg.ty != &args_types[i] {
                        let attempt = self.type_cast(arg.clone(), args_types[i].clone());

                        if attempt.is_ok() {
                            *arg = attempt.unwrap();
                        } else {
                            self.err(
                                ErrKind::UnexceptedArgs,
                                format!(
                                    "unexpected argument type, at arg {}, expected {:?}, got {:?}",
                                    i, args_types[i], arg.ty
                                ),
                            );
                            return Err(ErrKind::UnexceptedArgs);
                        }
                    }
                }

                let expr = Expr::FnCall { name, args };

                Ok(Node { expr, ty: *ret })
            }

            _ => {
                self.err(
                    ErrKind::UnexceptedTokenE,
                    format!("expected symbol to call got {:?}", name.expr),
                );
                Err(ErrKind::UnexceptedTokenE)
            }
        }
    }

    pub fn analyz_index(&mut self, parent: Node, index: Node) -> Result<Node, ErrKind> {
        let parent = self.analyz(parent)?;
        let index = self.analyz(index)?;
        if index.ty != ConstType::Int {
            return Err(ErrKind::InvaildType);
        }
        let ty = match parent.ty.clone() {
            ConstType::Str => ConstType::Str,
            ConstType::List(t) => *t,
            _ => return Err(ErrKind::InvaildType),
        };

        let expr = Expr::IndexExpr {
            parent: Box::new(parent),
            index: Box::new(index),
        };
        Ok(Node { expr, ty })
    }

    pub fn analyz_member(&mut self, parent: Node, child: String) -> Result<Node, ErrKind> {
        let parent = self.analyz(parent)?;

        let mut ty = parent.ty.get(&child);
        if ty.is_none() {
            ty = self.env.ty_parent_fn(&parent.ty, &child);
            if ty.is_none() {
                return Err(ErrKind::UndeclaredVar);
            }
        }

        let expr = Expr::MemberExpr {
            parent: Box::new(parent),
            child,
        };

        Ok(Node {
            ty: ty.unwrap(),
            expr,
        })
    }

    pub fn analyz_id(&mut self, id: Ident) -> Result<Node, ErrKind> {
        if !self.env.has(&id.val) {
            return Err(ErrKind::UndeclaredVar);
        }

        let ty = self.env.get_ty(&id.val).unwrap();

        let expr = Expr::Id(id.val);
        Ok(Node { expr, ty })
    }

    pub fn analyz_var_declare(&mut self, name: Ident, val: Node) -> Result<Node, ErrKind> {
        let val = self.analyz(val)?;

        if self.env.has(&name.val) {
            return Err(ErrKind::VarAlreadyDeclared);
        }
        let ty = val.ty.clone();
        self.env.add(&name.val, ty.clone());

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

        if let Expr::Id(ref name) = name.expr {
            self.env.modify(name, ty.clone());
        } else if val.ty != name.ty {
            if name.ty == ConstType::Unknown {
                ty = ConstType::Unknown;
            } else {
                self.err(ErrKind::InvaildType, format!("cannot set the value of an Obj property to a value of different type, got type {:?} expected {:?}, in expr {:?} = {:?}", val.ty, name.ty, name, val));
                return Err(ErrKind::InvaildType);
            }
        }

        let expr = Expr::VarAssign {
            name: Box::new(name),
            val: Box::new(val),
        };
        Ok(Node { expr, ty })
    }

    pub fn type_cast(&mut self, from: Node, into: ConstType) -> Result<Node, ErrKind> {
        if into == ConstType::Dynamic || into == ConstType::Str {
            Ok(ty_as(&into, from))
        } else {
            let _tmp = Expr::Id("temp".to_string());
            let _tmp = Node {
                expr: _tmp,
                ty: into,
            };
            let (try_conv, unchanged) = self.type_conv(from.clone(), _tmp.clone())?;
            if &unchanged != &_tmp {
                self.err(
                    ErrKind::InvaildType,
                    format!("cannot convert from {:?} into {:?}", from.ty, _tmp.ty),
                );
                return Err(ErrKind::InvaildType);
            }
            Ok(try_conv)
        }
    }

    pub fn type_conv(&mut self, mut lhs: Node, mut rhs: Node) -> Result<(Node, Node), ErrKind> {
        // this code is not good rn
        if lhs.ty != rhs.ty {
            if lhs.ty == ConstType::Float && rhs.ty == ConstType::Int {
                rhs = ty_as(&lhs.ty, rhs);
            } else if lhs.ty == ConstType::Int && rhs.ty == ConstType::Float {
                lhs = ty_as(&rhs.ty, lhs);
            } else if lhs.ty == ConstType::Str {
                rhs = ty_as(&lhs.ty, rhs);
            } else if rhs.ty == ConstType::Str {
                lhs = ty_as(&rhs.ty, lhs);
            } else if lhs.ty == ConstType::Dynamic {
                rhs = ty_as(&lhs.ty, rhs);
            } else if rhs.ty == ConstType::Dynamic {
                lhs = ty_as(&rhs.ty, lhs);
            } else if lhs.ty == ConstType::Unknown || rhs.ty == ConstType::Unknown {
            } else {
                self.err(
                    ErrKind::InvaildType,
                    format!("cannot make {:?} and {:?} as the same type", lhs.ty, rhs.ty),
                );
                return Err(ErrKind::InvaildType);
            }
        }

        Ok((lhs, rhs))
    }
}
