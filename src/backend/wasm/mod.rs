use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, FunctionSection, Instruction, Module, TypeSection,
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
pub struct Codegen {
    current: wasm_encoder::Function,
    module: Module,
    section: Section,
    ir: Vec<IROp>,
    ip: usize,
}
impl Codegen {
    pub fn new(ir: Vec<IROp>) -> Self {
        let mut section = Section::new();
        let _ty = section.types.function(vec![], vec![]);
        let _func = section.func.function(0);
        let _start = wasm_encoder::Function::new(vec![]);
        let module = Module::new();
        Codegen {
            current: _start,
            section,
            module,
            ir,
            ip: 0,
        }
    }

    fn insert(&mut self, inst: Instruction) {
        self.current.instruction(&inst);
        self.ip += 1;
    }

    pub fn codegen(&mut self) -> &mut Module {
        while self.ip <= self.ir.len() - 1 {
            self.compile(self.ir[self.ip].clone());
        }
        self.current.instruction(&wasm_encoder::Instruction::Return);

        self.current.instruction(&wasm_encoder::Instruction::End);
        self.module.section(&self.section.types);
        self.module.section(&self.section.func);

        self.section.exports.export("_start", ExportKind::Func, 0);
        self.module.section(&self.section.exports);

        self.module
            .section(self.section.code.function(&self.current))
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
