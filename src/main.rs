use std::io::{self, Write};
mod analysis;
mod backend;
mod compiler;
mod ir;
mod lexer;
mod parser;
mod source;
use std::path::Path;
// use std::process::Command;
use crate::compiler::{Backend, CSettings, CompilerConfig};
use std::{env, fs};
#[test]
fn test() {
    let prog = fs::read_to_string("TestProg/main.atoms").unwrap();

    CompilerConfig::new(
        prog,
        Backend::C(CSettings::new(None, Vec::new())),
        true,
        true,
        "/tmp/covalent/test.c".to_string(),
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
            Backend::C(CSettings::new(None, Vec::new())),
            is_debug,
            true,
            "repl".to_string(),
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

    let filename = path
        .file_name()
        .expect("file passed is a folder")
        .to_str()
        .unwrap()
        .to_string();
    CompilerConfig::new(
        prog.expect("invaild file name"),
        Backend::C(CSettings::new(None, Vec::new())),
        is_debug,
        false,
        filename.replace(".atoms", ""),
    )
    .run();
}
