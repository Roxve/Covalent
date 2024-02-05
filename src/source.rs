use std::collections::HashMap;

use inkwell::builder::Builder;
use inkwell::context::Context;
use inkwell::module::Module;
use inkwell::values::{FunctionValue, PointerValue};

#[derive(Debug, Clone, PartialEq)]
// open file as current -> tokenize
pub enum Token {
    Operator(String),
    Int(i32),
    Float(f32),
    Ident(String),
    Tag(String),
    Err(String), // error code and msg
    LeftParen,
    RightParen,
    LeftBracket,
    RightBracket,
    Colon,
    Comma,
    SetKw,
    EOF,
}

#[derive(Debug, Clone)]
pub enum ErrKind {
    UnknownCharE = 0,
    UnexceptedTokenE = 1,
    UndeclaredVar = 2,
    VarAlreadyDeclared = 3,
    CannotConvertRight = 4, // in binary expressions right is always coverted to left
}

#[derive(Debug, Clone)]
pub struct ATErr {
    pub kind: ErrKind,
    pub msg: String,
    pub line: u32,
    pub column: u32,
}

impl ATErr {
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
    pub fn_value: FunctionValue<'ctx>,
    pub variables: HashMap<String, PointerValue<'ctx>>,
    pub errors: Vec<ATErr>,
    pub warnings: Vec<ATErr>, // program can continue error
}

impl<'ctx> Source<'ctx> {
    pub fn new(code: String, context: &'ctx Context) -> Self {
        // todo set codegen stuff as parameters
        let module = context.create_module("temp");

        let main_fn_type = context.i32_type().fn_type(&[], false);
        let main_fn = module.add_function("main", main_fn_type, None);
        let builder = context.create_builder();
        let main = context.append_basic_block(main_fn, "entry");

        builder.position_at_end(main);
        let src = Source {
            code,
            line: 1,
            column: 0,
            current_tok: None,
            next_tok: None,
            context: &context,
            module,
            builder,
            fn_value: main_fn,
            variables: HashMap::new(),
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
