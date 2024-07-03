use core::panic;
use std::fmt::Display;

use crate::types::{self, AtomKind, AtomType, BasicType};
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
}

impl Literal {
    pub fn get_ty(&self) -> AtomType {
        AtomType {
            kind: match self {
                &Self::Int(_) => AtomKind::Basic(BasicType::Int),
                &Self::Float(_) => AtomKind::Basic(BasicType::Float),
                &Self::Str(_) => AtomKind::Atom(types::Str.clone()),
                &Self::Bool(_) => AtomKind::Basic(BasicType::Bool),
            },
            details: None,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Precedence {
    Low, // Lowest possible precedence
    Logic,
    Eq,
    Cmp,
    Add,
    Mul,
}

pub fn get_operator_level(op: &str) -> Precedence {
    match op {
        "&&" | "||" => Precedence::Logic,
        "==" => Precedence::Eq,
        "<" | ">" | ">=" | "<=" => Precedence::Cmp,
        "+" | "-" => Precedence::Add,
        "*" | "/" | "%" => Precedence::Mul,
        _ => todo!(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub body: Vec<Node>,
    pub functions: Vec<Blueprint>,
    pub types: Vec<Atom>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Atom {
    name: String,
    generics: Vec<String>,
    fields: Vec<Ident>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Use(String),
    Literal(Literal),
    ListExpr(Vec<Node>),

    BinaryExpr {
        op: String,
        left: Box<Node>,
        right: Box<Node>,
    },

    Ident(Ident),

    AtomDeclare {
        name: String,
        fields: Vec<Node>,
    },

    VarDeclare {
        name: Ident,
        val: Box<Node>,
    },

    VarAssign {
        name: Box<Node>,
        val: Box<Node>,
    },
    // fn declare ast is genereated in parser.functions
    FnCall {
        name: Box<Node>,
        args: Vec<Node>,
    },

    Import {
        module: String,
        name: String,
        params: Vec<AtomType>,
    },

    Func {
        ret: AtomType,
        name: String,
        args: Vec<Ident>,
        body: Vec<Node>,
    },

    Extern {
        name: Ident,
        params: Vec<Ident>,
    },

    IfExpr {
        condition: Box<Node>,
        body: Box<Node>,
        alt: Option<Box<Node>>,
    },

    WhileExpr {
        condition: Box<Node>,
        body: Box<Node>,
    },

    MemberExpr {
        parent: Box<Node>,
        child: String,
    },

    IndexExpr {
        parent: Box<Node>,
        index: Box<Node>,
    },
    SpecExpr {
        parent: Box<Node>,
        spec: Vec<Node>,
    },

    Discard(Box<Node>),
    Block(Vec<Node>),
    PosInfo(String, u16, u16), // debugging
    RetExpr(Box<Node>),
    As(Box<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub expr: Expr,
    pub ty: AtomType,
    pub line: u16,
    pub start: u16,
    pub end: u16,
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.expr {
            Expr::SpecExpr { parent, spec } => {
                write!(
                    f,
                    "{parent}({})",
                    spec.iter()
                        .map(|x| x.to_string())
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Expr::IndexExpr { parent, index } => {
                write!(f, "{parent}[{index}]")
            }

            Expr::Block(items) => {
                write!(
                    f,
                    "{{\n{}}}",
                    items
                        .iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<String>>()
                        .join("\n")
                )
            }

            Expr::Discard(d) => write!(f, "discard {d}"),
            Expr::BinaryExpr { op, left, right } => write!(f, "{left} {op} {right}"),

            Expr::As(e) => write!(f, "{e} as {}", &self.ty),
            Expr::RetExpr(ret) => write!(f, "ret {ret}"),
            _ => write!(f, "{:#?}", &self.expr),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ident {
    Tagged(Box<Node>, String),
    Typed(AtomType, String),
    UnTagged(String),
}

impl Ident {
    pub fn val(&self) -> &String {
        match self {
            Ident::Tagged(_, ref val) | Ident::UnTagged(ref val) | Ident::Typed(_, ref val) => val,
        }
    }

    pub fn val_mut(&mut self) -> &mut String {
        match self {
            Ident::Tagged(_, ref mut val)
            | Ident::UnTagged(ref mut val)
            | Ident::Typed(_, ref mut val) => val,
        }
    }

    pub fn tuple(self) -> (AtomType, String) {
        match self {
            Ident::Typed(ty, val) => (ty, val),
            Ident::UnTagged(val) => (
                AtomType {
                    kind: AtomKind::Any,
                    details: None,
                },
                val,
            ),
            _ => panic!(),
        }
    }

    pub fn ty(&self) -> &AtomType {
        match self {
            Ident::Typed(ref ty, _) => ty,
            Ident::UnTagged(_) => &AtomType {
                kind: AtomKind::Any,
                details: None,
            },
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Blueprint {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub body: Vec<Node>,
    // pub line: u16,
    // pub column: u16,
    // pub width: u16,
}
