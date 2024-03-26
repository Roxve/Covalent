pub mod codegen;
use crate::compiler::CompilerConfig;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::process::Command;

use wasm_encoder::{
    CodeSection, EntityType, ExportSection, Function, FunctionSection, ImportSection, Instruction,
    LinkingSection, MemoryType, Module, SymbolTable, TypeSection, ValType,
};

use crate::ir::{Const, IROp};
use crate::source::ConstType;

pub const _TYPE_INT: i32 = 0;
#[derive(Debug, Clone)]
pub struct Section {
    types: TypeSection,
    code: CodeSection,
    func: FunctionSection,
    imports: ImportSection,
    exports: ExportSection,
    linking: LinkingSection,
}

impl Section {
    pub fn new() -> Self {
        Section {
            types: TypeSection::new(),
            code: CodeSection::new(),
            func: FunctionSection::new(),
            imports: ImportSection::new()
                .import(
                    "mem",
                    "memory",
                    EntityType::Memory(MemoryType {
                        minimum: 1,
                        maximum: None,
                        memory64: false,
                        shared: false,
                    }),
                )
                .to_owned(),
            exports: ExportSection::new(),
            linking: LinkingSection::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Func<'a> {
    body: Vec<Instruction<'a>>,
    vars: Vec<(String, ValType)>,
}

impl ConstType {
    fn into_val_type(self) -> ValType {
        match self {
            ConstType::Int => ValType::I32,
            ConstType::Float => ValType::F32,
            ConstType::Dynamic => ValType::I32,
            _ => todo!("const into valtype"),
        }
    }
}

impl<'a> Func<'a> {
    fn new() -> Self {
        Self {
            body: vec![],
            vars: vec![],
        }
    }

    fn finish(&mut self) -> Function {
        let mut fun = Function::new_with_locals_types({
            let ve: Vec<ValType> = self.vars.clone().iter().map(|l| l.1).collect();
            ve
        });
        for ins in self.body.clone() {
            fun.instruction(&ins);
        }
        fun
    }

    fn add_var(&mut self, name: String, ty: ValType) {
        self.vars.push((name, ty));
    }

    fn get_var(&mut self, name: String) -> u32 {
        self.vars.iter().position(|k| k.0 == name).unwrap() as u32
    }
}

pub struct Codegen<'a> {
    current: Func<'a>,
    funcs: HashMap<String, (u32, Option<Func<'a>>)>,
    module: Module,
    section: Section,
    table: SymbolTable,
    ir: Vec<IROp>,
    ip: usize,
}

pub fn compile(config: &CompilerConfig, ir: Vec<IROp>) {
    let mut codegen = Codegen::new(ir);
    let module = codegen.codegen();
    dbg!(&module);
    let bytes = module.clone().finish();
    let path = config.output.as_str();
    let _ = fs::write(path, bytes);
    // generate relocs
    let _ = Command::new("wasm2wat")
        .arg(path)
        .arg("-o")
        .arg(format!("{}.wat", path))
        .spawn()
        .unwrap()
        .wait();
    let _ = Command::new("wat2wasm")
        .arg("--relocatable")
        .arg(format!("{}.wat", path))
        .arg("-o")
        .arg(path)
        .spawn()
        .unwrap()
        .wait();

    let libdir = env::current_exe()
        .unwrap()
        .to_str()
        .unwrap()
        .replace("covalent", "lib");
    // links with std runtime mem
    let _ = Command::new("wasm-ld")
        .arg("--relocatable")
        .arg(format!("{}/{}", libdir, "std.wasm"))
        .arg(format!("{}/{}", libdir, "runtime.wasm"))
        .arg(format!("{}/{}", libdir, "mem.wasm"))
        .arg(path)
        .arg("-o")
        .arg(path)
        .spawn()
        .unwrap()
        .wait();
    if config.repl {
        let _bytes = fs::read(path).unwrap();
    }
}
