use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstType {
    Int,
    Float,
    Str,
    Bool,
    Dynamic,
    Void,
    Func(Box<Self>, Vec<Self>),
    Obj(HashMap<String, Self>),
}

impl ConstType {
    pub fn get(&self, name: &String) -> Option<Self> {
        if let Self::Obj(o) = self {
            o.get(name).cloned()
        } else {
            None
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum ErrKind {
    UnknownCharE,
    UnexceptedTokenE,
    InvaildType,
    UndeclaredVar,
    VarAlreadyDeclared,
    OperationNotGranted,
    UnexceptedArgs,
}

#[derive(Debug, Clone)]
pub struct ATErr {
    pub kind: ErrKind,
    pub msg: String,
    pub line: u32,
    pub column: u32,
}

impl ATErr {
    pub fn get_error(&self) -> String {
        format!(
            "code:AT00{}\n{}\nat line:{}, column:{}",
            self.kind.clone() as u8,
            self.msg,
            self.line,
            self.column
        )
    }

    // customize later
    pub fn out_error(&self) {
        println!("{}", self.get_error());
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident {
    pub val: String,
    pub tag: Option<ConstType>,
}

#[derive(Debug, Clone)]
pub struct CompiledFunction {
    pub name: String,
    pub args: Vec<Ident>,
}

// #[derive(Debug, Hash, Clone, PartialEq)]
// pub struct Symbol {
//     id: String,
//     ty: ConstType,
//     pub children: Vec<Symbol>,
// }

// impl Symbol {
//     pub fn new(id: String, children: Vec<Symbol>, ty: ConstType) -> Self {
//         Self { id, children, ty }
//     }
//     pub fn get(&self) -> String {
//         self.id.clone()
//     }

//     pub fn get_ty(&self) -> ConstType {
//         self.ty.clone()
//     }
// }

#[derive(Clone, Debug)]
pub struct Enviroment {
    functions: Vec<CompiledFunction>,
    pub vars: HashMap<String, ConstType>,
    pub current: ConstType,
    pub parent: Option<Box<Enviroment>>,
}

impl Enviroment {
    pub fn new(parent: Option<Box<Self>>) -> Self {
        Self {
            functions: Vec::new(),
            vars: HashMap::new(),
            current: ConstType::Void,
            parent,
        }
    }

    pub fn child(&self) -> Enviroment {
        Enviroment::new(Some(Box::new(self.clone())))
    }

    pub fn parent(&self) -> Option<Enviroment> {
        if self.parent.is_none() {
            None
        } else {
            Some(*(self.parent.clone().unwrap()))
        }
    }

    pub fn get_ty(&self, name: &String) -> Option<ConstType> {
        if self.vars.contains_key(name) {
            return Some(self.vars[name].clone());
        }

        if self.parent.is_some() {
            return self.parent().unwrap().get_ty(name);
        } else {
            return None;
        }
    }

    pub fn has(&self, name: &String) -> bool {
        if self.vars.contains_key(name) {
            true
        } else if self.parent.is_some() {
            self.parent().unwrap().has(name)
        } else {
            false
        }
    }

    pub fn modify(&mut self, name: &String, ty: ConstType) {
        if self.vars.contains_key(name) {
            self.vars.get_mut(name).map(|val| *val = ty);
        } else if self.parent.is_some() {
            self.parent().unwrap().modify(name, ty);
        }
    }

    pub fn add(&mut self, name: &String, ty: ConstType) {
        self.vars.insert(name.clone(), ty);
    }

    pub fn push_function(&mut self, name: String, args: Vec<Ident>, ty: ConstType) {
        self.vars.insert(
            name.clone(),
            ConstType::Func(
                Box::new(ty),
                args.iter()
                    .map(|t| t.tag.clone().unwrap_or(ConstType::Dynamic))
                    .collect(),
            ),
        );
        self.functions.push(CompiledFunction { name, args });
    }

    pub fn get_function(&self, name: &String) -> Option<CompiledFunction> {
        for fun in self.functions.clone().into_iter() {
            if &fun.name == name {
                return Some(fun);
            }
        }
        if self.parent.is_some() {
            return self.parent.as_ref().unwrap().get_function(&name);
        }

        return None;
    }
}
#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Value,
    _Func(String),
    Top,
}

impl Scope {
    pub fn is_used(&self) -> bool {
        let owned = self.to_owned();

        owned == Scope::Value
    }
}
