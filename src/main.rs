use std::io::{self, Write};
mod ast;
mod codegen;
mod lexer;
mod parser;
mod source;

use std::path::Path;

use crate::ast::*;
use crate::codegen::*;
use crate::parser::*;
use crate::source::*;
use inkwell::context::Context;
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

        let res = src.compile_prog(prog);

        let _ = src
            .builder
            .build_return(Some(&src.context.i32_type().const_int(0, true)));
        src.module.verify().expect("invaild");

        src.module.print_to_stderr();

        let path = Path::new("test.ll");
        src.module.write_bitcode_to_path(path);
        println!("{:#?}", res);
    }
}
