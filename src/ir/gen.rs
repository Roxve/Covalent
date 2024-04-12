use super::{get_fn_type, get_ops_type, Codegen, Const, IROp};

use crate::{
    parser::ast::{Expr, Literal},
    parser::Function,
    source::{ConstType, ErrKind, Ident},
};

type IR = Vec<IROp>;
type IRRes = Result<IR, u8>;

pub trait IRGen {
    fn import(
        &mut self,
        ty: ConstType,
        module: &str,
        name: &str,
        args: Vec<ConstType>,
        ir: &mut IR,
    );
    fn gen_prog(&mut self, exprs: Vec<Expr>, funcs: Vec<Function>) -> IR;
    fn gen_func(&mut self, func: Function) -> IRRes;
    fn gen_expr(&mut self, expr: Expr) -> IRRes;

    fn gen_var_declare(&mut self, name: String, expr: Expr) -> IRRes;
    fn gen_var_assign(&mut self, name: String, expr: Expr) -> IRRes;
    fn gen_binary_expr(&mut self, op: String, left: Expr, right: Expr) -> IRRes;
}

impl IRGen for Codegen {
    fn import(
        &mut self,
        ty: ConstType,
        module: &str,
        name: &str,
        args: Vec<ConstType>,
        ir: &mut IR,
    ) {
        ir.reverse();
        ir.push(IROp::Import(
            ty.clone(),
            module.to_string(),
            name.to_string(),
            args,
        ));
        self.env.push_function(
            Ident {
                val: name.to_string(),
                tag: None,
            },
            vec![Ident {
                val: "...data".to_string(),
                tag: None,
            }],
            ty,
        );
        ir.reverse();
    }
    fn gen_prog(&mut self, exprs: Vec<Expr>, funcs: Vec<Function>) -> IR {
        let mut gen = vec![];

        for func in funcs {
            let compiled_func = self.gen_func(func);
            if compiled_func.is_ok() {
                gen.append(&mut compiled_func.unwrap());
            }
        }

        self.import(
            ConstType::Void,
            "std",
            "writeln",
            vec![ConstType::Dynamic],
            &mut gen,
        );
        for expr in exprs {
            let compiled_expr = self.gen_expr(expr);
            if compiled_expr.is_ok() {
                gen.append(&mut compiled_expr.unwrap());
            }
        }
        gen
    }

    fn gen_func(&mut self, func: Function) -> IRRes {
        let mut body = vec![];
        let args: Vec<String> = func.args.iter().map(|v| v.val.clone()).collect();

        self.env = self.env.child();

        for arg in args.clone() {
            self.env.add(&arg, ConstType::Dynamic, 0);
        }

        for expr in func.body {
            let mut compiled_expr = self.gen_expr(expr)?;
            body.append(&mut compiled_expr);
        }

        let ty = get_fn_type(&mut body);

        self.env = self.env.parent().unwrap();

        self.env
            .push_function(func.name.clone(), func.args, ty.clone());
        Ok(vec![IROp::Def(ty, func.name.val, args, body)])
    }

    fn gen_expr(&mut self, expr: Expr) -> IRRes {
        match expr {
            Expr::Literal(Literal::Int(i)) => Ok(vec![IROp::Const(ConstType::Int, Const::Int(i))]),
            Expr::Literal(Literal::Float(f)) => {
                Ok(vec![IROp::Const(ConstType::Float, Const::Float(f))])
            }
            Expr::BinaryExpr { op, left, right } => self.gen_binary_expr(op, *left, *right),
            Expr::VarDeclare { name, val } => self.gen_var_declare(name.val, *val),
            Expr::VarAssign { name, val } => self.gen_var_assign(name.val, *val),
            Expr::Ident(name) => {
                if !self.env.has(&name.val) {
                    self.err(
                        ErrKind::UndeclaredVar,
                        format!("var {} is not declared", name.val.clone()),
                    );
                    return Err(ErrKind::UndeclaredVar as u8);
                }
                Ok(vec![IROp::Load(
                    self.env.get_ty(&name.val).unwrap(),
                    name.val,
                )])
            }
            Expr::FnCall { name, args } => {
                let mut res: Vec<IROp> = vec![];
                let fun = self.env.get_function(&name);
                if fun.is_none() {
                    self.err(
                        ErrKind::UndeclaredVar,
                        format!("undeclared function {}", name.val),
                    );
                    return Err(ErrKind::UndeclaredVar as u8);
                }

                if (&fun.as_ref()).unwrap().args.len() != (&args).len() {
                    self.err(
                        ErrKind::UnexceptedArgs,
                        format!(
                            "unexpected args number for function {}, got {} args expected {}",
                            name.val,
                            args.len(),
                            fun.unwrap().args.len()
                        ),
                    );
                    return Err(ErrKind::UnexceptedArgs as u8);
                }

                for arg in args {
                    let mut compiled_arg = self.gen_expr(arg)?;
                    res.append(&mut compiled_arg);
                    res.push(IROp::Conv(ConstType::Dynamic, get_ops_type(&res)));
                }
                res.push(IROp::Call(self.env.get_ty(&name.val).unwrap(), name.val));

                Ok(res)
            }
            Expr::RetExpr(expr) => {
                let mut res = vec![];
                let mut compiled_expr = self.gen_expr(*expr)?;
                let ty = get_ops_type(&compiled_expr);
                res.append(&mut compiled_expr);
                res.push(IROp::Ret(ty));
                Ok(res)
            }
            Expr::PosInfo(_, _, _) => Ok(vec![]),
            Expr::Discard(dis) => {
                let mut compiled = self.gen_expr(*dis)?;
                compiled.append(&mut vec![IROp::Pop]);
                Ok(compiled)
            }
            _ => todo!("ast: {:?}", expr),
        }
    }

