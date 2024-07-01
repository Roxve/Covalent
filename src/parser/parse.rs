use crate::scope::Scope;

use super::ast::*;
use super::Parser;
use crate::err::ErrKind;

use crate::lexer::token::Token;

impl Parser {
    // consumes a parser and returns a program
    pub fn parse_prog(mut self) -> Program {
        while self.current() != Token::EOF {
            self.current_scope = Scope::Top;
            let expr = self.parse_level(Precedence::Low);
            if expr.is_ok() {
                let mut expr = expr.unwrap();

                if !self.current_scope.is_used() {
                    expr = self.untyped(Expr::Discard(Box::new(expr)));
                }

                self.program.body.push(expr);
            }
        }

        self.program
    }

    fn parse_level(&mut self, precedence: Precedence) -> Result<Node, ()> {
        let mut left = self.parse_index()?;
        let mut right;

        loop {
            // 5 (2*) 5 nothing (1+) 5
            if let Token::Operator(c) = self.current() {
                if c == "=" {
                    self.next();
                    self.current_scope = Scope::Value;
                    let right = self.parse_level(Precedence::Low)?;

                    left = self.untyped(Expr::VarAssign {
                        name: Box::new(left),
                        val: Box::new(right),
                    });
                    break;
                }

                let current_op_level = get_operator_level(c.as_str());
                if !(current_op_level > precedence) {
                    break;
                }

                self.next();
                right = self.parse_level(current_op_level)?;

                left = self.untyped(Expr::BinaryExpr {
                    op: c,
                    left: Box::new(left),
                    right: Box::new(right),
                });
            } else {
                break;
            }
        }

        Ok(left)
    }

    fn parse_index(&mut self) -> Result<Node, ()> {
        let expr = self.parse_call_fn()?;

        if self.current() == Token::LeftBrace {
            self.next();
            let index = Box::new(self.parse_level(Precedence::Low)?);
            self.except(Token::RightBrace);

            return Ok(self.untyped(Expr::IndexExpr {
                parent: Box::new(expr),
                index,
            }));
        }
        Ok(expr)
    }

    fn parse_call_fn(&mut self) -> Result<Node, ()> {
        let call = self.parse_spec()?;
        if self.current() == Token::Colon {
            self.next();
            let args = self.parse_list()?;
            return Ok(self.untyped(Expr::FnCall {
                name: Box::new(call),
                args,
            }));
        }

        if self.current() == Token::Exec {
            self.next();
            return Ok(self.untyped(Expr::FnCall {
                name: Box::new(call),
                args: Vec::new(),
            }));
        }

        Ok(call)
    }

    fn parse_spec(&mut self) -> Result<Node, ()> {
        let mut left = self.parse_member()?;

        if self.current() == Token::LeftParen && matches!(left.expr, Expr::Ident(_)) {
            self.next();
            let spec = self.parse_list_of(Self::parse_spec, |_| true, Token::RightParen)?;

            left = self.untyped(Expr::SpecExpr {
                parent: Box::new(left),
                spec,
            });
            self.except(Token::RightParen);
        }

        Ok(left)
    }

    fn parse_member(&mut self) -> Result<Node, ()> {
        let left = self.parse_expr()?;
        if self.current() == Token::Dot {
            self.next();
            let right = self.parse_expr()?;
            if let Expr::Ident(id) = right.expr {
                Ok(self.untyped(Expr::MemberExpr {
                    parent: Box::new(left),
                    child: id.val().clone(),
                }))
            } else {
                self.err(
                    ErrKind::UnexceptedTokenE,
                    format!("expected id in member expr got {:?}", right),
                );
                Ok(self.untyped(Expr::Literal(Literal::Int(0))))
            }
        } else {
            Ok(left)
        }
    }

    fn parse_list(&mut self) -> Result<Vec<Node>, ()> {
        self.parse_list_of(
            |this| this.parse_level(Precedence::Low),
            |_| true,
            Token::EOF,
        )
    }

    fn parse_list_of<F, OF>(&mut self, func: F, of: OF, term: Token) -> Result<Vec<Node>, ()>
    where
        F: Fn(&mut Self) -> Result<Node, ()>,
        OF: Fn(&Node) -> bool,
    {
        let mut items: Vec<Node> = Vec::new();

        loop {
            if self.current() == term {
                break;
            }

            let item = func(self)?;

            if of(&item) {
                items.push(item);
            } else {
                self.err(ErrKind::UnexceptedTokenE, format!("unexpected expr {item}"));
                return Err(());
            }

            if self.current() != Token::Comma {
                break;
            }
            self.next();
        }

        Ok(items)
    }

