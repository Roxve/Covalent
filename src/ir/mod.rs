use crate::parser::ast::{Ident, Literal};
use crate::types::AtomType;

#[derive(Debug, Clone)]
pub struct Program {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Instruction {
    pub op: IROp,
    pub ty: AtomType,
}

impl Instruction {
    pub fn new(op: IROp, ty: AtomType) -> Self {
        Self { op, ty }
    }

    pub fn ty(&self) -> AtomType {
        self.ty.clone()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Import(String, String, Vec<AtomType>), // ty mod fun arg count
    Extern(String, Vec<Ident>),
    Def(AtomType, String, Vec<Ident>, Vec<Instruction>),

    Call(u16),
    Ret,

    Add,
    Sub,
    Mul,
    Div,
    Mod,

    Comp, // acts like GE to peform LE switch left and right
    EComp,
    Eq,
    And,
    Or,

    Const(Literal),
    List(AtomType, Vec<Vec<IROp>>), // each item is a bunch of operations

    Conv(AtomType), // from

    Alloc(String),
    Dealloc(String), // when allocing a var with a new type we dealloc the old val
    Store(String),
    Set,
    Load(String),     // load loads an id
    Get,              // gets value
    LoadProp(String), // load prop loads a property from the id
    LoadIdx,          // loads an index

    Block(Vec<Instruction>),

    If(Vec<Instruction>, Vec<Instruction>),
    While(Vec<Instruction>),
    Pop,
}
