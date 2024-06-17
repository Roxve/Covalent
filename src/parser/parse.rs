use crate::scope::Scope;

use super::ast::*;
use super::Parser;
use crate::err::ErrKind;

use crate::lexer::token::Token;

pub trait Parse {
    fn parse_prog(&mut self) -> Vec<Node>;
    fn parse_level(&mut self, level: u8) -> Node;

    fn parse_index(&mut self) -> Node;
    fn parse_call_fn(&mut self) -> Node;
    fn parse_member(&mut self) -> Node;

    fn parse_expr(&mut self) -> Node;

    fn parse_extern(&mut self) -> Node;
    fn parse_declare(&mut self) -> Node;
    fn parse_declare_fn(&mut self, id: Ident) -> Node;

    fn parse_if_expr(&mut self) -> Node;
    fn parse_while_expr(&mut self) -> Node;
    fn parse_ret_expr(&mut self) -> Node;

    fn parse_body(&mut self) -> Vec<Node>;
    fn parse_list(&mut self) -> Vec<Node>;
}

macro_rules! tmp {
    () => {
        untyped(Expr::Literal(Literal::Int(0)))
    };
}
impl Parse for Parser {
    fn parse_prog(&mut self) -> Vec<Node> {
        let mut body = Vec::new();
        while self.current() != Token::EOF {
            self.current_scope = Scope::Top;
            let mut expr = self.parse_level(0);

            if !self.current_scope.is_used() {
                expr = untyped(Expr::Discard(Box::new(expr)));
            }

            body.push(expr);
        }

        body
    }

    fn parse_level(&mut self, level: u8) -> Node {
        let mut left = self.parse_index();
        let mut right;

        loop {
            // 5 (2*) 5 nothing (1+) 5
            if let Token::Operator(c) = self.current() {
                if c == "=" {
                    self.next();
                    self.current_scope = Scope::Value;
                    let right = self.parse_level(0);

                    left = untyped(Expr::VarAssign {
                        name: Box::new(left),
                        val: Box::new(right),
                    });
                    break;
                }

                let current_op_level = get_operator_level(c.as_str());
                if current_op_level < level {
                    break;
                }

                self.next();
                right = self.parse_level(current_op_level + 1);

                left = untyped(Expr::BinaryExpr {
                    op: c,
                    left: Box::new(left),
                    right: Box::new(right),
                });
            } else {
                break;
            }
        }

        left
    }

    fn parse_index(&mut self) -> Node {
        let expr = self.parse_call_fn();

        if self.current() == Token::LeftBrace {
            self.next();
            let index = Box::new(self.parse_level(0));
            self.except(Token::RightBrace);

            return untyped(Expr::IndexExpr {
                parent: Box::new(expr),
                index,
            });
        }
        expr
    }

    fn parse_call_fn(&mut self) -> Node {
        let call = self.parse_member();
        if self.current() == Token::Colon {
            self.next();
            let args = self.parse_list();
            return untyped(Expr::FnCall {
                name: Box::new(call),
                args,
            });
        }

        if self.current() == Token::Exec {
            self.next();
            return untyped(Expr::FnCall {
                name: Box::new(call),
                args: Vec::new(),
            });
        }

        call
    }

    fn parse_member(&mut self) -> Node {
        let left = self.parse_expr();
        if self.current() == Token::Dot {
            self.next();
            let right = self.parse_expr();
            if let Expr::Ident(id) = right.expr {
                untyped(Expr::MemberExpr {
                    parent: Box::new(left),
                    child: id.val().clone(),
                })
            } else {
                self.err(
                    ErrKind::UnexceptedTokenE,
                    format!("expected id in member expr got {:?}", right),
                );
                untyped(Expr::Literal(Literal::Int(0)))
            }
        } else {
            left
        }
    }

    fn parse_list(&mut self) -> Vec<Node> {
        let mut args: Vec<Node> = Vec::new();

        args.push(self.parse_level(0));
        while self.current() == Token::Comma {
            self.next();
            args.push(self.parse_level(0));
        }

        args
    }

    fn parse_expr(&mut self) -> Node {
        let tok = self.current();
        match tok {
            Token::Int(i) => {
                self.next();
                untyped(Expr::Literal(Literal::Int(i)))
            }
            Token::Float(f) => {
                self.next();
                untyped(Expr::Literal(Literal::Float(f)))
            }
            Token::Bool(val) => {
                self.next();
                untyped(Expr::Literal(Literal::Bool(val)))
            }
            Token::Str(s) => {
                self.next();
                untyped(Expr::Literal(Literal::Str(s)))
            }

            Token::Err(_) => {
                todo!()
            }

            Token::Ident(id) => {
                self.next();
                if self.current() == Token::Dash {
                    if let Token::Ident(tag) = self.next() {
                        self.next();
                        untyped(Expr::Ident(Ident::Tagged(tag, id)))
                    } else {
                        self.err(
                            ErrKind::UnexceptedTokenE,
                            format!("unexpected token after '@' expected type to tag {}", id),
                        );

                        tmp!()
                    }
                } else {
                    untyped(Expr::Ident(Ident::UnTagged(id)))
                }
            }
            // Token::Tag(tag) => {
            //     self.next();
            //     if let Token::Ident(id) = self.current() {
            //         self.next();
            //         return Expr::Ident(Ident {
            //             tag: Some(tag.to_string()),
            //             val: id,
            //         });
            //     }
            //     todo!()
            // }
            Token::LeftParen => {
                self.next();
                let expr = self.parse_level(0);
                self.except(Token::RightParen);
                expr
            }

            Token::LeftBrace => {
                self.next();
                let values = self.parse_list();
                self.except(Token::RightBrace);
                untyped(Expr::ListExpr(values))
            }
            Token::UseKw => {
                if let Token::Str(path) = self.next() {
                    self.current_scope = Scope::Use;
                    self.next();
                    untyped(Expr::Use(path))
                } else {
                    let tok = self.current();
                    self.err(
                        ErrKind::UnexceptedTokenE,
                        format!("unexcepted token [{:#?}]", tok),
                    );
                    todo!()
                }
            }
            Token::ExternKw => self.parse_extern(),

            Token::SetKw => self.parse_declare(),
            Token::WhileKw => self.parse_while_expr(),
            Token::IfKw => self.parse_if_expr(),
            Token::RetKw => self.parse_ret_expr(),
            _ => {
                self.err(
                    ErrKind::UnexceptedTokenE,
                    format!("unexcepted token [{:#?}]", tok),
                );
                self.next();

                // todo!(); // add ERR TODO <-
                tmp!()
            }
        }
    }

