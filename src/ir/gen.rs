use super::{Codegen, IROp};

use crate::{
    analysis::{AnalyzedExpr, TypedExpr},
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
    fn gen_prog(&mut self, exprs: Vec<TypedExpr>, funcs: Vec<Function>) -> IR;
    // fn gen_func(&mut self, func: Function) -> IRRes;
    fn gen_expr(&mut self, expr: TypedExpr) -> IRRes;

    fn gen_var_declare(&mut self, name: String, expr: TypedExpr) -> IRRes;
    fn gen_var_assign(&mut self, name: String, expr: TypedExpr) -> IRRes;
    fn gen_binary_expr(&mut self, op: String, left: TypedExpr, right: TypedExpr) -> IRRes;
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
    fn gen_prog(&mut self, exprs: Vec<TypedExpr>, funcs: Vec<Function>) -> IR {
        let mut gen = vec![];

        /*for func in funcs {
            let compiled_func = self.gen_func(func);
            if compiled_func.is_ok() {
                gen.append(&mut compiled_func.unwrap());
            }
        }*/

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

    /*    fn gen_func(&mut self, func: Function) -> IRRes {
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
    */
    fn gen_expr(&mut self, expr: TypedExpr) -> IRRes {
        match expr.expr {
            AnalyzedExpr::Literal(lit) => Ok(vec![IROp::Const(expr.ty, lit)]),

            AnalyzedExpr::BinaryExpr { op, left, right } => self.gen_binary_expr(op, *left, *right),
            AnalyzedExpr::VarDeclare { name, val } => self.gen_var_declare(name, *val),
            AnalyzedExpr::VarAssign { name, val } => self.gen_var_assign(name, *val),
            AnalyzedExpr::Id(name, rc) => {
                self.env.modify_rc(&name, rc);
                Ok(vec![IROp::Load(self.env.get_ty(&name).unwrap(), name)])
            }
            AnalyzedExpr::FnCall { name, args } => {
                let mut res: Vec<IROp> = vec![];
                res.push(IROp::Call(expr.ty, name));

                Ok(res)
            }
            AnalyzedExpr::RetExpr(expr) => {
                let mut res = vec![];
                let mut compiled_expr = self.gen_expr(*expr.clone())?;

                res.append(&mut compiled_expr);
                res.push(IROp::Ret(expr.ty));
                Ok(res)
            }
            AnalyzedExpr::Debug(_, _, _) => Ok(vec![]),
            AnalyzedExpr::Discard(dis) => {
                let mut compiled = self.gen_expr(*dis)?;
                compiled.append(&mut vec![IROp::Pop]);
                Ok(compiled)
            }
            _ => todo!("ast: {:?}", expr),
        }
    }

    fn gen_var_declare(&mut self, name: String, expr: TypedExpr) -> IRRes {
        let mut res = vec![];
        let mut g = self.gen_expr(expr.clone())?;
        let ty = expr.ty;

        res.push(IROp::Alloc(ty.clone(), name.clone()));
        self.env.add(&name, ty.clone(), 0);
        res.append(&mut g);
        res.push(IROp::Store(ty, name));

        Ok(res)
    }

    fn gen_var_assign(&mut self, name: String, expr: TypedExpr) -> IRRes {
        if !self.env.has(&name) {
            self.err(
                ErrKind::UndeclaredVar,
                format!("var {} is not declared", name.clone()),
            );
            return Err(ErrKind::UndeclaredVar as u8);
        }

        let mut res = vec![];
        let mut compiled_expr = self.gen_expr(expr.clone())?;
        let ty = expr.ty;

        if &self.env.get_ty(&name).unwrap() != &ty {
            res.push(IROp::Dealloc(self.env.get_ty(&name).unwrap(), name.clone()));
            res.push(IROp::Alloc(ty.clone(), name.clone()));

            self.env.modify(&name, ty.clone());
        }

        res.append(&mut compiled_expr);
        res.push(IROp::Store(ty, name));
        Ok(res)
    }

    fn gen_binary_expr(&mut self, op: String, left: TypedExpr, right: TypedExpr) -> IRRes {
        let mut res: IR = vec![];
        let mut lhs = self.gen_expr(left.clone())?;
        let mut rhs = self.gen_expr(right)?;
        let ty = left.ty;
        res.append(&mut rhs);
        res.append(&mut lhs);

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
