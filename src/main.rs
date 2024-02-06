use core::panic;
use std::io::{self, Write};
mod ast;
mod codegen;
mod lexer;
mod parser;
mod source;

use std::fs::{self, read_to_string};
use std::path::Path;

use crate::ast::*;
use crate::codegen::*;
use crate::parser::*;
use crate::source::*;
use inkwell::context::Context;

fn run(input: String) {
    let ctx = &Context::create();
    let mut src = Source::new(input, ctx);

    println!("entered {}", src.code);
    let prog: Vec<Expr> = src.parse_prog();
    println!("{:#?}", prog);

    let res = src.compile_prog(prog);

    dbg!(src.functions.clone());
    let _ = src
        .builder
        .build_return(Some(&src.context.i32_type().const_int(0, true)));

    src.module.print_to_stderr();
    src.module.verify().expect("invaild");

    src.module.print_to_stderr();

    let path = Path::new("test.ll");
    src.module.write_bitcode_to_path(path);
    println!("{:#?}", res);
}

fn main() {
    let mut buffer = String::with_capacity(4096);
    let stdin = io::stdin();

    print!("mode:-\n1. run test file\n2. run repl\n>> ");
    io::stdout().flush().unwrap();

    let _ = stdin.read_line(&mut buffer);
    match buffer.as_str() {
        "1\n" | "test\n" => {
            let data = fs::read_to_string("../TestProj/main.atoms").unwrap_or("5*5".to_string());

            dbg!(data.clone());
            run(data);
        }

        "2\n" | "repl\n" => loop {
            print!(">> ");
            io::stdout().flush().unwrap();

            let mut repl = String::with_capacity(4096);
            let _ = stdin.read_line(&mut repl);
            run(repl);
        },
        m => panic!("unknown mode {}", m),
    }
}
