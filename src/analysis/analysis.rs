use crate::types::*;

use super::*;
use crate::err;
use crate::err::*;

use crate::ir::{self, IROp, Instruction};

use crate::parser::ast;

fn get_instrs_type(instrs: &Vec<ir::Instruction>) -> AtomType {
    instrs[instrs.len() - 1].ty()
}

type Instructions = Result<Vec<ir::Instruction>, ErrKind>;

impl Analyzer {
    pub fn analyze_program(workdir: String, program: ast::Program) -> Result<ir::Program, ErrKind> {
        let mut analyzer = Analyzer::new(workdir);

        let mut instructions = Vec::new();

        for node in program.body {
            let node_instructions = analyzer.analyze(node)?;
            instructions.extend(node_instructions);
        }

        Ok(ir::Program { instructions })
    }

    pub fn analyze(&mut self, node: ast::Node) -> Instructions {
        use ast::Expr;
        match node.expr {
            Expr::Literal(lit) => Ok(vec![Instruction::new(
                IROp::Const(lit.clone()),
                lit.get_ty(),
            )]),

            Expr::Ident(id) => {
                match id {
                    Ident::UnTagged(_) => (),
                    _ => {
                        err!(
                            self,
                            ErrKind::UnexceptedTokenE,
                            format!("ID {} must be untagged (without @Type)", id.val())
                        );
                    }
                }

                let sym = self.env.get(&id.val());
                if sym.is_none() {
                    err!(
                        self,
                        ErrKind::UndeclaredVar,
                        format!("undeclared variable: {}", id.val())
                    );
                }

                let sym = sym.unwrap().clone();

                Ok(vec![Instruction::new(IROp::Load(sym.name), sym.ty)])
            }

            Expr::FnCall { name, args } => self.analyze_fn_call(*name, args),

            Expr::BinaryExpr { op, left, right } => {
                self.analyze_binary_expr(op.as_str(), *left, *right)
            }

            Expr::VarDeclare { name, val } => self.analyze_var_declare(name, *val),

            Expr::VarAssign { name, val } => self.analyze_var_assign(*name, *val),

            Expr::Discard(e) => {
                let mut results = self.analyze(*e)?;

                if get_instrs_type(&results).kind == AtomKind::Basic(BasicType::Void) {
                    return Ok(results); // no need to pop if we have void as it pushs nothing into the stack
                }

                let ty = AtomType {
                    kind: AtomKind::Basic(BasicType::Void),
                    details: None,
                };

                results.push(Instruction::new(IROp::Pop, ty));
                Ok(results)
            }

            Expr::Block(block) => {
                let mut results = Vec::new();
                for node in block {
                    results.extend(self.analyze(node)?);
                }
                let ty = get_instrs_type(&results);

                Ok(vec![Instruction::new(IROp::Block(results), ty)])
            }

            e => todo!("{:#?}", e),
        }
    }

    fn analyze_items(&mut self, items: Vec<Node>) -> Result<Vec<Vec<Instruction>>, ErrKind> {
        let mut instr = Vec::new();

        for item in items {
            instr.push(self.analyze(item)?);
        }

        Ok(instr)
    }

    pub fn type_unify(
        &self,
        mut left: Vec<Instruction>,
        mut right: Vec<Instruction>,
    ) -> Result<(Vec<Instruction>, Vec<Instruction>), ErrKind> {
        let left_type = get_instrs_type(&left);
        let right_type = get_instrs_type(&right);

        if left_type != right_type {
            if types::can_implicitly_convert(&left_type.kind, &right_type.kind) {
                left.push(Instruction::new(IROp::Conv(left_type), right_type));
            } else if types::can_implicitly_convert(&right_type.kind, &left_type.kind) {
                right.push(Instruction::new(IROp::Conv(right_type), left_type));
            } else {
                err!(
                    self,
                    ErrKind::TypeMismatch,
                    format!("type mismatch got {} and {}", left_type, right_type)
                );
            }
        }

        Ok((left, right))
    }

    fn analyze_fn_call(&mut self, name: Node, args: Vec<Node>) -> Instructions {
        let name_instructions = self.analyze(name)?;
        let ty = get_instrs_type(&name_instructions);

        match ty.kind {
            AtomKind::Function(func_t) => {
                self.handle_function_call(func_t, name_instructions, args)
            }
            _ => {
                err!(
                    self,
                    ErrKind::InvaildType,
                    format!("type {ty} doesnt impl Call, cannot call {ty}")
                );
            }
        }
    }

