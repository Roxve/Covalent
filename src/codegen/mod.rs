mod tools;
use crate::codegen::tools::*;
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
        &self,
        name: &str,
        ty: T,
    ) -> PointerValue<'ctx>;
    // fn compile_var_assign(&mut self, var: Ident, value: Expr) -> Result<BasicValueEnum<'ctx>, i8>;
    // fn compile_var_declare(&mut self, var: Ident, value: Expr) -> Result<BasicValueEnum<'ctx>, i8>;

    // fn compile_fn(
    //     &mut self,
    //     name: String,
    //     args_names: Vec<String>,
    //     types: Vec<BasicMetadataTypeEnum<'ctx>>,
    //     body: Vec<Expr>,
    // );
    // fn compile_fn_call(&mut self, name: Ident, args: Vec<Expr>)
    //     -> Result<BasicValueEnum<'ctx>, i8>;
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
        let lhs = self.compile(*left)?;
        let rhs = self.compile(*right)?;

        let left = self.mk_int(lhs.get_field_at_index(0).unwrap().into_array_value());
        let right = self.mk_int(rhs.get_field_at_index(0).unwrap().into_array_value());
        let value = self.use_int(self.builder.build_int_add(left, right, "iadd").unwrap());

        Ok(value)
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
}
