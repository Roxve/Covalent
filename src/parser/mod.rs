use crate::ast::*;
use crate::lexer::Tokenizer;
use crate::source::*;

pub trait ParserError {}

pub trait Parser {
    fn next(&mut self) -> Token;
    fn current(&mut self) -> Token;
    fn except(&mut self, tok: Token) -> Token; // || NULL;
    fn parse_prog(&mut self) -> Vec<Expr>;
    fn parse_level(&mut self, level: u8) -> Expr;
    fn parse_declare(&mut self) -> Expr;
    fn parse_expr(&mut self) -> Expr;
}

impl Parser for Source<'_> {
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

    fn except(&mut self, tok: Token) -> Token {
        if self.current() != tok {
            let t = self.current();
            self.tokenize();

            self.err(
                ErrKind::UnexceptedTokenE,
                format!("unexcepted token [{:?}] excepted [{:?}]", t, tok),
            );
            return Token::Err("unexcepted token".to_string());
        }

        return self.tokenize();
    }

    fn parse_prog(&mut self) -> Vec<Expr> {
        let mut body: Vec<Expr> = Vec::new();
        while self.current() != Token::EOF {
            body.push(self.parse_level(0));
        }

        return body;
    }

    fn parse_level(&mut self, level: u8) -> Expr {
        let mut left: Expr = self.parse_expr();
        let mut right: Expr;

        loop {
            // 5 (2*) 5 nothing (1+) 5
            if let Token::Operator(c) = self.current() {
                if c == "=" {
                    if let Expr::Ident(id) = left {
                        self.tokenize();

                        let right = self.parse_level(0);

                        left = Expr::VarAssign(id, Box::new(right));
                        break;
                    }

                    self.err(
                        ErrKind::UnexceptedTokenE,
                        "unexcepted token equal '=' which is used in assignment expr".to_string(),
                    );
                    return left;
                }

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
            Token::Ident(id) => {
                self.tokenize();
                Expr::Ident(Ident(id))
            }
            Token::SetKw => self.parse_declare(),
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

    fn parse_declare(&mut self) -> Expr {
        self.tokenize(); // n->t

        let left = self.parse_expr();

        if let Expr::Ident(id) = left {
            self.except(Token::Operator("=".to_string()));

            let expr = self.parse_level(0);
            return Expr::VarDeclare(id, Box::new(expr));
        } else {
            self.err(
                ErrKind::UnexceptedTokenE,
                format!(
                    "unexcept token in set expression [{:?}] excepted an id",
                    left
                ),
            );

            return left;
        }
    }
}
