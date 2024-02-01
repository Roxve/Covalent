use crate::ast::Expr;
use crate::source::*;

pub trait Codegen {
    fn codegen_prog(&mut self, exprs: Vec<Expr>);
    fn codegen(&mut self, expr: Expr);
    fn gen_binary_expr(&mut self, op: char, left: Expr, right: Expr);
}

impl Codegen for Source {
    fn codegen_prog(&mut self, exprs: Vec<Expr>) {
        for expr in exprs {
            self.codegen(expr);
        }
    }

    fn codegen(&mut self, expr: Expr) {
        match expr {
            Expr::Literal(literal) => {
                let ip = self.push_const(literal);
                self.push_instr(Op::Load(self.current_reg + 1, ip));
                self.current_reg += 1;
            }
            Expr::BinaryExpr(op, left, right) => {
                self.gen_binary_expr(op, *left, *right);
            }
        }
    }

    fn gen_binary_expr(&mut self, op: char, left: Expr, right: Expr) {
        self.codegen(left);
        self.codegen(right);

        match op {
            '+' => {
                self.push_instr(Op::Add(self.current_reg - 1, self.current_reg));
                self.current_reg -= 1;
            }
            '*' => {
                self.push_instr(Op::Mul(self.current_reg - 1, self.current_reg));
                self.current_reg -= 1;
            }
            _ => todo!(),
        }
    }
}
