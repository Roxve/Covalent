// use std::ffi::CStr;
// use std::fmt::Display;

use crate::source::ErrKind;
use crate::source::Source;
use inkwell::context::Context;
use inkwell::types::{AnyTypeEnum, BasicTypeEnum, StructType};
use inkwell::values::{ArrayValue, BasicValue, BasicValueEnum, FloatValue, IntValue, StructValue};
use inkwell::AddressSpace;

pub fn any_type_to_basic(ty: AnyTypeEnum) -> BasicTypeEnum {
    match ty {
        AnyTypeEnum::PointerType(p) => BasicTypeEnum::PointerType(p),
        AnyTypeEnum::IntType(i) => BasicTypeEnum::IntType(i),
        AnyTypeEnum::FloatType(f) => BasicTypeEnum::FloatType(f),
        AnyTypeEnum::ArrayType(t) => BasicTypeEnum::ArrayType(t),
        _ => todo!(),
    }
}

pub fn get_type_name(ty: BasicTypeEnum) -> String {
    match ty {
        BasicTypeEnum::IntType(i) => format!("_i{}", i.get_bit_width()),
        BasicTypeEnum::FloatType(_) => format!("_float"),
        BasicTypeEnum::ArrayType(t) => {
            "_arr_".to_owned() + get_type_name(t.get_element_type()).as_str()
        }
        BasicTypeEnum::PointerType(ref t) => {
            let el = t.get_element_type();
            let bel = &any_type_to_basic(el);
            let s = "_ptr_".to_owned() + get_type_name(bel.to_owned()).as_str();
            s
        }
        _ => todo!(),
    }
}
// making rust work with covalent
pub trait CovaObj<'ctx> {
    fn get_type(&self) -> &str;
    fn to_bytes(&self, ctx: &'ctx Context) -> Vec<IntValue<'ctx>>;
}

impl<'ctx> CovaObj<'ctx> for i32 {
    fn get_type(&self) -> &str {
        "int"
    }

    fn to_bytes(&self, ctx: &'ctx Context) -> Vec<IntValue<'ctx>> {
        let bytes = self.to_le_bytes().to_vec();
        let mut bytes_val = vec![];

        for byte in bytes {
            bytes_val.push(ctx.i8_type().const_int(byte as u64, false));
        }
        bytes_val
    }
}

impl<'ctx> CovaObj<'ctx> for f32 {
    fn get_type(&self) -> &str {
        "float"
    }

    fn to_bytes(&self, ctx: &'ctx Context) -> Vec<IntValue<'ctx>> {
        let bytes = self.to_le_bytes().to_vec();
        let mut bytes_val = vec![];

        for byte in bytes {
            bytes_val.push(ctx.i8_type().const_int(byte as u64, false));
        }
        bytes_val
    }
}

pub trait CovaLLVMObj<'ctx> {
    fn zero(&self) -> BasicValueEnum<'ctx>;
    fn zero_arr(&self) -> BasicValueEnum<'ctx>;
    fn null(&self) -> BasicValueEnum<'ctx>;
    fn get_ty(&self) -> i8;
    fn get_value(&self) -> ArrayValue<'ctx>;
    fn set_type(&mut self, ty: i8) -> Self;
    fn set_bytes(&mut self, bytes: ArrayValue<'ctx>) -> Self;
}

impl<'ctx> CovaLLVMObj<'ctx> for StructValue<'ctx> {
    // fix zeroinitiliazer use unwrap_or(self.zero()) when getting fields
    fn zero(&self) -> BasicValueEnum<'ctx> {
        self.get_type()
            .get_context()
            .i32_type()
            .const_zero()
            .as_basic_value_enum()
    }

    fn zero_arr(&self) -> BasicValueEnum<'ctx> {
        self.get_type()
            .get_context()
            .i8_type()
            .const_array(&[
                self.zero().into_int_value(),
                self.zero().into_int_value(),
                self.zero().into_int_value(),
                self.zero().into_int_value(),
            ])
            .as_basic_value_enum()
    }

    fn null(&self) -> BasicValueEnum<'ctx> {
        self.get_type()
            .get_context()
            .i8_type()
            .ptr_type(AddressSpace::default())
            .const_null()
            .as_basic_value_enum()
    }

    fn get_ty(&self) -> i8 {
        self.get_field_at_index(1)
            .unwrap_or(self.zero())
            .into_int_value()
            .get_sign_extended_constant()
            .unwrap() as i8
    }
    fn get_value(&self) -> ArrayValue<'ctx> {
        self.get_field_at_index(0)
            .unwrap_or(self.zero_arr())
            .into_array_value()
    }

    fn set_type(&mut self, ty: i8) -> Self {
        let ctx = self.get_type().get_context();
        let obj_type = self.get_type();
        return obj_type.const_named_struct(&[
            self.get_field_at_index(0).unwrap_or(self.zero_arr()),
            ctx.i8_type().const_int(ty as u64, true).into(),
            self.get_field_at_index(2).unwrap_or(self.null()),
        ]);
    }

    fn set_bytes(&mut self, bytes: ArrayValue<'ctx>) -> Self {
        let obj_type = self.get_type();
        return obj_type.const_named_struct(&[
            bytes.into(),
            self.get_field_at_index(1).unwrap_or(self.zero()),
            self.get_field_at_index(2).unwrap_or(self.null()),
        ]);
    }
}

