use crate::parser::ast::Expr;
use crate::parser::Function;
use crate::source::{ATErr, ConstType, ErrKind, Ident};

use super::*;

#[inline]
fn ty_as(ty: &ConstType, expr: TypedExpr) -> TypedExpr {
    TypedExpr {
        expr: AnalyzedExpr::As(Box::new(expr)),
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
        body: &mut Vec<TypedExpr>,
        ty: ConstType,
        module: &str,
        name: &str,
        args: Vec<(ConstType, String)>,
    ) {
        self.env.push_function(
            name.to_string(),
            args.clone()
                .into_iter()
                .map(|v| Ident {
                    tag: Some(v.0),
                    val: v.1,
                })
                .collect(),
            ty.clone(),
        );
        body.push(TypedExpr {
            expr: AnalyzedExpr::Import {
                module: module.to_string(),
                name: name.to_string(),
                args: args.into_iter().map(|v| v.0).collect(),
            },
            ty,
        })
    }

    pub fn analyz_func(&mut self, func: Function) -> Result<TypedExpr, ErrKind> {
        self.env = self.env.child();

        let args = func.args;

        for arg in &args {
            self.env
                .add(&arg.val, arg.tag.clone().unwrap_or(ConstType::Dynamic));
        }

        // allow calling self

        let mut body = vec![];
        for expr in func.body {
            body.push(self.analyz(expr)?);
        }

        let ty = get_fn_type(&body, ConstType::Void);
        self.env = self.env.parent().unwrap();
        self.env.modify(
            &func.name.val,
            ConstType::Func(
                Box::new(ty.clone()),
                args.clone()
                    .iter()
                    .map(|t| t.tag.clone().unwrap_or(ConstType::Dynamic))
                    .collect(),
            ),
        );

        let expr = AnalyzedExpr::Func {
            ret: ty.clone(),
            name: func.name.val,
            args,
            body,
        };

        Ok(TypedExpr { expr, ty })
    }

    pub fn analyz_prog(
        exprs: Vec<Expr>,
        functions: Vec<Function>,
    ) -> Result<Vec<TypedExpr>, ErrKind> {
        let mut analyzer = Analyzer::new();
        let mut analyzed_prog = Vec::new();

        analyzer.import(
            &mut analyzed_prog,
            ConstType::Void,
            "std",
            "writeln",
            vec![(ConstType::Dynamic, "x".to_string())],
        );
        for func in functions.clone() {
            analyzer.env.push_function(
                func.name.val.clone(),
                func.args.clone(),
                ConstType::Dynamic,
            );
        }
        for func in functions {
            analyzed_prog.push(analyzer.analyz_func(func)?);
        }

        for expr in exprs {
            let analyzed_expr = analyzer.analyz(expr)?;
            analyzed_prog.push(analyzed_expr);
        }

        analyzed_prog = analyzer.correct_prog(analyzed_prog)?;
        Ok(analyzed_prog)
    }

    #[inline]
    pub fn analyz_body(&mut self, body: Vec<Expr>) -> Result<Vec<TypedExpr>, ErrKind> {
        let mut analyzed_body = vec![];
        self.env = self.env.child();
        for expr in body {
            analyzed_body.push(self.analyz(expr)?);
        }
        self.env = self.env.parent().unwrap();
        Ok(analyzed_body)
    }

    #[inline]
    pub fn analyz_items(&mut self, items: Vec<Expr>) -> Result<Vec<TypedExpr>, ErrKind> {
        let mut analyzed_items = vec![];
        for expr in items {
            analyzed_items.push(self.analyz(expr)?);
        }
        Ok(analyzed_items)
    }

    pub fn analyz(&mut self, expr: Expr) -> Result<TypedExpr, ErrKind> {
        match expr {
            Expr::Literal(literal) => {
                let ty = literal.get_ty();
                Ok(TypedExpr {
                    expr: AnalyzedExpr::Literal(literal),
                    ty,
                })
            }

            Expr::ListExpr(items) => {
                let mut items = self.analyz_items(items)?;

                let item_ty = if items.len() > 0 {
                    (&items.first().unwrap()).ty.clone()
                } else {
                    ConstType::Void // empty list unknown type figure out type on push
                };

                for (i, item) in (&items).iter().enumerate() {
                    if &item.ty != &item_ty {
                        self.err(ErrKind::InvaildType, format!("list items have to be of the same type, item {} is of an invaild type", i-1));
                        return Err(ErrKind::InvaildType);
                    }
                }
                let ty = ConstType::List(Box::new(item_ty));
                let expr = AnalyzedExpr::List(items);
                Ok(TypedExpr { expr, ty })
            }

            Expr::BinaryExpr { op, left, right } => self.analyz_binary_expr(*left, *right, op),
            Expr::Ident(id) => self.analyz_id(id),
            Expr::VarDeclare { name, val } => self.analyz_var_declare(name, *val),
            Expr::VarAssign { name, val } => self.analyz_var_assign(*name, *val),
            Expr::Discard(expr) => {
                let ty = ConstType::Void;
                let expr = self.analyz(*expr)?;
                let expr = AnalyzedExpr::Discard(Box::new(expr));

                Ok(TypedExpr { expr, ty })
            }
            Expr::PosInfo(x, line, column) => {
                self.line = line;
                self.column = column;
                Ok(TypedExpr {
                    expr: AnalyzedExpr::Debug(x, line, column),
                    ty: ConstType::Void,
                })
            }

            Expr::RetExpr(expr) => {
                let expr = self.analyz(*expr)?;
                let ty = expr.ty.clone();

                let expr = AnalyzedExpr::RetExpr(Box::new(expr));
                Ok(TypedExpr { expr, ty })
            }

            Expr::FnCall { name, args } => self.analyz_call(*name, args),

            Expr::IfExpr {
                condition,
                body,
                alt,
            } => {
                let cond = Box::new(self.analyz(*condition)?);
                if &cond.ty != &ConstType::Bool {
                    self.err(
                        ErrKind::InvaildType,
                        format!(
                            "invaild condition for while loop expected Bool got {:?}",
                            cond.ty
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

                let expr = AnalyzedExpr::If {
                    cond,
                    body,
                    alt: analyzed_alt,
                };

                Ok(TypedExpr { expr, ty })
            }
            Expr::Block(block) => {
                let block = self.analyz_body(block)?;

                let last = block.last();
                let ty = if last.is_none() {
                    ConstType::Void
                } else {
                    last.unwrap().ty.clone()
                };
                let expr = AnalyzedExpr::Block(block);

                Ok(TypedExpr { expr, ty })
            }

            Expr::WhileExpr { condition, body } => {
                let cond = Box::new(self.analyz(*condition)?);
                if &cond.ty != &ConstType::Bool {
                    self.err(
                        ErrKind::InvaildType,
                        format!(
                            "invaild condition for while loop expected Bool got {:?}",
                            cond.ty
                        ),
                    );
                    return Err(ErrKind::InvaildType);
                }
                let body = self.analyz_body(body)?;

                let expr = AnalyzedExpr::While { cond, body };
                let ty = ConstType::Void;

                Ok(TypedExpr { expr, ty })
            }

            Expr::MemberExpr { parent, child } => self.analyz_member(*parent, child),
            Expr::IndexExpr { parent, index } => self.analyz_index(*parent, *index),
            // _ => todo!("expr {:#?}", expr),
        }
    }

    pub fn analyz_binary_expr(
        &mut self,
        left: Expr,
        right: Expr,
        op: String,
    ) -> Result<TypedExpr, ErrKind> {
        let mut lhs = self.analyz(left)?;
        let mut rhs = self.analyz(right)?;
        (lhs, rhs) = self.type_conv(lhs, rhs)?;

        let ty = match op.as_str() {
            "==" | ">" | "<" | ">=" | "<=" => ConstType::Bool,
            _ => lhs.ty.clone(),
        };

        if !supports_op(&lhs.ty, &op) {
            self.err(ErrKind::OperationNotGranted, format!("one of possible types [{:?}] does not support operator {}, use the do keyword to do it anyways", ty, op));
            return Err(ErrKind::OperationNotGranted);
        }
        let left = Box::new(lhs);
        let right = Box::new(rhs);

        let expr = AnalyzedExpr::BinaryExpr { op, left, right };
        Ok(TypedExpr { expr, ty })
    }

    pub fn analyz_call(&mut self, name: Expr, params: Vec<Expr>) -> Result<TypedExpr, ErrKind> {
        let mut name = Box::new(self.analyz(name)?);
        if let ConstType::Func(ty, args_ty) = name.ty.clone() {
            let mut args = self.analyz_items(params)?;

            // if its a member call pass parent as first arg and call the child instead
            if let AnalyzedExpr::Member(p, c) = (*name).clone().expr {
                if self.env.ty_parent_fn(&p.ty, &c).is_some() {
                    name = Box::new(TypedExpr {
                        expr: AnalyzedExpr::Id(c),
                        ty: name.ty,
                    });
                    let mut p = vec![*p];
                    p.append(&mut args);
                    args = p;
                }
            }

            if &args_ty.len() != &args.len() {
                self.err(ErrKind::UndeclaredVar, format!("not enough arguments got {} arguments, expected {} arguments for function {:?}", args.len(), args_ty.len(), name.expr));
                return Err(ErrKind::UndeclaredVar);
            }

            for (i, ty) in args_ty.into_iter().enumerate() {
                if &args[i].ty != &ty {
                    args[i] = self.type_cast(args[i].clone(), ty)?;
                }
            }

            let expr = AnalyzedExpr::FnCall { name, args };

            Ok(TypedExpr { expr, ty: *ty })
        } else {
            self.err(
                ErrKind::UnexceptedTokenE,
                format!("expected symbol to call got {:?}", name.expr),
            );
            Err(ErrKind::UnexceptedTokenE)
        }
    }

    pub fn analyz_index(&mut self, parent: Expr, index: Expr) -> Result<TypedExpr, ErrKind> {
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

        let expr = AnalyzedExpr::Index(Box::new(parent), Box::new(index));
        Ok(TypedExpr { expr, ty })
    }

    pub fn analyz_member(&mut self, parent: Expr, child: String) -> Result<TypedExpr, ErrKind> {
        let parent = self.analyz(parent)?;

        let mut ty = parent.ty.get(&child);
        if ty.is_none() {
            ty = self.env.ty_parent_fn(&parent.ty, &child);
            if ty.is_none() {
                return Err(ErrKind::UndeclaredVar);
            }
        }

        let expr = AnalyzedExpr::Member(Box::new(parent), child);

        Ok(TypedExpr {
            ty: ty.unwrap(),
            expr,
        })
    }

    pub fn analyz_id(&mut self, id: Ident) -> Result<TypedExpr, ErrKind> {
        if !self.env.has(&id.val) {
            return Err(ErrKind::UndeclaredVar);
        }

        let ty = self.env.get_ty(&id.val).unwrap();

        let expr = AnalyzedExpr::Id(id.val);
        Ok(TypedExpr { expr, ty })
    }

    pub fn analyz_var_declare(&mut self, name: Ident, val: Expr) -> Result<TypedExpr, ErrKind> {
        let val = self.analyz(val)?;
        let name = name.val;
        if self.env.has(&name) {
            return Err(ErrKind::VarAlreadyDeclared);
        }
        let ty = val.ty.clone();
        self.env.add(&name, ty.clone());

        let expr = AnalyzedExpr::VarDeclare {
            name,
            val: Box::new(val),
        };
        Ok(TypedExpr { expr, ty })
    }

    pub fn analyz_var_assign(&mut self, id: Expr, val: Expr) -> Result<TypedExpr, ErrKind> {
        let val = self.analyz(val)?;
        let name = self.analyz(id)?;
        let ty = val.ty.clone();
        if let AnalyzedExpr::Id(ref name) = name.expr {
            self.env.modify(name, ty.clone());
        } else if val.ty != name.ty {
            self.err(ErrKind::InvaildType, format!("cannot set the value of an Obj property to a value of different type, got type {:?} expected {:?}, in expr {:?} = {:?}", val.ty, name.ty, name, val));
            return Err(ErrKind::InvaildType);
        }
        let expr = AnalyzedExpr::VarAssign {
            name: Box::new(name),
            val: Box::new(val),
        };
        Ok(TypedExpr { expr, ty })
    }

    pub fn type_cast(&mut self, from: TypedExpr, into: ConstType) -> Result<TypedExpr, ErrKind> {
        if into == ConstType::Dynamic || into == ConstType::Str {
            Ok(ty_as(&into, from))
        } else {
            let _tmp = AnalyzedExpr::Id("temp".to_string());
            let _tmp = TypedExpr {
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

    pub fn type_conv(
        &mut self,
        mut lhs: TypedExpr,
        mut rhs: TypedExpr,
    ) -> Result<(TypedExpr, TypedExpr), ErrKind> {
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
