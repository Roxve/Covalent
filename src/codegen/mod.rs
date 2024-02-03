use inkwell::values::*;

use crate::ast::Expr;
use crate::ast::Literal;
use crate::source::*;

#[derive(Debug)]
pub enum RuntimeVal<'ctx> {
    Int(IntValue<'ctx>),
    Float(FloatValue<'ctx>),
}

fn mkint(val: IntValue) -> RuntimeVal {
    return RuntimeVal::Int(val);
}

pub trait Codegen<'ctx> {
    fn codegen_prog(&mut self, body: Vec<Expr>) -> Result<RuntimeVal<'ctx>, i8>;
    fn codegen_expr(&mut self, expr: Expr) -> Result<RuntimeVal<'ctx>, i8>;
}

impl<'ctx> Codegen<'ctx> for Source<'ctx> {
    fn codegen_prog(&mut self, body: Vec<Expr>) -> Result<RuntimeVal<'ctx>, i8> {
        let mut result: Result<RuntimeVal<'ctx>, i8> = Err(-1);
        for expr in body {
            result = self.codegen_expr(expr.clone());
        }
        return result;
    }

    fn codegen_expr(&mut self, expr: Expr) -> Result<RuntimeVal<'ctx>, i8> {
        match expr {
            Expr::Literal(Literal::Int(nb)) => {
                Ok(mkint(self.context.i32_type().const_int(nb as u64, true)))
            }
            _ => todo!(),
        }
    }
}
