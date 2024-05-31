use crate::{
    parser::ast::{Expr, Node},
    source::{ConstType, ErrKind},
};

use super::{get_fn_type, Analyzer};

impl Analyzer {
    pub fn correct_prog(&mut self, exprs: Vec<Node>) -> Result<Vec<Node>, ErrKind> {
        let mut corrected = vec![];
        for expr in exprs {
            if let Expr::Import { .. } = expr.expr {
                corrected.push(expr);
            } else {
                corrected.push(self.correct(expr)?);
            }
        }

        Ok(corrected)
    }

    fn correct(&mut self, node: Node) -> Result<Node, ErrKind> {
        match node.clone().expr {
            Expr::Func {
                ret,
                name,
                args,
                body,
            } => {
                self.env = self.env.child();
                let mut corrected_body = vec![];

                self.env.current = ret.clone();
                for arg in &args {
                    self.env
                        .add(&arg.val, arg.tag.clone().unwrap_or(ConstType::Dynamic));
                }
                for node in body {
                    corrected_body.push(self.correct(node)?);
                }

                // let ret = if &ret == &ConstType::Unknown {
                //     get_fn_type(&mut corrected_body, ConstType::Void)
                // } else {
                //     ret
                // };

                if &self.env.current != &ret {
                    for node in &mut corrected_body {
                        match &mut node.expr {
                            &mut Expr::RetExpr(ref mut ret) => {
                                *ret = Box::new(self.type_cast(*ret.clone(), ConstType::Dynamic)?);
                            }
                            _ => (),
                        }
                    }
                }

                self.env = self.env.parent().unwrap();

                // dbg!(&corrected_body);
                Ok(Node {
                    expr: Expr::Func {
                        ret: ret.clone(),
                        name,
                        args,
                        body: corrected_body,
                    },
                    ty: ret,
                })
            }
            Expr::RetExpr(ret) => {
                let ret = self.correct(*ret)?;
                if &self.env.current != &ret.ty {
                    self.env.current = ConstType::Dynamic;
                }

                Ok(Node {
                    expr: Expr::RetExpr(Box::new(ret.clone())),
                    ty: ret.ty,
                })
            }
            Expr::As(from) => {
                let from = self.correct(*from)?;
                if &from.ty != &node.ty {
                    Ok(Node {
                        expr: Expr::As(Box::new(from)),
                        ty: node.ty,
                    })
                } else {
                    Ok(from)
                }
            }

            _ => {
                if let ConstType::Unknown(_) = node.ty.clone() {
                    dbg!(&node);
                    let n = self.analyz(node);
                    dbg!(&n);
                    n
                } else {
                    Ok(node)
                }
            }
        }
    }
}
