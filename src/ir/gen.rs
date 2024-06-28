use super::{Codegen, IROp};

use crate::analysis::ty_as;
use crate::enviroment::Symbol;
use crate::err::ErrKind;
use crate::parser::ast::{Expr, Ident, Node};
use crate::types::{
    can_implicitly_convert, AtomDetails, AtomKind, AtomType, BasicType, FunctionType,
};

type IR = Vec<IROp>;
type IRRes = Result<IR, ErrKind>;

pub trait IRGen {
    fn replace_unknown_body(&mut self, body: &mut Vec<Node>) -> Result<(), ErrKind>;
    fn replace_unknown(&mut self, node: &mut Node) -> Result<(), ErrKind>;

    fn gen_body(&mut self, body: Vec<Node>) -> IRRes;
    fn gen_prog(&mut self, exprs: Vec<Node>) -> IRRes;
    fn gen_func(
        &mut self,
        name: String,
        params: Vec<Ident>,
        ret: AtomType,
        body: Vec<Node>,
    ) -> IRRes;
    fn gen_extern(&mut self, name: String, params: Vec<Ident>, ret: AtomType) -> IRRes;

    fn gen_expr(&mut self, expr: Node) -> IRRes;

    fn gen_var_declare(&mut self, name: String, expr: Node) -> IRRes;
    fn gen_var_assign(&mut self, name: Node, expr: Node) -> IRRes;
    fn gen_binary_expr(&mut self, ty: AtomType, op: String, left: Node, right: Node) -> IRRes;
}

impl IRGen for Codegen {
    #[inline]
    fn replace_unknown_body(&mut self, body: &mut Vec<Node>) -> Result<(), ErrKind> {
        // first we build up an environment for our body
        fn match_env(this: &mut Codegen, node: &Node) {
            match &node.expr {
                Expr::VarDeclare { name, .. } | Expr::Extern { name, .. } => {
                    this.env.add(Symbol {
                        name: name.val().clone(),
                        ty: node.ty.clone(),
                        value: None,
                        expected: None,
                    });
                }

                Expr::Func { name, .. } | Expr::Import { name, .. } => {
                    this.env.add(Symbol {
                        name: name.clone(),
                        ty: node.ty.clone(),
                        value: None,
                        expected: None,
                    });
                }

                Expr::Discard(e) => match_env(this, &**e),
                _ => (),
            }
        }

        for node in &*body {
            match_env(self, node);
        }

        // We now have to run tought the thing and replace any refrence of Unknown to the correct type
        for node in &mut *body {
            self.replace_unknown(node)?;
        }
        Ok(())
    }

    fn replace_unknown(&mut self, node: &mut Node) -> Result<(), ErrKind> {
        // first we see if we assuming something
        let assume = if let &Some(AtomDetails::Unknown(ref assume)) = &node.ty.details {
            Some((**assume).clone())
        } else {
            None
        };

        // replacing the insides of a node
        match &mut (*node).expr {
            &mut Expr::RetExpr(ref mut ret) => {
                self.replace_unknown(ret)?;

                node.ty = ret.ty.clone();
            }

            &mut Expr::BinaryExpr {
                ref mut left,
                ref mut right,
                ..
            } => {
                self.replace_unknown(&mut *left)?;
                self.replace_unknown(&mut *right)?;

                node.ty = left.ty.clone();
            }

            &mut Expr::IfExpr {
                ref mut condition,
                ref mut body,
                ref mut alt,
            } => {
                self.replace_unknown(&mut *condition)?;

                if alt.is_some() {
                    self.replace_unknown(alt.as_mut().unwrap())?;
                }
                self.replace_unknown_body(&mut *body)?;
            }

            &mut Expr::WhileExpr {
                ref mut condition,
                ref mut body,
            } => {
                self.replace_unknown(&mut *condition)?;
                self.replace_unknown_body(&mut *body)?;
            }

            &mut Expr::Block(ref mut body) => return self.replace_unknown_body(&mut *body),

            &mut Expr::FnCall {
                ref mut name,
                ref mut args,
            } => {
                self.replace_unknown(name)?;
                self.replace_unknown_body(args)?;

                let return_type = if let &AtomKind::Function(FunctionType {
                    ref return_type, ..
                }) = &name.ty.kind
                {
                    (**return_type).clone()
                } else {
                    dbg!(&name);
                    panic!()
                };

                node.ty = return_type;
            }

            &mut Expr::As(ref mut thing) | &mut Expr::Discard(ref mut thing) => {
                self.replace_unknown(&mut **thing)?;
            }

            &mut Expr::Ident(ref id) => {
                node.ty = self.env.get_ty(id.val()).unwrap();
            }

            Expr::MemberExpr { parent, child } => {
                self.replace_unknown(&mut **parent)?;

                let ty = parent.ty.get(child).unwrap();

                node.ty = ty.clone();
            }

            _ => (),
        }

        if Some(node.ty.clone()) != assume && assume.is_some() {
            let assume = assume.unwrap();

            if can_implicitly_convert(&node.ty.kind, &assume.kind) {
                *node = ty_as(&assume, node.clone());
            } else {
                dbg!(&assume);
                dbg!(&node);
                todo!() // codegen errors
            }
        }

        Ok(())
    }
    fn gen_body(&mut self, mut body: Vec<Node>) -> IRRes {
        let mut exprs = Vec::new();
        self.replace_unknown_body(&mut body)?;

        for node in body {
            exprs.append(&mut self.gen_expr(node)?);
        }

        for sym in self.env.symbols.values().clone() {
            if let &AtomKind::Atom(_) = &sym.ty.kind {
                exprs.push(IROp::Dealloc(sym.ty.clone(), sym.name.clone()));
            }
        }

        Ok(exprs)
    }

