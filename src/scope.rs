// scope is used for better errors and parsing context (not much rn expect)

#[derive(Debug, Clone, PartialEq)]
pub enum Scope {
    Value,
    _Func(String),
    Top,
    Use,
}

impl Scope {
    pub fn is_used(&self) -> bool {
        self == &Scope::Value || self == &Scope::Use
    }
}
