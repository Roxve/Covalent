#[derive(Debug, Clone, PartialEq)]
// open file as current -> tokenize
pub enum Token {
    Operator(char),
    Int(i32),
    Float(f32),
    Err(String), // error code and msg
    EOF,
}

#[derive(Debug, Clone)]
pub enum ATErrKind {
    UnknownCharE,
    UnexceptedTokenE,
}

#[derive(Debug, Clone)]
pub struct ATErr {
    pub kind: ATErrKind,
    pub msg: String,
    pub line: u32,
    pub column: u32,
}

impl ATErr {
    pub fn new(kind: ATErrKind, msg: String, line: u32, column: u32) -> Self {
        ATErr {
            kind,
            msg,
            line,
            column,
        }
    }

    pub fn get_error(&self) -> String {
        format!(
            "code:AT{}\n{}\nat line:{}, column:{}",
            self.kind as u8, self.msg, self.line, self.column
        )
    }

    // customize later
    pub fn out_error(&self) {
        println!("{}", self.get_error());
    }
}

pub struct Source {
    pub code: String,
    pub line: u32,
    pub column: u32,
    pub current_tok: Option<Token>,
    pub next_tok: Option<Token>,
    pub errors: Vec<ATErr>,
    pub warnings: Vec<ATErr>, // program can continue error
}

impl Source {
    pub fn new(code: String) -> Source {
        Source {
            code,
            line: 1,
            column: 0,
            current_tok: None,
            next_tok: None,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }
}
