use std::collections::HashMap;

use crate::parser::ast::Node;

#[derive(Debug, Clone, PartialEq)]
pub enum ConstType {
    Int,
    Float,
    Str,
    Bool,
    Dynamic,
    Void,
    Unknown(Option<Box<ConstType>>),
    List(Box<Self>),
    Func(Box<Self>, Vec<Self>, String),
    Blueprint { argc: u32, name: String },
    Obj(HashMap<String, Self>),
}

impl ConstType {
    pub fn get(&self, name: &String) -> Option<Self> {
        match self {
            Self::Obj(o) => o.get(name).cloned(),
            Self::List(_) => {
                if name == &"size".to_string() || name == &"elem_size".to_string() {
                    Some(ConstType::Int)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConstType::Int => "int",
            ConstType::Float => "float",
            ConstType::Str => "str",
            ConstType::Bool => "bool",
            _ => "none",
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
pub struct Blueprint {
    pub name: String,
    pub args: Vec<Ident>,
    pub body: Vec<Node>,
}

#[derive(Clone, Debug)]
pub struct Enviroment {
    pub vars: HashMap<String, ConstType>,
    pub current: ConstType,
    pub parent: Option<Box<Enviroment>>,
    pub blueprints: Vec<Blueprint>,
}
pub fn type_mangle(name: String, types: Vec<ConstType>) -> String {
    let mut mangle = String::new();
    mangle.push_str(name.as_str());

    for type_n in types {
        mangle.push('_');
        mangle.push_str(type_n.as_str());
    }
    return mangle;
}
impl Enviroment {
    pub fn new(parent: Option<Box<Self>>) -> Self {
        Self {
            vars: HashMap::new(),
            current: ConstType::Void,
            parent,
            blueprints: Vec::new(),
        }
    }

    pub fn blueprints(&mut self, blueprints: Vec<Blueprint>) {
        self.blueprints = blueprints.clone();
        for blueprint in blueprints {
            self.add(
                &blueprint.name,
                ConstType::Blueprint {
                    argc: blueprint.args.len() as u32,
                    name: blueprint.name.clone(),
                },
            );
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

    // member expr parent is passed to a function as first arg if it takes it as an arg for ex.
    // set push: List(T) self, T item -> List(T)
    pub fn ty_parent_fn(&self, ty: &ConstType, name: &String) -> Option<ConstType> {
        let parent = self.vars.get(name);
        if parent.is_some() {
            if let ConstType::Func(_, args, _) = parent.unwrap() {
                if &args[0] == ty {
                    return Some(parent.unwrap().to_owned());
                }
            }
        }
        None
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

    pub fn get_blueprint(&self, name: &String) -> Option<Blueprint> {
        for blueprint in &self.blueprints {
            if &blueprint.name == name {
                return Some(blueprint.clone());
            }
        }
        if self.parent.is_some() {
            for blueprint in &self.parent().unwrap().blueprints {
                if &blueprint.name == name {
                    return Some(blueprint.clone());
                }
            }
        }
        return None;
    }

    pub fn push_function(&mut self, name: String, args: Vec<ConstType>, ty: ConstType) {
        self.vars
            .insert(name.clone(), ConstType::Func(Box::new(ty), args, name));
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
