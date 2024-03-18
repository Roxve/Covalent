use std::env;
use std::fs;
use std::process::Command;
pub mod ast;
pub mod backend;
pub mod ir;
pub mod lexer;
pub mod parser;
pub mod source;
use crate::ast::Expr;
use crate::backend::wasm;
use crate::ir::gen::IRGen;
use crate::parser::Parser;
use crate::source::Source;

#[allow(unused)]
pub enum Backend {
    WASM,
    C,
    Custom(String),
}

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

pub enum BackendSettings {
    WASM,
    C(CSettings),
    Custom(Vec<String>),
}
pub struct CompilerConfig {
    input: String,
    backend: Backend,
    settings: BackendSettings,
    debug: bool,
    repl: bool,
}
impl CompilerConfig {
    pub fn new(
        input: String,
        backend: Backend,
        settings: BackendSettings,
        debug: bool,
        repl: bool,
    ) -> Self {
        Self {
            input,
            backend,
            settings,
            debug,
            repl,
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
        let mut codegen = wasm::Codegen::new(ir);
        let module = codegen.codegen();
        dbg!(&module);
        let bytes = module.clone().finish();
        let path = "/tmp/test.wasm";
        let _ = fs::write(path, bytes);

        match self.backend {
            Backend::WASM => {
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
                if self.repl {
                    let bytes = fs::read(path).unwrap();
                }
            }

            _ => todo!(),
        }
    }
}
