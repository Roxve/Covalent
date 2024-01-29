#[derive(Debug, Clone, PartialEq)]
// open file as current -> tokenize
pub enum Token {
    Operator(char),
    Number(String), // later parse number
    EOF,
}

pub struct Source {
    pub code: String,
    pub line: u32,
    pub colmun: u32,
    pub current_tok: Option<Token>,
}

impl Source {
    pub fn new(code: String) -> Source {
        Source {
            code,
            line: 1,
            colmun: 1,
            current_tok: None,
        }
    }
}
