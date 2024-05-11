use crate::{
    parser::ast::{Expr, Node},
    source::{ConstType, ErrKind},
};

use super::Analyzer;

impl Analyzer {
    pub fn correct_prog(&mut self, exprs: Vec<Node>) -> Result<Vec<Node>, ErrKind> {
        let mut corrected = vec![];
        for expr in exprs.clone() {
            if let Expr::Import { .. } = expr.expr {
                corrected.push(expr);
            } else {
                corrected.push(self.correct(expr)?);
            }
        }

        Ok(corrected)
    }

    fn correct(&mut self, expr: Node) -> Result<Node, ErrKind> {
        let matc = expr.clone();
        match matc.expr {
            Expr::BinaryExpr { left, right, op } => {
                let left = self.correct(*left)?;
                let right = self.correct(*right)?;

                if &left.ty != &right.ty {
                    return self.analyz_binary_expr(left, right, op);
                }
            }
            Expr::FnCall { name, args } => {
                let name = Box::new(self.correct(*name)?);
                let mut corrected_args = vec![];
                for arg in args {
                    let arg = self.correct(arg)?;
                    if arg.ty != ConstType::Dynamic {
                        corrected_args.push(Node {
                            expr: Expr::As(Box::new(arg)),
                            ty: ConstType::Dynamic,
                        });
                    } else {
                        corrected_args.push(arg);
                    }
                }

                if let ConstType::Func(ty, _) = name.ty.clone() {
                    return Ok(Node {
                        expr: Expr::FnCall {
                            name,
                            args: corrected_args,
                        },
                        ty: *ty,
                    });
                } else {
                    panic!()
                }
            }

            Expr::Id(id) => {
                let ty = if self.env.has(&id) {
                    self.env.get_ty(&id).unwrap()
                } else {
                    self.env.add(&id, expr.ty.clone());
                    expr.ty
                };
                return Ok(Node {
                    ty,
                    expr: Expr::Id(id),
                });
            }

            Expr::Func {
                ret,
                name,
                args,
                body,
            } => {
                self.env.current = ret.clone();
                let body = self.correct_prog(body.clone())?;
                self.env.current = ConstType::Void;
                return Ok(Node {
                    expr: Expr::Func {
                        ret,
                        name,
                        args,
                        body,
                    },
                    ty: expr.ty,
                });
            }

            Expr::IfExpr {
                condition,
                body,
                alt,
            } => {
                let cond = self.correct(*condition)?;
                let body = self.correct_prog(body)?;
                let alt = if alt.is_some() {
                    Some(Box::new(self.correct(*alt.unwrap())?))
                } else {
                    None
                };
                return Ok(Node {
                    expr: Expr::IfExpr {
                        condition: Box::new(cond),
                        body,
                        alt,
                    },
                    ty: expr.ty,
                });
            }

            Expr::WhileExpr { condition, body } => {
                let cond = self.correct(*condition)?;
                let body = self.correct_prog(body)?;

                return Ok(Node {
                    expr: Expr::WhileExpr {
                        condition: Box::new(cond),
                        body,
                    },
                    ty: expr.ty,
                });
            }

            Expr::As(old) => {
                let old = self.correct(*old)?;
                return Ok(Node {
                    expr: Expr::As(Box::new(old)),
                    ty: expr.ty,
                });
            }
            Expr::RetExpr(ret) => {
                let ret = Box::new(self.correct(*ret)?);
                if ret.ty == self.env.current.clone() {
                    return Ok(Node {
                        expr: Expr::RetExpr(ret.clone()),
                        ty: ret.ty,
                    });
                } else {
                    return Ok(Node {
                        ty: self.env.current.clone(),
                        expr: Expr::RetExpr(Box::new(Node {
                            expr: Expr::As(ret),
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
