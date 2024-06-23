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
    pub line: u16,
    pub column: u16,
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

#[macro_export]
macro_rules! err {
    ($self: ident, $kind: path, $msg: literal) => {
        ATErr {
            kind: $kind.clone(),
            msg: $msg.to_string(),
            line: $self.line,
            column: $self.column,
        }
        .out_error();

        return Err($kind);
    };

    ($self: ident, $kind: path, $msg: expr) => {
        ATErr {
            kind: $kind.clone(),
            msg: $msg,
            line: $self.line,
            column: $self.column,
        }
        .out_error();

        return Err($kind);
    };
}
