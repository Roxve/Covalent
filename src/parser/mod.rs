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
    fn parse_declare_fn(&mut self, id: Ident) -> Expr;
    fn parse_call_fn(&mut self) -> Expr;
    fn parse_list(&mut self) -> Vec<Expr>;
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
        let mut left: Expr = self.parse_call_fn();
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

    fn parse_call_fn(&mut self) -> Expr {
        let call = self.parse_expr();

        if let Expr::Ident(id) = &call {
            if self.current() == Token::Colon {
                self.tokenize();
                let args = self.parse_list();
                return Expr::FnCall(id.to_owned(), args);
            }
        }
        return call;
    }

    fn parse_list(&mut self) -> Vec<Expr> {
        let mut args: Vec<Expr> = Vec::new();

        args.push(self.parse_level(0));
        while self.current() == Token::Comma {
            self.tokenize();
            args.push(self.parse_level(0));
        }

        return args;
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
            Token::Str(s) => {
                self.tokenize();
                Expr::Literal(Literal::Str(s))
            }
            Token::Err(_) => {
                todo!()
            }
            Token::Ident(id) => {
                self.tokenize();
                Expr::Ident(Ident(id))
            }
            Token::Tag(tag) => {
                self.tokenize();
                if let Token::Ident(id) = self.current() {
                    self.tokenize();
                    return Expr::TaggedIdent(Tag(tag.to_string()), Ident(id));
                }
                todo!()
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
            if Token::Operator("=".to_string()) == self.current() {
                self.tokenize();

                let expr = self.parse_level(0);
                return Expr::VarDeclare(id, Box::new(expr));
            }

            return self.parse_declare_fn(id);
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
    fn parse_declare_fn(&mut self, id: Ident) -> Expr {
        let mut body: Vec<Expr> = Vec::new();

        let mut id_args: Vec<Ident> = Vec::new();

        if self.current() == Token::Colon {
            self.tokenize();
            let args = self.parse_list();

            for arg in args {
                if let Expr::Ident(id) = arg {
                    id_args.push(id);
                } else {
                    self.err(
                        ErrKind::UnexceptedArgs,
                        "excepted an id for arg".to_string(),
                    );
                    return self.parse_level(0);
                }
            }
        }
        self.except(Token::LeftBracket);
        while self.current() != Token::RightBracket && self.current() != Token::EOF {
            body.push(self.parse_level(0));
        }
        self.except(Token::RightBracket);

        self.push_function(id, id_args, body);
        return self.parse_level(0);
    }
}