    fn handle_function_call(
        &mut self,
        func_t: FunctionType,
        name: Vec<Instruction>,
        args: Vec<Node>,
    ) -> Instructions {
        let mut result = Vec::new();
        let count = args.len();

        let mut args = self.analyze_items(args)?;

        if count != func_t.params.len() {
            err!(
                self,
                ErrKind::UnexceptedArgs,
                format!("expected {} args got {count}", func_t.params.len())
            );
        }

        for (index, arg) in args.iter_mut().enumerate() {
            let ty = get_instrs_type(&arg);

            if ty != func_t.params[index] {
                if can_implicitly_convert(&func_t.params[index].kind, &ty.kind) {
                    arg.extend(vec![Instruction::new(
                        IROp::Conv(ty),
                        func_t.params[index].clone(),
                    )])
                } else {
                    err!(
                        self,
                        ErrKind::TypeMismatch,
                        format!(
                            "expected arg of type {} got {}, at arg {}",
                            func_t.params[index],
                            get_instrs_type(&arg),
                            index
                        )
                    );
                }
            }
        }

        let ty = get_instrs_type(&name);
        result.extend(args.into_iter().flatten());
        result.extend(name);

        result.push(Instruction::new(IROp::Call(count as u16), ty));

        Ok(result)
    }

    fn analyze_binary_expr(&mut self, op: &str, left: ast::Node, right: ast::Node) -> Instructions {
        let mut result = Vec::new();

        let left_instructions = self.analyze(left)?;
        let right_instructions = self.analyze(right)?;

        let (mut left, mut right) = self.type_unify(left_instructions, right_instructions)?;

        let op = match op {
            "+" => IROp::Add,
            "-" => IROp::Sub,
            "*" => IROp::Mul,
            "/" => IROp::Div,
            "%" => IROp::Mod,
            "==" => IROp::Eq,

            "<" => {
                (left, right) = (right, left);
                IROp::Comp // peforms GT that is why we gotta swap to peform LT
            }
            ">" => IROp::Comp,

            "<=" => {
                (left, right) = (right, left);
                IROp::EComp // same here
            }

            ">=" => IROp::EComp,
            "&&" => IROp::And,
            "||" => IROp::Or,
            _ => panic!("unsupported binary operator: {}", op),
        };

        let ty = match &op {
            IROp::EComp | IROp::Comp | IROp::And | IROp::Or => AtomType {
                kind: AtomKind::Basic(BasicType::Bool),
                details: None,
            },
            _ => get_instrs_type(&left),
        };

        result.extend(left);
        result.extend(right);

        let op = Instruction::new(op, ty);
        result.push(op);

        Ok(result)
    }

    fn analyze_unknown_id(&mut self, mut id: Ident) -> Result<Ident, ErrKind> {
        if let &Ident::Tagged(ref tag, ref name) = &id {
            let ty = get_instrs_type(&self.analyze(*tag.clone())?);

            if ty.details != Some(AtomDetails::Type) {
                err!(
                    self,
                    ErrKind::InvaildType,
                    format!("got {} but expected a type", tag)
                );
            }

            id = Ident::Typed(ty, name.clone());
        }

        Ok(id)
    }

    fn analyze_var_declare(&mut self, name: Ident, val: Node) -> Instructions {
        let name = self.analyze_unknown_id(name)?;

        let mut result = Vec::new();

        let expected_ty = AtomType {
            kind: name.ty().kind.clone(),
            details: None,
        }; // it is any if no type provided, removes details

        let val_instructions = self.analyze(val)?;
        let ty = get_instrs_type(&val_instructions);

        if ty != expected_ty {
            err!(
                self,
                ErrKind::InvaildType,
                format!("got {} but expected {}", ty, expected_ty)
            );
        }

        self.env.add(Symbol {
            name: name.val().to_owned(),
            ty: ty.clone(),
            value: None,
            expected: None,
        });

        result.extend(val_instructions);
        result.push(Instruction::new(IROp::Store(name.val().to_owned()), ty));
        Ok(result)
    }

    fn analyze_var_assign(&mut self, name: Node, val: Node) -> Instructions {
        let name_instructions = self.analyze(name.clone())?; // may be used later on error

        let mut result = Vec::new();

        let val_instructions = self.analyze(val)?;
        let ty = get_instrs_type(&val_instructions);

        if ty != get_instrs_type(&name_instructions) {
            err!(
                self,
                ErrKind::TypeMismatch,
                format!(
                    "type mismatch got {} and {}, cannot assign to {} with a value of different type",
                    ty,
                    get_instrs_type(&name_instructions),
                    name
                )
            );
        }

        result.extend(val_instructions);
        result.push(Instruction::new(IROp::Set, ty));
        Ok(result)
    }
}
