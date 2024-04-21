use crate::parser::ast::Literal;
use crate::source::ConstType;

pub mod gen;
pub mod tools;

#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Import(ConstType, String, String, Vec<ConstType>), // ty mod fun arg count
    Def(ConstType, String, Vec<Ident>, Vec<IROp>),
    Call(ConstType, String, u16),
    Ret(ConstType),
    Add(ConstType),
    Sub(ConstType),
    Mul(ConstType),
    Div(ConstType),
    Comp(ConstType), // acts like GE to peform LE switch left and right
    EComp(ConstType),
    Eq(ConstType),
    Const(ConstType, Literal),
    Conv(ConstType, ConstType),
    Alloc(ConstType, String),
    Dealloc(ConstType, String), // when allocing a var with a new type we dealloc the old val
    Store(ConstType, String),
    Load(ConstType, String),
    If(ConstType, Vec<IROp>, Vec<IROp>),

    Pop,
}

use crate::source::{ATErr, ErrKind, Ident};
use std::collections::HashMap;

use self::IROp::*;
pub fn get_op_type(op: &IROp) -> ConstType {
    match op {
        Import(t, _, _, _) => t,
        Def(t, _, _, _) => t,
        Call(t, _, _) => t,
        Ret(t) => t,
        Add(t) => t,
        Sub(t) => t,
        Mul(t) => t,
        Div(t) => t,
        Comp(t) => t,
        EComp(t) => t,
        Eq(t) => t,
        Const(t, _) => t,
        Conv(t, _) => t,
        Store(t, _) => t,
        Load(t, _) => t,
        Alloc(t, _) => t,
        Dealloc(t, _) => t,
        If(t, _, _) => t,
        Pop => &ConstType::Void,
    }
    .clone()
}

#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub name: Ident,
    pub args: Vec<Ident>,
}
#[derive(Clone)]
pub struct Enviroment {
    functions: Vec<CompiledFunction>,
    vars: HashMap<String, ConstType>,
    pub parent: Option<Box<Enviroment>>,
}

impl Enviroment {
    pub fn new(parent: Option<Box<Self>>) -> Self {
        Self {
            functions: Vec::new(),
            vars: HashMap::new(),
            parent,
        }
    }

    pub fn child(&self) -> Enviroment {
        Enviroment::new(Some(Box::new(self.clone())))
    }

    pub fn parent(&self) -> Option<Enviroment> {
        if self.parent.is_none() {
            None
        } else {
            Some(*(self.parent.clone().unwrap()))
        }
    }

    pub fn get_ty(&self, name: &String) -> Option<ConstType> {
        if self.vars.contains_key(name) {
            return Some(self.vars[name].clone());
        }

        if self.parent.is_some() {
            return self.parent().unwrap().get_ty(name);
        } else {
            return None;
        }
    }

    pub fn has(&self, name: &String) -> bool {
        if self.vars.contains_key(name) {
            true
        } else if self.parent.is_some() {
            self.parent().unwrap().has(name)
        } else {
            false
        }
    }
    pub fn modify(&mut self, name: &String, ty: ConstType) {
        if self.vars.contains_key(name) {
            self.vars.get_mut(name).map(|val| *val = ty);
        } else if self.parent.is_some() {
            self.parent().unwrap().modify(name, ty);
        }
    }

    pub fn add(&mut self, name: &String, ty: ConstType) {
        self.vars.insert(name.clone(), ty);
    }

    pub fn push_function(&mut self, name: Ident, args: Vec<Ident>, ty: ConstType) {
        self.vars.insert(name.val.clone(), ty);
        self.functions.push(CompiledFunction { name, args });
    }

    pub fn get_function(&self, name: &Ident) -> Option<CompiledFunction> {
        for fun in self.functions.clone().into_iter() {
            if &fun.name == name {
                return Some(fun);
            }
        }
        if self.parent.is_some() {
            return self.parent.as_ref().unwrap().get_function(&name);
        }

        return None;
    }
}
pub struct Codegen {
    env: Enviroment,
    errors: Vec<ATErr>,
    _warnings: Vec<ATErr>, // program can continue error
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            env: Enviroment::new(None),
            errors: Vec::new(),
            _warnings: Vec::new(),
        }
    }

    pub fn err(&mut self, kind: ErrKind, msg: String) {
        let err = ATErr {
            kind,
            msg,
            line: 0,
            column: 0,
        };
        self.errors.push(err.clone());
        err.out_error();
    }
}
