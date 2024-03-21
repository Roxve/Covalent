use crate::ast::Expr;
use crate::backend::c;
use crate::backend::wasm;
use crate::ir::gen::IRGen;
use crate::parser::Parser;
use crate::source::Source;

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
pub struct WASMSettings {}

impl WASMSettings {
    pub fn new() -> Self {
        Self {}
    }
}

macro_rules! unwarp {
    ($back: expr, $vari: path) => {
        match $back {
            $vari(i) => i,
            _ => panic!(),
        }
    };
}

#[allow(unused)]
pub enum Backend {
    WASM(WASMSettings),
    C(CSettings),
    Custom { name: String, settings: Vec<String> },
}
pub struct CompilerConfig {
    input: String,
    pub backend: Backend,
    pub debug: bool,
    pub repl: bool,
    pub output: String,
}
impl CompilerConfig {
    pub fn new(input: String, backend: Backend, debug: bool, repl: bool, output: String) -> Self {
        Self {
            input,
            backend,
            debug,
            repl,
            output,
        }
    }
    pub fn run(&self) {
        let mut src = Source::new(self.input.clone());

        let prog: Vec<Expr> = src.parse_prog();
        if self.debug {
            println!("parsed prog:\n {:#?}\nsrc: \n{:#?}", prog, src);
        }
        let ir = src.gen_prog(prog);
        dbg!(&ir);

        match self.backend {
            Backend::WASM(_) => {
                wasm::compile(&self, ir);
            }
            Backend::C(_) => {
                let mut codegen = c::Codegen::new();
                let str = codegen.codegen(ir);
                println!("{}", str);
            }
            _ => todo!(),
        }
    }
} 
