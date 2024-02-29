use wasm_encoder::{CodeSection, ExportKind, ExportSection, FunctionSection, Module, TypeSection};

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
    pub fn codegen(&mut self) -> &mut Module {
        while self.ip <= self.ir.len() - 1 {
            match self.ir[self.ip].clone() {
                IROp::Const(ConstType::Int, Const::Int(i)) => {
                    self.current
                        .instruction(&wasm_encoder::Instruction::I32Const(i));
                    self.ip += 1;
                }
                IROp::Const(ConstType::Float, Const::Float(_)) => todo!(),
                _ => todo!(),
            }
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
}
