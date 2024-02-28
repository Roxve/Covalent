use super::{get_fn_type, get_ops_type, Const, ConstType, IROp};
use crate::{
    ast::{Expr, Literal},
    source::{ErrKind, Function, Source},
};

type IR = Vec<IROp>;
type IRRes = Result<IR, u8>;

pub trait IRGen {
    fn gen_prog(&mut self, exprs: Vec<Expr>) -> IR;
    fn gen_func(&mut self, func: Function) -> IRRes;
    fn gen_expr(&mut self, expr: Expr) -> IRRes;

    fn gen_var_declare(&mut self, name: String, expr: Expr) -> IRRes;
    fn gen_var_assign(&mut self, name: String, expr: Expr) -> IRRes;
    fn gen_binary_expr(&mut self, op: String, left: Expr, right: Expr) -> IRRes;
}

impl IRGen for Source {
    fn gen_prog(&mut self, exprs: Vec<Expr>) -> IR {
        let mut gen = vec![];
        for func in self.functions.clone() {
            let compiled_func = self.gen_func(func);
            if compiled_func.is_ok() {
                gen.append(&mut compiled_func.unwrap());
            }
        }

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
        let args: Vec<String> = func.args.iter().map(|v| v.0.clone()).collect();

        let old_vars = self.vars.clone();
        self.vars.clear();
        self.vars.reserve(args.len());

        for arg in args.clone() {
            // alloc dynamic type for arguments
            body.push(IROp::Alloc(ConstType::Dynamic, arg.clone()));
            self.vars.insert(arg, ConstType::Dynamic);
        }

        for expr in func.body {
            let mut compiled_expr = self.gen_expr(expr)?;
            body.append(&mut compiled_expr);
        }

        let ty = get_fn_type(&mut body);
        self.vars = old_vars;
        Ok(vec![IROp::Def(ty, func.name.0, args, body)])
    }

    fn gen_expr(&mut self, expr: Expr) -> IRRes {
        match expr {
            Expr::Literal(Literal::Int(i)) => Ok(vec![IROp::Const(ConstType::Int, Const::Int(i))]),
            Expr::Literal(Literal::Float(f)) => {
                Ok(vec![IROp::Const(ConstType::Float, Const::Float(f))])
            }
            Expr::BinaryExpr(op, left, right) => self.gen_binary_expr(op, *left, *right),
            Expr::VarDeclare(name, expr) => self.gen_var_declare(name.0, *expr),
            Expr::VarAssign(name, expr) => self.gen_var_assign(name.0, *expr),
            Expr::Ident(name) => {
                if !self.vars.contains_key(&name.0) {
                    self.err(
                        ErrKind::UndeclaredVar,
                        format!("var {} is not declared", name.0.clone()),
                    );
                    return Err(ErrKind::UndeclaredVar as u8);
                }
                Ok(vec![IROp::Load(
                    self.vars.get(&name.0).unwrap().clone(),
                    name.0,
                )])
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
            _ => todo!(),
        }
    }

    fn gen_var_declare(&mut self, name: String, expr: Expr) -> IRRes {
        if self.vars.contains_key(&name.clone()) {
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
        self.vars.insert(name.clone(), ty.clone());
        res.append(&mut g);
        res.push(IROp::Store(ty, name));

        Ok(res)
    }

    fn gen_var_assign(&mut self, name: String, expr: Expr) -> IRRes {
        if !self.vars.contains_key(&name) {
            self.err(
                ErrKind::UndeclaredVar,
                format!("var {} is not declared", name.clone()),
            );
            return Err(ErrKind::UndeclaredVar as u8);
        }

        let mut res = vec![];
        let mut compiled_expr = self.gen_expr(expr)?;
        let ty = get_ops_type(&compiled_expr);

        if self.vars.get(&name).unwrap() != &ty {
            res.push(IROp::Dealloc(
                self.vars.get(&name).unwrap().clone(),
                name.clone(),
            ));
            res.push(IROp::Alloc(ty.clone(), name.clone()));
            self.vars.remove(&name);
            self.vars.insert(name.clone(), ty.clone());
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
                res.append(&mut lhs);
                res.append(&mut rhs);
                res.append(&mut vec![IROp::Conv(ConstType::Float)]);
                ty = lhs_ty;
            } else if lhs_ty == ConstType::Int && rhs_ty == ConstType::Float {
                res.append(&mut lhs);
                res.append(&mut vec![IROp::Conv(ConstType::Float)]);
                res.append(&mut rhs);
                ty = rhs_ty;
            } else {
                // NaN
                ty = lhs_ty;
            }
        } else {
            ty = get_ops_type(&lhs);
            res.append(&mut lhs);
            res.append(&mut rhs);
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
