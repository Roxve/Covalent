use core::fmt::Display;
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicType {
    Int,
    Float,
    Str,
    Void,
}

impl Display for BasicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => write!(f, "float"),
            Self::Int => write!(f, "int"),
            Self::Void => write!(f, "void"),
            Self::Str => write!(f, "str"),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct FunctionType {
    pub params: Vec<AtomType>,
    pub return_type: Box<AtomType>,
}

impl Display for FunctionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let params: Vec<String> = self.params.iter().map(|param| param.to_string()).collect();
        write!(f, "Fn({}) -> {}", params.join(", "), self.return_type)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct BlueprintType {
    pub name: String,
    pub overlords: Vec<String>,
}

impl Display for BlueprintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blueprint({}, {})", self.name, self.overlords.join(", "))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    pub name: String,
    pub fields: HashMap<String, AtomType>,
    pub generics: HashMap<String, AtomType>,
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let generics = if !self.generics.is_empty() {
            let gen_str: Vec<String> = self
                .generics
                .iter()
                .map(|(gen_name, gen)| {
                    if let AtomType::Unknown(None) = gen {
                        gen_name.clone()
                    } else {
                        gen.to_string()
                    }
                })
                .collect();
            format!("({})", gen_str.join(", "))
        } else {
            String::new()
        };
        write!(f, "{}{}", self.name, generics)
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum AtomType {
    Basic(BasicType),
    Atom(Atom),
    Function(FunctionType),
    Blueprint(BlueprintType),
    Dynamic,
    Unknown(Option<Box<Self>>),
    Any,
    Type(Box<AtomType>), // used for ids and things that reference another AtomType
}

impl Display for AtomType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Any => write!(f, "any"),
            Self::Dynamic => write!(f, "Dynamic"),
            Self::Basic(b) => write!(f, "{}", b),
            Self::Atom(a) => write!(f, "{}", a),
            Self::Blueprint(b) => write!(f, "{}", b),
            Self::Function(fun) => write!(f, "{}", fun),
            Self::Unknown(None) => write!(f, "Unknown"),
            Self::Unknown(Some(t)) => write!(f, "Unknown({})", t),
            Self::Type(t) => write!(f, "TypeID({})", t), // Handle new variant
        }
    }
}

impl AtomType {
    pub fn get(&self, name: &String) -> Option<&Self> {
        match self {
            Self::Atom(a) => a.fields.get(name),

            _ => None,
        }
    }

    pub fn generics(&self) -> i32 {
        match self {
            Self::Atom(a) => a.generics.len() as i32,
            Self::Type(t) => (&**t).generics(),
            _ => 0,
        }
    }
}

pub fn type_mangle(mut name: String, types: Vec<AtomType>) -> String {
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

        mangle.push_str(
            type_n
                .to_string()
                .replace("(", "__")
                .replace(")", "__")
                .replace(",", "_")
                .as_str(),
        );
    }

    return mangle;
}

pub fn mangle_types(mangle: String) -> Vec<String> {
    let types = mangle.get(mangle.find('$').unwrap() + 1..).unwrap();
    types.split('_').map(|s| s.to_string()).collect()
}

// impl Display for AtomType {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             &AtomKind::Int => write!(f, "int"),
//             &AtomKind::Float => write!(f, "float"),
//             &AtomKind::Bool => write!(f, "bool"),
//             &AtomKind::Str => write!(f, "str"),

//             &AtomKind::Any => write!(f, "any"),
//             &AtomKind::Dynamic => write!(f, "dynamic"),
//             &AtomKind::Void => write!(f, "void"),

//             &AtomKind::Unknown(ref assume) => match assume {
//                 &Some(ref t) => write!(f, "Unknown(some({}))", t.to_string()),
//                 &None => write!(f, "Unknown(none)"),
//             },

//             &AtomKind::Backend(ref t) => write!(f, "Back({})", t.to_string()),
//             &AtomKind::Const(ref t) => write!(f, "Const({})", t.to_string()),

//             &AtomKind::List(ref ty) => write!(f, "List({})", ty.to_string()),
//             &AtomKind::Type(ref ty) => write!(f, "Type({})", ty.to_string()),
//             &AtomKind::Func(ref ret, ref args, ref name) => write!(
//                 f,
//                 "{}@{}{}",
//                 name,
//                 ret,
//                 if args.len() > 0 {
//                     ": ".to_owned()
//                         + &args
//                             .iter()
//                             .map(|arg| arg.to_string())
//                             .collect::<Vec<String>>()
//                             .join(", ")
//                 } else {
//                     "!".to_string()
//                 }
//             ),

//             &AtomKind::Blueprint(ref ref_name, _) => write!(f, "Function(\"{}\")", ref_name),
//             &AtomKind::Obj(_) => todo!(),
//         }
//     }
// }
