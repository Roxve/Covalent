use crate::{
    parser::ast::{Expr, Node},
    source::{ConstType, ErrKind},
};

use super::{get_fn_type, Analyzer};

impl Analyzer {
    pub fn correct_prog(&mut self, exprs: Vec<Node>) -> Result<Vec<Node>, ErrKind> {
        let mut corrected = vec![];
        for expr in exprs.clone() {
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
                let mut corrected_body = vec![];

                for node in body {
                    corrected_body.push(self.correct(node)?);
                }

                let ret = if &ret == &ConstType::Unknown {
                    get_fn_type(&name, &corrected_body, ConstType::Void)
                } else {
                    ret
                };

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
                if node.clone().ty == ConstType::Unknown {
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
