pub mod gen;
pub mod tools;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ConstType {
    Int = 0u8,
    Float = 2u8,
    Str = 3u8,
    Dynamic = 4u8, // once you go dynamic there is no turning back
    Void = 5u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Const {
    Int(i32),
    Float(f32),
    Str(String),
}
#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Def(Option<ConstType>, String, Vec<String>, Vec<IROp>),
    Call(ConstType, String),
    Ret(ConstType),
    Add(ConstType),
    Sub(ConstType),
    Mul(ConstType),
    Div(ConstType),
    Const(ConstType, Const),
    Conv(ConstType, ConstType),
    Alloc(ConstType, String),
    Dealloc(ConstType, String), // when allocing a var with a new type we dealloc the old val
    Store(ConstType, String),
    Load(ConstType, String),
}

use self::IROp::*;
pub fn get_op_type(op: &IROp) -> ConstType {
    match op {
        Def(t, _, _, _) => &t.as_ref().unwrap_or(&ConstType::Void),
        Call(t, _) => t,
        Ret(t) => t,
        Add(t) => t,
        Sub(t) => t,
        Mul(t) => t,
        Div(t) => t,
        Const(t, _) => t,
        Conv(t, _) => t,
        Store(t, _) => t,
        Load(t, _) => t,
        Alloc(t, _) => t,
        Dealloc(t, _) => t,
    }
    .clone()
}

pub fn get_ops_type(ops: &Vec<IROp>) -> ConstType {
    dbg!(&ops);
    get_op_type(ops.last().unwrap())
}

pub fn get_fn_type(ops: &mut Vec<IROp>) -> Option<ConstType> {
    let mut ty: Option<ConstType> = None;
    for op in ops.clone() {
        if let IROp::Ret(t) = op {
            if ty.is_some_and(|v| v != t.clone()) {
                // loop again and convert each return into dynamic
                let mut _mod_op: Vec<IROp> = ops
                    .into_iter()
                    .map(|op| match op {
                        IROp::Ret(_) => IROp::Ret(ConstType::Dynamic),
                        a => a.clone(),
                    })
                    .collect();
                _mod_op.clone_into(ops);
                return Some(ConstType::Dynamic);
            }
            ty = Some(t.clone());
        }
    }
    ty
}
