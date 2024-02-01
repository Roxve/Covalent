#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
}

pub fn get_operator_level(op: char) -> u8 {
    match op {
        '+' | '-' => 1,
        '*' | '/' => 2,
        _ => todo!(),
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Expr {
    Literal(Literal),
    BinaryExpr(char, Box<Expr>, Box<Expr>),
}