    fn gen_prog(&mut self, exprs: Vec<Node>) -> IRRes {
        self.gen_body(exprs)
    }

    fn gen_func(
        &mut self,
        name: String,
        params: Vec<Ident>,
        ret: AtomType,
        body: Vec<Node>,
    ) -> IRRes {
        self.env.child();
        for param in &params {
            self.env.add(Symbol {
                name: param.val().clone(),
                ty: param.ty().clone(),
                value: None,
                expected: Some(param.ty().clone()),
            });
        }

        let body = self.gen_body(body)?;
        self.env.parent();
        Ok(vec![IROp::Def(ret, name, params, body)])
    }

    fn gen_expr(&mut self, expr: Node) -> IRRes {
        match expr.expr {
            Expr::Import {
                module,
                name,
                params,
            } => Ok(vec![IROp::Import(expr.ty, module, name, params)]),

            Expr::Func {
                ret,
                name,
                args,
                body,
            } => self.gen_func(name, args, ret, body),
            Expr::Extern { name, params } => {
                self.gen_extern(name.val().clone(), params, name.ty().clone())
            }

            Expr::Literal(lit) => Ok(vec![IROp::Const(lit)]),

            Expr::BinaryExpr { op, left, right } => {
                self.gen_binary_expr(expr.ty, op, *left, *right)
            }

            Expr::VarDeclare { name, val } => self.gen_var_declare(name.val().clone(), *val),
            Expr::VarAssign { name, val } => self.gen_var_assign(*name, *val),
            Expr::Ident(name) => Ok(vec![IROp::Load(expr.ty, name.val().clone())]),

            Expr::ListExpr(items) => {
                let mut bonded = vec![];
                for item in items {
                    bonded.push(self.gen_expr(item)?);
                }

                Ok(vec![IROp::List(expr.ty, bonded)])
            }

            Expr::MemberExpr { parent, child } => {
                let parent = self.gen_expr(*parent)?;
                let mut res = parent;
                res.push(IROp::LoadProp(expr.ty, child));

                Ok(res)
            }

            Expr::IndexExpr {
                parent: expr,
                index,
            } => {
                let expr = self.gen_expr(*expr)?;
                let idx = self.gen_expr(*index.clone())?;
                Ok([expr, idx, vec![IROp::LoadIdx(index.ty)]].concat())
            }

            Expr::FnCall { name, args } => {
                let mut res: Vec<IROp> = vec![];
                let count = args.len().clone() as u16;

                for arg in args {
                    res.append(&mut self.gen_expr(arg)?);
                }
                res.append(&mut self.gen_expr(*name)?);
                res.push(IROp::Call(expr.ty, count));

                Ok(res)
            }
            Expr::RetExpr(expr) => {
                let mut res = vec![];
                let mut compiled_expr = self.gen_expr(*expr.clone())?;

                res.append(&mut compiled_expr);
                res.push(IROp::Ret(expr.ty));
                Ok(res)
            }

            Expr::As(conv) => {
                let mut res = vec![];
                let mut inside = self.gen_expr(*conv.clone())?;

                res.append(&mut inside);
                res.push(IROp::Conv(expr.ty, (*conv).ty));
                Ok(res)
            }

            Expr::PosInfo(_, _, _) => Ok(vec![]),
            Expr::Discard(dis) => {
                let mut compiled = self.gen_expr(*dis.clone())?;
                if dis.ty.kind != AtomKind::Basic(BasicType::Void) {
                    compiled.append(&mut vec![IROp::Pop]);
                }
                Ok(compiled)
            }

            Expr::IfExpr {
                condition,
                body,
                alt,
            } => {
                let mut cond = self.gen_expr(*condition)?;

                // TODO func which generates scope body
                self.env.child();
                let body = self.gen_body(body)?;

                // for sym in self.env.symbols.values() {
                //     body.push(IROp::Dealloc(sym.ty.clone(), sym.name.clone()));
                // }
                self.env.parent();

                let alt = if alt.is_none() {
                    vec![]
                } else {
                    self.gen_expr(*alt.unwrap())?
                };

                let mut res = Vec::new();
                res.append(&mut cond);
                res.push(IROp::If(expr.ty, body, alt));
                Ok(res)
            }

            Expr::Block(block) => {
                self.env.child();
                let body = self.gen_body(block)?;

                // for sym in self.env.symbols.values() {
                //     body.push(IROp::Dealloc(sym.ty.clone(), sym.name.clone()));
                // }
                self.env.parent();
                Ok(body)
            }

            Expr::WhileExpr { condition, body } => {
                let mut cond = self.gen_expr(*condition)?;

                self.env.child();
                let body = self.gen_body(body)?;

                let mut res = Vec::new();
                res.append(&mut cond);
                res.push(IROp::While(body));

                self.env.parent();
                Ok(res)
            }
            _ => todo!("{:#?}", expr),
        }
    }

