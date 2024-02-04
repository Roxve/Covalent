macro_rules! extract {
    ($e: expr, $variant:path, $fields:tt) => {
        match $e {
            $variant($fields) => $fields,
            variant => panic!("unexcepted variant: {:?}", variant),
        }
    };
}

use inkwell::types::BasicType;
use inkwell::values::*;

use crate::ast::Expr;
use crate::ast::Ident;
use crate::ast::Literal;

use crate::source::*;

pub trait Codegen<'ctx> {
    fn con_num(&mut self, val: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx>;
    fn compile_prog(&mut self, body: Vec<Expr>) -> Result<BasicValueEnum<'ctx>, i8>;
    fn compile(&mut self, expr: Expr) -> Result<BasicValueEnum<'ctx>, i8>;
    fn compile_binary_expr(
        &mut self,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;

    fn create_entry_block_alloca<T: BasicType<'ctx>>(
        &self,
        name: &str,
        ty: T,
    ) -> PointerValue<'ctx>;
    fn compile_var_assign(&mut self, var: Ident, value: Expr) -> Result<BasicValueEnum<'ctx>, i8>;
    fn compile_var_declare(&mut self, var: Ident, value: Expr) -> Result<BasicValueEnum<'ctx>, i8>;
}

impl<'ctx> Codegen<'ctx> for Source<'ctx> {
    fn con_num(&mut self, val: BasicValueEnum<'ctx>) -> BasicValueEnum<'ctx> {
        match val {
            BasicValueEnum::IntValue(nb) => BasicValueEnum::FloatValue(
                self.builder
                    .build_signed_int_to_float(nb, self.context.f32_type(), "fcon")
                    .unwrap(),
            ),
            BasicValueEnum::FloatValue(nb) => BasicValueEnum::IntValue(
                self.builder
                    .build_float_to_signed_int(nb, self.context.i32_type(), "icon")
                    .unwrap(),
            ),
            _ => todo!(),
        }
    }

    fn compile_prog(&mut self, body: Vec<Expr>) -> Result<BasicValueEnum<'ctx>, i8> {
        let mut result: Result<BasicValueEnum<'ctx>, i8> = Err(-1);

        for expr in body {
            result = self.compile(expr.clone());
        }
        return result;
    }

    fn compile(&mut self, expr: Expr) -> Result<BasicValueEnum<'ctx>, i8> {
        match expr {
            Expr::Literal(Literal::Int(nb)) => {
                Ok(self.context.i32_type().const_int(nb as u64, true).into())
            }
            Expr::Literal(Literal::Float(f)) => {
                Ok(self.context.f32_type().const_float(f as f64).into())
            }
            Expr::Ident(Ident(ref name)) => match self.variables.get(name) {
                Some(var) => Ok(self.builder.build_load(*var, name).unwrap()),
                None => Err(-2),
            },
            Expr::BinaryExpr(op, left, right) => self.compile_binary_expr(op, left, right),
            Expr::VarDeclare(id, value) => self.compile_var_declare(id, *value),
            Expr::VarAssign(id, value) => self.compile_var_assign(id, *value),
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
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        let mut lhs = self.compile(*left)?;
        let mut rhs = self.compile(*right)?;
        let etype = {
            match lhs {
                BasicValueEnum::IntValue(_) => {
                    if let BasicValueEnum::FloatValue(_) = rhs {
                        rhs = self.con_num(rhs);
                    }

                    "int"
                }
                BasicValueEnum::FloatValue(_) => {
                    if let BasicValueEnum::IntValue(_) = rhs {
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
                    let left: IntValue = lhs.into_int_value();
                    let right: IntValue = rhs.into_int_value();

                    let res = self.builder.build_int_add(left, right, "tmpadd").unwrap();
                    Ok(res.into())
                }

                "float" => {
                    let left: FloatValue = lhs.into_float_value();
                    let right: FloatValue = rhs.into_float_value();

                    Ok(self
                        .builder
                        .build_float_add(left, right, "tmpadd")
                        .unwrap()
                        .into())
                }
                _ => todo!(),
            },

            "-" => match etype {
                "int" => {
                    let left = lhs.into_int_value();
                    let right = rhs.into_int_value();

                    Ok(self
                        .builder
                        .build_int_sub(left, right, "tmpsub")
                        .unwrap()
                        .into())
                }

                "float" => {
                    let left: FloatValue = lhs.into_float_value();
                    let right: FloatValue = rhs.into_float_value();

                    Ok(self
                        .builder
                        .build_float_sub(left, right, "tmpsub")
                        .unwrap()
                        .into())
                }
                _ => todo!(),
            },

            "*" => match etype {
                "int" => {
                    let left = lhs.into_int_value();
                    let right = rhs.into_int_value();

                    Ok(self
                        .builder
                        .build_int_mul(left, right, "tmpmul")
                        .unwrap()
                        .into())
                }

                "float" => {
                    let left: FloatValue = lhs.into_float_value();
                    let right: FloatValue = rhs.into_float_value();

                    Ok(self
                        .builder
                        .build_float_mul(left, right, "tmpmul")
                        .unwrap()
                        .into())
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

                let left = lhs.into_float_value();
                let right = rhs.into_float_value();
                Ok(self
                    .builder
                    .build_float_div(left, right, "tmpdiv")
                    .unwrap()
                    .into())
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

    fn compile_var_assign(&mut self, var: Ident, value: Expr) -> Result<BasicValueEnum<'ctx>, i8> {
        let var_name = extract!(var, Ident, str);
        if !self.variables.contains_key(&var_name) {
            return Err(-2);
        }

        let val = self.compile(value.clone())?;

        if self
            .variables
            .get(&var_name)
            .unwrap()
            .get_type()
            .as_basic_type_enum()
            != val.get_type()
        {
            self.variables.remove(&var_name);
            return self.compile_var_declare(Ident(var_name), value);
        }

        let _ = self
            .builder
            .build_store(*self.variables.get(&var_name).unwrap(), val);
        return Ok(val);
    }

    fn compile_var_declare(&mut self, var: Ident, value: Expr) -> Result<BasicValueEnum<'ctx>, i8> {
        let var_name = extract!(var, Ident, str);

        if self.variables.contains_key(&var_name) {
            return Err(-2);
        }

        let init = self.compile(value)?;
        let tt = init.get_type();
        let alloca = self.create_entry_block_alloca(&var_name, tt);
        let _ = self.builder.build_store(alloca, init);

        self.variables.insert(var_name, alloca);

        return Ok(init);
    }
}
