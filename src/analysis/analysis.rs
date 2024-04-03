use crate::source::ErrKind;
use crate::parser::ast::Expr;

use super::Analyzer;
use super::TypedExpr;
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
         _ => Ok(TypedExpr {
                expr,
                rc: 0
            })
        }
    }
}