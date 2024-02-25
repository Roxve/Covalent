pub mod gen;
pub mod tools;

#[derive(Debug, Clone, PartialEq)]
#[repr(u8)]
pub enum ConstType {
    Int = 0u8,
    Float = 2u8,
    Str = 3u8,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Const {
    Int(i32),
    Float(f32),
    Str(String),
}
#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Def(String, Vec<IROp>),
    Ret(ConstType),
    Add(ConstType),
    Sub(ConstType),
    Mul(ConstType),
    Div(ConstType),
    Const(ConstType, Const),
    Conv(ConstType),
}

use self::IROp::*;
pub fn get_op_type(op: &IROp) -> ConstType {
    match op {
        Def(_, ops) => get_op_type(ops.last().unwrap()),
        Ret(t) => t.clone(),
        Add(t) => t.clone(),
        Sub(t) => t.clone(),
        Mul(t) => t.clone(),
        Div(t) => t.clone(),
        Const(t, _) => t.clone(),
        Conv(t) => t.clone(),
    }
}

pub fn get_ops_type(ops: &Vec<IROp>) -> ConstType {
    get_op_type(ops.last().unwrap())
}