    fn gen_var_declare(&mut self, name: String, expr: Expr) -> IRRes {
        if self.env.vars.contains_key(&name.clone()) {
            self.err(
                ErrKind::VarAlreadyDeclared,
                format!("var {} is already declared", name.clone()),
            );
            return Err(ErrKind::VarAlreadyDeclared as u8);
        }

        let mut res = vec![];
        let mut g = self.gen_expr(expr)?;
        let ty = get_ops_type(&g);
        res.push(IROp::Alloc(ty.clone(), name.clone()));
        self.env.add(&name, ty.clone(), 0);
        res.append(&mut g);
        res.push(IROp::Store(ty, name));

        Ok(res)
    }

    fn gen_var_assign(&mut self, name: String, expr: Expr) -> IRRes {
        if !self.env.has(&name) {
            self.err(
                ErrKind::UndeclaredVar,
                format!("var {} is not declared", name.clone()),
            );
            return Err(ErrKind::UndeclaredVar as u8);
        }

        let mut res = vec![];
        let mut compiled_expr = self.gen_expr(expr)?;
        let ty = get_ops_type(&compiled_expr);

        if &self.env.get_ty(&name).unwrap() != &ty {
            res.push(IROp::Dealloc(self.env.get_ty(&name).unwrap(), name.clone()));
            res.push(IROp::Alloc(ty.clone(), name.clone()));

            self.env.modify(&name, ty.clone());
        }

        res.append(&mut compiled_expr);
        res.push(IROp::Store(ty, name));
        Ok(res)
    }

    fn gen_binary_expr(&mut self, op: String, left: Expr, right: Expr) -> IRRes {
        let mut res: IR = vec![];
        let mut lhs = self.gen_expr(left)?;
        let mut rhs = self.gen_expr(right)?;
        let ty;
        if get_ops_type(&lhs) != get_ops_type(&rhs) {
            // beform type conv
            let lhs_ty = get_ops_type(&lhs);
            let rhs_ty = get_ops_type(&rhs);
            if lhs_ty == ConstType::Float && rhs_ty == ConstType::Int {
                res.append(&mut rhs);
                res.append(&mut vec![IROp::Conv(ConstType::Float, rhs_ty)]);
                res.append(&mut lhs);
                ty = lhs_ty;
            } else if lhs_ty == ConstType::Int && rhs_ty == ConstType::Float {
                res.append(&mut rhs);
                res.append(&mut lhs);
                res.append(&mut vec![IROp::Conv(ConstType::Float, lhs_ty)]);
                ty = rhs_ty;
            } else {
                // NaN
                ty = lhs_ty;
            }
        } else {
            ty = get_ops_type(&lhs);
            res.append(&mut rhs);
            res.append(&mut lhs);
        }

        res.append(&mut vec![match op.as_str() {
            "+" => IROp::Add(ty),
            "-" => IROp::Sub(ty),
            "*" => IROp::Mul(ty),
            "/" => IROp::Div(ty),
            o => todo!("add op {}", o),
        }]);
        Ok(res)
    }
}
