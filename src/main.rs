use std::io::{self, Write};
mod lexer;
mod source;

use crate::lexer::*;
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
        let mut tokens: Vec<Token> = Vec::new();
        while src.tokenize() != Ok(0) {
            let t = src.current();
            println!("is {:#?}", t);
            tokens.push(t);
        }
    }
}
