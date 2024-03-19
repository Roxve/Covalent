use super::Codegen;
use super::Item;
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
        _args: Vec<(ConstType, String)>,
        _ret: ConstType,
        body: Vec<IROp>,
    ) -> Vec<String> {
        let mut lines = Vec::new();
        lines.push(format!("int {}() {{", name));
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
            IROp::Const(_, con) => self.push(Item::Const(con)),
            IROp::Import(_, _, _, _) => return None,
            _ => return self.bond_binary(op), // attempt to bond binary expr instead
        }
        None
    }

    pub fn bond_binary(&mut self, op: IROp) -> Option<String> {
        match op {
            IROp::Add(_) => Some(format!("{} {} {}", self.pop_str(), "+", self.pop_str())),
            _ => todo!("unimplented op {:#?}", op),
        }
    }
}
