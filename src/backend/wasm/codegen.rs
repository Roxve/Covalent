use super::*;
use std::collections::HashMap;
use wasm_encoder::{ExportKind, SymbolTable};

impl<'a> Codegen<'a> {
    pub fn add_extern(&mut self, name: String) {
        self.funcs.insert(name, (self.funcs.len() as u32, None));
    }
    pub fn get_fun(&self, name: String) -> u32 {
        self.funcs.get(&name).unwrap().0
    }
    pub fn import(
        &mut self,
        module: &str,
        name: &str,
        args: Vec<ValType>,
        res: Vec<ValType>,
    ) -> &mut Self {
        self.section
            .imports
            .import(module, name, EntityType::Function(self.funcs.len() as u32));
        self.table.function(
            SymbolTable::WASM_SYM_UNDEFINED | SymbolTable::WASM_SYM_EXPLICIT_NAME,
            self.funcs.len() as u32,
            Some(name),
        );

        self.section.types.function(args, res);

        self.add_extern(name.to_string());
        self
    }

    pub fn new(ir: Vec<IROp>) -> Self {
        let section = Section::new();

        let module = Module::new();
        let mut res = Self {
            current: Func::new(),
            funcs: HashMap::new(),
            section,
            table: SymbolTable::new(),
            module,
            ir,
            ip: 0,
        };

        res.import("mem", "talloc", vec![ValType::I32], vec![ValType::I32])
            .import("mem", "mk_int", vec![ValType::I32], vec![ValType::I32])
            .import(
                "runtime",
                "__add__",
                vec![ValType::I32, ValType::I32],
                vec![ValType::I32],
            );
        for import in res.ir.clone() {
            if let IROp::Import(ty, modu, name, args) = import {
                res.ir.remove(0);
                let resu = {
                    if ty == ConstType::Void {
                        vec![]
                    } else {
                        vec![ty.into_val_type()]
                    }
                };
                res.import(
                    modu.as_str(),
                    name.as_str(),
                    args.into_iter().map(|k| k.into_val_type()).collect(),
                    resu,
                );
            } else {
                break;
            }
        }

        res.section.types.function(vec![], vec![]);
        res.add_extern("_start".to_string());

        res.section.func.function(res.get_fun("_start".to_string()));
        // linking our _start
        // weak and exported!!!
        res.table.function(
            SymbolTable::WASM_SYM_BINDING_WEAK | SymbolTable::WASM_SYM_EXPORTED,
            res.get_fun("_start".to_string()),
            Some("_start"),
        );

        res
    }

    fn insert(&mut self, inst: Instruction<'a>) {
        self.current.body.push(inst);
        self.ip += 1;
    }

    // fn insertp(&mut self, inst: Instruction<'a>) {
    //     self.current.body.push(inst);
    // }

    fn call(&mut self, name: &str) {
        self.insert(Instruction::Call(self.get_fun(name.to_string())))
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

        let mut funcs_ordered: Vec<(u32, Option<Func>)> =
            self.funcs.clone().into_values().collect();
        funcs_ordered.sort_by_key(|k| k.0);

        // giving start a body
        funcs_ordered[self.get_fun("_start".to_string()) as usize] = (
            self.get_fun("_start".to_string()),
            Some(self.current.clone()),
        );

        for (_, func) in funcs_ordered {
            if func.is_some() {
                self.section.code.function(&func.unwrap().finish());
            }
        }

        self.module.section(&self.section.code);
        self.section.linking.symbol_table(&self.table);
        self.module.section(&self.section.linking)
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
                self.bond_func_atoms(ty, name, args, body);
            }
            IROp::Conv(into, from) => match into {
                ConstType::Dynamic => match from {
                    ConstType::Int => self.call("mk_int"),
                    ConstType::Float => self.call("mk_float"),
                    ConstType::Dynamic => self.ip += 1,
                    _ => todo!("add conv {:?} into dynamic", from),
                },
                _ => todo!("add whole conv {:?}", into),
            },
            IROp::Call(_, name) => {
                let idx = self.funcs.get(&name).unwrap().0;
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
                ConstType::Dynamic => self.call("__add__"),
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
        let idx = self.funcs.len();
        self.funcs
            .insert(name.clone(), (idx as u32, Some(self.current.clone())));
        self.table.function(
            SymbolTable::WASM_SYM_BINDING_LOCAL,
            idx as u32,
            Some(name.as_str()),
        );
        let _ = self.section.func.function(func_type);
        // retreat fr
        self.current = old;
        self.ir = old_instr;
        self.ip = old_ip + 1;
    }
}
