use std::io::{self, Write};
mod ast;
mod codegen;
mod cova_std;
mod lexer;
mod parser;
mod runtime;
mod source;

use std::path::Path;
use std::{env, fs};

use crate::ast::*;
use crate::codegen::*;
use crate::parser::*;
use crate::runtime::Runtime;
use crate::source::*;
use std::process::Command;

use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::passes::PassManager;
fn run_passes_on(module: &Module) {
    let fpm = PassManager::create(());

    fpm.add_instruction_combining_pass();
    fpm.add_reassociate_pass();
    fpm.add_gvn_pass();
    fpm.add_cfg_simplification_pass();
    fpm.add_basic_alias_analysis_pass();
    fpm.add_promote_memory_to_register_pass();

    fpm.run_on(module);
}
fn run(input: String, is_debug: bool, is_repl: bool, name: String) {
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
    src.build_runtime_funcs();

    let res = src.compile_prog(prog);

    src.fn_value.verify(false);

    if is_debug {
        println!("{:#?}", src.mk_val(res.unwrap()));
    }
    let _ = src
        .builder
        .build_return(Some(&src.context.i32_type().const_int(0, true)));

    if is_debug {
        println!("module!: ");
        src.module.print_to_stderr();
    }

    if src.errors.len() >= 1 {
        panic!("exiting duo to previous errors");
    }

    src.module.verify().expect("invaild module");
    run_passes_on(&src.module);

    let byte_path = format!("/tmp/{}.ll", name);

    let path = Path::new(byte_path.as_str());

    src.module.write_bitcode_to_path(path);
    // compiling
    let out = {
        // add is repl
        if is_debug || is_repl {
            format!("/tmp/{}", name)
        } else {
            format!("./{}", name)
        }
    };

    let _ = Command::new("clang")
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
        .arg(&out)
        .spawn()
        .expect("failed compiling file(maybe you dont have clang installed in path)")
        .wait();
    if is_repl {
        let output = Command::new(out)
            .spawn()
            .expect("failed running compiled prog")
            .wait();
        println!("output: {:?}", output);
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
    let mut is_debug = false;
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