impl<'ctx> Source<'ctx> {
    pub fn conv_into(
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

    pub fn obj_type(&mut self) -> StructType<'ctx> {
        self.context.struct_type(
            &[
                self.context.i8_type().array_type(4).into(),
                self.context.i8_type().into(),
                self.context
                    .i8_type()
                    .ptr_type(AddressSpace::default())
                    .into(),
            ],
            false,
        )
    }
    pub fn mk_obj<T: CovaObj<'ctx>>(&mut self, obj: T) -> StructValue<'ctx> {
        let ptr_type = self.context.i8_type().ptr_type(AddressSpace::default());
        let int_type = self.context.i32_type();
        let arr_type = self.context.i8_type();

        let (bytes, ty, str) = match obj.get_type() {
            "int" => (
                arr_type.const_array(&obj.to_bytes(self.context).as_slice()),
                int_type.const_zero(),
                ptr_type.const_null(),
            ),
            "float" => (
                arr_type.const_array(&obj.to_bytes(self.context).as_slice()),
                int_type.const_int(1 as u64, true),
                ptr_type.const_null(),
            ),
            _ => todo!(),
        };

        self.obj_type()
            .const_named_struct(&[bytes.into(), ty.into(), str.into()])
    }

    pub fn mk_basic_obj(&mut self, obj: BasicValueEnum<'ctx>) -> StructValue<'ctx> {
        match obj.get_type() {
            BasicTypeEnum::IntType(_) => self.use_int(obj.into_int_value()),
            BasicTypeEnum::FloatType(_) => self.use_float(obj.into_float_value()),
            _ => todo!("basic type to obj"),
        }
    }

    pub fn use_int(&mut self, val: IntValue<'ctx>) -> StructValue<'ctx> {
        let mut bytes = vec![];

        for i in 0..4 {
            let shift = self.context.i8_type().const_int((i * 8) as u64, false);
            let byte = val.const_shl(shift).const_truncate(self.context.i8_type());
            bytes.push(byte);
        }
        let array = self.context.i8_type().const_array(&bytes);

        let obj_type = self.obj_type();
        let llvm_obj = obj_type.const_named_struct(&[
            array.into(),
            self.context.i8_type().const_zero().into(),
            self.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .const_null()
                .into(),
        ]);
        llvm_obj
    }

    pub fn use_float(&mut self, val: FloatValue<'ctx>) -> StructValue<'ctx> {
        let mut bytes = vec![];

        let bit_cast_val = self
            .builder
            .build_bitcast(val, self.context.i32_type(), "icast")
            .unwrap()
            .into_int_value();
        for i in 0..4 {
            let shift = self.context.i8_type().const_int((i * 8) as u64, false);

            //idk why it works **i think**
            let shifted_byte = self
                .builder
                .build_right_shift(bit_cast_val, shift, false, "shr")
                .unwrap();
            let byte = shifted_byte.const_truncate(self.context.i8_type());
            bytes.push(byte.clone());
        }
        let array = self.context.i8_type().const_array(&bytes);

        let obj_type = self.obj_type();
        let llvm_obj = obj_type.const_named_struct(&[
            array.into(),
            self.context.i8_type().const_int(1, true).into(),
            self.context
                .i8_type()
                .ptr_type(AddressSpace::default())
                .const_null()
                .into(),
        ]);
        llvm_obj
    }

    pub fn mk_int(&mut self, val: ArrayValue<'ctx>) -> IntValue<'ctx> {
        let mut result = self.context.i32_type().const_zero();

        for i in 0..val.get_type().len() {
            let byte = self
                .builder
                .build_extract_value(val, i, "byte")
                .unwrap()
                .into_int_value();
            let byte_as32 = self
                .builder
                .build_int_z_extend_or_bit_cast(byte, self.context.i32_type(), "cast")
                .unwrap();
            let shifted_byte = self
                .builder
                .build_left_shift(
                    byte_as32,
                    self.context.i32_type().const_int((i * 8) as u64, false),
                    "shift",
                )
                .unwrap();
            result = self.builder.build_or(result, shifted_byte, "OR").unwrap();
        }
        result
    }
    pub fn mk_float(&mut self, val: ArrayValue<'ctx>) -> FloatValue<'ctx> {
        let mut result = self.context.i32_type().const_zero();

        for i in 0..val.get_type().len() {
            let byte = self
                .builder
                .build_extract_value(val, i, "byte")
                .unwrap()
                .into_int_value();
            let byte_as32 = self
                .builder
                .build_int_z_extend_or_bit_cast(byte, self.context.i32_type(), "cast")
                .unwrap();
            let shifted_byte = self
                .builder
                .build_left_shift(
                    byte_as32,
                    self.context.i32_type().const_int((i * 8) as u64, false),
                    "shift",
                )
                .unwrap();

            result = self.builder.build_or(result, shifted_byte, "OR").unwrap();
        }
        self.builder
            .build_bitcast(result, self.context.f32_type(), "fcast")
            .unwrap()
            .into_float_value()
    }

    pub fn mk_val(&mut self, val: StructValue<'ctx>) -> BasicValueEnum<'ctx> {
        match val.get_ty() {
            0 => self.mk_int(val.get_value()).as_basic_value_enum(),
            1 => self.mk_float(val.get_value()).as_basic_value_enum(),
            _ => todo!("mk val for type"),
        }
    }
}
