use std::ffi::CStr;
use std::fmt::Display;

use inkwell::types::{AnyTypeEnum, BasicTypeEnum};
use inkwell::values::{ArrayValue, BasicValue, BasicValueEnum, IntValue, StructValue};

use crate::source::{ErrKind, Source};

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
}

pub union Value {
    pub int: i32,
    pub float: f32,
    pub bool: bool,
    pub string: *const i8,
}

pub struct Object {
    pub value: Value,
    pub obj: i8,
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self)
    }
}

impl Display for Object {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.obj {
            0 => write!(f, "[\nint: {}\n]", unsafe { self.value.int }),
            1 => write!(f, "[\nfloat: {}\n]", unsafe { self.value.float }),
            2 => write!(f, "[\nbool: {}\n]", unsafe { self.value.bool }),
            3 => write!(f, "[\nstring: {}\n]", unsafe {
                CStr::from_ptr(self.value.string as *const u8)
                    .to_str()
                    .unwrap()
            }),
            t => todo!("unknown value to display type {}", t),
        }
    }
}

impl Object {
    pub fn new(obj: i8, value: Value) -> Self {
        Object { value, obj }
    }
}

impl<'ctx> Source<'ctx> {
    pub fn use_obj(&mut self, obj: Object) -> StructValue<'ctx> {
        let obj_type = self.context.struct_type(
            &[
                self.context.i8_type().array_type(4).into(),
                self.context.i8_type().into(),
            ],
            false,
        );
        let val_bytes = {
            match obj.obj {
                0 => unsafe { obj.value.int.to_le_bytes() },
                1 => unsafe { obj.value.float.to_le_bytes() },
                _ => todo!(),
            }
        };
        let mut bytes = vec![];
        for byte in val_bytes {
            bytes.push(self.context.i8_type().const_int(byte as u64, false));
        }

        let llvm_obj = obj_type.const_named_struct(&[
            self.context.i8_type().const_array(&bytes).into(),
            self.context
                .i8_type()
                .const_int(obj.obj as u64, true)
                .into(),
        ]);
        llvm_obj
    }
    pub fn use_int(&mut self, val: IntValue<'ctx>) -> StructValue<'ctx> {
        let mut bytes = vec![];

        for i in 0..4 {
            let shift = self.context.i8_type().const_int((i * 8) as u64, false);
            let byte = val.const_shl(shift).const_truncate(self.context.i8_type());
            bytes.push(byte);
        }
        let array = self.context.i8_type().const_array(&bytes);

        let obj_type = self.context.struct_type(
            &[
                self.context.i8_type().array_type(4).into(),
                self.context.i8_type().into(),
            ],
            false,
        );

        let llvm_obj = obj_type
            .const_named_struct(&[array.into(), self.context.i8_type().const_zero().into()]);
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
                .build_int_s_extend_or_bit_cast(byte, self.context.i32_type(), "cast")
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
}

impl Value {
    pub fn unpack_int(&self) -> i32 {
        unsafe { self.int }
    }

    pub fn unpack_float(&self) -> f32 {
        unsafe { self.float }
    }
}
