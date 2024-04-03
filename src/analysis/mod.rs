use crate::ir::Enviroment;

pub struct Analyizer {
    env: Enviroment
}

impl Analyizer {
    pub fn new() -> Self {
        Self {
            env: Enviroment::new(None)
        }
    }
}