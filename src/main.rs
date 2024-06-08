use std::env::current_exe;
use std::io::{self, Write};
mod analysis;
mod backend;
mod compiler;
mod enviroment;
mod err;
mod ir;
mod lexer;
mod parser;
mod scope;
mod types;

use std::path::Path;
// use std::process::Command;
use crate::compiler::{Backend, CSettings, CompilerConfig};
use std::{env, fs, process::Command};
#[test]
fn test() {
    let path = "TestProg/main.atoms";
    let prog = fs::read_to_string(path).unwrap();

    CompilerConfig::new(
        prog,
        Backend::C(CSettings::new(None, Vec::new())),
        true,
        "/tmp/covalent/test.c".to_string(),
        path.to_string(),
    )
    .compile();
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
            "/tmp/covalent/repl".to_string(),
            current_exe().unwrap().to_str().unwrap().to_string(),
        )
        .compile();
        let _ = Command::new("/tmp/covalent/repl")
            .spawn()
            .expect("failed to execute repl exe")
            .wait();
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

            if args.len() == 0 {
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
        filename.replace(".atoms", ""),
        path.parent()
            .unwrap_or(&Path::new(""))
            .to_str()
            .unwrap()
            .to_string(),
    )
    .compile();
}
