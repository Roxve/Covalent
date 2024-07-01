use self::types::*;

use super::*;
use crate::err;
use crate::err::*;
use crate::ir;
use crate::ir::IROp;
use crate::ir::Instruction;
use crate::parser::ast;

fn get_instrs_type(instrs: &Vec<ir::Instruction>) -> AtomType {
    instrs[instrs.len() - 1].ty()
}

impl Analyzer {
    pub fn analyze_program(workdir: String, program: ast::Program) -> Result<ir::Program, ErrKind> {
        let analyzer = Analyzer::new(workdir);

        let mut instructions = Vec::new();

        for node in program.body {
            let node_instructions = analyzer.analyze(node)?;
            instructions.extend(node_instructions);
        }

        Ok(ir::Program { instructions })
    }

    pub fn analyze(&self, node: ast::Node) -> Result<Vec<ir::Instruction>, ErrKind> {
        use ast::Expr;
        match node.expr {
            Expr::BinaryExpr { op, left, right } => {
                self.analyze_binary_expr(op.as_str(), *left, *right)
            }

            Expr::Literal(lit) => Ok(vec![Instruction::new(
                IROp::Const(lit.clone()),
                lit.get_ty(),
            )]),

            Expr::Discard(e) => {
                let mut results = self.analyze(*e)?;

                let ty = AtomType {
                    kind: AtomKind::Basic(BasicType::Void),
                    details: None,
                };

                results.push(ir::Instruction::new(IROp::Pop, ty));
                Ok(results)
            }

            e => todo!("{:#?}", e),
        }
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
                right.push(Instruction::new(IROp::Conv(left_type), right_type));
            } else if types::can_implicitly_convert(&right_type.kind, &left_type.kind) {
                left.push(Instruction::new(IROp::Conv(right_type), left_type));
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

    pub fn analyze_binary_expr(
        &self,
        op: &str,
        left: ast::Node,
        right: ast::Node,
    ) -> Result<Vec<ir::Instruction>, ErrKind> {
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
                IROp::Comp // peforms GE that is why we gotta swap
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
}
