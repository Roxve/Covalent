use inkwell::basic_block::BasicBlock;
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
    fn compile_prog(&mut self, body: Vec<Expr>, main: BasicBlock) -> Result<RuntimeVal<'ctx>, i8>;
    fn compile(&mut self, expr: Expr) -> Result<RuntimeVal<'ctx>, i8>;
    fn compile_binary_expr(
        &mut self,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<RuntimeVal<'ctx>, i8>;
}

impl<'ctx> Codegen<'ctx> for Source<'ctx> {
    fn compile_prog(&mut self, body: Vec<Expr>, main: BasicBlock) -> Result<RuntimeVal<'ctx>, i8> {
        let mut result: Result<RuntimeVal<'ctx>, i8> = Err(-1);

        self.builder.position_at_end(main);

        for expr in body {
            result = self.compile(expr.clone());
        }
        return result;
    }

    fn compile(&mut self, expr: Expr) -> Result<RuntimeVal<'ctx>, i8> {
        match expr {
            Expr::Literal(Literal::Int(nb)) => {
                Ok(mkint(self.context.i32_type().const_int(nb as u64, true)))
            }
            Expr::BinaryExpr(op, left, right) => self.compile_binary_expr(op, left, right),
            _ => todo!(),
        }
    }

    fn compile_binary_expr(
        &mut self,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<RuntimeVal<'ctx>, i8> {
        let lhs = self.compile(*left)?;
        let mut rhs = self.compile(*right)?;

        let etype = {
            match lhs {
                RuntimeVal::Int(_) => {
                    if let RuntimeVal::Float(nb) = rhs {
                        let num: i64 = nb.to_string().parse().unwrap();
                        rhs = RuntimeVal::Int(self.context.i32_type().const_int(num as u64, true));
                    }

                    "int"
                }
                _ => todo!(),
            }
        };
        match op.as_str() {
            "+" => match etype {
                "int" => {
                    let mut left: Option<IntValue> = None;
                    let mut right: Option<IntValue> = None;

                    if let RuntimeVal::Int(nb) = lhs {
                        left = Some(nb);
                    }

                    if let RuntimeVal::Int(nb) = rhs {
                        right = Some(nb);
                    }

                    let res = self.builder.build_int_add(
                        left.expect("Null left"),
                        right.expect("Null right"),
                        "tmpadd",
                    );
                    println!("{:#?}", res);
                    Ok(RuntimeVal::Int(res.unwrap()))
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}
