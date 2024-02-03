use inkwell::builder::Builder;
use inkwell::context::{AsContextRef, Context, ContextRef};
use inkwell::module::Module;

#[derive(Debug, Clone, PartialEq)]
// open file as current -> tokenize
pub enum Token {
    Operator(String),
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
            "code:AT00{}\n{}\nat line:{}, column:{}",
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

// #[derive(Debug, Clone)]
//todo remove VM after replacing with LLVM
#[derive(Debug)]
pub struct Source<'ctx> {
    pub code: String,
    pub line: u32,
    pub column: u32,
    pub current_tok: Option<Token>,
    pub next_tok: Option<Token>,
    pub context: &'ctx Context,
    pub module: Module<'ctx>,
    pub builder: Builder<'ctx>,
    pub errors: Vec<ATErr>,
    pub warnings: Vec<ATErr>, // program can continue error
}

impl<'ctx> Source<'ctx> {
    pub fn new(code: String, context: &'ctx Context) -> Self {
        // todo set codegen stuff as parameters
        let module = context.create_module("temp");

        let src = Source {
            code,
            line: 1,
            column: 0,
            current_tok: None,
            next_tok: None,
            context: &context,
            module: module.to_owned(),
            builder: context.create_builder(),
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        return src;
    }

    pub fn err(&mut self, kind: ErrKind, msg: String) {
        let err = ATErr {
            kind,
            msg,
            line: self.line,
            column: self.column,
        };
        self.errors.push(err.clone());
        err.out_error();
    }
}
