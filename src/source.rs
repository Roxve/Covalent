use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstType {
    Int,
    Float,
    Str,
    Bool,
    Dynamic,
    Void,
    List(Box<Self>),
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

#[derive(Clone, Debug)]
pub struct Enviroment {
    pub vars: HashMap<String, ConstType>,
    pub current: ConstType,
    pub parent: Option<Box<Enviroment>>,
}

impl Enviroment {
    pub fn new(parent: Option<Box<Self>>) -> Self {
        Self {
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
