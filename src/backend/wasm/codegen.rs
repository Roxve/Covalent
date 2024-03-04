use super::*;
use std::collections::HashMap;
use wasm_encoder::{ExportKind, MemArg};

impl<'a> Codegen<'a> {
    pub fn add_extern(&mut self, name: String) {
        self.funcs.insert(name, (self.funcs.len() as u32, None));
    }
    pub fn get_fun(&self, name: String) -> u32 {
        self.funcs.get(&name).unwrap().0
    }
    pub fn import(&mut self, module: &str, name: &str) -> &mut Self {
        self.section
            .imports
            .import(module, name, EntityType::Function(self.funcs.len() as u32));
        self.add_extern(name.to_string());
        self
    }

    pub fn new(ir: Vec<IROp>) -> Self {
        let mut section = Section::new();
        let _ty = section.types.function(vec![], vec![]);
        let _func = section.func.function(0);
        let module = Module::new();
        let mut res = Self {
            current: Func::new(),
            funcs: HashMap::new(),
            section,
            module,
            ir,
            ip: 0,
        };

        res.import("mem", "talloc");
        res.add_extern("_start".to_string());

        res
    }

    fn insert(&mut self, inst: Instruction<'a>) {
        self.current.body.push(inst);
        self.ip += 1;
    }

    fn insertp(&mut self, inst: Instruction<'a>) {
        self.current.body.push(inst);
    }
    pub fn codegen(&mut self) -> &mut Module {
        while self.ip <= self.ir.len() - 1 {
            self.bond(self.ir[self.ip].clone());
        }
        self.insert(Instruction::Return);

        self.insert(Instruction::End);

        self.module.section(&self.section.types);

        self.module.section(&self.section.imports);
        self.module.section(&self.section.func);

        self.section.exports.export(
            "_start",
            ExportKind::Func,
            self.get_fun("_start".to_string()),
        );
        self.module.section(&self.section.exports);

        // finnaly adding all functions indx by order

        // _start
        self.section.code.function(&self.current.finish());

        let mut funcs_ordered: Vec<(u32, Option<Func>)> =
            self.funcs.clone().into_values().collect();
        funcs_ordered.sort_by_key(|k| k.0);

        for (_, func) in funcs_ordered {
            if func.is_some() {
                self.section.code.function(&func.unwrap().finish());
            }
        }

        self.module.section(&self.section.code)
    }

    pub fn bond(&mut self, op: IROp) {
        match op {
            IROp::Const(ConstType::Int, Const::Int(i)) => self.insert(Instruction::I32Const(i)),
            IROp::Const(ConstType::Float, Const::Float(f)) => {
                self.insert(Instruction::F32Const(f));
            }
            IROp::Add(_) | IROp::Mul(_) | IROp::Div(_) | IROp::Sub(_) => self.bond_binary_atoms(op),
            IROp::Alloc(ty, name) => {
                self.current.add_var(name, ty.into_val_type());
                self.ip += 1;
            }
            IROp::Dealloc(_, _) => self.insert(Instruction::Nop),
            IROp::Store(_, name) => {
                let idx = self.current.get_var(name);
                self.insert(Instruction::LocalSet(idx));
            }
            IROp::Load(_, name) => {
                let idx = self.current.get_var(name);
                self.insert(Instruction::LocalGet(idx));
            }
            IROp::Ret(_) => {
                self.insert(Instruction::End);
            }
            IROp::Def(ty, name, args, body) => {
                self.bond_func_atoms(ty.unwrap_or(ConstType::Void), name, args, body);
            }
            IROp::Conv(into, from) => {
                match into {
                    ConstType::Dynamic => match from {
                        ConstType::Int => {
                            self.insert(Instruction::I32Store(MemArg {
                                offset: 1,
                                align: 2,
                                memory_index: 0,
                            }));
                            // first we write the type
                            let idx = self.current.get_var("alloc".to_string());
                            self.insertp(Instruction::LocalGet(idx));
                            self.insertp(Instruction::I32Const(TYPE_INT));
                            self.insertp(Instruction::I32Store8(MemArg {
                                offset: 0,
                                align: 0,
                                memory_index: 0,
                            }));
                            self.insertp(Instruction::LocalGet(idx));
                        }
                        _ => todo!("add conv {:?} into dynamic", from),
                    },
                    _ => todo!("add whole conv {:?}", into),
                }
            }
            IROp::Call(_, name) => {
                let idx = self.funcs.get(&name).unwrap().0;
                dbg!(&idx);
                self.insert(Instruction::Call(idx));
            }

            _ => todo!("op not implented for the wasm backend {:?}", op),
        }
    }

    pub fn bond_binary_atoms(&mut self, op: IROp) {
        match op {
            IROp::Add(ty) => match ty {
                ConstType::Int => self.insert(Instruction::I32Add),
                ConstType::Float => self.insert(Instruction::F32Add),
                _ => todo!("add + for type {:?}", ty),
            },
            _ => todo!("add operator {:?}", op),
        }
    }

    pub fn bond_func_atoms(
        &mut self,
        ty: ConstType,
        name: String,
        args: Vec<String>,
        body: Vec<IROp>,
    ) {
        let mut func = Func::new();

        // all args are dynamic for now
        for arg in &args {
            func.add_var(arg.to_owned(), ValType::I32);
        }
        let params: Vec<ValType> = args.iter().map(|_| ValType::I32).collect();

        let old = self.current.clone();
        let old_ip = self.ip;
        let old_instr = self.ir.clone();

        self.current = func;
        self.ir = body;
        dbg!(&self.ir);
        self.ip = 0;

        // EXECUTE CODEGEN!!!!!

        while self.ip <= self.ir.len() - 1 {
            self.bond(self.ir[self.ip].clone());
        }

        let func_type = self
            .section
            .types
            .function(params, vec![ty.into_val_type()])
            .len()
            - 1;
        self.funcs
            .insert(name, (func_type.clone() as u32, Some(self.current.clone())));

        let _ = self.section.func.function(func_type);
        // retreat fr
        self.current = old;
        self.ir = old_instr;
        self.ip = old_ip + 1;
    }
}
