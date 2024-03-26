use crate::source::ConstType;

pub mod gen;
pub mod tools;

#[derive(Debug, Clone, PartialEq)]
pub enum Const {
    Int(i32),
    Float(f32),
    Str(String),
}
#[derive(Debug, Clone, PartialEq)]
pub enum IROp {
    Import(ConstType, String, String, Vec<ConstType>), // ty mod fun arg count
    Def(ConstType, String, Vec<String>, Vec<IROp>),
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
    Pop,
}

use crate::source::{ATErr, ErrKind, Ident};
use std::collections::HashMap;

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
        Const(t, _) => t,
        Conv(t, _) => t,
        Store(t, _) => t,
        Load(t, _) => t,
        Alloc(t, _) => t,
        Dealloc(t, _) => t,
        Pop => &ConstType::Void,
    }
    .clone()
}

pub fn get_ops_type(ops: &Vec<IROp>) -> ConstType {
    dbg!(&ops);
    get_op_type(ops.last().unwrap())
}

pub fn get_fn_type(ops: &mut Vec<IROp>) -> ConstType {
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
                return ConstType::Dynamic;
            }
            ty = Some(t.clone());
        }
    }
    ty.unwrap_or(ConstType::Void)
}

#[derive(Debug, Clone)]
pub struct CompiledFunction {
    name: Ident,
    args: Vec<Ident>,
}
pub struct Codegen {
    functions: Vec<CompiledFunction>,
    vars: HashMap<String, ConstType>,

    errors: Vec<ATErr>,
    warnings: Vec<ATErr>, // program can continue error
}

impl Codegen {
    pub fn new() -> Self {
        Self {
            functions: Vec::new(),
            vars: HashMap::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
    pub fn push_function(&mut self, name: Ident, args: Vec<Ident>) {
        self.functions.push(CompiledFunction { name, args })
    }

    pub fn get_function(&self, name: &Ident) -> Option<CompiledFunction> {
        for fun in self.functions.clone().into_iter() {
            if &fun.name == name {
                return Some(fun);
            }
        }
        return None;
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
