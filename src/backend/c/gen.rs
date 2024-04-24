use super::type_to_c;
use super::Emit;

use super::types_to_cnamed;
use super::Codegen;
use super::Item;
use crate::ir::get_op_type;
use crate::ir::IROp;
use crate::source::ConstType;

impl Codegen {
    #[inline]
    fn call_one(&self, name: &str, arg: String) -> String {
        format!("{}({})", name, arg)
    }
    #[inline]
    fn call(&self, name: &str, args: Vec<String>) -> String {
        format!("{}({})", name, args.join(", "))
    }

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
            } else {
                break;
            }
        }

        self.bond_fn("main".to_string(), Vec::new(), ConstType::Int, ir);

        self.module.finish()
    }
    fn bond_fn(
        &mut self,
        name: String,
        args: Vec<(ConstType, String)>,
        ret: ConstType,
        body: Vec<IROp>,
    ) {
        let ty = type_to_c(ret);
        let args = types_to_cnamed(args);
        let mut emiter = self.emiter();
        emiter.emit_header(format!("{} {}({}) {{", ty, name, args));
        for op in body {
            let emit = self.bond(op);

            emiter.embed(emit);
        }
        emiter.end();
        self.module.func(emiter.finish());
    }
    pub fn bond(&mut self, op: IROp) -> Emit {
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

            IROp::Alloc(_, _) => (),
            IROp::Dealloc(ty, name) => {
                if ty == ConstType::Dynamic || ty == ConstType::Str {
                    let line = self.call_one("free", name);
                    return Emit::Line(line);
                }
            }

            IROp::Const(_, con) => self.push(Item::Const(con)),

            IROp::Store(ty, name) => {
                let b = self.borrow();
                let b_ty = b.get_ty();

                // clone dynamic and strings because they are pointers
                let val = if !(b.is_var() && (b_ty == ConstType::Dynamic || b_ty == ConstType::Str))
                {
                    self.pop_str()
                } else if b_ty == ConstType::Dynamic {
                    let s = self.pop_str();
                    self.call_one("__clone__", s)
                } else {
                    let s = self.pop_str();
                    self.call_one("__strclone__", s)
                };
                let tyc = type_to_c(ty);

                let var = self.variables.get(&name);
                if var.is_none() || var.unwrap().1 != ty {
                    let name = self.var(name, ty);

                    return Emit::Line(format!("{} {} = {}", tyc, name, val));
                } else {
                    let name = self.get_var(name);
                    return Emit::Line(format!("{} = {}", name, val));
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
                    return Emit::Line(call);
                } else {
                    self.push(Item::Expr(ty, call));
                }
            }

            IROp::While(body) => return self.bond_while(body),
            IROp::If(_, body, alt) => return self.bond_if(body, alt),

            IROp::Conv(into, from) => {
                self.bond_conv(into, from);
            }

            IROp::Pop => {
                if self.stack.len() > 0 {
                    return Emit::Line(self.pop_str());
                }
            }

            IROp::Ret(_) => {
                let val = self.pop_str();
                return Emit::Line(format!("return {}", val));
            }
            _ => return self.bond_binary(op), // attempt to bond binary expr instead
        }
        Emit::None
    }
    fn bond_while(&mut self, body: Vec<IROp>) -> Emit {
        let mut emiter = self.emiter();
        let cond = self.pop_str();

        emiter.emit_header(format!("while ({}) {{", cond));
        for expr in body {
            let emit = self.bond(expr);

            emiter.embed(emit);
        }

        emiter.end();
        Emit::Body(emiter.finish())
    }

    fn bond_if(&mut self, body: Vec<IROp>, alt: Vec<IROp>) -> Emit {
        let mut emiter = self.emiter();

        let cond = self.pop_str();
        emiter.emit_header(format!("if ({}) {{", cond));
        for expr in body {
            let emit = self.bond(expr);

            emiter.embed(emit);
        }

        emiter.end();

        if alt.len() > 0 {
            let mut compiled_alt = vec![];
            for expr in alt {
                let emit = self.bond(expr);
                match emit {
                    Emit::Body(lines) => {
                        for line in lines {
                            compiled_alt.push(line);
                        }
                    }
                    Emit::Line(line) => compiled_alt.push(format!("{};", line)),
                    Emit::None => (),
                }
            }

            if compiled_alt[0].starts_with("if") {
                compiled_alt[0] = format!("else {}", compiled_alt[0]);
                emiter.lines(compiled_alt);
            } else {
                emiter.emit_header("else {");
                emiter.lines(compiled_alt);
                emiter.end();
            }
        }

        Emit::Body(emiter.finish())
    }

    #[inline]
    fn genbinary(&mut self, op: &str, ty: ConstType) -> Item {
        if &self.borrow().get_ty() == &ConstType::Str {
            let binop = match op {
                "+" => "__stradd__",
                "-" => "__strsub__",
                "==" => "__streq__",
                ">" => "__strcomp__",
                ">=" => "__strecomp__",
                _ => panic!(),
            };
            Item::Expr(
                ty,
                format!("{}({}, {})", binop, self.pop_str(), self.pop_str()),
            )
        } else {
            Item::Expr(ty, format!("{} {} {}", self.pop_str(), op, self.pop_str()))
        }
    }
    #[inline]
    fn binary(&mut self, op: &str) -> Item {
        let ty = self.borrow().get_ty();
        self.genbinary(op, ty)
    }

    #[inline]
    fn binaryb(&mut self, op: &str) -> Item {
        self.genbinary(op, ConstType::Bool)
    }

    fn bond_binary(&mut self, op: IROp) -> Emit {
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
        Emit::None
    }

    fn bond_conv(&mut self, into: ConstType, from: ConstType) {
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
