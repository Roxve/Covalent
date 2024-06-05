use crate::parser::ast::{Ident, Literal};

use crate::enviroment::Enviroment;
use crate::types::AtomKind;

pub mod gen;
pub mod tools;

#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Import(AtomKind, String, String, Vec<AtomKind>), // ty mod fun arg count
    Def(AtomKind, String, Vec<Ident>, Vec<IROp>),
    Call(AtomKind, u16),
    Ret(AtomKind),
    Add(AtomKind),
    Sub(AtomKind),
    Mul(AtomKind),
    Div(AtomKind),
    Mod(AtomKind),
    Comp, // acts like GE to peform LE switch left and right
    EComp,
    Eq,
    And,
    Or,
    Const(Literal),
    List(AtomKind, Vec<Vec<IROp>>), // each item is a bunch of operations
    Conv(AtomKind, AtomKind),
    Alloc(AtomKind, String),
    Dealloc(AtomKind, String), // when allocing a var with a new type we dealloc the old val
    Store(AtomKind, String),
    Set(AtomKind),
    Load(AtomKind, String),     // load loads an id
    LoadProp(AtomKind, String), // load prop loads a property from the id
    LoadIdx(AtomKind),          // loads an index

    If(AtomKind, Vec<IROp>, Vec<IROp>),
    While(Vec<IROp>),
    Pop,
}
use crate::err::ATErr;

use self::IROp::*;
// TODO: better op impl
pub fn get_op_type(op: &IROp) -> AtomKind {
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
        And => &AtomKind::Bool,
        Or => &AtomKind::Bool,
        Comp => &AtomKind::Bool,
        EComp => &AtomKind::Bool,
        Eq => &AtomKind::Bool,
        List(ref ty, _) => return AtomKind::List(Box::new(ty.clone())),
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
        While(_) => &AtomKind::Void,
        Pop => &AtomKind::Void,
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
