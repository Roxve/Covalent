use std::collections::HashMap;

use crate::parser::ast::{Blueprint, Literal};
use crate::types::{AtomType, FunctionType};

#[derive(Debug, Clone, PartialEq)]
pub struct Symbol {
    pub name: String,
    pub ty: AtomType,

    pub refers_to_atom: bool,
    pub value: Option<Literal>,
}

#[derive(Clone, Debug)]
pub struct Enviroment {
    pub symbols: HashMap<String, Symbol>,
    pub expects: HashMap<String, AtomType>,
    pub parent: Option<Box<Enviroment>>,
    pub blueprints: Vec<Blueprint>,
}

impl Enviroment {
    pub fn new(parent: Option<Box<Self>>) -> Self {
        Self {
            symbols: HashMap::new(),
            expects: HashMap::new(),
            parent,
            blueprints: Vec::new(),
        }
    }

    pub fn child(&mut self) {
        *self = Enviroment::new(Some(Box::new(self.clone())));
    }

    pub fn parent(&mut self) {
        *self = *self.parent.as_ref().unwrap().clone();
    }

    pub fn get_ty(&self, name: &String) -> Option<AtomType> {
        let sym = self.get(name);

        if sym.is_some() {
            Some(sym.unwrap().ty.clone())
        } else {
            None
        }
    }

    pub fn get(&self, name: &String) -> Option<&Symbol> {
        if self.symbols.contains_key(name) {
            return Some(&self.symbols[name]);
        }

        if self.parent.is_some() {
            self.parent.as_ref().unwrap().get(name)
        } else {
            None
        }
    }

    pub fn has(&self, name: &String) -> bool {
        if self.symbols.contains_key(name) {
            true
        } else if self.parent.is_some() {
            self.parent.as_ref().unwrap().has(name)
        } else {
            false
        }
    }

    //TODO: REMOVE
    // member expr parent is passed to a function as first arg if it takes it as an arg for ex.
    // set push: List(T) self, T item -> List(T)
    pub fn ty_parent_fn(&self, ty: &AtomType, name: &String) -> Option<AtomType> {
        let parent = self.symbols.get(name);

        if parent.is_some() {
            if let &AtomType::Function(ref f) = &parent.unwrap().ty {
                if &f.params[0] == ty {
                    return Some(parent.unwrap().to_owned().ty);
                }
            }
        }
        None
    }

    pub fn modify_ty(&mut self, name: &String, ty: AtomType) {
        if self.symbols.contains_key(name) {
            self.symbols.get_mut(name).map(|val| val.ty = ty);
        } else if self.parent.is_some() {
            self.parent.as_mut().unwrap().modify_ty(name, ty);
        }
    }

    pub fn add(&mut self, sym: Symbol) {
        let name = sym.name.clone();
        if self.symbols.contains_key(&name) {
            self.modify_ty(&name, sym.ty)
        } else {
            self.symbols.insert(name, sym);
        }
    }

    pub fn get_blueprint(&self, name: &String) -> Option<Blueprint> {
        for blueprint in &self.blueprints {
            if blueprint.name.val() == name {
                return Some(blueprint.clone());
            }
        }

        if self.parent.is_some() {
            return self.parent.as_ref().unwrap().get_blueprint(name);
        }
        return None;
    }

    pub fn top(&mut self) -> &mut Enviroment {
        if self.parent.is_none() {
            self
        } else {
            self.parent.as_mut().unwrap().top()
        }
    } // returns the top level enviroment

    pub fn push_function(&mut self, name: String, func: FunctionType) {
        self.top().add(Symbol {
            name,
            ty: AtomType::Function(func),
            refers_to_atom: false,
            value: None,
        });
    }

    pub fn expect(&mut self, name: &String, ty: AtomType) {
        self.expects.insert(name.clone(), ty);
    }

    pub fn get_expected(&mut self, name: &String) -> &AtomType {
        let expect = self.expects.get(name);

        if expect.is_none() {
            return self.parent.as_mut().unwrap().get_expected(name);
        }

        expect.unwrap()
    }

    pub fn is_expected(&mut self, name: &String, ty: &AtomType) -> bool {
        if self.expects.get(name).is_some_and(|x| x == ty)
            || self.expects.get(name).is_some_and(|x| x == &AtomType::Any)
        {
            return true;
        } else if self.expects.get(name).is_some() {
            return false;
        }

        if self.parent.is_some() {
            return self.parent.as_mut().unwrap().is_expected(name, ty);
        }

        true
    }
}
