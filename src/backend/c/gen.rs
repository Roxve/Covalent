use super::TypeToC;

use super::Codegen;
use super::Item;
use super::TypesToCNamed;
use crate::ir::{ConstType, IROp};

impl Codegen {
    pub fn codegen(&mut self, ir: Vec<IROp>) -> String {
        let main = self.bond_fn("main".to_string(), Vec::new(), ConstType::Int, ir);
        self.module.func(main);
        self.module.finish()
    }
    pub fn bond_fn(
        &mut self,
        name: String,
        args: Vec<(ConstType, String)>,
        ret: ConstType,
        body: Vec<IROp>,
    ) -> Vec<String> {
        let mut lines = Vec::new();
        let ty = TypeToC(ret);
        let args = TypesToCNamed(args);

        lines.push(format!("{} {}({}) {{", ty, name, args));
        for op in body {
            let line = self.bond(op);
            if line.is_some() {
                lines.push(line.unwrap() + ";");
            }
        }

        lines.push("}".to_string());
        lines
    }
    pub fn bond(&mut self, op: IROp) -> Option<String> {
        match op {
            IROp::Alloc(_, _) => return None,
            IROp::Dealloc(_, _) => return None,
            IROp::Const(_, con) => self.push(Item::Const(con)),
            IROp::Store(ty, name) => {
                let val = self.pop_str();
                let ty = TypeToC(ty);
                let name = self.var(name);

                return Some(format!("{} {} = {}", ty, name, val));
            }
            IROp::Import(_, _, _, _) => return None,
            _ => return self.bond_binary(op), // attempt to bond binary expr instead
        }
        None
    }

    pub fn bond_binary(&mut self, op: IROp) -> Option<String> {
        let item = match op {
            IROp::Add(_) => Item::Expr(format!("{} {} {}", self.pop_str(), "+", self.pop_str())),
            _ => todo!("unimplented op {:#?}", op),
        };
        self.push(item);

        None
    }
}
