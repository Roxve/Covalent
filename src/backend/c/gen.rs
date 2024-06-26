use core::panic;

use super::{type_to_c, types_to_cnamed, Codegen, Emit, Item};
use crate::{
    ir::{get_op_type, IROp},
    parser::ast::Ident,
    types::{self, AtomKind, AtomType, BasicType},
};

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
                    args.into_iter().map(|i| i.tuple().clone()).collect(),
                    ret,
                    body,
                );

                ir.remove(0);
            } else {
                break;
            }
        }

        self.bond_fn(
            "main".to_string(),
            Vec::new(),
            AtomType {
                kind: AtomKind::Basic(BasicType::Int),
                details: None,
            },
            ir,
        );

        self.module.finish()
    }
    fn bond_fn(
        &mut self,
        name: String,
        args: Vec<(AtomType, String)>,
        ret: AtomType,
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

    fn bond_extern(&mut self, ret: AtomType, name: String, params: Vec<Ident>) -> Emit {
        let ty = type_to_c(ret);
        let params = types_to_cnamed(params.iter().map(|x| x.clone().tuple()).collect());
        self.module.extern_add(format!("{ty} {name}({params});"));
        Emit::None
    }

    pub fn bond(&mut self, op: IROp) -> Emit {
        match op {
            IROp::Def(ret, name, args, body) => {
                self.bond_fn(
                    name,
                    args.into_iter().map(|i| i.tuple().clone()).collect(),
                    ret,
                    body,
                );
            }

            IROp::Extern(ret, name, params) => return self.bond_extern(ret, name, params),

            IROp::Alloc(_, _) => (),
            IROp::Dealloc(ty, name) => {
                // free heap allocated types
                if ty.kind == AtomKind::Atom(types::Str.clone()) {
                    let line = self.call_one("free", name);
                    return Emit::Line(line);
                }
            }

            IROp::Const(con) => self.push(Item::Const(con)),
            IROp::List(ty, items) => {
                for item in items.clone() {
                    for expr in item {
                        self.bond(expr);
                    }
                }
                self.push(Item::List(ty, items.len() as u16));
            }

            IROp::Store(ty, name) => {
                let val = self.pop_str();
                let tyc = type_to_c(ty.clone());

                return Emit::Line(format!("{} {} = {}", tyc, name, val));
            }

            IROp::Load(ty, name) => {
                let name = self.get_var(name);
                self.push(Item::Var(ty, name));
            }

            IROp::LoadProp(ty, name) => {
                let id = self.pop_str();
                self.push(Item::Expr(ty, format!("{}->{}", id, name)));
            }

            IROp::LoadIdx(ty) => {
                let idx = self.pop_str();
                let expr = self.pop_str();

                self.push(Item::Expr(
                    ty.clone(),
                    format!("__listget__({expr}, {}, {idx})", type_to_c(ty)),
                ))
            }

            IROp::Call(ty, count) => {
                let arg_count = count;
                let name = self.pop_str();
                let args = self.pop_amount(arg_count).join(", ");
                let call = format!("{}({})", name, args);
                if &ty.kind == &AtomKind::Basic(BasicType::Void) {
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

            IROp::Set(ty) => {
                let val = self.pop_str();

                let name = self.pop_str();
                let tyc = type_to_c(ty.clone());

                let var = self.variables.get(&name);
                if var.is_some() {
                    if var.unwrap().1 != ty {
                        let name = self.var(name, ty);

                        return Emit::Line(format!("{} {} = {}", tyc, name, val));
                    }
                } else {
                    let name = self.get_var(name);
                    return Emit::Line(format!("{} = {}", name, val));
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
    fn genbinary(&mut self, op: &str, ty: AtomType) -> Item {
        if &self.borrow().get_ty().kind == &AtomKind::Atom(types::Str.clone()) {
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
        self.genbinary(
            op,
            AtomType {
                kind: AtomKind::Basic(BasicType::Bool),
                details: None,
            },
        )
    }

    fn bond_binary(&mut self, op: IROp) -> Emit {
        let item = if get_op_type(&op).kind == AtomKind::Dynamic
            || self.borrow().get_ty().kind == AtomKind::Dynamic
        {
            let ops = vec![self.pop_str(), self.pop_str()];
            Item::Expr(
                AtomType {
                    kind: AtomKind::Dynamic,
                    details: None,
                },
                self.call(
                    match op {
                        IROp::Add(_) => "__add__",
                        IROp::Sub(_) => "__sub__",
                        IROp::Mul(_) => "__mul__",
                        IROp::Div(_) => "__div__",
                        IROp::Mod(_) => "__mod__",
                        IROp::Comp => "__comp__",
                        IROp::EComp => "__ecomp__",
                        IROp::Eq => "__eq__",
                        IROp::And => "__and__",
                        IROp::Or => "__or__",
                        _ => todo!(),
                    },
                    ops,
                ),
            )
        } else {
            match op {
                IROp::Add(_) => self.binary("+"),
                IROp::Sub(_) => self.binary("-"),
                IROp::Mul(_) => self.binary("*"),
                IROp::Div(_) => self.binary("/"),
                IROp::Mod(_) => self.binary("%"),
                IROp::Comp => self.binaryb(">"),
                IROp::Eq => self.binaryb("=="),
                IROp::EComp => self.binaryb(">="),
                IROp::And => self.binaryb("&&"),
                IROp::Or => self.binaryb("||"),
                _ => todo!("unimplented op {:#?}", op),
            }
        };
        self.push(item);
        Emit::None
    }

    fn bond_conv(&mut self, into: AtomType, from: AtomType) {
        let item = self.pop_str();
        let conv = match &into.kind {
            &AtomKind::Dynamic => match from.kind {
                AtomKind::Basic(BasicType::Int) => self.call_one("__int__", item),
                AtomKind::Basic(BasicType::Float) => self.call_one("__float__", item),
                AtomKind::Basic(BasicType::Bool) => self.call_one("__bool__", item),

                AtomKind::Atom(ref atom) if atom == &*types::Str => self.call_one("__str__", item),

                AtomKind::Dynamic => item,
                _ => todo!("add conv dynamic from {:?}", from),
            },

            &AtomKind::Basic(BasicType::Float) => match from.kind {
                AtomKind::Basic(BasicType::Int) => format!("(float){item}"),
                _ => panic!(),
            },

            &AtomKind::Atom(ref atom) if atom == &*types::Str => match from.kind {
                AtomKind::Basic(BasicType::Int) => format!("itos({item})"),
                _ => panic!(),
            },

            _ => match &from.kind {
                _ => todo!("add conv into {:?} from {:?}", into, from),
            },
        };

        self.push(Item::Expr(into, conv));
    }
}
