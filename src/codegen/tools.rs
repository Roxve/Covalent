// use std::ffi::CStr;
// use std::fmt::Display;

use crate::source::Source;
use inkwell::context::Context;
use inkwell::types::StructType;
use inkwell::values::{ArrayValue, IntValue, StructValue};
use inkwell::AddressSpace;

// pub union Value {
//     pub int: i32,
//     pub float: f32,
//     pub bool: bool,
//     pub string: *const i8,
// }

// pub struct Object {
//     pub value: Value,
//     pub obj: i8,
// }

// impl Display for Value {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{}", self)
//     }
// }

// impl Display for Object {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self.obj {
//             0 => write!(f, "[\nint: {}\n]", unsafe { self.value.int }),
//             1 => write!(f, "[\nfloat: {}\n]", unsafe { self.value.float }),
//             2 => write!(f, "[\nbool: {}\n]", unsafe { self.value.bool }),
//             3 => write!(f, "[\nstring: {}\n]", unsafe {
//                 CStr::from_ptr(self.value.string as *const u8)
//                     .to_str()
//                     .unwrap()
//             }),
//             t => todo!("unknown value to display type {}", t),
//         }
//     }
// }
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

impl<'ctx> Source<'ctx> {
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
            "int" | "float" => (
                arr_type.const_array(&obj.to_bytes(self.context).as_slice()),
                int_type.const_zero(),
                ptr_type.const_null(),
            ),
            _ => todo!(),
        };

        self.obj_type()
            .const_named_struct(&[bytes.into(), ty.into(), str.into()])
    }

    // pub fn use_obj(&mut self, obj: Object) -> StructValue<'ctx> {
    //     let obj_type = self.obj_type();
    //     let val_bytes = {
    //         match obj.obj {
    //             0 => unsafe { obj.value.int.to_le_bytes() },
    //             1 => unsafe { obj.value.float.to_le_bytes() },
    //             _ => todo!(),
    //         }
    //     };
    //     let mut bytes = vec![];
    //     for byte in val_bytes {
    //         bytes.push(self.context.i8_type().const_int(byte as u64, false));
    //     }

    //     let llvm_obj = obj_type.const_named_struct(&[
    //         self.context.i8_type().const_array(&bytes).into(),
    //         self.context
    //             .i8_type()
    //             .const_int(obj.obj as u64, true)
    //             .into(),
    //         self.context
    //             .i8_type()
    //             .ptr_type(AddressSpace::default())
    //             .const_null()
    //             .into(),
    //     ]);
    //     llvm_obj
    // }
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
