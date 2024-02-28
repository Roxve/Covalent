use binaryen::{set_global_codegen_config, CodegenConfig, Module};

use crate::ir::IROp;

struct Codegen {
    module: Module.
    ir: Vec<IROp>.
    ip: i32,
} 
impl Codegen {
    pub fn new(ir: Vec<IROp) -> Self {
        let module = Module::new();
        Codegen {
            module,
            ir,
            0
        }
    }
    pub fn codegen(&self) -> Module {
        let conf = CodegenConfig {
            shrink_level: 1,
            optimization_level: 3,
        };
        set_global_codegen_config(&conf);
    }
}
