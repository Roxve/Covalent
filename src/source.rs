use crate::ast::*;

// 32 bit max
// type 1: op(8b), dest(8b), reg(8b)
// ADD R0, R1

type RegIP = u8;
#[derive(Debug, Clone)]
pub enum Op {
    Add(RegIP, RegIP),
    Sub(RegIP, RegIP),
    Mul(RegIP, RegIP),
    Div(RegIP, RegIP), // we dont talk about div or floats
    Load(RegIP, u16),
}

type Insturactions = Vec<Op>;

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
pub enum ErrKind {
    UnknownCharE = 0,
    UnexceptedTokenE = 1,
}

#[derive(Debug, Clone)]
pub struct ATErr {
    pub kind: ErrKind,
    pub msg: String,
    pub line: u32,
    pub column: u32,
}

impl ATErr {
    pub fn new(kind: ErrKind, msg: String, line: u32, column: u32) -> Self {
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
            self.kind.clone() as u8,
            self.msg,
            self.line,
            self.column
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
    pub current_reg: RegIP,
    pub consts: Vec<Literal>,
    pub codegen: Insturactions,
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
            current_reg: 0,
            consts: Vec::new(),
            codegen: Vec::new(),
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    pub fn push_const(&mut self, pconst: Literal) -> u16 {
        let mut found = false;
        let mut ip = 0;

        for constant in self.consts.clone() {
            if constant == pconst {
                found = true;
            }
            ip += 1;
        }

        if !found {
            self.consts.push(pconst);
        }

        return ip;
    }

    pub fn push_instr(&mut self, instr: Op) {
        self.codegen.push(instr);
    }

    pub fn err(&mut self, kind: ErrKind, msg: String) {
        let err = ATErr::new(kind, msg, self.line, self.column);
        self.errors.push(err.clone());
        err.out_error();
    }
}
