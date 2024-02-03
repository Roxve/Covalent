use std::io::{self, Write};
mod ast;
mod codegen;
mod lexer;
mod parser;
mod source;

use crate::ast::*;
use crate::codegen::*;
use crate::parser::*;
use crate::source::*;
use inkwell::context::Context;
use inkwell::execution_engine::ExecutionEngine;
use inkwell::execution_engine::JitFunction;

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::with_capacity(4096);
        let stdin = io::stdin();

        let _ = stdin.read_line(&mut buffer);

        let ctx = &Context::create();
        let mut src = Source::new(buffer, ctx);

        println!("entered {}", src.code);
        let prog: Vec<Expr> = src.parse_prog();
        println!("{:#?}", prog);

        let main_fn_type = src.context.void_type().fn_type(&[], false);
        let main_fn = src.module.add_function("main", main_fn_type, None);
        let main = src.context.append_basic_block(main_fn, "entry");
        let res = src.compile_prog(prog, main);

        println!("{:#?}", res);
        println!("{:#?}", main_fn);
    }
}
