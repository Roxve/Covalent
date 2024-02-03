macro_rules! extract {
    ($e: expr, $variant:path, $fields:tt) => {
        match $e {
            $variant($fields) => $fields,
            variant => panic!("unexcepted variant: {:?}", variant),
        }
    };
}

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

macro_rules! con_num {
    ($self: expr,$e: expr) => {
        match $e {
            RuntimeVal::Int(val) => {
                let i: i32 = val.to_string().replace("i32 ", "").parse().unwrap();
                let res: f32 = i as f32;
                RuntimeVal::Float($self.context.f32_type().const_float(res as f64))
            }
            RuntimeVal::Float(val) => {
                let f: f32 = val
                    .to_string()
                    .replace("float ", "")
                    .replace("e+", "")
                    .parse()
                    .unwrap();
                let res: i32 = f.round() as i32;
                RuntimeVal::Int($self.context.i32_type().const_int(res as u64, true))
            } // _ => todo!(),
        }
    };
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
            Expr::Literal(Literal::Float(f)) => Ok(RuntimeVal::Float(
                self.context.f32_type().const_float(f as f64),
            )),
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
                    if let RuntimeVal::Float(_) = rhs {
                        rhs = con_num!(self, rhs);
                    }

                    "int"
                }
                RuntimeVal::Float(_) => {
                    if let RuntimeVal::Int(_) = rhs {
                        rhs = con_num!(self, rhs);
                    }

                    "float"
                }
                _ => todo!(),
            }
        };
        match op.as_str() {
            "+" => match etype {
                "int" => {
                    let left: IntValue = extract!(lhs, RuntimeVal::Int, val);
                    let right: IntValue = extract!(rhs, RuntimeVal::Int, val);

                    let res = self.builder.build_int_add(left, right, "tmpadd").unwrap();
                    println!("{:#?}", res);
                    Ok(RuntimeVal::Int(res))
                }
                _ => todo!(),
            },

            "-" => match etype {
                "int" => {
                    let left = extract!(lhs, RuntimeVal::Int, val);
                    let right = extract!(lhs, RuntimeVal::Int, val);

                    Ok(RuntimeVal::Int(
                        self.builder.build_int_sub(left, right, "tmpmul").unwrap(),
                    ))
                }
                _ => todo!(),
            },

            "*" => match etype {
                "int" => {
                    let left = extract!(lhs, RuntimeVal::Int, val);
                    let right = extract!(lhs, RuntimeVal::Int, val);

                    Ok(RuntimeVal::Int(
                        self.builder.build_int_mul(left, right, "tmpmul").unwrap(),
                    ))
                }
                _ => todo!(),
            },

            "/" => match etype {
                "int" => {
                    let left = extract!(lhs, RuntimeVal::Int, val);
                    let right = extract!(lhs, RuntimeVal::Int, val);

                    Ok(RuntimeVal::Float(
                        self.builder
                            .build_float_div(
                                extract!(
                                    con_num!(self, RuntimeVal::Int(left)),
                                    RuntimeVal::Float,
                                    val
                                ),
                                extract!(
                                    con_num!(self, RuntimeVal::Int(right)),
                                    RuntimeVal::Float,
                                    val
                                ),
                                "tmpdiv",
                            )
                            .unwrap(),
                    ))
                }
                _ => todo!(),
            },
            _ => todo!(),
        }
    }
}
