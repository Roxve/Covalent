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

        println!("{:#?}", body);
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
            if let Token::Operator(c) = self.current() {
                let current_op_level = get_operator_level(c);
                if current_op_level < level {
                    break;
                }

                self.tokenize();
                right = self.parse_level(level + 1);

                // swap left and right operators for right order
                let mut left_op: char = c;
                let mut right_op: Option<char> = None;
                let mut right_l: Option<Box<Expr>> = None;
                let mut right_r: Option<Box<Expr>> = None;

                if let Expr::BinaryExpr(op, l_e, r_e) = right.clone() {
                    right_op = Some(op);
                    right_l = Some(l_e);
                    right_r = Some(r_e);
                }

                if (right_op.is_some())
                    && (get_operator_level(left_op) > get_operator_level(right_op.unwrap()))
                {
                    right = Expr::BinaryExpr(left_op, right_l.unwrap(), right_r.unwrap());
                    left_op = right_op.unwrap();
                }
                // end swap
                // maybe not best thing but i cannot think of a better idea rn (works without swap but miss up in some things)
                left = Expr::BinaryExpr(left_op, Box::new(left), Box::new(right));
            } else {
                break;
            }
        }

        return left;
    }
}
