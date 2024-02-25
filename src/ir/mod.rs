pub mod gen;
pub mod tools;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstType {
    Int = 0,
    Float = 2,
    Str = 3,
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
    Ret,
    iAdd,
    iSub,
    iMul,
    iDiv,
    fAdd,
    fSub,
    fMul,
    fDiv,
    Const(ConstType, Const),
    Conv(ConstType),
}
