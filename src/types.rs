use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AtomKind {
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

    // if we only have no args, later we should analyze blueprints with only 1 possible instance
    if types.len() == 0 {
        mangle.push_str("_void");
    }
    for type_n in types {
        mangle.push('_');
        mangle.push_str(type_n.as_str());
    }

    return mangle;
}
