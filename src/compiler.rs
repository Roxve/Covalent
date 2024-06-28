use std::env::current_exe;

use crate::analysis::Analyzer;
use crate::backend::c;
use crate::ir::gen::IRGen;
use crate::ir::Codegen;
use crate::parser::parse::Parse;
use crate::parser::Parser;

#[allow(unused)]
pub struct CSettings {
    compiler: Option<String>,
    flags: Vec<String>,
}

impl CSettings {
    pub fn new(compiler: Option<String>, flags: Vec<String>) -> Self {
        Self { compiler, flags }
    }
}

#[allow(unused)]
pub enum Backend {
    C(CSettings),
    Custom { name: String, settings: Vec<String> },
}
pub struct CompilerConfig {
    input: String,
    pub libdir: String,
    pub backend: Backend,
    pub debug: bool,
    pub output: String,
    pub workdir: String,
}
impl CompilerConfig {
    pub fn new(
        input: String,
        backend: Backend,
        debug: bool,
        output: String,
        workdir: String,
    ) -> Self {
        Self {
            input,
            libdir: format!(
                "{}/lib",
                current_exe().unwrap().parent().unwrap().to_str().unwrap()
            ),
            backend,
            debug,
            output,
            workdir,
        }
    }
    pub fn compile(&self) {
        let mut parser = Parser::new(self.input.clone());
        let prog = parser.parse_prog();

        let prog = Analyzer::analyz_prog(prog, parser.functions, self.workdir.clone()).unwrap();
        if self.debug {
            dbg!(&prog);
        }

        let mut codegen = Codegen::new();
        let ir = codegen.gen_prog(prog).unwrap();
        if self.debug {
            dbg!(&ir);
        }
        match self.backend {
            Backend::C(_) => {
                c::compile(self, ir);
            }
            _ => todo!(),
        }
    }
}
