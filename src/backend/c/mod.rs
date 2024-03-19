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
pub struct Module {
    includes: Vec<String>,
    functions: Vec<Vec<String>>,
}

impl Module {
    pub fn new() -> Self {
        Self {
            includes: Vec::new(),
            functions: Vec::new(),
        }
    }
    pub fn include(&mut self, include: String) {
        self.includes.push(include);
    }
    pub fn line(&mut self, line: String) {
        let last = self.functions.len() - 1;
        self.functions[last].push(line);
    }

    pub fn finish(&mut self) -> String {
        let mut func_lines: Vec<String> = (&self.functions)
            .into_iter()
            .flat_map(|inner| inner.clone())
            .collect();
        self.functions.clear();
        let mut lines = Vec::new();
        lines.append(&mut self.includes);
        lines.append(&mut func_lines);
        lines.join("\n")
    }
}
#[derive(Debug, Clone)]
pub struct Codegen {
    stack: Vec<Item>,
    module: Module, // code we are generating
}

impl Codegen {
    fn new() -> Self {
        Self {
            stack: Vec::new(),
            module: Module::new(),
        }
    }
}
