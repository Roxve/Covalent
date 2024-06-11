use core::panic;

use crate::types::AtomKind;
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
}

impl Literal {
    pub fn get_ty(&self) -> AtomKind {
        match self {
            &Self::Int(_) => AtomKind::Int,
            &Self::Float(_) => AtomKind::Float,
            &Self::Str(_) => AtomKind::Str,
            &Self::Bool(_) => AtomKind::Bool,
        }
    }
}

pub fn get_operator_level(op: &str) -> u8 {
    match op {
        "&&" | "||" => 1,
        "==" => 2,
        "<" | ">" | ">=" | "<=" => 3,
        "+" | "-" => 4,
        "*" | "/" | "%" => 5,
        _ => todo!(),
    }
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
        args: Vec<AtomKind>,
    },

    Func {
        ret: AtomKind,
        name: String,
        args: Vec<Ident>,
        body: Vec<Node>,
    },

    IfExpr {
        condition: Box<Node>,
        body: Vec<Node>,
        alt: Option<Box<Node>>,
    },

    WhileExpr {
        condition: Box<Node>,
        body: Vec<Node>,
    },

    MemberExpr {
        parent: Box<Node>,
        child: String,
    },

    IndexExpr {
        parent: Box<Node>,
        index: Box<Node>,
    },
    Discard(Box<Node>),
    Block(Vec<Node>),
    PosInfo(String, u32, u32), // debugging
    RetExpr(Box<Node>),
    As(Box<Node>),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub expr: Expr,
    pub ty: AtomKind,
}

impl Node {
    pub fn is_typed(&self) -> bool {
        if let &AtomKind::Unknown(None) = &self.ty {
            false
        } else {
            true
        }
    }
}

pub fn untyped(expr: Expr) -> Node {
    Node {
        expr,
        ty: AtomKind::Unknown(None),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Ident {
    Tagged(String, String),
    Typed(AtomKind, String),
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

    pub fn tuple(self) -> (AtomKind, String) {
        match self {
            Ident::Typed(ty, val) => (ty, val),
            Ident::UnTagged(val) => (AtomKind::Dynamic, val),
            _ => panic!(),
        }
    }

    pub fn ty(&self) -> &AtomKind {
        match self {
            Ident::Typed(ref ty, _) => ty,
            Ident::UnTagged(_) => &AtomKind::Dynamic,
            _ => panic!(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Blueprint {
    pub name: Ident,
    pub args: Vec<Ident>,
    pub body: Vec<Node>,
}
