use crate::source::*;

pub fn is_num(c: char) -> bool {
    return "01234.56789".contains(c);
}

pub trait Tokenizer {
    fn eat(&mut self) -> char;
    fn at(&self) -> char;
    fn set(&mut self, tok: Token) -> Token;
    fn current(&mut self) -> Token;
    fn parse_num(&mut self, x: String) -> Token;
    fn tokenize(&mut self) -> Token;
}

impl Tokenizer for Source {
    fn eat(&mut self) -> char {
        let p = self.at();
        self.code.remove(0);
        self.colmun += 1;
        return p;
    }

    fn at(&self) -> char {
        return self.code.as_bytes()[0] as char;
    }

    fn set(&mut self, tok: Token) -> Token {
        self.current_tok = Some(tok.clone());
        return tok;
    }

    fn current(&mut self) -> Token {
        return self.current_tok.clone().expect("None");
    }

    fn parse_num(&mut self, x: String) -> Token {
        if x.contains('.') {
            return self.set(Token::Float(x.parse().unwrap()));
        }
        return self.set(Token::Int(x.parse().unwrap()));
    }

    fn tokenize(&mut self) -> Token {
        loop {
            if self.code.len() <= 0 {
                return self.set(Token::EOF);
            }
            match self.at() {
                ' ' | '\t' => {
                    self.eat();
                }
                '\n' => {
                    self.eat();
                    self.colmun = 0;
                    self.line += 1;
                }
                _ => break,
            }
        }

        match self.at() {
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut res = String::from("");
                while is_num(self.at()) {
                    res.push(self.eat())
                }
                return self.parse_num(res);
            }
            '+' | '-' | '*' | '/' | '^' => {
                let op = self.eat();
                return self.set(Token::Operator(op));
            }
            _ => {
                if false {
                    return self.set(Token::Err("how did we get here?".to_string()));
                } else {
                    let c = self.eat();
                    return self.set(Token::Err(format!(
                        "AT0001::UNKNOWN_CHAR::{}::{}:{}",
                        c, self.line, self.colmun
                    )));
                }
            }
        }
    }
}