    fn parse_expr(&mut self) -> Result<Node, ()> {
        let tok = self.current();
        match tok {
            Token::Int(i) => {
                self.next();
                Ok(self.untyped(Expr::Literal(Literal::Int(i))))
            }
            Token::Float(f) => {
                self.next();
                Ok(self.untyped(Expr::Literal(Literal::Float(f))))
            }
            Token::Bool(val) => {
                self.next();
                Ok(self.untyped(Expr::Literal(Literal::Bool(val))))
            }
            Token::Str(s) => {
                self.next();
                Ok(self.untyped(Expr::Literal(Literal::Str(s))))
            }

            Token::Err(_) => Err(()),

            Token::Ident(id) => {
                self.next();
                if self.current() == Token::Dash {
                    self.next();
                    let tag = self.parse_spec()?;

                    Ok(self.untyped(Expr::Ident(Ident::Tagged(Box::new(tag), id))))
                } else {
                    Ok(self.untyped(Expr::Ident(Ident::UnTagged(id))))
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
                let expr = self.parse_level(Precedence::Low);
                self.except(Token::RightParen);
                expr
            }

            Token::LeftBrace => {
                self.next();
                let values = self.parse_list()?;
                self.except(Token::RightBrace);
                Ok(self.untyped(Expr::ListExpr(values)))
            }
            Token::UseKw => {
                if let Token::Str(path) = self.next() {
                    self.current_scope = Scope::Use;
                    self.next();
                    Ok(self.untyped(Expr::Use(path)))
                } else {
                    let tok = self.current();
                    self.err(
                        ErrKind::UnexceptedTokenE,
                        format!("unexcepted token [{:#?}]", tok),
                    );
                    Err(())
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
                Err(())
            }
        }
    }

    fn parse_extern(&mut self) -> Result<Node, ()> {
        self.next();

        let name = self.parse_expr()?;

        if let Expr::Ident(id) = name.expr {
            if let Ident::Tagged(_, _) = id {
                let name = id;

                self.except(Token::Colon);

                let params = self.parse_list()?;

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

                Ok(self.untyped(Expr::Extern { name, params }))
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

    fn parse_declare(&mut self) -> Result<Node, ()> {
        self.next();

        let left = self.parse_expr()?;
        self.current_scope = Scope::Value;
        if let Expr::Ident(name) = left.expr {
            if Token::Operator("=".to_string()) == self.current() {
                self.next();

                let expr = self.parse_level(Precedence::Low)?;
                Ok(self.untyped(Expr::VarDeclare {
                    name,
                    val: Box::new(expr),
                }))
            } else if self.current() == Token::LeftBracket {
                self.parse_declare_atom(name)
            } else {
                self.parse_declare_fn(name)
            }
        } else {
            self.err(
                ErrKind::UnexceptedTokenE,
                format!(
                    "unexcept token in set expression [{:?}] excepted an id",
                    left
                ),
            );

            Ok(left)
        }
    }

    fn parse_declare_atom(&mut self, name: Ident) -> Result<Node, ()> {
        let name = match name {
            Ident::UnTagged(id) => id,
            _ => {
                self.err(
                    ErrKind::UnexceptedTokenE,
                    format!("expected an untagged id as a name for Atom in Atom declare"),
                );
                return Err(());
            }
        };

        self.next();

        let fields = self.parse_list_of(
            Self::parse_expr,
            |node| {
                if let Expr::Ident(Ident::Tagged(_, _)) = node.expr {
                    return true;
                }
                false
            },
            Token::RightBracket,
        )?;

        self.except(Token::RightBracket);

        Ok(self.untyped(Expr::AtomDeclare { name, fields }))
    }

    fn parse_declare_fn(&mut self, id: Ident) -> Result<Node, ()> {
        let mut id_args: Vec<Ident> = Vec::new();

        if self.current() == Token::Colon {
            self.next();
            let args = self.parse_list()?;

            for arg in args {
                if let Expr::Ident(id) = arg.expr {
                    id_args.push(id);
                } else {
                    self.err(
                        ErrKind::UnexceptedArgs,
                        "excepted an id for arg".to_string(),
                    );
                    return self.parse_level(Precedence::Low);
                }
            }
        } else {
            self.except(Token::Exec);
        }
        let body = self.parse_body();

        self.push_function(id.clone(), id_args, body);
        self.current_scope = Scope::Value;
        Ok(self.untyped(Expr::PosInfo(id.val().clone(), self.line, self.column)))
    }

    fn parse_if_expr(&mut self) -> Result<Node, ()> {
        self.next(); // remove if
        self.current_scope = Scope::Value;
        let condition = self.parse_level(Precedence::Low)?;
        let body = self.parse_body();

        let mut alt: Option<Box<Node>> = None;
        if self.current() == Token::ElseKw {
            self.next();
            if self.current() == Token::IfKw {
                alt = Some(Box::new(self.parse_if_expr()?));
            } else {
                let body = self.parse_body();

                alt = Some(Box::new(self.untyped(Expr::Block(body))));
            }
        }

        Ok(self.untyped(Expr::IfExpr {
            condition: Box::new(condition),
            body,
            alt,
        }))
    }
    fn parse_while_expr(&mut self) -> Result<Node, ()> {
        self.next();
        self.current_scope = Scope::Value;
        let condition = self.parse_level(Precedence::Low)?;
        let body = self.parse_body();

        Ok(self.untyped(Expr::WhileExpr {
            condition: Box::new(condition),
            body,
        }))
    }

    #[inline]
    fn parse_body(&mut self) -> Vec<Node> {
        let mut body = vec![];

        self.except(Token::LeftBracket);
        while self.current() != Token::RightBracket && self.current() != Token::EOF {
            self.current_scope = Scope::Top;
            let expr = self.parse_level(Precedence::Low);
            if expr.is_ok() {
                let mut expr = expr.unwrap();

                if !self.current_scope.is_used() {
                    expr = self.untyped(Expr::Discard(Box::new(expr)));
                }

                body.push(expr);
            }
        }
        self.except(Token::RightBracket);

        body
    }

    fn parse_ret_expr(&mut self) -> Result<Node, ()> {
        self.next();
        self.current_scope = Scope::Value;
        let expr = self.parse_level(Precedence::Low)?;
        Ok(self.untyped(Expr::RetExpr(Box::new(expr))))
    }
}
