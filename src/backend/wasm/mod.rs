pub mod codegen;

use std::collections::HashMap;

use wasm_encoder::{
    CodeSection, EntityType, ExportSection, Function, FunctionSection, ImportSection, Instruction,
    MemorySection, MemoryType, Module, TypeSection, ValType,
};

use crate::ir::{Const, ConstType, IROp};

pub const TYPE_INT: i32 = 0;
#[derive(Debug, Clone)]
pub struct Section {
    types: TypeSection,
    code: CodeSection,
    func: FunctionSection,
    mem: MemorySection,
    imports: ImportSection,
    exports: ExportSection,
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
            mem: MemorySection::new()
                .memory(MemoryType {
                    minimum: 1,
                    maximum: None,
                    memory64: false,
                    shared: false,
                })
                .to_owned(),
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
            _ => todo!("const into val"),
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
    ir: Vec<IROp>,
    ip: usize,
}
