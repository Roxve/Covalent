use std::borrow::BorrowMut;

use crate::{
    ir::Enviroment,
    parser::ast::Expr,
    source::{ConstType, ErrKind, Ident},
};

use super::{AnalyzedExpr, Analyzer, TypedExpr};

pub fn analyzed_expr_to_expr(expr: AnalyzedExpr) -> Expr {
    match expr {
        _ => todo!(),
    }
}

impl Analyzer {
    pub fn correct(&mut self, exprs: Vec<TypedExpr>) -> Vec<TypedExpr> {
        todo!()
    }

    fn correct_expr(&mut self, expr: TypedExpr) -> Result<TypedExpr, ErrKind> {
        let matc = expr.clone();
        match matc.expr {
            AnalyzedExpr::BinaryExpr { left, right, op } => {
                if let AnalyzedExpr::FnCall { name, .. } = right.expr {
                    let ty = self.env.get_ty(&name).unwrap();
                    if ty != ConstType::Dynamic {
                        // convert it into smth than reanalyze

                        self.analyz(analyzed_expr_to_expr(expr.expr))
                    } else {
                        Ok(expr)
                    }
                } else {
                    Ok(expr)
                }
            }
            AnalyzedExpr::Func {
                ret,
                name,
                args,
                body,
            } => {
                let id = Ident {
                    val: name.clone(),
                    tag: None,
                };
                self.env.push_function(id, args.clone(), ret);
                let body = self.correct(body.clone());
                Ok(TypedExpr {
                    expr: AnalyzedExpr::Func {
                        ret,
                        name,
                        args,
                        body,
                    },
                    ty: expr.ty,
                })
            }
            _ => Ok(expr),
        }
    }
}
