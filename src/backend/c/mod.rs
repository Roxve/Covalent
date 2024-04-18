pub mod gen;
use crate::compiler::CompilerConfig;
use crate::ir::IROp;
use crate::parser::ast::Literal;
use crate::source::ConstType;
use std::collections::HashMap;

use std::fs;
use std::process::Command;

pub fn compile(config: &CompilerConfig, ir: Vec<IROp>) {
    let mut codegen = Codegen::new();
    let code = codegen.codegen(ir);
    drop(codegen);
    let _ = Command::new("mkdir")
        .arg("-p")
        .arg("/tmp/covalent")
        .spawn()
        .unwrap()
        .wait();
    let outpath = format!("/tmp/covalent/`{}`.c", &config.output);

    fs::write(&outpath, code).expect(
        format!("err writing to /tmp/covalent make sure covalent can access that path!").as_str(),
    );
    dbg!(&config.libdir);
    let _ = Command::new("gcc")
        .arg(format!("-I{}", &config.libdir))
        .arg(format!("-o {}", &config.output))
        .arg(outpath)
        .arg(format!("{}/runtime.o", &config.libdir))
        .arg(format!("{}/gc.o", &config.libdir))
        .spawn()
        .unwrap()
        .wait();
}

pub fn type_to_c(ty: ConstType) -> String {
    match ty {
        ConstType::Int => "int".to_string(),
        ConstType::Float => "float".to_string(),
        ConstType::Dynamic => "void*".to_string(),
        ConstType::Str => "Str".to_string(),
        ConstType::Bool => "_Bool".to_string(),
        ConstType::Void => "void".to_string(),
    }
}

pub fn types_to_cnamed(tys: Vec<(ConstType, String)>) -> String {
    let mut str = String::from("");
    let tys_len = tys.len();
    for (i, ty) in tys.into_iter().enumerate() {
        str += (type_to_c(ty.0) + " ").as_str();
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
    Const(Literal),
    Var(ConstType, String),
    Expr(ConstType, String),
}

impl Item {
    #[inline]
    pub fn get_ty(&self) -> ConstType {
        match self {
            &Self::Expr(ty, _) => ty.clone(),
            &Self::Var(ty, _) => ty.clone(),
            Self::Const(literal) => (&literal).get_ty(),
        }
    }
    #[inline]
    pub fn is_var(&self) -> bool {
        match self {
            &Self::Var(_, _) => true,
            _ => false,
        }
    }
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
        let include_line = format!("#include \"{}.h\"", include);
        if !self.includes.contains(&include_line) {
            self.includes.push(include_line);
        }
    }

    pub fn func(&mut self, func: Vec<String>) {
        self.functions.push(func);
    }

    pub fn finish(&mut self) -> String {
        let mut func_lines: Vec<String> = (&self.functions).join(&String::from("\n\n"));
        self.functions.clear();
        let mut lines = Vec::new();
        lines.append(&mut self.includes);
        lines.append(&mut func_lines);
        let code = lines.join("\n");

        code
    }
}
#[derive(Debug, Clone)]
pub struct Codegen {
    stack: Vec<Item>,
    variables: HashMap<String, (i32, ConstType)>, // c doesnt allow redeclaration of vars with different types
    pub module: Module,                           // code we are generating
}

impl Codegen {
    pub fn push(&mut self, item: Item) {
        self.stack.push(item);
    }

    pub fn borrow(&mut self) -> &Item {
        self.stack.last().unwrap()
    }
    pub fn pop(&mut self) -> Item {
        self.stack.pop().expect("no stack item")
    }
    pub fn pop_str(&mut self) -> String {
        let item = self.pop();
        match item {
            Item::Const(con) => match con {
                Literal::Int(i) => i.to_string(),
                Literal::Float(f) => f.to_string(),
                Literal::Str(s) => format!("__strnew__(\"{}\")", s),
                Literal::Bool(b) => (b as u8).to_string(),
            },
            Item::Var(_, name) => name,
            Item::Expr(_, expr) => expr,
        }
    }

    pub fn pop_amount(&mut self, count: u16) -> Vec<String> {
        let mut results = Vec::new();
        for _ in [1..count] {
            results.push(self.pop_str());
        }

        return results;
    }

    pub fn new() -> Self {
        Self {
            stack: Vec::new(),
            variables: HashMap::new(),
            module: Module::new(),
        }
    }

    pub fn get_var(&mut self, name: String) -> String {
        let count = self.variables.get(&name);
        if count.is_none() {
            return name;
        }
        let count = count.unwrap().0;

        if count == 0 {
            name
        } else {
            name + count.to_string().as_str()
        }
    }

    pub fn var(&mut self, name: String, ty: ConstType) -> String {
        let count = self.variables.get(&name);
        if count.is_none() {
            self.variables.insert(name.clone(), (0, ty));
            name
        } else {
            let count = count.unwrap().0.to_owned() + 1;
            self.variables.remove(&name);
            self.variables.insert(name.clone(), (count, ty));
            self.get_var(name)
        }
    }
}
