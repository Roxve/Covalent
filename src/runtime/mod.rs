use inkwell::{
    context::Context,
    module::Module,
    types::{BasicType, StructType},
    values::BasicValue,
    AddressSpace,
};

use crate::source::Source;

// the covalent runtime
// here we impl runtime stuff sush as runtime dynamic handling functions
const INT: u64 = 0;
const FLOAT: u64 = 1;
const STR: u64 = 2;

pub trait Runtime<'ctx> {
    // fn build_runtime_funcs(&'ctx mut self);
    fn build_mk_int(module: &Module<'ctx>, context: &'ctx Context);
    fn build_mk_float(module: &Module<'ctx>, context: &'ctx Context);
    fn build_use_int(module: &Module<'ctx>, context: &'ctx Context);
    fn build_use_float(module: &Module<'ctx>, context: &'ctx Context);

    fn build_new_obj(module: &Module<'ctx>, context: &'ctx Context);
}
pub fn obj_type<'ctx>(ctx: &'ctx Context) -> StructType<'ctx> {
    ctx.struct_type(
        &[
            ctx.i8_type().array_type(4).as_basic_type_enum(),
            ctx.i8_type().as_basic_type_enum(),
            ctx.i8_type()
                .ptr_type(AddressSpace::default())
                .as_basic_type_enum(),
        ],
        false,
    )
}

