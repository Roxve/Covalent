use std::io::{self, Write};
mod ast;
mod codegen;
mod lexer;
mod parser;
mod source;
mod cova_std;

use std::path::Path;
use std::{env, fs};

use crate::ast::*;
use crate::codegen::*;
use crate::parser::*;
use crate::source::*;
use std::process::Command;

use inkwell::context::Context;

fn run(input: String, is_debug: bool, name: String) {
    let ctx = &Context::create();
    let mut src = Source::new(input, ctx);

    let prog: Vec<Expr> = src.parse_prog();
    if is_debug {
        println!(
            "parsed prog:\n {:#?}\nfunctions: \n{:#?}",
            prog,
            src.functions.clone()
        );
    }

    let res = src.compile_prog(prog);

    let _ = src
        .builder
        .build_return(Some(&src.context.i32_type().const_int(0, true)));

    if is_debug {
        println!("module!: ");
        src.module.print_to_stderr();
    }

    src.module.verify().expect("invaild module");

    let byte_path = format!("/tmp/{}.ll", name);

    let path = Path::new(byte_path.as_str());
    src.module.write_bitcode_to_path(path);

    if is_debug {
        println!("{:#?}", res);
    }

    // compiling

    Command::new("clang")
        .arg("-Wno-everything")
        .arg(byte_path)
        .arg(format!(
            "-L{}/libs",
            env::current_exe()
                .expect("no path to exe")
                .parent()
                .unwrap()
                .to_str()
                .expect("no path to exe dir"),
        ))
        .arg("-lstd")
        .arg("-o")
        .arg(format!("./{}", name).as_str())
        .spawn()
        .expect("failed compiling file(maybe you dont have clang installed in path)");
    println!("compiled success!");
}
fn repl() {
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
        run(repl, true, "repl".to_string());
    }
}

fn main() {
    let mut args = env::args();

    if args.len() <= 1 {
        return repl();
    }
    let file = args.nth(1).unwrap();
    let prog = fs::read_to_string(file.clone());

    let path = Path::new(file.as_str());

    let filename = path.file_name().expect("file passed is a folder");
    run(
        prog.expect("file doesnt exist"),
        false,
        filename.to_str().unwrap().to_string().replace(".atoms", ""),
    )
}
