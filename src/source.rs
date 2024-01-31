#[derive(Debug, Clone, PartialEq)]
// open file as current -> tokenize
pub enum Token {
    Operator(char),
    Int(i32),
    Float(f32),
    Err(String), // error code and msg
    EOF,
}

pub struct Source {
    pub code: String,
    pub line: u32,
    pub colmun: u32,
    pub current_tok: Option<Token>,
    pub next_tok: Option<Token>,
}

impl Source {
    pub fn new(code: String) -> Source {
        Source {
            code,
            line: 1,
            colmun: 0,
            current_tok: None,
            next_tok: None,
        }
    }
}
