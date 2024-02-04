macro_rules! extract {
    ($e: expr, $variant:path, $fields:tt) => {
        match $e {
            $variant($fields) => $fields,
            variant => panic!("unexcepted variant: {:?}", variant),
        }
    };
}

use inkwell::basic_block::BasicBlock;

use inkwell::types::BasicType;
use inkwell::values::*;

use crate::ast::Expr;
use crate::ast::Ident;
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
    fn con_num(&mut self, val: RuntimeVal<'ctx>) -> RuntimeVal<'ctx>;
    fn compile_prog(&mut self, body: Vec<Expr>) -> Result<RuntimeVal<'ctx>, i8>;
    fn compile(&mut self, expr: Expr) -> Result<RuntimeVal<'ctx>, i8>;
    fn compile_binary_expr(
        &mut self,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<RuntimeVal<'ctx>, i8>;

    fn create_entry_block_alloca<T: BasicType<'ctx>>(
        &self,
        name: &str,
        ty: T,
    ) -> PointerValue<'ctx>;
    fn compile_var_declare(&mut self, var: Ident, value: Expr) -> Result<RuntimeVal<'ctx>, i8>;
}

impl<'ctx> Codegen<'ctx> for Source<'ctx> {
    fn con_num(&mut self, val: RuntimeVal<'ctx>) -> RuntimeVal<'ctx> {
        match val {
            RuntimeVal::Int(nb) => RuntimeVal::Float(
                self.builder
                    .build_signed_int_to_float(nb, self.context.f32_type(), "fcon")
                    .unwrap(),
            ),
            RuntimeVal::Float(nb) => RuntimeVal::Int(
                self.builder
                    .build_float_to_signed_int(nb, self.context.i32_type(), "icon")
                    .unwrap(),
            ), // _ => todo!(),
        }
    }

    fn compile_prog(&mut self, body: Vec<Expr>) -> Result<RuntimeVal<'ctx>, i8> {
        let mut result: Result<RuntimeVal<'ctx>, i8> = Err(-1);

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
            Expr::VarDeclare(id, value) => self.compile_var_declare(id, *value),
            e => {
                println!("{:#?}", e);
                todo!()
            }
        }
    }

    fn compile_binary_expr(
        &mut self,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<RuntimeVal<'ctx>, i8> {
        let mut lhs = self.compile(*left)?;
        let mut rhs = self.compile(*right)?;

        let etype = {
            match lhs {
                RuntimeVal::Int(_) => {
                    if let RuntimeVal::Float(_) = rhs {
                        rhs = self.con_num(rhs);
                    }

                    "int"
                }
                RuntimeVal::Float(_) => {
                    if let RuntimeVal::Int(_) = rhs {
                        rhs = self.con_num(rhs);
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
                    Ok(RuntimeVal::Int(res))
                }

                "float" => {
                    let left: FloatValue = extract!(lhs, RuntimeVal::Float, val);
                    let right: FloatValue = extract!(rhs, RuntimeVal::Float, val);

                    Ok(RuntimeVal::Float(
                        self.builder.build_float_add(left, right, "tmpadd").unwrap(),
                    ))
                }
                _ => todo!(),
            },

            "-" => match etype {
                "int" => {
                    let left = extract!(lhs, RuntimeVal::Int, val);
                    let right = extract!(rhs, RuntimeVal::Int, val);

                    Ok(RuntimeVal::Int(
                        self.builder.build_int_sub(left, right, "tmpsub").unwrap(),
                    ))
                }

                "float" => {
                    let left: FloatValue = extract!(lhs, RuntimeVal::Float, val);
                    let right: FloatValue = extract!(rhs, RuntimeVal::Float, val);

                    Ok(RuntimeVal::Float(
                        self.builder.build_float_sub(left, right, "tmpsub").unwrap(),
                    ))
                }
                _ => todo!(),
            },

            "*" => match etype {
                "int" => {
                    let left = extract!(lhs, RuntimeVal::Int, val);
                    let right = extract!(rhs, RuntimeVal::Int, val);

                    Ok(RuntimeVal::Int(
                        self.builder.build_int_mul(left, right, "tmpmul").unwrap(),
                    ))
                }

                "float" => {
                    let left: FloatValue = extract!(lhs, RuntimeVal::Float, val);
                    let right: FloatValue = extract!(rhs, RuntimeVal::Float, val);

                    Ok(RuntimeVal::Float(
                        self.builder.build_float_mul(left, right, "tmpmul").unwrap(),
                    ))
                }
                _ => todo!(),
            },

            "/" => {
                match etype {
                    "int" => {
                        lhs = self.con_num(lhs);
                        rhs = self.con_num(rhs);
                    }
                    _ => todo!(),
                }

                let left = extract!(lhs, RuntimeVal::Float, val);
                let right = extract!(rhs, RuntimeVal::Float, val);
                Ok(RuntimeVal::Float(
                    self.builder.build_float_div(left, right, "tmpdiv").unwrap(),
                ))
            }
            _ => todo!(),
        }
    }

    fn create_entry_block_alloca<T: BasicType<'ctx>>(
        &self,
        name: &str,
        ty: T,
    ) -> PointerValue<'ctx> {
        let builder = self.context.create_builder();

        let entry = self.fn_value.get_first_basic_block().unwrap();

        match entry.get_first_instruction() {
            Some(first_instr) => builder.position_before(&first_instr),
            None => builder.position_at_end(entry),
        }

        builder.build_alloca(ty, name).unwrap()
    }

    fn compile_var_declare(&mut self, var: Ident, value: Expr) -> Result<RuntimeVal<'ctx>, i8> {
        let var_name = extract!(var, Ident, str);

        if self.variables.contains_key(&var_name) {
            return Err(-2);
        }

        let init = self.compile(value)?;
        let alloca = {
            match init {
                RuntimeVal::Int(i) => {
                    let alloca =
                        self.create_entry_block_alloca(var_name.as_str(), self.context.i32_type());
                    self.builder.build_store(alloca, i);
                    alloca
                }
                RuntimeVal::Float(f) => {
                    let alloca =
                        self.create_entry_block_alloca(var_name.as_str(), self.context.f32_type());
                    self.builder.build_store(alloca, f);
                    alloca
                }
            }
        };

        self.variables.insert(var_name, alloca);

        return Ok(init);
    }
}
