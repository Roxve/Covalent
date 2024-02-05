#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident(pub String);

#[derive(Debug, Clone, PartialEq)]
pub struct Tag(pub String);

pub fn get_operator_level(op: &str) -> u8 {
    match op {
        "+" | "-" => 1,
        "*" | "/" => 2,
        _ => todo!(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    BinaryExpr(String, Box<Expr>, Box<Expr>),
    Ident(Ident),
    TaggedIdent(Tag, Ident),
    VarDeclare(Ident, Box<Expr>),
    VarAssign(Ident, Box<Expr>),
    FnDeclare(
        /* id */ Ident,
        /* args */ Vec<Expr>,
        /* body */ Vec<Expr>,
    ),
    FnCall(/* id */ Ident, /* args */ Vec<Expr>),
}