    fn parse_extern(&mut self) -> Node {
        self.next();

        let name = self.parse_expr();

        if let Expr::Ident(id) = name.expr {
            if let Ident::Tagged(_, _) = id {
                let name = id;

                self.except(Token::Colon);

                let params = self.parse_list();

                let mut id_params = Vec::new();

                for (i, node) in params.iter().enumerate() {
                    if let Expr::Ident(ref id) = node.expr {
                        if let Ident::Tagged(_, _) = id {
                            id_params.push(id.clone());
                            continue;
                        }
                    }

                    self.err(
                        ErrKind::UnexceptedTokenE,
                        format!("expected a typed id as extern param {i}"),
                    );
                }
                let params = id_params;

                untyped(Expr::Extern { name, params })
            } else {
                self.err(
                    ErrKind::UnexceptedTokenE,
                    format!("expected a typed id as extern name"),
                );

                todo!()
            }
        } else {
            self.err(
                ErrKind::UnexceptedTokenE,
                format!("expected an id in extern"),
            );

            todo!()
        }
    }

    fn parse_declare(&mut self) -> Node {
        self.next();

        let left = self.parse_expr();
        self.current_scope = Scope::Value;
        if let Expr::Ident(name) = left.expr {
            if Token::Operator("=".to_string()) == self.current() {
                self.next();

                let expr = self.parse_level(0);
                return untyped(Expr::VarDeclare {
                    name,
                    val: Box::new(expr),
                });
            }

            self.parse_declare_fn(name)
        } else {
            self.err(
                ErrKind::UnexceptedTokenE,
                format!(
                    "unexcept token in set expression [{:?}] excepted an id",
                    left
                ),
            );

            left
        }
    }
    fn parse_declare_fn(&mut self, id: Ident) -> Node {
        let mut id_args: Vec<Ident> = Vec::new();

        if self.current() == Token::Colon {
            self.next();
            let args = self.parse_list();

            for arg in args {
                if let Expr::Ident(id) = arg.expr {
                    id_args.push(id);
                } else {
                    self.err(
                        ErrKind::UnexceptedArgs,
                        "excepted an id for arg".to_string(),
                    );
                    return self.parse_level(0);
                }
            }
        } else {
            self.except(Token::Exec);
        }
        let body = self.parse_body();

        self.push_function(id.clone(), id_args, body);
        self.current_scope = Scope::Value;
        untyped(Expr::PosInfo(id.val().clone(), self.line, self.column))
    }

    fn parse_if_expr(&mut self) -> Node {
        self.next(); // remove if
        self.current_scope = Scope::Value;
        let condition = self.parse_level(0);
        let body = self.parse_body();

        let mut alt: Option<Box<Node>> = None;
        if self.current() == Token::ElseKw {
            self.next();
            if self.current() == Token::IfKw {
                alt = Some(Box::new(self.parse_if_expr()));
            } else {
                alt = Some(Box::new(untyped(Expr::Block(self.parse_body()))));
            }
        }

        untyped(Expr::IfExpr {
            condition: Box::new(condition),
            body,
            alt,
        })
    }
    fn parse_while_expr(&mut self) -> Node {
        self.next();
        self.current_scope = Scope::Value;
        let condition = self.parse_level(0);
        let body = self.parse_body();

        untyped(Expr::WhileExpr {
            condition: Box::new(condition),
            body,
        })
    }

    #[inline]
    fn parse_body(&mut self) -> Vec<Node> {
        let mut body = vec![];

        self.except(Token::LeftBracket);
        while self.current() != Token::RightBracket && self.current() != Token::EOF {
            self.current_scope = Scope::Top;
            let mut expr = self.parse_level(0);

            if !self.current_scope.is_used() {
                expr = untyped(Expr::Discard(Box::new(expr)));
            }

            body.push(expr);
        }
        self.except(Token::RightBracket);

        body
    }

    fn parse_ret_expr(&mut self) -> Node {
        self.next();
        self.current_scope = Scope::Value;
        let expr = self.parse_level(0);
        untyped(Expr::RetExpr(Box::new(expr)))
    }
}
