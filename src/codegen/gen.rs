use crate::codegen::tools::any_type_to_basic;
use crate::source::Source;
use inkwell::types::BasicTypeEnum::*;
use inkwell::{FloatPredicate, IntPredicate};

use inkwell::values::{BasicValue, BasicValueEnum};

use super::tools::get_type_name;

pub trait Build<'ctx> {
    fn build_add(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;
    fn build_sub(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;
    fn build_mul(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;
    fn build_div(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;

    fn build_bigger_than(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;
    fn build_smaller_than(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;

    fn build_and(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;
    fn build_or(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;
    fn build_equals(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8>;
}

impl<'ctx> Build<'ctx> for Source<'ctx> {
    fn build_add(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        match lhs.get_type() {
            IntType(_) => Ok(self
                .builder
                .build_int_add(lhs.into_int_value(), rhs.into_int_value(), "iadd")
                .unwrap()
                .as_basic_value_enum()),
            FloatType(_) => Ok(self
                .builder
                .build_float_add(lhs.into_float_value(), rhs.into_float_value(), "fadd")
                .unwrap()
                .as_basic_value_enum()),
            PointerType(p) => match any_type_to_basic(p.get_element_type()) {
                IntType(i) => {
                    if i.get_bit_width() == 8 {
                        let strcat = self.module.get_function("strcat_ptr__i8_ptr__i8").unwrap();

                        return Ok(self
                            .builder
                            .build_call(strcat, &[lhs.into(), rhs.into()], "sadd")
                            .unwrap()
                            .try_as_basic_value()
                            .left()
                            .unwrap()
                            .as_basic_value_enum());
                    } else {
                        todo!()
                    }
                }
                _ => todo!(),
            },
            _ => todo!("add + for ..."),
        }
    }

    fn build_sub(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        match lhs.get_type() {
            IntType(_) => Ok(self
                .builder
                .build_int_sub(lhs.into_int_value(), rhs.into_int_value(), "isub")
                .unwrap()
                .as_basic_value_enum()),
            FloatType(_) => Ok(self
                .builder
                .build_float_sub(lhs.into_float_value(), rhs.into_float_value(), "fsub")
                .unwrap()
                .as_basic_value_enum()),
            _ => todo!("add - for ..."),
        }
    }

    fn build_mul(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        match lhs.get_type() {
            IntType(_) => Ok(self
                .builder
                .build_int_mul(lhs.into_int_value(), rhs.into_int_value(), "imul")
                .unwrap()
                .as_basic_value_enum()),
            FloatType(_) => Ok(self
                .builder
                .build_float_mul(lhs.into_float_value(), rhs.into_float_value(), "fmul")
                .unwrap()
                .as_basic_value_enum()),
            _ => todo!("add * for ..."),
        }
    }

    fn build_div(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        match lhs.get_type() {
            IntType(_) => {
                let left = self
                    .conv_into(lhs, self.context.f32_type().into())
                    .unwrap()
                    .into_float_value();
                let right = self
                    .conv_into(rhs, self.context.f32_type().into())
                    .unwrap()
                    .into_float_value();
                Ok(self
                    .builder
                    .build_float_div(left, right, "idiv")
                    .unwrap()
                    .as_basic_value_enum())
            }

            FloatType(_) => Ok(self
                .builder
                .build_float_div(lhs.into_float_value(), rhs.into_float_value(), "fdiv")
                .unwrap()
                .as_basic_value_enum()),
            _ => todo!(),
        }
    }

    fn build_bigger_than(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        match lhs.get_type() {
            IntType(_) => Ok(self
                .builder
                .build_int_compare(
                    IntPredicate::SGT,
                    lhs.into_int_value(),
                    rhs.into_int_value(),
                    "icmp",
                )
                .unwrap()
                .as_basic_value_enum()),
            FloatType(_) => Ok(self
                .builder
                .build_float_compare(
                    FloatPredicate::OGT,
                    lhs.into_float_value(),
                    rhs.into_float_value(),
                    "fcmp",
                )
                .unwrap()
                .as_basic_value_enum()),
            _ => todo!(),
        }
    }
    fn build_smaller_than(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        match lhs.get_type() {
            IntType(_) => Ok(self
                .builder
                .build_int_compare(
                    IntPredicate::SLT,
                    lhs.into_int_value(),
                    rhs.into_int_value(),
                    "icmp",
                )
                .unwrap()
                .as_basic_value_enum()),
            FloatType(_) => Ok(self
                .builder
                .build_float_compare(
                    FloatPredicate::OLT,
                    lhs.into_float_value(),
                    rhs.into_float_value(),
                    "fcmp",
                )
                .unwrap()
                .as_basic_value_enum()),
            _ => todo!(),
        }
    }

    fn build_and(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        if get_type_name(lhs.get_type()) != "_i1" {
            todo!("{}", get_type_name(lhs.get_type()))
        }

        Ok(self
            .builder
            .build_and(lhs.into_int_value(), rhs.into_int_value(), "and")
            .unwrap()
            .as_basic_value_enum())
    }
    fn build_or(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        if get_type_name(lhs.get_type()) != "_i1" {
            todo!("NaN")
        }

        Ok(self
            .builder
            .build_or(lhs.into_int_value(), rhs.into_int_value(), "or")
            .unwrap()
            .as_basic_value_enum())
    }

    fn build_equals(
        &mut self,
        lhs: BasicValueEnum<'ctx>,
        rhs: BasicValueEnum<'ctx>,
    ) -> Result<BasicValueEnum<'ctx>, i8> {
        match lhs.get_type() {
            IntType(_) => Ok(self
                .builder
                .build_int_compare(
                    IntPredicate::EQ,
                    lhs.into_int_value(),
                    rhs.into_int_value(),
                    "icmp",
                )
                .unwrap()
                .as_basic_value_enum()),
            FloatType(_) => Ok(self
                .builder
                .build_float_compare(
                    FloatPredicate::OEQ,
                    lhs.into_float_value(),
                    rhs.into_float_value(),
                    "fcmp",
                )
                .unwrap()
                .as_basic_value_enum()),
            _ => todo!("add cmp for ..."),
        }
    }
}