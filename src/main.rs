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

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();

        let mut buffer = String::with_capacity(4096);
        let stdin = io::stdin();

        let _ = stdin.read_line(&mut buffer);

        let mut src = Source::new(buffer);

        println!("entered {}", src.code);
        let prog: Vec<Expr> = src.parse_prog();
        println!("{:#?}", prog);
    }
}
