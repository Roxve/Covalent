use crate::parser::ast::{Ident, Literal};

use crate::enviroment::Enviroment;
use crate::types::{self,AtomType, BasicType};

pub mod gen;
pub mod tools;

#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Import(AtomType, String, String, Vec<AtomType>), // ty mod fun arg count
    Extern(AtomType, String, Vec<Ident>),
    Def(AtomType, String, Vec<Ident>, Vec<IROp>),

    Call(AtomType, u16),
    Ret(AtomType),

    Add(AtomType),  
    Sub(AtomType),
    Mul(AtomType),
    Div(AtomType),
    Mod(AtomType),

    Comp, // acts like GE to peform LE switch left and right
    EComp,
    Eq,
    And,
    Or,

    Const(Literal),
    List(AtomType, Vec<Vec<IROp>>), // each item is a bunch of operations
    Conv(AtomType, AtomType),
    Alloc(AtomType, String),
    Dealloc(AtomType, String), // when allocing a var with a new type we dealloc the old val
    Store(AtomType, String),
    Set(AtomType),
    Load(AtomType, String),     // load loads an id
    LoadProp(AtomType, String), // load prop loads a property from the id
    LoadIdx(AtomType),          // loads an index

    If(AtomType, Vec<IROp>, Vec<IROp>),
    While(Vec<IROp>),
    Pop,
}
use crate::err::ATErr;

use self::IROp::*;
// TODO: better op impl
pub fn get_op_type(op: &IROp) -> AtomType {
    match op {
        Import(t, _, _, _) => t,
        Extern(t, _, _) => t,
        Def(t, _, _, _) => t,

        Call(t, _) => t,
        Ret(t) => t,

        Add(t) => t,
        Sub(t) => t,
        Mul(t) => t,
        Div(t) => t,
        Mod(t) => t,

        And => &AtomType::Basic(BasicType::Bool),
        Or => &AtomType::Basic(BasicType::Bool),
        Comp => &AtomType::Basic(BasicType::Bool),
        EComp => &AtomType::Basic(BasicType::Bool),
        Eq => &AtomType::Basic(BasicType::Bool),

        List(ref ty, _) => return AtomType::Atom(types::List.spec(&[ty.clone()])),
        Const(lit) => return lit.get_ty(),
        Conv(t, _) => t,
        Store(t, _) => t,
        Set(t) => t,
        Load(t, _) => t,
        LoadProp(t, _) => t,
        LoadIdx(t) => t,
        // Get(t) => t,
        Alloc(t, _) => t,
        Dealloc(t, _) => t,
        If(t, _, _) => t,
        While(_) => &AtomType::Basic(BasicType::Void),
        Pop => &AtomType::Basic(BasicType::Void),
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
