use std::fmt::format;

use super::type_to_c;

use super::types_to_cnamed;
use super::Codegen;
use super::Item;
use crate::ir::get_op_type;
use crate::ir::IROp;
use crate::source::ConstType;

impl Codegen {
    pub fn codegen(&mut self, ir: Vec<IROp>) -> String {
        let main = self.bond_fn("main".to_string(), Vec::new(), ConstType::Int, ir);
        self.module.func(main);
        self.module.finish()
    }
    pub fn bond_fn(
        &mut self,
        name: String,
        args: Vec<(ConstType, String)>,
        ret: ConstType,
        body: Vec<IROp>,
    ) -> Vec<String> {
        let mut lines = Vec::new();
        let ty = type_to_c(ret);
        let args = types_to_cnamed(args);

        lines.push(format!("{} {}({}) {{", ty, name, args));
        for op in body {
            let line = self.bond(op);
            if line.is_some() {
                lines.push(line.unwrap() + ";");
            }
        }

        lines.push("}".to_string());
        lines
    }
    pub fn bond(&mut self, op: IROp) -> Option<String> {
        match op {
            IROp::Def(ret, name, args, body) => {
                let func = self.bond_fn(
                    name,
                    args.into_iter()
                        .map(|i| (i.tag.unwrap_or(ConstType::Dynamic), i.val))
                        .collect(),
                    ret,
                    body,
                );
                self.module.func(func);
            }

            IROp::Alloc(_, _) => return None,
            IROp::Dealloc(_, _) => return None,

            IROp::Const(_, con) => self.push(Item::Const(con)),

            IROp::Store(ty, name) => {
                let val = self.pop_str();
                let ty = type_to_c(ty);
                let name = self.var(name);

                return Some(format!("{} {} = {}", ty, name, val));
            }
            IROp::Load(ty, name) => {
                let name = self.get_var(name);
                self.push(Item::Expr(ty, name));
            }

            IROp::Import(_, module, _, _) => self.module.include(module),
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
        let item = if get_op_type(&op) == ConstType::Dynamic {
            let ops = vec![self.pop_str(), self.pop_str()];
            Item::Expr(
                ConstType::Dynamic,
                match op {
                    IROp::Add(_) => self.call("__add__", ops),
                    IROp::Sub(_) => self.call("__sub__", ops),
                    IROp::Mul(_) => self.call("__mul__", ops),
                    IROp::Div(_) => self.call("__div__", ops),
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
