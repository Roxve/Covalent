use wasm_encoder::{CodeSection, Module};

use crate::ir::{Const, ConstType, IROp};

pub struct Codegen {
    current: wasm_encoder::Function,
    module: Module,
    section: CodeSection,
    ir: Vec<IROp>,
    ip: usize,
}
impl Codegen {
    pub fn new(ir: Vec<IROp>) -> Self {
        let _start = wasm_encoder::Function::new(vec![]);
        let section = CodeSection::new();
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
        self.module.section(self.section.function(&self.current))
    }
}
