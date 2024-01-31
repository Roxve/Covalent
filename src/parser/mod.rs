use crate::ast::*;
use crate::lexer::Tokenizer;
use crate::source::*;

pub trait Parser {
    fn next(&mut self) -> Token;
    fn current(&mut self) -> Token;
    fn parse_prog(&mut self) -> Vec<Expr>;
    fn parse_expr(&mut self) -> Expr;
    fn parse_level(&mut self, level: u8) -> Expr;
}

impl Parser for Source {
    fn next(&mut self) -> Token {
        if self.next_tok.is_none() {
            if self.current_tok.is_none() {
                self.tokenize();
            }
            self.current_tok = self.next_tok.clone();
        }

        return self.next_tok.clone().expect("None");
    }

    fn current(&mut self) -> Token {
        if self.current_tok.is_none() {
            self.tokenize();
            self.current_tok = self.next_tok.clone();
            self.tokenize();
        }
        return self.current_tok.clone().expect("None");
    }

    fn parse_prog(&mut self) -> Vec<Expr> {
        let mut body: Vec<Expr> = Vec::new();
        while self.current() != Token::EOF {
            println!("{:#?}:{:?}:{:?}", body, self.current(), self.next());
            body.push(self.parse_level(0));
        }
        return body;
    }

    fn parse_expr(&mut self) -> Expr {
        println!("e{:?}", self.current());
        match self.current() {
            Token::Int(i) => {
                self.tokenize();
                Expr::Literal(Literal::Int(i))
            }
            Token::Float(f) => {
                self.tokenize();
                Expr::Literal(Literal::Float(f))
            }
            Token::EOF => Expr::Literal(Literal::Int(0)),
            _ => self.parse_level(1),
        }
    }

    fn parse_level(&mut self, level: u8) -> Expr {
        let mut left: Expr = self.parse_expr();
        let mut right: Expr;
        let mut op: Operator;

        println!("l{:?}", self.next());
        loop {
            if let Token::Operator(c) = self.current() {
                let op_level = get_operator_level(c);
                if op_level < level {
                    break;
                }

                op = match c {
                    '+' => Operator::Plus,
                    '-' => Operator::Minus,
                    '*' => Operator::Multi,
                    '/' => Operator::Divide,
                    _ => Operator::Divide,
                };
                self.tokenize();
                right = self.parse_level(level + 1);
                left = Expr::BinaryExpr(op, Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        return left;
    }
}
