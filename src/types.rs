use core::fmt::Display;
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum AtomKind {
    Type(Box<Self>),
    Backend(Box<Self>),
    Const(Box<Self>),
    Unknown(Option<Box<Self>>),

    Int,
    Float,
    Str,
    Bool,

    Dynamic,
    Any,
    Void,

    List(Box<Self>),

    Func(Box<Self>, Vec<Self>, String),
    Blueprint(String, Vec<String>),
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

    pub fn generics(&self) -> i32 {
        match self {
            &Self::List(_) | &Self::Backend(_) | &Self::Const(_) => 1,
            &Self::Type(ref t) => (&**t).generics(),
            _ => 0,
        }
    }
}

pub fn type_mangle(mut name: String, types: Vec<AtomKind>) -> String {
    let name = {
        let idx = name.find('$');
        if idx.is_some() {
            let idx = idx.unwrap();
            name.truncate(idx);
        }
        name
    }; // removes any previous mangles from name

    let mut mangle = String::new();
    mangle.push_str(name.as_str());
    mangle.push('$'); // type start

    if types.len() == 0 {
        mangle.push_str("empty");
    }

    let mut first = true; // if its the first time running loop (under)

    for type_n in types {
        if !first {
            mangle.push('_');
        } else {
            first = false
        }

        mangle.push_str(type_n.to_string().as_str());
    }

    return mangle;
}

pub fn mangle_types(mangle: String) -> Vec<String> {
    let types = mangle.get(mangle.find('$').unwrap() + 1..).unwrap();
    types.split('_').map(|s| s.to_string()).collect()
}
impl Display for AtomKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            &AtomKind::Int => write!(f, "int"),
            &AtomKind::Float => write!(f, "float"),
            &AtomKind::Bool => write!(f, "bool"),
            &AtomKind::Str => write!(f, "str"),

            &AtomKind::Any => write!(f, "any"),
            &AtomKind::Dynamic => write!(f, "dynamic"),
            &AtomKind::Void => write!(f, "void"),

            &AtomKind::Unknown(ref assume) => match assume {
                &Some(ref t) => write!(f, "Unknown(some({}))", t.to_string()),
                &None => write!(f, "Unknown(none)"),
            },

            &AtomKind::Backend(ref t) => write!(f, "Back({})", t.to_string()),
            &AtomKind::Const(ref t) => write!(f, "Const({})", t.to_string()),

            &AtomKind::List(ref ty) => write!(f, "List({})", ty.to_string()),
            &AtomKind::Type(ref ty) => write!(f, "Type({})", ty.to_string()),
            &AtomKind::Func(ref ret, ref args, ref name) => write!(
                f,
                "{}@{}{}",
                name,
                ret,
                if args.len() > 0 {
                    ": ".to_owned()
                        + &args
                            .iter()
                            .map(|arg| arg.to_string())
                            .collect::<Vec<String>>()
                            .join(", ")
                } else {
                    "!".to_string()
                }
            ),

            &AtomKind::Blueprint(ref ref_name, _) => write!(f, "Function(\"{}\")", ref_name),
            &AtomKind::Obj(_) => todo!(),
        }
    }
}
