macro_rules! extract {
    ($e: expr, $variant:path, $fields:tt) => {
        match $e {
            $variant($fields) => $fields,
        }
    };
}

use inkwell::types::BasicMetadataTypeEnum;
use inkwell::types::BasicType;
use inkwell::types::BasicTypeEnum;
use inkwell::values::*;

use crate::ast::Expr;
use crate::ast::Ident;
use crate::ast::Literal;
use crate::ast::Tag;

use crate::source::*;

pub trait Codegen<'ctx> {
    fn conv_into(
        &mut self,
        from: BasicValueEnum<'ctx>,
        into: BasicTypeEnum<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>>;

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

    fn compile_fn_declare(
        &mut self,
        name: Ident,
        args: Vec<Expr>,
        body: Vec<Expr>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;
    fn compile_fn_call(&mut self, name: Ident, args: Vec<Expr>)
        -> Result<BasicValueEnum<'ctx>, i8>;
}

impl<'ctx> Codegen<'ctx> for Source<'ctx> {
    fn conv_into(
        &mut self,
        from: BasicValueEnum<'ctx>,
        into: BasicTypeEnum<'ctx>,
    ) -> Option<BasicValueEnum<'ctx>> {
        if from.get_type() == into {
            return Some(from);
        }

        match from.get_type() {
            BasicTypeEnum::FloatType(_) => {
                if !into.is_int_type() {
                    // todo err here
                    return None;
                }
                return Some(
                    self.builder
                        .build_float_to_signed_int(
                            from.into_float_value(),
                            into.into_int_type(),
                            "fcon",
                        )
                        .unwrap()
                        .as_basic_value_enum(),
                );
            }

            BasicTypeEnum::IntType(_) => {
                if !into.is_float_type() {
                    return None;
                }

                return Some(
                    self.builder
                        .build_signed_int_to_float(
                            from.into_int_value(),
                            into.into_float_type(),
                            "icon",
                        )
                        .unwrap()
                        .as_basic_value_enum(),
                );
            }
            _ => {
                self.err(
                    ErrKind::CannotConvertRight,
                    "cannot convert right to left (usually in binary expressions)".to_string(),
                );

                None
            } // err
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
            Expr::FnDeclare(id, args, body) => self.compile_fn_declare(id, args, body),
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

    fn compile_fn_declare(
        &mut self,
        name: Ident,
        args: Vec<Expr>,
        body: Vec<Expr>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        let mut types: Vec<BasicMetadataTypeEnum> = vec![];
        let mut names: Vec<String> = Vec::new();

        for arg in args.clone() {
            if let Expr::TaggedIdent(Tag(tag), Ident(id)) = arg {
                match tag.as_str() {
                    "int" => types.push(self.context.i32_type().into()),
                    "float" => types.push(self.context.f32_type().into()),
                    _ => todo!(),
                }
                names.push(id);
            } else {
                // invaild argument
                self.err(ErrKind::UnexceptedTokenE, "invaild argument functions must have typed args! (this is temp), arg should look like (type arg) ex: int a".to_string());
                return Err(-2);
            }
        }

        if body.len() == 0 {
            let fn_type = self.context.void_type().fn_type(types.as_slice(), false);
            let fn_val =
                self.module
                    .add_function(extract!(name, Ident, str).as_str(), fn_type, None);
            fn_val.verify(true);

            return Ok(self.context.i8_type().const_zero().as_basic_value_enum());
        } else {
            let fn_type = self.context.i32_type().fn_type(types.as_slice(), false);
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
                let arg_name = names[i].as_str();

                match arg.get_type() {
                    BasicTypeEnum::IntType(_) => arg.into_int_value().set_name(arg_name),
                    BasicTypeEnum::FloatType(_) => arg.into_float_value().set_name(arg_name),
                    _ => todo!(),
                }

                let alloca = self.create_entry_block_alloca(arg_name, arg.get_type());

                self.builder.build_store(alloca, arg).unwrap();
                self.variables.insert(arg_name.to_string(), alloca);
            }

            let mut res: Option<BasicValueEnum> = None;
            for expr in body.clone() {
                let e = self.compile(expr)?;
                res = Some(e);
            }

            // convert fn type to res (regenerate function)
            let full_fn_type = res.unwrap().get_type().fn_type(types.as_slice(), false);
            let full_fn =
                self.module
                    .add_function(extract!(name, Ident, str).as_str(), full_fn_type, None);

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
                let arg_name = names[i].as_str();

                match arg.get_type() {
                    BasicTypeEnum::IntType(_) => arg.into_int_value().set_name(arg_name),
                    BasicTypeEnum::FloatType(_) => arg.into_float_value().set_name(arg_name),
                    _ => todo!(),
                }

                let alloca = self.create_entry_block_alloca(arg_name, arg.get_type());

                self.builder.build_store(alloca, arg).unwrap();
                self.variables.insert(arg_name.to_string(), alloca);
            }

            for expr in body.clone() {
                let e = self.compile(expr)?;
                res = Some(e);
            }

            let _ = match res.unwrap().get_type().as_basic_type_enum() {
                BasicTypeEnum::IntType(_) => self
                    .builder
                    .build_return(Some(&res.unwrap().into_int_value())),
                BasicTypeEnum::FloatType(_) => self
                    .builder
                    .build_return(Some(&res.unwrap().into_float_value())),
                _ => todo!(),
            };

            self.fn_value.verify(true);

            self.variables.clear();
            self.variables = old_vars;

            let prev_entry = prev.get_first_basic_block().unwrap();
            self.fn_value = prev;

            self.builder.position_at_end(prev_entry);

            return Ok(res.unwrap().as_basic_value_enum());
        }
    }

    fn compile_fn_call(
        &mut self,
        name: Ident,
        args: Vec<Expr>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        let fn_name = extract!(&name, Ident, str).as_str();
        let fun = self.module.get_function(&fn_name);
        match fun {
            Some(f) => {
                if f.count_params() != args.len() as u32 {
                    return Err(ErrKind::UnexceptedArgs as i8);
                }
                let mut compiled_args = Vec::with_capacity(args.len());

                for arg in args {
                    compiled_args.push(self.compile(arg)?);
                }

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
                    _ => todo!(),
                }
            }
            None => todo!(),
        }
    }
}
