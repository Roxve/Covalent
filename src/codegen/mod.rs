mod gen;
pub mod tools;
use self::gen::Build;
use crate::codegen::tools::*;

use inkwell::types::BasicMetadataTypeEnum;
use inkwell::types::BasicType;
use inkwell::values::*;

use crate::ast::Expr;
use crate::ast::Ident;
use crate::ast::Literal;

use crate::source::*;

pub trait Codegen<'ctx> {
    fn compile_prog(&mut self, body: Vec<Expr>) -> Result<StructValue<'ctx>, i8>;
    fn compile(&mut self, expr: Expr) -> Result<StructValue<'ctx>, i8>;
    fn compile_binary_expr(
        &mut self,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<StructValue<'ctx>, i8>;

    fn create_entry_block_alloca<T: BasicType<'ctx>>(
        &mut self,
        name: &str,
        ty: T,
    ) -> PointerValue<'ctx>;
    fn compile_var_assign(&mut self, var: Ident, value: Expr) -> Result<StructValue<'ctx>, i8>;
    fn compile_var_declare(&mut self, var: Ident, value: Expr) -> Result<StructValue<'ctx>, i8>;

    fn compile_fn(&mut self, func: Function) -> Result<FunctionValue<'ctx>, i8>;
    fn compile_fn_call(&mut self, name: Ident, args: Vec<Expr>) -> Result<StructValue<'ctx>, i8>;
}

