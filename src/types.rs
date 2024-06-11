use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AtomKind {
    Type(Box<Self>),
    Int,
    Float,
    Str,
    Bool,
    Dynamic,
    Void,
    Unknown(Option<Box<Self>>),
    List(Box<Self>),
    Func(Box<Self>, Vec<Self>, String),
    Blueprint { argc: u32, name: String },
    Obj(HashMap<String, Self>),
}

impl AtomKind {
    pub fn get(&self, name: &String) -> Option<Self> {
        match self {
            Self::Obj(o) => o.get(name).cloned(),
            Self::List(_) => {
                if name == &"size".to_string() || name == &"elem_size".to_string() {
                    Some(Self::Int)
                } else {
                    None
                }
            }
            _ => None,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            Self::Int => "int",
            Self::Float => "float",
            Self::Str => "str",
            Self::Bool => "bool",
            _ => "none",
        }
    }
}

pub fn type_mangle(name: String, types: Vec<AtomKind>) -> String {
    let mut mangle = String::new();
    mangle.push_str(name.as_str());
    if types.len() == 0 {
        mangle.push_str("_empty");
    }

    for type_n in types {
        mangle.push('_');
        mangle.push_str(type_n.as_str());
    }

    return mangle;
}
