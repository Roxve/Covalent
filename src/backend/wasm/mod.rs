use std::collections::HashMap;

use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::ir::{Const, ConstType, IROp};
#[derive(Debug, Clone)]
pub struct Section {
    types: TypeSection,
    code: CodeSection,
    func: FunctionSection,
    exports: ExportSection,
}

impl Section {
    pub fn new() -> Self {
        Section {
            types: TypeSection::new(),
            code: CodeSection::new(),
            func: FunctionSection::new(),
            exports: ExportSection::new(),
        }
    }
}
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
    module: Module,
    section: Section,
    ir: Vec<IROp>,
    ip: usize,
}
impl<'a> Codegen<'a> {
    pub fn new(ir: Vec<IROp>) -> Self {
        let mut section = Section::new();
        let _ty = section.types.function(vec![], vec![]);
        let _func = section.func.function(0);
        let module = Module::new();
        Codegen {
            current: Func::new(),
            section,
            module,
            ir,
            ip: 0,
        }
    }

    fn insert(&mut self, inst: Instruction<'a>) {
        self.current.body.push(inst);
        self.ip += 1;
    }

    pub fn codegen(&mut self) -> &mut Module {
        while self.ip <= self.ir.len() - 1 {
            self.compile(self.ir[self.ip].clone());
        }
        self.insert(Instruction::Return);

        self.insert(Instruction::End);
        self.module.section(&self.section.types);
        self.module.section(&self.section.func);

        self.section.exports.export("_start", ExportKind::Func, 0);
        self.module.section(&self.section.exports);

        self.module
            .section(self.section.code.function(&self.current.finish()))
    }

    pub fn compile(&mut self, op: IROp) {
        match op {
            IROp::Const(ConstType::Int, Const::Int(i)) => self.insert(Instruction::I32Const(i)),
            IROp::Const(ConstType::Float, Const::Float(f)) => {
                self.insert(Instruction::F32Const(f));
            }
            IROp::Add(_) | IROp::Mul(_) | IROp::Div(_) | IROp::Sub(_) => {
                return self.compile_binary(op);
            }
            IROp::Alloc(ty, name) => {
                self.current.add_var(name, ty.into_val_type());
                self.ip += 1;
            }
            IROp::Dealloc(_, _) => self.insert(Instruction::Nop),
            IROp::Store(_, name) => {
                let idx = self.current.get_var(name);
                self.insert(Instruction::LocalSet(idx));
            }
            IROp::Load(_, name) => {
                let idx = self.current.get_var(name);
                self.insert(Instruction::LocalGet(idx));
            }

            _ => todo!(),
        }
    }

    pub fn compile_binary(&mut self, op: IROp) {
        match op {
            IROp::Add(ty) => match ty {
                ConstType::Int => self.insert(Instruction::I32Add),
                ConstType::Float => self.insert(Instruction::F32Add),
                _ => todo!("add + for type {:?}", ty),
            },
            _ => todo!(),
        }
    }
}