    fn gen_extern(&mut self, name: String, params: Vec<Ident>, ret: AtomType) -> IRRes {
        Ok(vec![IROp::Extern(ret, name, params)])
    }

    fn gen_var_declare(&mut self, name: String, expr: Node) -> IRRes {
        let mut res = vec![];
        let mut g = self.gen_expr(expr.clone())?;
        let ty = expr.ty;

        res.push(IROp::Alloc(ty.clone(), name.clone()));

        self.env.add(Symbol {
            name: name.clone(),
            ty: ty.clone(),
            value: None,
            expected: None,
        });

        res.append(&mut g);
        res.push(IROp::Store(ty, name));

        Ok(res)
    }

    fn gen_var_assign(&mut self, name: Node, expr: Node) -> IRRes {
        let mut res = vec![];
        res.append(&mut self.gen_expr(name)?);
        let mut compiled_expr = self.gen_expr(expr.clone())?;
        let ty = expr.ty;

        res.append(&mut compiled_expr);
        res.push(IROp::Set(ty));
        Ok(res)
    }

    fn gen_binary_expr(&mut self, ty: AtomType, op: String, left: Node, right: Node) -> IRRes {
        let mut res: IR = vec![];
        let mut lhs = self.gen_expr(left.clone())?;
        let mut rhs = self.gen_expr(right)?;
        res.append(&mut rhs);
        res.append(&mut lhs);
        if op.as_str() == "<" || op.as_str() == "<=" {
            res.reverse();
        }
        res.append(&mut vec![match op.as_str() {
            "+" => IROp::Add(ty),
            "-" => IROp::Sub(ty),
            "*" => IROp::Mul(ty),
            "/" => IROp::Div(ty),
            "%" => IROp::Mod(ty),
            ">" | "<" => IROp::Comp,
            ">=" | "<=" => IROp::EComp,
            "==" => IROp::Eq,
            "&&" => IROp::And,
            "||" => IROp::Or,
            o => todo!("add op {}", o),
        }]);
        Ok(res)
    }
}