impl<'ctx> Codegen<'ctx> for Source<'ctx> {
    fn compile_prog(&mut self, body: Vec<Expr>) -> Result<StructValue<'ctx>, i8> {
        let mut result: Result<StructValue<'ctx>, i8> = Err(-1);

        for expr in body {
            result = self.compile(expr.clone());
        }
        return result;
    }

    fn compile(&mut self, expr: Expr) -> Result<StructValue<'ctx>, i8> {
        match expr {
            Expr::Literal(Literal::Int(nb)) => Ok(self.mk_obj(nb)),
            Expr::Literal(Literal::Float(f)) => Ok(self.mk_obj(f)),
            Expr::Ident(Ident(ref name)) => match self.variables.get(name) {
                Some(var) => Ok(self
                    .builder
                    .build_load(*var, name)
                    .unwrap()
                    .into_struct_value()),
                None => {
                    self.err(ErrKind::UndeclaredVar, format!("undeclared var {}", name));
                    Err(ErrKind::UndeclaredVar as i8)
                }
            },
            Expr::BinaryExpr(op, left, right) => self.compile_binary_expr(op, left, right),
            Expr::VarDeclare(name, value) => self.compile_var_declare(name, *value),
            Expr::VarAssign(var, value) => self.compile_var_assign(var, *value),
            Expr::FnCall(name, args) => self.compile_fn_call(name, args),
            Expr::RetExpr(expr) => {
                let compiled = self.compile(*expr)?;
                let _ = self.builder.build_return(Some(&compiled));
                Ok(compiled)
            }
            e => {
                println!("if you are a normal user please report this!, if you are a dev fix it!");
                dbg!(e);
                todo!("the above expr ^^^")
            }
        }
    }

    fn compile_binary_expr(
        &mut self,
        op: String,
        left: Box<Expr>,
        right: Box<Expr>,
    ) -> Result<StructValue<'ctx>, i8> {
        let left = self.compile(*left)?;
        let right = self.compile(*right)?;

        let mut lhs = self.mk_val(left);
        let mut rhs = self.mk_val(right);
        let lhs_type = left.get_ty(self);
        let rhs_type = right.get_ty(self);

        if lhs_type != rhs_type {
            // 0 int 1 float
            if lhs_type == 0 && rhs_type == 1 {
                lhs = self
                    .builder
                    .build_signed_int_to_float(
                        lhs.into_int_value(),
                        rhs.get_type().into_float_type(),
                        "fcon",
                    )
                    .unwrap()
                    .as_basic_value_enum();
            } else if lhs_type == 1 && rhs_type == 0 {
                rhs = self
                    .builder
                    .build_signed_int_to_float(
                        rhs.into_int_value(),
                        lhs.get_type().into_float_type(),
                        "fcon",
                    )
                    .unwrap()
                    .as_basic_value_enum();
            }
        }

        let result = match op.as_str() {
            "+" => self.build_add(lhs, rhs),
            "-" => self.build_sub(lhs, rhs),
            "*" => self.build_mul(lhs, rhs),
            "/" => self.build_div(lhs, rhs),
            _ => todo!(),
        }?;
        Ok(self.mk_basic_obj(result))
    }

    fn create_entry_block_alloca<T: BasicType<'ctx>>(
        &mut self,
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

    fn compile_var_declare(&mut self, var: Ident, value: Expr) -> Result<StructValue<'ctx>, i8> {
        let var_name = var.0;

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

    fn compile_var_assign(&mut self, var: Ident, value: Expr) -> Result<StructValue<'ctx>, i8> {
        let var_name = var.0;
        if !self.variables.contains_key(&var_name) {
            self.err(
                ErrKind::UndeclaredVar,
                format!("cannot assign to {} because its not declared", var_name),
            );
            return Err(ErrKind::UndeclaredVar as i8);
        }

        let val = self.compile(value.clone())?;

        let _ = self
            .builder
            .build_store(*self.variables.get(&var_name).unwrap(), val);
        return Ok(val);
    }
    fn compile_fn(&mut self, func: Function) -> Result<FunctionValue<'ctx>, i8> {
        let mut args: Vec<BasicMetadataTypeEnum> = vec![];

        for _ in func.args.clone() {
            args.push(self.obj_type().into());
        }

        let func_ty = self.obj_type().fn_type(&args, false);
        let compiled = self
            .module
            .add_function(func.name.0.as_str(), func_ty, None);

        let old_vars = self.variables.clone();
        self.variables.clear();
        self.variables.reserve(func.args.clone().len());

        let prev_fn = self.fn_value;
        self.fn_value = compiled;

        let prev_block = self.builder.get_insert_block().unwrap();
        let entry = self.context.append_basic_block(compiled, "entry");
        self.builder.position_at_end(entry);

        // set func args names and alloc them
        let args_names: Vec<&str> = func.args.iter().map(|v| v.0.as_str()).collect();
        for (i, param) in compiled.get_param_iter().enumerate() {
            let name = args_names[i];
            param.set_name(name);

            let ty = self.obj_type();
            let alloc = self.create_entry_block_alloca(name, ty);
            let _ = self.builder.build_store(alloc, param);
            self.variables.insert(name.to_string(),alloc);
        }

        for expr in func.body {
            self.compile(expr)?;
        }

        self.fn_value = prev_fn;
        self.variables = old_vars;

        self.builder.position_at_end(prev_block);
        Ok(compiled)
    }

    fn compile_fn_call(&mut self, name: Ident, args: Vec<Expr>) -> Result<StructValue<'ctx>, i8> {
        let compiled_args: Vec<BasicMetadataValueEnum> = args
            .iter()
            .map(|arg| {
                self.compile(arg.clone())
                    .unwrap()
                    .as_basic_value_enum()
                    .into()
            })
            .collect();

        match self.module.get_function(name.clone().0.as_str()) {
            Some(func) => Ok(self
                .builder
                .build_call(func, &compiled_args, name.0.as_str())
                .unwrap()
                .try_as_basic_value()
                .unwrap_left()
                .into_struct_value()),

            None => match self.get_function(name.clone().0) {
                Some(f) => {
                    let func = self.compile_fn(f)?;
                    Ok(self
                        .builder
                        .build_call(func, &compiled_args, name.0.as_str())
                        .unwrap()
                        .try_as_basic_value()
                        .unwrap_left()
                        .into_struct_value())
                }

                None => {
                    self.err(
                        ErrKind::UndeclaredVar,
                        format!("undeclared function {}", name.0),
                    );
                    Err(ErrKind::UndeclaredVar as i8)
                }
            },
        }
    }
}
