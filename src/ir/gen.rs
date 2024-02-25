use super::{Const, ConstType, IROp};
use crate::{
    ast::{Expr, Literal},
    source::Source,
};

type IR = Vec<IROp>;
type IRRes = Result<&mut IR, u8>;

pub trait IRGen {
    fn gen(&mut self, op: &mut IR) -> IRRes;
    fn gen_prog(&mut self, exprs: Vec<Expr>) -> IR;
    fn gen_expr(&mut self, expr: Expr) -> IRRes;

    fn gen_binary_expr(&mut self, op: String, left: Expr, right: Expr) -> IRRes;
}

impl IRGen for Source {
    fn gen(&mut self, ops: &mut IR) -> IRRes {
        self.IR.append(ops);
        Ok(ops)
    }
    fn gen_prog(&mut self, exprs: Vec<Expr>) -> IR {
        for expr in exprs {
            let _ = self.gen_expr(expr);
        }
        self.IR.clone()
    }

    fn gen_expr(&mut self, expr: Expr) -> IRRes {
        match expr {
            Expr::Literal(Literal::Int(i)) => {
                Ok(&mut vec![IROp::Const(ConstType::Int, Const::Int(i))])
            }
            Expr::Literal(Literal::Float(f)) => {
                Ok(&mut vec![IROp::Const(ConstType::Float, Const::Float(f))])
            }
            Expr::BinaryExpr(op, left, right) => self.gen_binary_expr(op, *left, *right),
            _ => todo!(),
        }
    }

    fn gen_binary_expr(&mut self, op: String, left: Expr, right: Expr) -> IRRes {
        self.gen_expr(left)?;
        self.gen_expr(right)?;
        match op.as_str() {
            "+" => self.gen(IROp::Add),
            o => todo!("add op {}", o),
        }
    }
}
