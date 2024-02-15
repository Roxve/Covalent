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
    fn get_ty(&self, src: &Source<'ctx>) -> i8;
    fn get_value(&self, builder: &Source<'ctx>) -> ArrayValue<'ctx>;
    fn set_type(&mut self, ty: i8) -> Self;
    fn set_bytes(&mut self, bytes: ArrayValue<'ctx>) -> Self;
}

impl<'ctx> CovaLLVMObj<'ctx> for StructValue<'ctx> {
    // fix zeroinitiliazer use unwrap_or(self.zero()) when getting fields
    fn zero(&self) -> BasicValueEnum<'ctx> {
        self.get_type()
            .get_context()
            .i8_type()
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

    fn get_ty(&self, src: &Source<'ctx>) -> i8 {
        let val = self.get_field_at_index(1).unwrap_or(self.zero());
        if val.is_pointer_value() {
            let field =
                src.builder
                    .build_struct_gep(val.into_pointer_value(), 1, "gep")
                    .unwrap_or(src.context.i32_type().const_int(0, false).const_to_pointer(
                        src.context.i32_type().ptr_type(AddressSpace::default()),
                    ));
            let result = src
                .builder
                .build_load(field, "load_type")
                .unwrap()
                .into_int_value();
            return result.get_sign_extended_constant().unwrap_or(0) as i8;
        }
        val.into_int_value().get_sign_extended_constant().unwrap() as i8
    }
    fn get_value(&self, builder: &Source<'ctx>) -> ArrayValue<'ctx> {
        let val = self.get_field_at_index(0).unwrap_or(self.zero_arr());
        // handel variables...
        if val.is_pointer_value() {
            let field = builder
                .builder
                .build_struct_gep(val.into_pointer_value(), 0, "gep")
                .unwrap();

            let result = builder
                .builder
                .build_load(field, "load_bytes")
                .unwrap()
                .into_array_value();
            return result;
        }

        return val.into_array_value();
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
    pub fn build_mk_float(&mut self) {
        let fn_ty = self
            .context
            .f32_type()
            .fn_type(&[self.context.i8_type().array_type(4).into()], false);
        let func = self.module.add_function("mk_float", fn_ty, None);
        let builder = self.context.create_builder();
        let arr = func.get_nth_param(0).unwrap();
        arr.set_name("arr");

        let entry = self.context.append_basic_block(func, "entry");
        let _ = builder.position_at_end(entry);
        let alloca = { builder.build_alloca(arr.get_type(), "arr").unwrap() };

        let _ = builder.build_store(alloca, arr);

        let mut result = self.context.i32_type().const_zero();
        for i in 0..4 {
            let byte = builder
                .build_extract_value(arr.into_array_value(), i, "extract")
                .unwrap()
                .into_int_value();
            let byte32 = builder
                .build_int_z_extend_or_bit_cast(byte, self.context.i32_type(), "iextend")
                .unwrap();

            let shifted = builder
                .build_left_shift(
                    byte32,
                    self.context.i32_type().const_int((i * 8) as u64, false),
                    "lsh",
                )
                .unwrap();
            result = builder.build_or(result, shifted, "OR").unwrap();
        }
        let _ = builder.build_return(Some(
            &builder
                .build_bitcast(result, self.context.f32_type(), "fcast")
                .unwrap()
                .into_float_value(),
        ));
    }

    pub fn build_mk_int(&mut self) {
        let fn_ty = self
            .context
            .i32_type()
            .fn_type(&[self.context.i8_type().array_type(4).into()], false);
        let func = self.module.add_function("mk_int", fn_ty, None);

        let builder = self.context.create_builder();
        let arr = func.get_nth_param(0).unwrap();
        arr.set_name("arr");

        let entry = self.context.append_basic_block(func, "entry");
        let _ = builder.position_at_end(entry);
        // let alloca = { builder.build_alloca(arr.get_type(), "arr").unwrap() };

        // let _ = builder.build_store(alloca, arr);

        let mut result = self.context.i32_type().const_zero();
        for i in 0..4 {
            let byte = builder
                .build_extract_value(arr.into_array_value(), i, "extract")
                .unwrap()
                .into_int_value();
            let byte32 = builder
                .build_int_z_extend_or_bit_cast(byte, self.context.i32_type(), "iextend")
                .unwrap();

            let shifted = builder
                .build_left_shift(
                    byte32,
                    self.context.i32_type().const_int((i * 8) as u64, false),
                    "lsh",
                )
                .unwrap();
            result = builder.build_or(result, shifted, "OR").unwrap();
        }
        let _ = builder.build_return(Some(&result));
    }

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
                arr_type.const_zero(),
                ptr_type.const_null(),
            ),
            "float" => (
                arr_type.const_array(&obj.to_bytes(self.context).as_slice()),
                arr_type.const_int(1 as u64, true),
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
    pub fn build_use_int(&mut self) {
        let fn_ty = self
            .obj_type()
            .fn_type(&[self.context.i32_type().into()], false);
        let func = self.module.add_function("use_int", fn_ty, None);

        let builder = self.context.create_builder();
        let intv = func.get_nth_param(0).unwrap().into_int_value();
        intv.set_name("intv");

        let entry = self.context.append_basic_block(func, "entry");
        let _ = builder.position_at_end(entry);

        let mut bytes = vec![];
        for i in 0..4 {
            let shift = self.context.i32_type().const_int((i * 8) as u64, false);
            let shl = intv.const_shl(shift);

            let byte = shl.const_truncate(self.context.i8_type());

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
        let _ = builder.build_return(Some(&llvm_obj));
    }

    pub fn use_int(&mut self, val: IntValue<'ctx>) -> StructValue<'ctx> {
        let fun = self.module.get_function("use_int").unwrap();
        self.builder
            .build_call(fun, &[val.into()], "iuse")
            .unwrap()
            .try_as_basic_value()
            .unwrap_left()
            .into_struct_value()
    }

    pub fn use_float(&mut self, val: FloatValue<'ctx>) -> StructValue<'ctx> {
        let mut bytes = vec![];

        let bit_cast_val = self
            .builder
            .build_bitcast(val, self.context.i32_type(), "icast")
            .unwrap()
            .into_int_value();
        for i in 0..4 {
            let shift = self
                .builder
                .build_int_s_extend_or_bit_cast(
                    self.context.i8_type().const_int((i * 8) as u64, false),
                    self.context.i32_type(),
                    "fcast",
                )
                .unwrap();

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
        let fun = self.module.get_function("mk_int").unwrap();
        self.builder
            .build_call(fun, &[val.into()], "int")
            .unwrap()
            .try_as_basic_value()
            .unwrap_left()
            .into_int_value()
    }

    pub fn mk_float(&mut self, val: ArrayValue<'ctx>) -> FloatValue<'ctx> {
        let fun = self.module.get_function("mk_float").unwrap();
        self.builder
            .build_call(fun, &[val.into()], "float")
            .unwrap()
            .try_as_basic_value()
            .unwrap_left()
            .into_float_value()
    }

    pub fn mk_val(&mut self, val: StructValue<'ctx>) -> BasicValueEnum<'ctx> {
        match val.get_ty(self) {
            0 => self.mk_int(val.get_value(self)).as_basic_value_enum(),
            1 => self.mk_float(val.get_value(self)).as_basic_value_enum(),
            _ => todo!("mk val for type"),
        }
    }
}
