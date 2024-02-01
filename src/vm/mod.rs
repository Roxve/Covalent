use crate::ast::Literal;
use crate::source::*;

pub struct Reg(pub Literal);

pub struct VM {
    consts: Vec<Literal>,
    insturactions: Insturactions,
    ip: u16,
    regs: Vec<Reg>,
}

impl VM {
    fn new(consts: Vec<Literal>, insturactions: Insturactions) -> Self {
        VM {
            consts,
            insturactions,
            ip: 0,
            regs: Vec::new(),
        }
    }

    fn into_reg(&mut self, ip: RegIP, item: Literal) {
        while self.regs.len() <= ip {
            self.regs.push(Literal::Int(0));
        }

        self.regs[ip] = item;
    }
}
