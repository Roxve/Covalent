#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
}

#[derive(Debug, Clone, PartialEq)]
pub struct Ident(pub String);

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
    VarDeclare(Ident, Box<Expr>),
    VarAssign(Ident, Box<Expr>),
}
