macro_rules! extract {
    ($e: expr, $variant:path, $fields:tt) => {
        match $e {
            $variant($fields) => $fields,
        }
    };
}

mod tools;
use crate::codegen::tools::*;

use inkwell::types::BasicMetadataTypeEnum;
use inkwell::types::BasicType;
use inkwell::types::BasicTypeEnum;
use inkwell::values::*;

use crate::ast::Expr;
use crate::ast::Ident;
use crate::ast::Literal;

use crate::source::*;

pub trait Codegen<'ctx> {
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

    fn compile_fn(
        &mut self,
        name: String,
        args_names: Vec<String>,
        types: Vec<BasicMetadataTypeEnum<'ctx>>,
        body: Vec<Expr>,
    );
    fn compile_fn_call(&mut self, name: Ident, args: Vec<Expr>)
        -> Result<BasicValueEnum<'ctx>, i8>;
}

impl<'ctx> Codegen<'ctx> for Source<'ctx> {
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
            Expr::Literal(Literal::Str(s)) => Ok(self
                .builder
                .build_global_string_ptr(s.as_str(), "str")
                .unwrap()
                .as_pointer_value()
                .as_basic_value_enum()),

            Expr::Ident(Ident(ref name)) => match self.variables.get(name) {
                Some(var) => Ok(self.builder.build_load(*var, name).unwrap()),
                None => Err(-2),
            },

            Expr::BinaryExpr(op, left, right) => self.compile_binary_expr(op, left, right),

            Expr::VarDeclare(id, value) => self.compile_var_declare(id, *value),
            Expr::VarAssign(id, value) => self.compile_var_assign(id, *value),

            Expr::FnCall(id, args) => self.compile_fn_call(id, args),
            e => {
                println!("if you are a normal user please report this!, if you are a dev fix it!");
                dbg!(e);
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

        // we compile everything then terminate to make sure every error is encoutared
        rhs = self.conv_into(rhs, lhs.get_type()).unwrap_or(lhs);

        let etype = {
            match lhs {
                BasicValueEnum::IntValue(_) => "int",
                BasicValueEnum::FloatValue(_) => "float",
                BasicValueEnum::ArrayValue(v) => {
                    if v.is_const_string() {
                        "string"
                    } else {
                        todo!()
                    }
                }

                BasicValueEnum::PointerValue(ptr) => {
                    if get_type_name(ptr.get_type().as_basic_type_enum()) == "_ptr__i8" {
                        "string"
                    } else {
                        todo!()
                    }
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

                "string" => {
                    let _left = lhs.into_pointer_value();
                    let _right = rhs.into_pointer_value();

                    todo!("build add for strings and arrs")
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
                        lhs = self
                            .conv_into(lhs, self.context.f32_type().as_basic_type_enum())
                            .unwrap();
                        rhs = self
                            .conv_into(rhs, self.context.f32_type().as_basic_type_enum())
                            .unwrap();
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
            self.err(
                ErrKind::UndeclaredVar,
                format!("cannot assign to {} because its not declared", var_name),
            );
            return Err(ErrKind::UndeclaredVar as i8);
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
            self.err(ErrKind::VarAlreadyDeclared, format!("var {} already declared, covalent is dynamic so you can assign the var to a new type using the '=' operator.", var_name));
            return Err(ErrKind::VarAlreadyDeclared as i8);
        }

        let init = self.compile(value)?;
        let tt = init.get_type();
        let alloca = self.create_entry_block_alloca(&var_name, tt);
        let _ = self.builder.build_store(alloca, init);

        self.variables.insert(var_name, alloca);

        return Ok(init);
    }

    fn compile_fn(
        &mut self,
        name: String,
        arg_names: Vec<String>,
        args: Vec<BasicMetadataTypeEnum<'ctx>>,
        body: Vec<Expr>,
    ) {
        if body.len() == 0 {
            let fn_type = self.context.void_type().fn_type(args.as_slice(), false);
            let fn_val = self.module.add_function(name.as_str(), fn_type, None);
            fn_val.verify(true);
        } else {
            let fn_type = self.context.i32_type().fn_type(args.as_slice(), false);
            let fn_value = self.module.add_function("temp", fn_type, None);

            // func only vars

            let old_vars = self.variables.clone();

            self.variables.clear();
            self.variables.reserve(args.len());

            let entry = self.context.append_basic_block(fn_value, "entry");
            self.builder.position_at_end(entry);

            let prev = self.fn_value;

            self.fn_value = fn_value;

            for (i, arg) in fn_value.get_param_iter().enumerate() {
                let arg_name = arg_names[i].as_str();

                arg.set_name(arg_name);

                let alloca = self.create_entry_block_alloca(arg_name, arg.get_type());

                self.builder.build_store(alloca, arg).unwrap();
                self.variables.insert(arg_name.to_string(), alloca);
            }

            let mut result: Option<BasicValueEnum> = None;
            for expr in body.clone() {
                let e = self.compile(expr);
                if let Err(_) = e {
                    break;
                }
                result = Some(e.unwrap());
            }

            // convert fn type to res (regenerate function)
            let full_fn_type = result.unwrap().get_type().fn_type(args.as_slice(), false);

            let full_fn = self.module.add_function(name.as_str(), full_fn_type, None);

            let full_entry = self.context.append_basic_block(full_fn, "entry");
            self.builder.position_at_end(full_entry);

            self.variables.clear();

            self.fn_value = full_fn;
            for inst in entry.get_instructions() {
                inst.remove_from_basic_block();
            }
            unsafe {
                fn_value.delete();
            }

            for (i, arg) in full_fn.get_param_iter().enumerate() {
                let arg_name = arg_names[i].as_str();
                arg.set_name(arg_name);

                let alloca = self.create_entry_block_alloca(arg_name, arg.get_type());

                self.builder.build_store(alloca, arg).unwrap();
                self.variables.insert(arg_name.to_string(), alloca);
            }

            for expr in body.clone() {
                let e = self.compile(expr);
                if let Err(_) = e {
                    break;
                }
                result = Some(e.unwrap());
            }

            let _ = match result.unwrap().get_type().as_basic_type_enum() {
                BasicTypeEnum::IntType(_) => self
                    .builder
                    .build_return(Some(&result.unwrap().into_int_value())),
                BasicTypeEnum::FloatType(_) => self
                    .builder
                    .build_return(Some(&result.unwrap().into_float_value())),
                BasicTypeEnum::ArrayType(_) => self
                    .builder
                    .build_return(Some(&result.unwrap().into_array_value())),
                BasicTypeEnum::PointerType(_) => self
                    .builder
                    .build_return(Some(&result.unwrap().into_pointer_value())),
                _ => todo!(),
            };

            self.fn_value.verify(true);

            self.variables.clear();
            self.variables = old_vars;

            let prev_entry = prev.get_first_basic_block().unwrap();
            self.fn_value = prev;

            self.builder.position_at_end(prev_entry);
        }
    }

    fn compile_fn_call(
        &mut self,
        name: Ident,
        args: Vec<Expr>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        let mut compiled_args = Vec::with_capacity(args.len());

        for arg in args.clone() {
            compiled_args.push(self.compile(arg)?);
        }

        let fn_name = extract!(&name, Ident, str).as_str();
        let mut fn_typed_name = String::from(fn_name);

        for arg in compiled_args.clone() {
            fn_typed_name.push_str(get_type_name(arg.get_type()).as_str());
        }

        let fun = self.module.get_function(&fn_typed_name);
        match fun {
            Some(f) => {
                let argsv: Vec<BasicMetadataValueEnum> = compiled_args
                    .iter()
                    .by_ref()
                    .map(|&val| val.into())
                    .collect();

                match self
                    .builder
                    .build_call(f, argsv.as_slice(), "call")
                    .unwrap()
                    .try_as_basic_value()
                    .left()
                {
                    Some(val) => Ok(val),
                    None => Ok(self.context.bool_type().const_zero().as_basic_value_enum()),
                }
            }
            None => match self.get_function(fn_name.to_string()) {
                Some(f) => {
                    if f.args.len() != args.len() {
                        self.err(
                            ErrKind::UnexceptedArgs,
                            format!(
                                "unexcepted arg count for function {}\nexcepted {} args got {}",
                                f.get_name(),
                                f.args.len(),
                                args.len()
                            ),
                        );
                        return Err(ErrKind::UnexceptedArgs as i8);
                    }

                    let types: Vec<BasicMetadataTypeEnum> = compiled_args
                        .iter()
                        .map(|arg| arg.get_type().into())
                        .collect();

                    self.compile_fn(
                        fn_typed_name,
                        f.args.iter().map(|val| val.0.clone()).collect(),
                        types,
                        f.body,
                    );

                    return self.compile_fn_call(name, args);
                }
                None => todo!(),
            },
        }
    }
}
