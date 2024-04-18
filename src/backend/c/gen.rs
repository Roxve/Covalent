use super::type_to_c;

use super::types_to_cnamed;
use super::Codegen;
use super::Item;
use crate::ir::get_op_type;
use crate::ir::IROp;
use crate::source::ConstType;

impl Codegen {
    pub fn codegen(&mut self, mut ir: Vec<IROp>) -> String {
        // generate function and import section
        for op in ir.clone() {
            if let IROp::Import(_, module, _, _) = op {
                self.module.include(module);
                ir.remove(0);
            } else if let IROp::Def(ret, name, args, body) = op {
                self.bond_fn(
                    name,
                    args.into_iter()
                        .map(|i| (i.tag.unwrap_or(ConstType::Dynamic), i.val))
                        .collect(),
                    ret,
                    body,
                );

                ir.remove(0);

                dbg!(&ir);
            } else {
                break;
            }
        }

        self.bond_fn("main".to_string(), Vec::new(), ConstType::Int, ir);

        self.module.finish()
    }
    pub fn bond_fn(
        &mut self,
        name: String,
        args: Vec<(ConstType, String)>,
        ret: ConstType,
        body: Vec<IROp>,
    ) {
        let ty = type_to_c(ret);
        let args = types_to_cnamed(args);

        self.module
            .emit_header(format!("{} {}({}) {{", ty, name, args));
        for op in body {
            let line = self.bond(op);
            if line.is_some() {
                self.module.emit(line.unwrap());
            }
        }
        self.module.end_fn();
    }
    pub fn bond(&mut self, op: IROp) -> Option<String> {
        match op {
            IROp::Def(ret, name, args, body) => {
                self.bond_fn(
                    name,
                    args.into_iter()
                        .map(|i| (i.tag.unwrap_or(ConstType::Dynamic), i.val))
                        .collect(),
                    ret,
                    body,
                );
            }

            IROp::Alloc(_, _) => return None,
            IROp::Dealloc(_, _) => return None,

            IROp::Const(_, con) => self.push(Item::Const(con)),

            IROp::Store(ty, name) => {
                let b = self.borrow();
                let b_ty = b.get_ty();

                // clone dynamic and strings because they are pointers
                let val = if !(b.is_var() && (b_ty == ConstType::Dynamic || b_ty == ConstType::Str))
                {
                    self.pop_str()
                } else {
                    let s = self.pop_str();
                    self.call_one("__clone__", s)
                };
                let tyc = type_to_c(ty);

                let var = self.variables.get(&name);
                if var.is_none() || var.unwrap().1 != ty {
                    let name = self.var(name, ty);

                    return Some(format!("{} {} = {}", tyc, name, val));
                } else {
                    let name = self.get_var(name);
                    return Some(format!("{} = {}", name, val));
                }
            }
            IROp::Load(ty, name) => {
                let name = self.get_var(name);
                self.push(Item::Var(ty, name));
            }
            IROp::Call(ty, name, count) => {
                let arg_count = count;
                let args = self.pop_amount(arg_count).join(", ");
                let call = format!("{}({})", name, args);
                if &ty == &ConstType::Void {
                    // our compiler only insert a line when the stack is empty, void functions doesnt push anything to the stack
                    return Some(call);
                } else {
                    self.push(Item::Expr(ty, call));
                }
            }

            IROp::If(_, body, alt) => {
                let cond = self.pop_str();
                self.module.emit_header(format!("if ({}) {{", cond));

                for expr in body {
                    let expr_line = self.bond(expr);
                    if expr_line.is_some() {
                        self.module.emit(expr_line.unwrap());
                    }
                }

                if alt.len() > 0 {
                    let mut compiled_alt = String::new();
                    for expr in alt {
                        let expr_line = self.bond(expr);
                        if expr_line.is_some() {
                            compiled_alt.push_str(format!("\n{};", expr_line.unwrap()).as_str());
                        }
                    }
                    compiled_alt.remove(0);
                    self.module.end();
                    self.module.emit_header("else ");
                    if compiled_alt.starts_with("if") {
                        self.module.emit(compiled_alt);
                    } else {
                        self.module.emit("{\n");
                        self.module.add_col();
                        self.module.emit(compiled_alt);
                    }
                }

                self.module.try_end();
            }

            IROp::Conv(into, from) => {
                self.bond_conv(into, from);
            }

            IROp::Pop => {
                if self.stack.len() > 0 {
                    return Some(self.pop_str());
                }
            }

            IROp::Ret(_) => {
                let val = self.pop_str();
                return Some(format!("return {}", val));
            }
            _ => return self.bond_binary(op), // attempt to bond binary expr instead
        }
        None
    }
    #[inline]
    pub fn genbinary(&mut self, op: &str, ty: ConstType) -> Item {
        if &self.borrow().get_ty() == &ConstType::Str {
            Item::Expr(
                ty,
                format!("__stradd__({}, {})", self.pop_str(), self.pop_str()),
            )
        } else {
            Item::Expr(ty, format!("{} {} {}", self.pop_str(), op, self.pop_str()))
        }
    }
    #[inline]
    pub fn binary(&mut self, op: &str) -> Item {
        let ty = self.borrow().get_ty();
        self.genbinary(op, ty)
    }

    #[inline]
    pub fn binaryb(&mut self, op: &str) -> Item {
        self.genbinary(op, ConstType::Bool)
    }

    pub fn bond_binary(&mut self, op: IROp) -> Option<String> {
        let item = if get_op_type(&op) == ConstType::Dynamic
            || self.borrow().get_ty() == ConstType::Dynamic
        {
            let ops = vec![self.pop_str(), self.pop_str()];
            Item::Expr(
                ConstType::Dynamic,
                match op {
                    IROp::Add(_) => self.call("__add__", ops),
                    IROp::Sub(_) => self.call("__sub__", ops),
                    IROp::Mul(_) => self.call("__mul__", ops),
                    IROp::Div(_) => self.call("__div__", ops),
                    IROp::Comp(_) => self.call("__comp__", ops),
                    IROp::EComp(_) => self.call("__ecomp__", ops),
                    IROp::Eq(_) => self.call("__eq__", ops),
                    _ => todo!(),
                },
            )
        } else {
            match op {
                IROp::Add(_) => self.binary("+"),
                IROp::Sub(_) => self.binary("-"),
                IROp::Mul(_) => self.binary("*"),
                IROp::Div(_) => self.binary("/"),
                IROp::Comp(_) => self.binaryb(">"),
                IROp::Eq(_) => self.binaryb("=="),
                IROp::EComp(_) => self.binaryb(">="),
                _ => todo!("unimplented op {:#?}", op),
            }
        };
        self.push(item);
        None
    }

    #[inline]
    fn call_one(&self, name: &str, arg: String) -> String {
        format!("{}({})", name, arg)
    }
    #[inline]
    fn call(&self, name: &str, args: Vec<String>) -> String {
        format!("{}({})", name, args.join(", "))
    }

    pub fn bond_conv(&mut self, into: ConstType, from: ConstType) {
        let item = self.pop_str();
        let conv = match &into {
            &ConstType::Dynamic => match from {
                ConstType::Int => self.call_one("__int__", item),
                ConstType::Float => self.call_one("__float__", item),
                ConstType::Str => self.call_one("__str__", item),
                ConstType::Bool => self.call_one("__bool__", item),
                ConstType::Dynamic => item,
                _ => todo!("add conv dynamic from {:?}", from),
            },
            _ => todo!("add conv into {:?}", into),
        };

        self.push(Item::Expr(into, conv));
    }
}
