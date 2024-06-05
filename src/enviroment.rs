use std::collections::HashMap;

use crate::parser::ast::Blueprint;
use crate::types::AtomKind;

#[derive(Clone, Debug)]
pub struct Enviroment {
    pub vars: HashMap<String, AtomKind>,
    pub current: AtomKind,
    pub parent: Option<Box<Enviroment>>,
    pub blueprints: Vec<Blueprint>,
}

impl Enviroment {
    pub fn new(parent: Option<Box<Self>>) -> Self {
        Self {
            vars: HashMap::new(),
            current: AtomKind::Void,
            parent,
            blueprints: Vec::new(),
        }
    }

    pub fn child(&self) -> Enviroment {
        Enviroment::new(Some(Box::new(self.clone())))
    }

    pub fn parent(&self) -> Option<Enviroment> {
        if self.parent.is_none() {
            None
        } else {
            Some(*(self.parent.clone().unwrap()))
        }
    }

    pub fn get_ty(&self, name: &String) -> Option<AtomKind> {
        if self.vars.contains_key(name) {
            return Some(self.vars[name].clone());
        }

        if self.parent.is_some() {
            return self.parent().unwrap().get_ty(name);
        } else {
            return None;
        }
    }

    pub fn has(&self, name: &String) -> bool {
        if self.vars.contains_key(name) {
            true
        } else if self.parent.is_some() {
            self.parent().unwrap().has(name)
        } else {
            false
        }
    }

    // member expr parent is passed to a function as first arg if it takes it as an arg for ex.
    // set push: List(T) self, T item -> List(T)
    pub fn ty_parent_fn(&self, ty: &AtomKind, name: &String) -> Option<AtomKind> {
        let parent = self.vars.get(name);
        if parent.is_some() {
            if let AtomKind::Func(_, args, _) = parent.unwrap() {
                if &args[0] == ty {
                    return Some(parent.unwrap().to_owned());
                }
            }
        }
        None
    }

    pub fn modify(&mut self, name: &String, ty: AtomKind) {
        if self.vars.contains_key(name) {
            self.vars.get_mut(name).map(|val| *val = ty);
        } else if self.parent.is_some() {
            self.parent().unwrap().modify(name, ty);
        }
    }

    pub fn add(&mut self, name: &String, ty: AtomKind) {
        self.vars.insert(name.clone(), ty);
    }

    pub fn get_blueprint(&self, name: &String) -> Option<Blueprint> {
        for blueprint in &self.blueprints {
            if blueprint.name.val() == name {
                return Some(blueprint.clone());
            }
        }
        if self.parent.is_some() {
            for blueprint in &self.parent().unwrap().blueprints {
                if blueprint.name.val() == name {
                    return Some(blueprint.clone());
                }
            }
        }
        return None;
    }

    pub fn push_function(&mut self, name: String, args: Vec<AtomKind>, ty: AtomKind) {
        self.vars
            .insert(name.clone(), AtomKind::Func(Box::new(ty), args, name));
    }
}
