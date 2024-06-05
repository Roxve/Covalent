// compiletime errors

#[repr(u8)]
#[derive(Debug, Clone)]
pub enum ErrKind {
    UnknownCharE,
    UnexceptedTokenE,
    InvaildType,
    UndeclaredVar,
    VarAlreadyDeclared,
    OperationNotGranted,
    UnexceptedArgs,
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
