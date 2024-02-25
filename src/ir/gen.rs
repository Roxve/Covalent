use super::{get_ops_type, Const, ConstType, IROp};
use crate::{
    ast::{Expr, Literal},
    source::Source,
};

type IR = Vec<IROp>;
type IRRes = Result<IR, u8>;

pub trait IRGen {
    fn gen_prog(&mut self, exprs: Vec<Expr>) -> IR;
    fn gen_expr(&mut self, expr: Expr) -> IRRes;

    fn gen_binary_expr(&mut self, op: String, left: Expr, right: Expr) -> IRRes;
}

impl IRGen for Source {
    fn gen_prog(&mut self, exprs: Vec<Expr>) -> IR {
        for expr in exprs {
            let g = self.gen_expr(expr);
            if g.is_ok() {
                self.IR.append(&mut g.unwrap());
            }
        }
        self.IR.clone()
    }

    fn gen_expr(&mut self, expr: Expr) -> IRRes {
        match expr {
            Expr::Literal(Literal::Int(i)) => Ok(vec![IROp::Const(ConstType::Int, Const::Int(i))]),
            Expr::Literal(Literal::Float(f)) => {
                Ok(vec![IROp::Const(ConstType::Float, Const::Float(f))])
            }
            Expr::BinaryExpr(op, left, right) => self.gen_binary_expr(op, *left, *right),
            _ => todo!(),
        }
    }

    fn gen_binary_expr(&mut self, op: String, left: Expr, right: Expr) -> IRRes {
        let mut res: IR = vec![];
        let mut lhs = self.gen_expr(left)?;
        let mut rhs = self.gen_expr(right)?;
        let ty;
        if get_ops_type(&lhs) != get_ops_type(&rhs) {
            // beform type conv
            let lhs_ty = get_ops_type(&lhs);
            let rhs_ty = get_ops_type(&rhs);
            if lhs_ty == ConstType::Float && rhs_ty == ConstType::Int {
                res.append(&mut lhs);
                res.append(&mut rhs);
                res.append(&mut vec![IROp::Conv(ConstType::Float)]);
                ty = lhs_ty;
            } else if lhs_ty == ConstType::Int && rhs_ty == ConstType::Float {
                res.append(&mut lhs);
                res.append(&mut vec![IROp::Conv(ConstType::Float)]);
                res.append(&mut rhs);
                ty = rhs_ty;
            } else {
                // NaN
                ty = lhs_ty;
            }
        } else {
            ty = get_ops_type(&lhs);
            res.append(&mut lhs);
            res.append(&mut rhs);
        }

        res.append(&mut vec![match op.as_str() {
            "+" => IROp::Add(ty),
            o => todo!("add op {}", o),
        }]);
        Ok(res)
    }
}
