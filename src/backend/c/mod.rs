pub mod gen;
use crate::ir::{Const, ConstType};
use std::collections::HashMap;
use std::collections::VecDeque;

pub fn TypeToC(ty: ConstType) -> String {
    match ty {
        ConstType::Int => "int".to_string(),
        ConstType::Float => "float".to_string(),
        _ => todo!("convert type into c {:?}", ty),
    }
}

pub fn TypesToCNamed(tys: Vec<(ConstType, String)>) -> String {
    let mut str = String::from("");
    let tys_len = tys.len();
    for (i, ty) in tys.into_iter().enumerate() {
        str += TypeToC(ty.0).as_str();
        str += ty.1.as_str();

        if i != tys_len - 1 {
            str += ", ";
        }
    }
    str
}
// or ir is stack based so we need to simulate a stack
#[derive(Debug, Clone)]
pub enum Item {
    Const(Const),
    Var(Option<ConstType>, String),
    Expr(String), // push into stack except if the op doesnt push ig
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
    pub fn func(&mut self, func: Vec<String>) {
        self.functions.push(func);
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
    stack: VecDeque<Item>,
    variables: HashMap<String, i32>, // c doesnt allow redeclaration of vars with different types
    pub module: Module,              // code we are generating
}

impl Codegen {
    pub fn push(&mut self, item: Item) {
        self.stack.push_front(item);
    }
    pub fn pop(&mut self) -> Item {
        self.stack.pop_back().expect("no stack item")
    }
    pub fn pop_str(&mut self) -> String {
        let item = self.pop();
        match item {
            Item::Const(con) => match con {
                Const::Int(i) => i.to_string(),
                Const::Float(f) => f.to_string(),
                _ => todo!("conv a const item into string {:?}", con),
            },
            Item::Expr(expr) => expr,
            _ => todo!("conv an item into a string {:?}", item),
        }
    }
    pub fn pop_all(&mut self) -> Vec<String> {
        let mut results = Vec::new();
        for i in self.stack.clone() {
            results.push(self.pop_str());
        }
        results
    }
    pub fn new() -> Self {
        Self {
            stack: VecDeque::new(),
            variables: HashMap::new(),
            module: Module::new(),
        }
    }

    pub fn get_var(&mut self, name: String) -> String {
        let count = self.variables.get(&name);

        if count.is_none() {
            self.variables.insert(name.clone(), 0);
            name
        } else if count.unwrap() == &0 {
            name
        } else {
            name + count.unwrap().to_string().as_str()
        }
    }

    pub fn var(&mut self, name: String) -> String {
        let count = self.variables.get(&name);
        if count.is_none() {
            self.variables.insert(name.clone(), 0);
            name
        } else {
            let count = count.unwrap().to_owned() + 1;
            self.variables.remove(&name);
            self.variables.insert(name.clone(), count);
            self.get_var(name)
        }
    }
}
