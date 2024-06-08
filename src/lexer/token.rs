#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Operator(String),
    // convert these into literal
    Int(i32),
    Float(f32),
    Str(String),
    Bool(bool),

    Ident(String),
    Tag(String),
    Err(String), // error code and msg
    LeftParen,
    RightParen,
    LeftBrace,
    RightBrace,
    LeftBracket,
    RightBracket,
    Exec,
    Colon,
    Comma,
    Dot,
    Access,
    IfKw,
    ElseKw,
    WhileKw,
    BreakKw,
    Continuekw,
    SetKw,
    RetKw,
    UseKw,
    EOF,
}
