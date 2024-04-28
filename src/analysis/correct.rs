use crate::{
    parser::ast::Expr,
    source::{ConstType, ErrKind, Ident},
};

use super::{AnalyzedExpr, Analyzer, TypedExpr};

impl TypedExpr {
    pub fn to_expr(self) -> Expr {
        match self.expr {
            AnalyzedExpr::Id(id) => Expr::Ident(Ident { val: id, tag: None }),
            AnalyzedExpr::Literal(lit) => Expr::Literal(lit),
            AnalyzedExpr::As(a) => (*a).to_expr(),
            AnalyzedExpr::Discard(dis) => Expr::Discard(Box::new((*dis).to_expr())),
            AnalyzedExpr::BinaryExpr { op, left, right } => Expr::BinaryExpr {
                op,
                left: Box::new((*left).to_expr()),
                right: Box::new((*right).to_expr()),
            },
            AnalyzedExpr::FnCall { name, args } => Expr::FnCall {
                name: Box::new(name.to_expr()),
                args: args.into_iter().map(|v| v.to_expr()).collect(),
            },
            AnalyzedExpr::Debug(name, line, column) => Expr::PosInfo(name, line, column),
            _ => todo!(),
        }
    }
}

impl Analyzer {
    pub fn correct_prog(&mut self, exprs: Vec<TypedExpr>) -> Result<Vec<TypedExpr>, ErrKind> {
        let mut corrected = vec![];
        for expr in exprs.clone() {
            if let AnalyzedExpr::Import { .. } = expr.expr {
                corrected.push(expr);
            } else {
                corrected.push(self.correct(expr)?);
            }
        }

        Ok(corrected)
    }

    fn correct(&mut self, expr: TypedExpr) -> Result<TypedExpr, ErrKind> {
        let matc = expr.clone();
        match matc.expr {
            AnalyzedExpr::BinaryExpr { left, right, op } => {
                let left = self.correct(*left)?;
                let right = self.correct(*right)?;

                if &left.ty != &right.ty {
                    return self.analyz_binary_expr(left.to_expr(), right.to_expr(), op);
                }
            }
            AnalyzedExpr::FnCall { name, args } => {
                let name = Box::new(self.correct(*name)?);
                let mut corrected_args = vec![];
                for arg in args {
                    let arg = self.correct(arg)?;
                    if arg.ty != ConstType::Dynamic {
                        corrected_args.push(TypedExpr {
                            expr: AnalyzedExpr::As(Box::new(arg)),
                            ty: ConstType::Dynamic,
                        });
                    } else {
                        corrected_args.push(arg);
                    }
                }

                if let ConstType::Func(ty, _) = name.ty.clone() {
                    return Ok(TypedExpr {
                        expr: AnalyzedExpr::FnCall {
                            name,
                            args: corrected_args,
                        },
                        ty: *ty,
                    });
                } else {
                    panic!()
                }
            }

            AnalyzedExpr::Id(id) => {
                dbg!(&self.env);
                dbg!(&id);

                let ty = if self.env.has(&id) {
                    self.env.get_ty(&id).unwrap()
                } else {
                    self.env.add(&id, expr.ty.clone());
                    expr.ty
                };
                return Ok(TypedExpr {
                    ty,
                    expr: AnalyzedExpr::Id(id),
                });
            }

            AnalyzedExpr::Func {
                ret,
                name,
                args,
                body,
            } => {
                self.env.current = ret.clone();
                let body = self.correct_prog(body.clone())?;
                self.env.current = ConstType::Void;
                return Ok(TypedExpr {
                    expr: AnalyzedExpr::Func {
                        ret,
                        name,
                        args,
                        body,
                    },
                    ty: expr.ty,
                });
            }

            AnalyzedExpr::If { cond, body, alt } => {
                let cond = self.correct(*cond)?;
                let body = self.correct_prog(body)?;
                let alt = if alt.is_some() {
                    Some(Box::new(self.correct(*alt.unwrap())?))
                } else {
                    None
                };
                return Ok(TypedExpr {
                    expr: AnalyzedExpr::If {
                        cond: Box::new(cond),
                        body,
                        alt,
                    },
                    ty: expr.ty,
                });
            }

            AnalyzedExpr::While { cond, body } => {
                let cond = self.correct(*cond)?;
                let body = self.correct_prog(body)?;

                return Ok(TypedExpr {
                    expr: AnalyzedExpr::While {
                        cond: Box::new(cond),
                        body,
                    },
                    ty: expr.ty,
                });
            }

            AnalyzedExpr::As(old) => {
                let old = self.correct(*old)?;
                return Ok(TypedExpr {
                    expr: AnalyzedExpr::As(Box::new(old)),
                    ty: expr.ty,
                });
            }
            AnalyzedExpr::RetExpr(ret) => {
                let ret = Box::new(self.correct(*ret)?);
                if ret.ty == self.env.current.clone() {
                    return Ok(TypedExpr {
                        expr: AnalyzedExpr::RetExpr(ret.clone()),
                        ty: ret.ty,
                    });
                } else {
                    return Ok(TypedExpr {
                        ty: self.env.current.clone(),
                        expr: AnalyzedExpr::RetExpr(Box::new(TypedExpr {
                            expr: AnalyzedExpr::As(ret),
                            ty: self.env.current.clone(),
                        })),
                    });
                }
            }
            _ => (),
        };
        Ok(expr)
    }
}
