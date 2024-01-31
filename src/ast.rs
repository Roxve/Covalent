#[derive(Debug, Clone, PartialEq)]
pub enum Literal {
    Int(i32),
    Float(f32),
}
// 33 -> token int -> literal int? literal is a connection with expr and token

// operator -> level
// #[derive(Debug, Clone, PartialEq)]
// pub enum Operator {
//     Plus,
//     Minus,
//     Multi,
//     Divide,
// }
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