impl<'ctx> Runtime<'ctx> for Source<'ctx> {
    fn build_mk_float(module: &Module<'ctx>, context: &'ctx Context) {
        let fn_ty = context
            .f32_type()
            .fn_type(&[context.i8_type().array_type(4).into()], false);
        let func = module.add_function("mk_float", fn_ty, None);
        let builder = context.create_builder();
        let arr = func.get_nth_param(0).unwrap();
        arr.set_name("arr");

        let entry = context.append_basic_block(func, "entry");
        let _ = builder.position_at_end(entry);
        let alloca = { builder.build_alloca(arr.get_type(), "arr").unwrap() };

        let _ = builder.build_store(alloca, arr);

        let mut result = context.i32_type().const_zero();
        for i in 0..4 {
            let byte = builder
                .build_extract_value(arr.into_array_value(), i, "extract")
                .unwrap()
                .into_int_value();
            let byte32 = builder
                .build_int_z_extend_or_bit_cast(byte, context.i32_type(), "iextend")
                .unwrap();

            let shifted = builder
                .build_left_shift(
                    byte32,
                    context.i32_type().const_int((i * 8) as u64, false),
                    "lsh",
                )
                .unwrap();
            result = builder.build_or(result, shifted, "OR").unwrap();
        }
        let _ = builder.build_return(Some(
            &builder
                .build_bitcast(result, context.f32_type(), "fcast")
                .unwrap()
                .into_float_value(),
        ));
    }

    fn build_mk_int(module: &Module<'ctx>, context: &'ctx Context) {
        let fn_ty = context
            .i32_type()
            .fn_type(&[context.i8_type().array_type(4).into()], false);
        let func = module.add_function("mk_int", fn_ty, None);

        let builder = context.create_builder();
        let arr = func.get_nth_param(0).unwrap();
        arr.set_name("arr");

        let entry = context.append_basic_block(func, "entry");
        let _ = builder.position_at_end(entry);
        // let alloca = { builder.build_alloca(arr.get_type(), "arr").unwrap() };

        // let _ = builder.build_store(alloca, arr);

        let mut result = context.i32_type().const_zero();
        for i in 0..4 {
            let byte = builder
                .build_extract_value(arr.into_array_value(), i, "extract")
                .unwrap()
                .into_int_value();
            let byte32 = builder
                .build_int_z_extend_or_bit_cast(byte, context.i32_type(), "iextend")
                .unwrap();

            let shifted = builder
                .build_left_shift(
                    byte32,
                    context.i32_type().const_int((i * 8) as u64, false),
                    "lsh",
                )
                .unwrap();
            result = builder.build_or(result, shifted, "OR").unwrap();
        }
        let _ = builder.build_return(Some(&result));
    }

    fn build_new_obj(module: &Module<'ctx>, context: &'ctx Context) {
        let fn_ty = obj_type(context).fn_type(
            &[
                context.i8_type().array_type(4).into(),
                context.i8_type().into(),
                context.i8_type().ptr_type(AddressSpace::default()).into(),
            ],
            false,
        );
        let func = module.add_function("new_obj", fn_ty, None);

        let builder = context.create_builder();
        let bytes = func.get_nth_param(0).unwrap().into_array_value();
        let ty = func.get_nth_param(1).unwrap().into_int_value();
        let str = func.get_nth_param(2).unwrap().into_pointer_value();
        bytes.set_name("bytes");
        ty.set_name("type");
        str.set_name("str");

        let entry = context.append_basic_block(func, "entry");
        let _ = builder.position_at_end(entry);
        let _ = builder.build_return(Some(
            &obj_type(context)
                .const_named_struct(&[bytes.into(), ty.into(), str.into()])
                .as_basic_value_enum(),
        ));
    }

    fn build_use_float(module: &Module<'ctx>, context: &'ctx Context) {
        let fn_ty = obj_type(context).fn_type(&[context.f32_type().into()], false);
        let func = module.add_function("use_float", fn_ty, None);
        let new_obj = module.get_function("new_obj").unwrap();

        let builder = context.create_builder();
        let floatv = func.get_nth_param(0).unwrap().into_float_value();
        floatv.set_name("floatv");

        let entry = context.append_basic_block(func, "entry");
        let _ = builder.position_at_end(entry);
        let mut arr = context.i8_type().array_type(4).const_zero();

        let intv = builder
            .build_bitcast(floatv, context.i32_type(), "ibitc")
            .unwrap()
            .into_int_value();

        let mut bytes = vec![];
        for i in 0..4 {
            let shift = context.i32_type().const_int((i * 8) as u64, false);

            let byte = builder.build_left_shift(intv, shift, "shl").unwrap();
            arr = builder
                .build_insert_value(
                    arr,
                    builder
                        .build_int_truncate(byte, context.i8_type(), "trunc")
                        .unwrap(),
                    i,
                    "ins",
                )
                .unwrap()
                .into_array_value();
            bytes.push(byte);
        }

        let llvm_obj = builder
            .build_call(
                new_obj,
                &[
                    arr.into(),
                    context.i8_type().const_int(FLOAT, false).into(),
                    context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .const_null()
                        .into(),
                ],
                "new_val",
            )
            .unwrap()
            .try_as_basic_value()
            .unwrap_left();

        let _ = builder.build_return(Some(&llvm_obj));
    }

    fn build_use_int(module: &Module<'ctx>, context: &'ctx Context) {
        let fn_ty = obj_type(context).fn_type(&[context.i32_type().into()], false);
        let func = module.add_function("use_int", fn_ty, None);
        let new_obj = module.get_function("new_obj").unwrap();

        let builder = context.create_builder();
        let intv = func.get_nth_param(0).unwrap().into_int_value();
        intv.set_name("intv");

        let entry = context.append_basic_block(func, "entry");
        let _ = builder.position_at_end(entry);
        let mut arr = context.i8_type().array_type(4).const_zero();

        let mut bytes = vec![];
        for i in 0..4 {
            let shift = context.i32_type().const_int((i * 8) as u64, false);

            let byte = builder.build_left_shift(intv, shift, "shl").unwrap();
            arr = builder
                .build_insert_value(
                    arr,
                    builder
                        .build_int_truncate(byte, context.i8_type(), "trunc")
                        .unwrap(),
                    i,
                    "ins",
                )
                .unwrap()
                .into_array_value();
            bytes.push(byte);
        }

        let llvm_obj = builder
            .build_call(
                new_obj,
                &[
                    arr.into(),
                    context.i8_type().const_int(INT, false).into(),
                    context
                        .i8_type()
                        .ptr_type(AddressSpace::default())
                        .const_null()
                        .into(),
                ],
                "new_val",
            )
            .unwrap()
            .try_as_basic_value()
            .unwrap_left();

        let _ = builder.build_return(Some(&llvm_obj));
    }
}
