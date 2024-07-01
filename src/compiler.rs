use std::env::current_exe;

use crate::analysis::Analyzer;
use crate::backend::c;
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
        let parser = Parser::new(self.input.clone());
        let prog = parser.parse_prog();

        let prog = Analyzer::analyze_program(self.workdir.clone(), prog).unwrap();
        if self.debug {
            dbg!(&prog);
        }

        match self.backend {
            Backend::C(_) => {
                // c::compile(self, prog);
                todo!()
            }
            _ => todo!(),
        }
    }
}
