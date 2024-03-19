pub mod gen;
use crate::ir::{Const, ConstType};

// or ir is stack based so we need to simulate a stack
#[derive(Debug, Clone)]
pub enum Item {
    Const(Const),
    Var(Option<ConstType>, String),
    Call(String), // we generate func call as string then we push it into the stack
}
#[derive(Debug, Clone)]
pub struct Codegen {
    stack: Vec<Item>,
    code: String, // code we are generating
}
