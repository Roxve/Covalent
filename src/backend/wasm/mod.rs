use std::collections::HashMap;

use wasm_encoder::{
    CodeSection, ExportKind, ExportSection, Function, FunctionSection, Instruction, Module,
    TypeSection, ValType,
};

use crate::ir::{Const, ConstType, IROp};
#[derive(Debug, Clone)]
pub struct Section {
    types: TypeSection,
    code: CodeSection,
    func: FunctionSection,
    exports: ExportSection,
}

impl Section {
    pub fn new() -> Self {
        Section {
            types: TypeSection::new(),
            code: CodeSection::new(),
            func: FunctionSection::new(),
            exports: ExportSection::new(),
        }
    }
}
#[derive(Debug, Clone)]
pub struct Func<'a> {
    body: Vec<Instruction<'a>>,
    vars: Vec<(String, ValType)>,
}

impl ConstType {
    fn into_val_type(self) -> ValType {
        match self {
            ConstType::Int => ValType::I32,
            ConstType::Float => ValType::F32,
            _ => todo!("const into val"),
        }
    }
}

impl<'a> Func<'a> {
    fn new() -> Self {
        Self {
            body: vec![],
            vars: vec![],
        }
    }

    fn finish(&mut self) -> Function {
        let mut fun = Function::new_with_locals_types({
            let ve: Vec<ValType> = self.vars.clone().iter().map(|l| l.1).collect();
            ve
        });
        for ins in self.body.clone() {
            fun.instruction(&ins);
        }
        fun
    }

    fn add_var(&mut self, name: String, ty: ValType) {
        self.vars.push((name, ty));
    }

    fn get_var(&mut self, name: String) -> u32 {
        self.vars.iter().position(|k| k.0 == name).unwrap() as u32
    }
}

pub struct Codegen<'a> {
    current: Func<'a>,
    funcs: HashMap<String, (u32, Func<'a>)>,
    module: Module,
    section: Section,
    ir: Vec<IROp>,
    ip: usize,
}
impl<'a> Codegen<'a> {
    pub fn new(ir: Vec<IROp>) -> Self {
        let mut section = Section::new();
        let _ty = section.types.function(vec![], vec![]);
        let _func = section.func.function(0);
        let module = Module::new();
        Codegen {
            current: Func::new(),
            funcs: HashMap::new(),
            section,
            module,
            ir,
            ip: 0,
        }
    }

    fn insert(&mut self, inst: Instruction<'a>) {
        self.current.body.push(inst);
        self.ip += 1;
    }

    pub fn codegen(&mut self) -> &mut Module {
        while self.ip <= self.ir.len() - 1 {
            self.bond(self.ir[self.ip].clone());
        }
        self.insert(Instruction::Return);

        self.insert(Instruction::End);
        self.module.section(&self.section.types);
        self.module.section(&self.section.func);

        self.section.exports.export("_start", ExportKind::Func, 0);
        self.module.section(&self.section.exports);
        // finnaly adding all functions indx by order

        // _start
        self.section.code.function(&self.current.finish());

        let mut funcs_ordered: Vec<(u32, Func)> = self.funcs.clone().into_values().collect();
        funcs_ordered.sort_by_key(|k| k.0);

        for (_, mut func) in funcs_ordered {
            self.section.code.function(&func.finish());
        }

        self.module.section(&self.section.code)
    }

    pub fn bond(&mut self, op: IROp) {
        match op {
            IROp::Const(ConstType::Int, Const::Int(i)) => self.insert(Instruction::I32Const(i)),
            IROp::Const(ConstType::Float, Const::Float(f)) => {
                self.insert(Instruction::F32Const(f));
            }
            IROp::Add(_) | IROp::Mul(_) | IROp::Div(_) | IROp::Sub(_) => {
                return self.bond_binary_atoms(op);
            }
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
        dbg!(&func_type);
        self.funcs
            .insert(name, (func_type.clone() as u32, self.current.clone()));

        let _ = self.section.func.function(func_type);
        // retreat fr
        self.current = old;
        self.ir = old_instr;
        self.ip = old_ip + 1;
    }
}
