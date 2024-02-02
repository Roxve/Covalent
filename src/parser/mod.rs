use crate::ast::*;
use crate::lexer::Tokenizer;
use crate::source::*;

pub trait ParserError {}

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
            body.push(self.parse_level(0));
        }

        return body;
    }

    fn parse_expr(&mut self) -> Expr {
        let tok = self.current();
        match tok {
            Token::Int(i) => {
                self.tokenize();
                Expr::Literal(Literal::Int(i))
            }
            Token::Float(f) => {
                self.tokenize();
                Expr::Literal(Literal::Float(f))
            }
            Token::Err(_) => {
                todo!()
            }
            _ => {
                self.err(
                    ErrKind::UnexceptedTokenE,
                    format!("unexcepted token [{:#?}]", tok),
                );
                self.tokenize();

                // todo!(); // add null
                Expr::Literal(Literal::Int(0))
            }
        }
    }

    fn parse_level(&mut self, level: u8) -> Expr {
        let mut left: Expr = self.parse_expr();
        let mut right: Expr;

        loop {
            // 5 (2*) 5 nothing (1+) 5
            if let Token::Operator(c) = self.current() {
                let current_op_level = get_operator_level(c.as_str());
                if current_op_level < level {
                    break;
                }

                self.tokenize();
                right = self.parse_level(current_op_level + 1);

                left = Expr::BinaryExpr(c, Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        return left;
    }
}
