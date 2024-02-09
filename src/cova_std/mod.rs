use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::types::{BasicMetadataTypeEnum, BasicType, BasicTypeEnum};
use inkwell::AddressSpace;

struct Builtin<'a> {
    name: String,
    args: Vec<BasicMetadataTypeEnum<'a>>,
    return_ty: Option<BasicTypeEnum<'a>>,
}

fn add<'a>(
    name: &str,
    args: Vec<BasicMetadataTypeEnum<'a>>,
    return_ty: Option<BasicTypeEnum<'a>>,
) -> Builtin<'a> {
    return Builtin {
        name: name.to_string(),
        args,
        return_ty,
    };
}

pub fn add_std<'a>(module: &Module<'a>, ctx: &'a Context) {
    let ptr_i8 = ctx.i8_type().ptr_type(AddressSpace::default());

    let funcs = vec![
        add(
            // used for adding two strings...
            "strcat",
            vec![ptr_i8.into(), ptr_i8.into()],
            Some(ptr_i8.as_basic_type_enum()),
        ),
        add(
            "writefn_ptr__i8",
            vec![ctx.i8_type().ptr_type(AddressSpace::default()).into()],
            None,
        ),
        add("writefn_float", vec![ctx.f32_type().into()], None),
        add("writefn_i32", vec![ctx.i32_type().into()], None),
    ];

    for fun in funcs {
        if fun.return_ty == None {
            let fun_type = ctx.void_type().fn_type(fun.args.as_slice(), false);
            let _ = module.add_function(fun.name.as_str(), fun_type, None);
        } else {
            let fun_type = fun.return_ty.unwrap().fn_type(fun.args.as_slice(), false);
            let _ = module.add_function(fun.name.as_str(), fun_type, None);
        }
    }
}
