use core::fmt::Display;
use indexmap::IndexMap;
use std::collections::HashMap;

use lazy_static::lazy_static;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BasicType {
    Int,
    Float,

    Bool,
    Void,
}

impl Display for BasicType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Float => write!(f, "float"),
            Self::Int => write!(f, "int"),
            Self::Void => write!(f, "void"),
            Self::Bool => write!(f, "bool"),
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
    pub overloads: Vec<String>,
}

impl Display for BlueprintType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Blueprint({}, {})", self.name, self.overloads.join(", "))
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    pub name: String,
    pub fields: HashMap<String, AtomType>,
    pub generics: IndexMap<String, AtomType>,
}

impl Atom {
    pub fn new(
        name: String,
        fields: HashMap<String, AtomType>,
        generics: IndexMap<String, AtomType>,
    ) -> Atom {
        Atom {
            name,
            fields,
            generics,
        }
    }

    // populates generics with given specs
    pub fn spec(&self, specs: &[AtomType]) -> Self {
        let mut this = self.clone();
        for (idx, spec) in specs.iter().enumerate() {
            *this.generics.get_index_mut(idx).unwrap().1 = spec.clone();
        }
        this
    }
}

// makes an atom easily
macro_rules! complex {
    ($name:expr, { $($field_name:expr => $field_type:expr),* }, { $($generic_name:expr),* }) => {
        Atom::new(
            $name.to_owned(),
            HashMap::from([$(($field_name.to_owned(), $field_type)),*]),
            IndexMap::from([$(($generic_name.to_owned(), AtomType::Unknown(None))),*]),
        )
    };
}

lazy_static! {
    pub static ref List: Atom =
        complex!("List", {"size" => AtomType::Basic(BasicType::Int)}, {"T"});
    pub static ref Str: Atom = complex!("str", {"size" => AtomType::Basic(BasicType::Int)}, {"T"});
    pub static ref Back: Atom = complex!("Back", {}, { "T" });
    pub static ref Const: Atom = complex!("Const", {"T" => AtomType::Unknown(None)}, {"T"});
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
    Dynamic,                    // may be scrapped, says that type is only known at runtime
    Unknown(Option<Box<Self>>), // Unknown(None) means that expr type is unknown later on it should be replaced with Unknown(Some(AtomType)) where some is an assumption and even later it is unwarped or converted to the Some type (may be replaced to be simpler)
    Any, // mainly used for mangling and Symbol.expected, means that symbol can be of Any type
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
