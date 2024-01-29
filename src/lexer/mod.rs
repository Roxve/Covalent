use std::io::{self};

use crate::source::*;

pub fn is_num(c: char) -> bool {
    return "0123456789".contains(c);
}

pub trait Tokenizer {
    fn eat(&mut self) -> char;
    fn at(&self) -> char;
    fn set(&mut self, tok: Token);
    fn current(&mut self) -> Token;
    fn tokenize(&mut self) -> Result<u32, String>;
}

impl Tokenizer for Source {
    fn eat(&mut self) -> char {
        let p = self.at();
        self.code.remove(0);
        return p;
    }

    fn at(&self) -> char {
        return self.code.as_bytes()[0] as char;
    }

    fn set(&mut self, tok: Token) {
        self.current_tok = Some(tok);
    }

    fn current(&mut self) -> Token {
        return self.current_tok.clone().expect("None");
    }

    fn tokenize(&mut self) -> Result<u32, String> {
        while self.code.len() > 0 && (self.at() == ' ' || self.at() == '\t' || self.at() == '\n') {
            while self.code.len() > 0 && (self.at() == ' ' || self.at() == '\t') {
                self.colmun += 1;
                self.eat();
                continue;
            }

            while self.code.len() > 0 && self.at() == '\n' {
                self.eat();
                self.colmun = 1;
                self.line += 1;
                continue;
            }
        }

        if self.code.len() <= 0 {
            self.set(Token::EOF);
            return Ok(0);
        }

        match self.at() {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut res = String::from("");
                while is_num(self.at()) {
                    res.push(self.eat())
                }
                self.set(Token::Number(res));
                return Ok(1);
            }
            '+' | '-' | '*' | '/' | '^' => {
                let op = self.eat();
                self.set(Token::Operator(op));
                return Ok(2);
            }
            _ => {
                if false {
                    return Err("how did we get here?".to_string());
                } else {
                    let c = self.eat();
                    return Err(format!("AT0001::UNKNOWN_CHAR_{}", c));
                }
            }
        }
    }
}
