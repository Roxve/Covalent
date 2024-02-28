use std::io::{self, Write};
mod ast;
mod ir;
mod lexer;
mod parser;
mod source;

use std::path::Path;
use std::{env, fs};

use ir::gen::IRGen;

use crate::ast::*;
use crate::parser::*;
use crate::source::*;

fn run(input: String, is_debug: bool, is_repl: bool, name: String) {
    let mut src = Source::new(input);

    let prog: Vec<Expr> = src.parse_prog();
    if is_debug {
        println!("parsed prog:\n {:#?}\nsrc: \n{:#?}", prog, src);
    }
    let gen = src.gen_prog(prog);
    dbg!(&gen);
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
