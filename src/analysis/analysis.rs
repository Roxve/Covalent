use crate::parser::ast::Expr;
use crate::source::{ConstType, ErrKind};

use super::*;

#[inline]
fn ty_as(ty: &ConstType, expr: TypedExpr) -> TypedExpr {
    TypedExpr {
        expr: AnalyzedExpr::As(Box::new(expr)),
        ty: ty.clone(),
    }
}

impl Analyzer {
    pub fn analyz_prog(exprs: Vec<Expr>) -> Result<Vec<TypedExpr>, ErrKind> {
        let mut analyzer = Analyzer::new();
        let mut analyzed_prog = Vec::new();
        for expr in exprs {
            let analyzed_expr = analyzer.analyz(expr)?;
            analyzed_prog.push(analyzed_expr);
        }
        Ok(analyzed_prog)
    }

    pub fn analyz(&mut self, expr: Expr) -> Result<TypedExpr, ErrKind> {
        match expr {
            Expr::BinaryExpr { op, left, right } => self.analyz_binary_expr(*left, *right, op),
            _ => todo!("add typed expr {:?}", expr),
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

        if lhs.ty != rhs.ty {
            if lhs.ty == ConstType::Float && rhs.ty == ConstType::Int {
                rhs = ty_as(&lhs.ty, rhs);
            } else if lhs.ty == ConstType::Int && rhs.ty == ConstType::Float {
                lhs = ty_as(&rhs.ty, lhs);
            }
        }
        let ty = lhs.ty.clone();
        let left = Box::new(lhs);
        let right = Box::new(rhs);

        let expr = AnalyzedExpr::BinaryExpr { op, left, right };
        Ok(TypedExpr { expr, ty })
    }
}
