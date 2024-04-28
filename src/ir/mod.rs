use crate::parser::ast::Literal;
use crate::source::{ConstType, Enviroment};

pub mod gen;
pub mod tools;

#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Import(ConstType, String, String, Vec<ConstType>), // ty mod fun arg count
    Def(ConstType, String, Vec<Ident>, Vec<IROp>),
    Call(ConstType, u16),
    Ret(ConstType),
    Add(ConstType),
    Sub(ConstType),
    Mul(ConstType),
    Div(ConstType),
    Mod(ConstType),
    Comp, // acts like GE to peform LE switch left and right
    EComp,
    Eq,
    And,
    Or,
    Const(ConstType, Literal),
    Conv(ConstType, ConstType),
    Alloc(ConstType, String),
    Dealloc(ConstType, String), // when allocing a var with a new type we dealloc the old val
    Store(ConstType, String),
    Set(ConstType),
    Load(ConstType, String),
    If(ConstType, Vec<IROp>, Vec<IROp>),
    While(Vec<IROp>),
    Pop,
}
use crate::source::{ATErr, Ident};

use self::IROp::*;
pub fn get_op_type(op: &IROp) -> ConstType {
    match op {
        Import(t, _, _, _) => t,
        Def(t, _, _, _) => t,
        Call(t, _) => t,
        Ret(t) => t,
        Add(t) => t,
        Sub(t) => t,
        Mul(t) => t,
        Div(t) => t,
        Mod(t) => t,
        And => &ConstType::Bool,
        Or => &ConstType::Bool,
        Comp => &ConstType::Bool,
        EComp => &ConstType::Bool,
        Eq => &ConstType::Bool,
        Const(t, _) => t,
        Conv(t, _) => t,
        Store(t, _) => t,
        Set(t) => t,
        Load(t, _) => t,
        Alloc(t, _) => t,
        Dealloc(t, _) => t,
        If(t, _, _) => t,
        While(_) => &ConstType::Void,
        Pop => &ConstType::Void,
    }
    .clone()
}

pub struct Codegen {
    env: Enviroment,
    _warnings: Vec<ATErr>, // program can continue error
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            env: Enviroment::new(None),
            _warnings: Vec::new(),
        }
    }
}
