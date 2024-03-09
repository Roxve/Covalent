use std::io::{self, Write};
mod ast;
mod backend;
mod ir;
mod lexer;
mod parser;
mod source;

use wasm3::Environment;
use wasm3::Module;

use std::path::Path;
use std::process::Command;
use std::{env, fs};

use ir::gen::IRGen;

use crate::ast::*;
use crate::backend::wasm::*;
use crate::parser::*;
use crate::source::*;
#[test]
fn test() {
    let prog = fs::read_to_string("TestProg/main.atoms").unwrap();
    run(prog, true, false, "test".to_string());
}

fn run(input: String, is_debug: bool, is_repl: bool, name: String) {
    let mut src = Source::new(input);

    let prog: Vec<Expr> = src.parse_prog();
    if is_debug {
        println!("parsed prog:\n {:#?}\nsrc: \n{:#?}", prog, src);
    }
    let ir = src.gen_prog(prog);
    dbg!(&ir);
    let mut codegen = Codegen::new(ir);
    let module = codegen.codegen();
    dbg!(&module);
    let bytes = module.clone().finish();
    let path = "/tmp/test.wasm";
    let _ = fs::write(path, bytes);

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
    if is_repl {
        let env = Environment::new().expect("unable to create repl enviroment");
        let runtime = env
            .create_runtime(1024)
            .expect("unable to create repl runtime");
        let bytes = fs::read(path).unwrap();
        let module = Module::parse(&env, bytes).expect("cannot load generated repl");
        let module = runtime.load_module(module).unwrap();
        let func = module
            .find_function::<(), ()>("_start")
            .expect("cannot find start in results");
        func.call().expect("failed to run prog");
    }
}

fn repl(is_debug: bool) {
    let mut buffer = String::with_capacity(4096);
    let stdin = io::stdin();

    io::stdout().flush().unwrap();

    let _ = stdin.read_line(&mut buffer);

    println!("running repl...(VERY WIP PLEASE COMPILE FILE INSTEAD)");
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut repl = String::with_capacity(4096);
        let _ = stdin.read_line(&mut repl);
        run(repl, is_debug, true, "repl".to_string());
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
    run(
        prog.expect("file doesnt exist"),
        is_debug,
        false,
        filename.to_str().unwrap().to_string().replace(".atoms", ""),
    )
}
