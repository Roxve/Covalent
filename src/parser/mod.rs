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

        println!("l{:?}", self.next());
        loop {
            if let Token::Operator(c) = self.current() {
                let op_level = get_operator_level(c);
                if op_level < level {
                    break;
                }

                self.tokenize();
                right = self.parse_level(level + 1);

                //left = Expr::BinaryExpr(c, Box::new(left), Box::new(right.clone()));

                // swap left and right operators for right order
                let mut op1: char = c;
                let mut op2: Option<char> = None;
                let mut right_l: Option<Box<Expr>> = None;
                let mut right_r: Option<Box<Expr>> = None;
                // if let Expr::BinaryExpr(c1, _, _) = left {
                // op1 = Some(c);
                //}

                if let Expr::BinaryExpr(c2, rh_l, rh_r) = right.clone() {
                    op2 = Some(c2);
                    right_l = Some(rh_l);
                    right_r = Some(rh_r);
                }

                if (op2.is_some()) && (get_operator_level(op1) > get_operator_level(op2.unwrap())) {
                    right = Expr::BinaryExpr(op1, right_l.unwrap(), right_r.unwrap());
                    op1 = op2.unwrap();
                }
                // end swap
                // maybe not best thing but i cannot think of a better idea rn
                left = Expr::BinaryExpr(op1, Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        return left;
    }
}
