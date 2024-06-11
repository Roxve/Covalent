use std::fs;

use crate::parser::parse::Parse;
use crate::types::{type_mangle, AtomKind};

use crate::err::{ATErr, ErrKind};
use crate::parser::ast::{Blueprint, Expr, Ident, Node};

use super::*;

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
        ty: AtomKind,
        module: &str,
        name: &str,
        args: Vec<AtomKind>,
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

    pub fn analyz_blueprint(
        &mut self,
        blueprint: Blueprint,
        types: Vec<AtomKind>,
    ) -> Result<String, ErrKind> {
        let mangle = type_mangle(blueprint.name.val().clone(), types.clone());
        if self.env.has(&mangle) {
            if let AtomKind::Func(_, _, _) = self.env.get_ty(&mangle).unwrap() {
                return Ok(mangle);
            }
        }

        self.env = self.env.child();
        // allows for the function to call itself
        self.env
            .push_function(mangle.clone(), Vec::new(), AtomKind::Unknown(None));

        let mut typed_params = Vec::new();
        for (i, arg) in (&blueprint.args).into_iter().enumerate() {
            self.env.add(&arg.val(), types[i].clone());
            typed_params.push(Ident::Typed(types[i].clone(), arg.val().clone()))
        }

        let mut body = self.analyz_body(blueprint.body, false)?;
        let ty = get_fn_type(&body);

        replace_body_ty(
            &mut body,
            &AtomKind::Func(
                Box::new(AtomKind::Unknown(None)),
                Vec::new(),
                mangle.clone(),
            ),
            &AtomKind::Func(Box::new(ty.clone()), types.clone(), mangle.clone()),
        );

        self.env = self.env.parent().unwrap();

        self.env.push_function(mangle.clone(), types, ty.clone());

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

    pub fn analyz_prog(
        exprs: Vec<Node>,
        functions: Vec<Blueprint>,
        workdir: String,
    ) -> Result<Vec<Node>, ErrKind> {
        let mut analyzer = Analyzer::new(workdir);
        let mut analyzed_prog = Vec::new();

        analyzer.import(
            &mut analyzed_prog,
            AtomKind::Void,
            "std",
            "writeln",
            vec![AtomKind::Dynamic],
        );

        macro_rules! btype {
            ($name: expr, $ty: path) => {
                analyzer
                    .env
                    .add(&($name).to_string(), AtomKind::Type(Box::new($ty)));
            };
        }
        btype!("int", AtomKind::Int);
        btype!("float", AtomKind::Float);

        // setting our env blueprints to our uncompiled functions (blueprints are then compiled pased on call arguments)
        analyzer.blueprints(functions);
        analyzed_prog.append(&mut analyzer.analyz_body(exprs, true)?);

        analyzed_prog = [
            analyzer.imports.clone(),
            analyzer.functions.clone(),
            analyzed_prog,
        ]
        .concat();
        // analyzed_prog = analyzer.correct_prog(analyzed_prog)?;
        Ok(analyzed_prog)
    }

    #[inline]
    pub fn analyz_body(&mut self, body: Vec<Node>, top: bool) -> Result<Vec<Node>, ErrKind> {
        let mut analyzed_body = vec![];
        if !top {
            self.env = self.env.child();
        }

        for node in body {
            if let &Expr::Use(ref path) = &node.expr {
                let abs = format!("{}/{}", self.workdir.clone(), path);
                let read = fs::read_to_string(abs)
                    .expect(format!("failed to open path {} to use", path).as_str());

                use crate::parser::Parser;
                let mut parser = Parser::new(read);

                let ast = parser.parse_prog();

                self.blueprints(parser.functions);
                let mut ast = self.analyz_body(ast, true)?;

                analyzed_body.append(&mut ast);
                continue;
            }
            analyzed_body.push(self.analyz(node)?);
        }

        if !top {
            self.env = self.env.parent().unwrap();
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

    pub fn analyz(&mut self, node: Node) -> Result<Node, ErrKind> {
        match node.expr.clone() {
            Expr::As(_) => Ok(node), // this type of expressions happens in correction

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
                    AtomKind::Void // empty list unknown type figure out type on push
                };

                for (i, item) in (&items).iter().enumerate() {
                    if let &AtomKind::Unknown(_) = &item_ty {
                        break;
                    }

                    if &item.ty != &item_ty {
                        self.err(ErrKind::InvaildType, format!("list items have to be of the same type, item {} is of an invaild type", i-1));
                        return Err(ErrKind::InvaildType);
                    }
                }
                let ty = AtomKind::List(Box::new(item_ty));
                let expr = Expr::ListExpr(items);
                Ok(Node { expr, ty })
            }

            Expr::BinaryExpr { op, left, right } => self.analyz_binary_expr(*left, *right, op),
            Expr::Ident(id) => self.analyz_id(id),
            Expr::VarDeclare { name, val } => self.analyz_var_declare(name, *val),
            Expr::VarAssign { name, val } => self.analyz_var_assign(*name, *val),
            Expr::Discard(expr) => {
                let ty = AtomKind::Void;
                let expr = self.analyz(*expr)?;
                let expr = Expr::Discard(Box::new(expr));

                Ok(Node { expr, ty })
            }
            Expr::PosInfo(x, line, column) => {
                self.line = line;
                self.column = column;
                Ok(Node {
                    expr: Expr::PosInfo(x, line, column),
                    ty: AtomKind::Void,
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
                if &condition.ty != &AtomKind::Bool {
                    self.err(
                        ErrKind::InvaildType,
                        format!(
                            "invaild condition for while loop expected Bool got {:?}",
                            condition.ty
                        ),
                    );
                    return Err(ErrKind::InvaildType);
                }
                let body = self.analyz_body(body, false)?;

                let analyzed_alt = if alt.is_none() {
                    None
                } else {
                    Some(Box::new(self.analyz(*alt.unwrap())?))
                };

                let last = body.last();

                let ty = if last.is_none() {
                    AtomKind::Void
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
                let block = self.analyz_body(block, false)?;

                let last = block.last();
                let ty = if last.is_none() {
                    AtomKind::Void
                } else {
                    last.unwrap().ty.clone()
                };
                let expr = Expr::Block(block);

                Ok(Node { expr, ty })
            }

            Expr::WhileExpr { condition, body } => {
                let condition = Box::new(self.analyz(*condition)?);
                if &condition.ty != &AtomKind::Bool {
                    self.err(
                        ErrKind::InvaildType,
                        format!(
                            "invaild condition for while loop expected Bool got {:?}",
                            condition.ty
                        ),
                    );
                    return Err(ErrKind::InvaildType);
                }
                let body = self.analyz_body(body, false)?;

                let expr = Expr::WhileExpr { condition, body };
                let ty = AtomKind::Void;

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

        if &rhs.ty == &AtomKind::Unknown(None) && &lhs.ty == &AtomKind::Unknown(None) {
        } else if let &AtomKind::Unknown(None) = &rhs.ty {
            rhs.ty = AtomKind::Unknown(Some(Box::new(lhs.ty.clone()))); // unknownize the expression if any of the sides is unknown so we can figure it out later
        } else if let &AtomKind::Unknown(None) = &lhs.ty {
            lhs.ty = AtomKind::Unknown(Some(Box::new(rhs.ty.clone())));
        }

        (lhs, rhs) = self.type_conv(lhs, rhs)?;
        let ty = match op.as_str() {
            "==" | ">" | "<" | ">=" | "<=" => AtomKind::Bool,
            _ => {
                if let &AtomKind::Unknown(Some(ref ty)) = &lhs.ty {
                    (**ty).clone()
                } else {
                    lhs.ty.clone()
                }
            }
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
            AtomKind::Blueprint { argc, name } => {
                let args = self.analyz_items(args)?;

                // TODO! if its a member call pass parent as first arg and call the child instead
                if &argc != &(args.len() as u32) {
                    self.err(ErrKind::UndeclaredVar, format!("not enough arguments got {} arguments, expected {} arguments for function {:?}", args.len(), argc, name));
                    return Err(ErrKind::UndeclaredVar);
                }
                let args_types: Vec<AtomKind> = args.iter().map(|arg| arg.ty.clone()).collect();
                let mangle = type_mangle(name.clone(), args_types.clone());

                let (expr, ty) = if self.env.has(&mangle) {
                    let id_ty = self.env.get_ty(&mangle).unwrap();

                    let ty = if let AtomKind::Func(ref ret, _, _) = id_ty {
                        *ret.clone()
                    } else {
                        panic!()
                    };

                    (
                        Expr::FnCall {
                            name: Box::new(Node {
                                expr: Expr::Ident(Ident::UnTagged(mangle)),
                                ty: id_ty,
                            }),
                            args,
                        },
                        ty,
                    )
                } else {
                    // building a function from blueprint
                    let blueprint = self.env.get_blueprint(&name).unwrap();

                    let fun = self.analyz_blueprint(blueprint, args_types)?;
                    let ty = self.env.get_ty(&fun).unwrap(); // calling the built function

                    let ret = if let AtomKind::Func(ret, _, _) = ty.clone() {
                        *ret
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
                    (expr, ret)
                };

                Ok(Node { expr, ty })
            }

            AtomKind::Func(ret, args_types, _) => {
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
        if index.ty != AtomKind::Int {
            self.err(ErrKind::InvaildType, format!("index is not an int"));
            return Err(ErrKind::InvaildType);
        }
        let ty = match parent.ty.clone() {
            AtomKind::Str => AtomKind::Str,
            AtomKind::List(t) => *t,
            _ => {
                self.err(
                    ErrKind::InvaildType,
                    format!("cannot index {:?}", parent.ty),
                );
                return Err(ErrKind::InvaildType);
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
        let ty = val.ty.clone();
        self.env.add(&name.val(), ty.clone());

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
            self.env.modify(&name.val(), ty.clone());
        } else if val.ty != name.ty {
            if let AtomKind::Unknown(assume) = name.ty.clone() {
                ty = AtomKind::Unknown(assume);
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

    // who tf wrote this code
    pub fn type_cast(&mut self, from: Node, into: AtomKind) -> Result<Node, ErrKind> {
        if into == AtomKind::Dynamic || into == AtomKind::Str {
            Ok(ty_as(&into, from))
        } else {
            let _tmp = Expr::Ident(Ident::UnTagged("temp".to_string()));
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
            if let &AtomKind::Unknown(Some(ref ty)) = &lhs.ty {
                if &**ty == &rhs.ty {
                    return Ok((lhs, rhs));
                }
            }

            if let &AtomKind::Unknown(Some(ref ty)) = &rhs.ty {
                if &**ty == &lhs.ty {
                    return Ok((lhs, rhs));
                }
            }

            if lhs.ty == AtomKind::Float && rhs.ty == AtomKind::Int {
                rhs = ty_as(&lhs.ty, rhs);
            } else if lhs.ty == AtomKind::Int && rhs.ty == AtomKind::Float {
                lhs = ty_as(&rhs.ty, lhs);
            } else if lhs.ty == AtomKind::Str {
                rhs = ty_as(&lhs.ty, rhs);
            } else if rhs.ty == AtomKind::Str {
                lhs = ty_as(&rhs.ty, lhs);
            } else if lhs.ty == AtomKind::Dynamic {
                rhs = ty_as(&lhs.ty, rhs);
            } else if rhs.ty == AtomKind::Dynamic {
                lhs = ty_as(&rhs.ty, lhs);
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
