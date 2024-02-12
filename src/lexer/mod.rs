use crate::source::*;

pub fn is_num(c: char) -> bool {
    return "01234.56789".contains(c);
}
pub fn is_id(c: char) -> bool {
    return !(" \t\n+-*/<&|>=@#%:!?$,[{('`)}]").contains(c);
}

pub trait Tokenizer {
    fn eat(&mut self) -> char;
    fn at(&self) -> char;
    fn set(&mut self, tok: Token) -> Token;
    fn parse_num(&mut self, x: String) -> Token;
    fn tokenize(&mut self) -> Token;
}

impl Tokenizer for Source<'_> {
    fn eat(&mut self) -> char {
        let p = self.at();
        self.code.remove(0);
        self.column += 1;
        return p;
    }

    fn at(&self) -> char {
        return self.code.as_bytes()[0] as char;
    }

    fn set(&mut self, tok: Token) -> Token {
        self.current_tok = self.next_tok.clone();
        self.next_tok = Some(tok.clone());
        return tok;
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
                    self.column = 0;
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

            '"' | '\'' => {
                let op = self.eat();

                let start_line = self.line;
                let start_column = self.column;

                let mut res = String::from("");

                while self.code.len() > 0 && self.at() != op {
                    res.push(self.eat());
                }

                if self.code.len() > 0 && self.at() == op {
                    self.eat();
                } else {
                    self.err(
                        ErrKind::UnknownCharE,
                        format!(
                            "reached end of file and didnt finish string started at line {}, colmun {}",
                            start_line,
                            start_column
                        ),
                    );
                }

                return self.set(Token::Str(res));
            }
            '=' => {
                self.eat();
                if self.code.len() > 0 && self.at() == '=' {
                    self.eat();
                    return self.set(Token::Operator("==".to_string()));
                }

                return self.set(Token::Operator("=".to_string()));
            }
            '+' | '-' | '*' | '/' | '^' | '<' | '>' | '&' | '|' => {
                let op = self.eat();
                return self.set(Token::Operator(op.to_string()));
            }

            '(' => {
                self.eat();
                return self.set(Token::LeftParen);
            }

            ')' => {
                self.eat();
                return self.set(Token::RightParen);
            }

            '{' => {
                self.eat();
                return self.set(Token::LeftBracket);
            }

            '}' => {
                self.eat();
                return self.set(Token::RightBracket);
            }

            ':' => {
                self.eat();
                return self.set(Token::Colon);
            }

            ',' => {
                self.eat();
                return self.set(Token::Comma);
            }

            c => {
                if is_id(c) {
                    let mut res = String::from("");
                    while is_id(self.at()) {
                        res.push(self.eat());
                    }

                    match res.as_str() {
                        // keywords
                        "set" => self.set(Token::SetKw),
                        // tags(types(old WIP))
                        "__int__" => self.set(Token::Tag("int".to_string())),
                        "__float__" => self.set(Token::Tag("float".to_string())),
                        // bools
                        "true" => self.set(Token::Bool(true)),
                        "false" => self.set(Token::Bool(false)),
                        // operations
                        "==" => self.set(Token::Operator(res)),
                        _ => self.set(Token::Ident(res)),
                    }
                } else {
                    let c = self.eat();
                    self.err(ErrKind::UnknownCharE, format!("unknown char '{}'", c));

                    // to remove?
                    return self.set(Token::Err(format!(
                        "AT0001::UNKNOWN_CHAR::{}::{}",
                        self.line, self.column
                    )));
                }
            }
        }
    }
}
