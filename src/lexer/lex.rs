use super::*;
use crate::source::ErrKind;

pub fn is_num(c: char) -> bool {
    return "01234.56789".contains(c);
}
pub fn is_id(c: char) -> bool {
    return !(" \t\n+-*/<&|>=@#%:!?$,[{('`)}]").contains(c);
}

impl Lexer {
    pub fn parse_num(&mut self, x: String) -> Token {
        if x.contains('.') {
            return Token::Float(x.parse().unwrap());
        }
        return Token::Int(x.parse().unwrap());
    }

    pub fn tokenize(&mut self) -> Token {
        loop {
            if !self.not_eof() {
                return Token::EOF;
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
            '#' => {
                while self.not_eof() && self.at() != '\n' {
                    self.eat();
                }
                self.line += 1;
                self.tokenize()
            }
            '0' | '1' | '2' | '3' | '4' | '5' | '6' | '7' | '8' | '9' => {
                let mut res = String::from("");
                while self.not_eof() && is_num(self.at()) {
                    res.push(self.eat())
                }
                self.parse_num(res)
            }

            '"' | '\'' => {
                let op = self.eat();

                let start_line = self.line;
                let start_column = self.column;

                let mut res = String::from("");

                while self.not_eof() && self.at() != op {
                    res.push(self.eat());
                }

                if self.not_eof() && self.at() == op {
                    self.eat();
                } else {
                    return self.err(
                        format!(
                            "reached end of file and didnt finish string started at line {}, colmun {}",
                            start_line,
                            start_column
                        ),
                        ErrKind::UnknownCharE
                    );
                }

                Token::Str(res)
            }
            '=' => {
                self.eat();
                if self.not_eof() && self.at() == '=' {
                    self.eat();
                    Token::Operator("==".to_string())
                } else {
                    Token::Operator("=".to_string())
                }
            }

            '-' => {
                self.eat();
                if self.not_eof() && self.at() == '>' {
                    self.eat();
                    Token::Access
                } else {
                    Token::Operator('-'.to_string())
                }
            }

            '&' | '|' => {
                let mut op = self.eat().to_string();
                if self.not_eof() && self.at() == op.as_bytes()[0].into() {
                    op.push(self.eat());
                }
                Token::Operator(op)
            }
            '+' | '*' | '/' | '%' | '^' => {
                let op = self.eat();
                Token::Operator(op.to_string())
            }

            '<' | '>' => {
                let op = self.eat();

                if self.not_eof() && self.at() == '=' {
                    let mut op = op.to_string();
                    op.push(self.eat());
                    Token::Operator(op)
                } else {
                    Token::Operator(op.to_string())
                }
            }

            '(' => {
                self.eat();
                Token::LeftParen
            }

            ')' => {
                self.eat();
                Token::RightParen
            }

            '{' => {
                self.eat();
                Token::LeftBracket
            }

            '}' => {
                self.eat();
                Token::RightBracket
            }

            '[' => {
                self.eat();
                Token::LeftBrace
            }

            ']' => {
                self.eat();
                Token::RightBrace
            }

            ':' => {
                self.eat();
                Token::Colon
            }

            ',' => {
                self.eat();
                Token::Comma
            }

            '.' => {
                self.eat();
                Token::Dot
            }

            '!' => {
                self.eat();
                Token::Exec
            }

            c => {
                if is_id(c) {
                    let mut res = String::from("");
                    while is_id(self.at()) {
                        res.push(self.eat());
                    }

                    match res.as_str() {
                        // keywords
                        "set" => Token::SetKw,
                        "if" => Token::IfKw,
                        "else" => Token::ElseKw,
                        "while" => Token::WhileKw,
                        "break" => Token::BreakKw,
                        "continue" => Token::Continuekw,
                        "ret" => Token::RetKw,
                        // tags(types(old WIP))
                        "__int__" => Token::Tag("int".to_string()),
                        "__float__" => Token::Tag("float".to_string()),
                        // bools
                        "true" => Token::Bool(true),
                        "false" => Token::Bool(false),
                        _ => Token::Ident(res),
                    }
                } else {
                    let c = self.eat();
                    self.err(format!("unknown char '{}'", c), ErrKind::UnknownCharE)
                }
            }
        }
    }
}
