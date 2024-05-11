use crate::source::{ConstType, Ident};
#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),
}

impl Literal {
    pub fn get_ty(&self) -> ConstType {
        match self {
            &Self::Int(_) => ConstType::Int,
            &Self::Float(_) => ConstType::Float,
            &Self::Str(_) => ConstType::Str,
            &Self::Bool(_) => ConstType::Bool,
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
    Literal(Literal),
    ListExpr(Vec<Node>),
    BinaryExpr {
        op: String,
        left: Box<Node>,
        right: Box<Node>,
    },
    Ident(Ident),
    Id(String),
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
        args: Vec<ConstType>,
    },

    Func {
        ret: ConstType,
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
    pub ty: ConstType,
}

impl Node {
    pub fn is_typed(&self) -> bool {
        self.ty != ConstType::Unknown
    }
}

pub fn untyped(expr: Expr) -> Node {
    Node {
        expr,
        ty: ConstType::Unknown,
    }
}
