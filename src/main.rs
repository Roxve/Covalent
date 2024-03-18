use std::io::{self, Write};
mod ast;
mod backend;
mod ir;
mod lexer;
mod parser;
mod source;

use std::path::Path;
// use std::process::Command;
use std::{env, fs};

use covalent::CompilerConfig;

// use ir::gen::IRGen;

// use crate::ast::*;
// use crate::backend::wasm::*;
// use crate::parser::*;
// use crate::source::*;

#[test]
fn test() {
    let prog = fs::read_to_string("TestProg/main.atoms").unwrap();

    CompilerConfig::new(
        prog,
        covalent::Backend::WASM,
        covalent::BackendSettings::WASM,
        true,
        true,
        "/tmp/covalent/test.wasm".to_string(),
    )
    .run();
}

fn repl(is_debug: bool) {
    let mut buffer = String::with_capacity(4096);
    let stdin = io::stdin();

    io::stdout().flush().unwrap();

    println!("running repl...(VERY WIP PLEASE COMPILE FILE INSTEAD)");
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let _ = stdin.read_line(&mut buffer);
        CompilerConfig::new(
            buffer.clone(),
            covalent::Backend::WASM,
            covalent::BackendSettings::WASM(covalent::WASMSettings::new()),
            true,
            true,
            "/tmp/covalent/repl.wasm".to_string(),
        )
        .run();
    }
}

fn main() {
    let mut is_debug = true;
    let mut args = env::args();

    if args.len() <= 1 {
        return repl(is_debug);
    }

    let file = {
        let arg = args.nth(1).unwrap();
        if arg == "test" {
            is_debug = true;

            if args.len() < 2 {
                return repl(is_debug);
            }

            args.nth(0).unwrap()
        } else {
            arg
        }
    };

    let prog = fs::read_to_string(file.clone());

    let path = Path::new(file.as_str());

    let filename = path.file_name().expect("file passed is a folder");
    CompilerConfig::new(
        prog.expect("invaild file name"),
        covalent::Backend::WASM,
        covalent::BackendSettings::WASM,
        is_debug,
        false,
        "/tmp/covalent/test.wasm".to_string(),
    )
    .run();
}
